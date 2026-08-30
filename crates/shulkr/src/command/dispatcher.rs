use std::sync::Arc;

use parking_lot::RwLock;

use crate::command::{
    builder::LiteralArgumentBuilder,
    context::CommandContextBuilder,
    error::{self, Error},
    string_reader::StringReader,
    suggestion::{StringRange, Suggestions, SuggestionsBuilder},
    tree::CommandNode,
};

pub const ARGUMENT_SEPARATOR: char = ' ';

pub struct ParseResults<S> {
    context: CommandContextBuilder<S>,
    reader: StringReader,
    errors: Vec<(Arc<CommandNode<S>>, Error)>,
}

impl<S> ParseResults<S> {
    pub fn reader(&self) -> &StringReader {
        &self.reader
    }

    pub fn errors(&self) -> impl Iterator<Item = &Error> {
        self.errors.iter().map(|(_, e)| e)
    }
}

pub struct CommandDispatcher<S> {
    root: RwLock<Arc<CommandNode<S>>>,
}

impl<S> Default for CommandDispatcher<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> CommandDispatcher<S> {
    pub fn new() -> Self {
        Self {
            root: RwLock::new(Arc::new(CommandNode::root())),
        }
    }

    pub fn root(&self) -> Arc<CommandNode<S>> {
        self.root.read().clone()
    }

    pub fn register(&self, command: LiteralArgumentBuilder<S>) -> Arc<CommandNode<S>> {
        let node = Arc::new(command.build());

        let mut root = self.root.write();
        let mut children: Vec<Arc<CommandNode<S>>> = root
            .children
            .iter()
            .filter(|child| child.name() != node.name())
            .cloned()
            .collect();
        children.push(node.clone());

        let mut new_root = CommandNode::root();
        new_root.children = children;
        *root = Arc::new(new_root);

        node
    }

    pub fn register_all(&self, commands: impl IntoIterator<Item = LiteralArgumentBuilder<S>>) {
        for command in commands {
            self.register(command);
        }
    }

    pub fn unregister(&self, name: &str) {
        let mut root = self.root.write();
        let mut new_root = CommandNode::root();
        new_root.children = root
            .children
            .iter()
            .filter(|child| child.name() != name)
            .cloned()
            .collect();
        *root = Arc::new(new_root);
    }
}

impl<S: Clone> CommandDispatcher<S> {
    pub fn parse(&self, input: impl Into<String>, source: S) -> ParseResults<S> {
        self.parse_reader(StringReader::new(input), source)
    }

    pub fn parse_reader(&self, reader: StringReader, source: S) -> ParseResults<S> {
        let root = self.root();
        let context = CommandContextBuilder::new(root.clone(), source, reader.cursor());
        parse_nodes(&root, reader, context)
    }

    pub fn execute(&self, input: impl Into<String>, source: S) -> Result<i32, Error> {
        let input = input.into();
        let parse = self.parse(input, source);
        self.execute_parsed(parse)
    }

    pub fn execute_parsed(&self, parse: ParseResults<S>) -> Result<i32, Error> {
        if parse.reader.can_read() {
            return Err(unparsed_error(&parse));
        }

        let input = parse.reader.string().to_string();
        let original = parse.context.build(&input);

        let mut result = 0;
        let mut successful_forks = 0;
        let mut forked = false;
        let mut found_command = false;

        let mut contexts = vec![original];
        while !contexts.is_empty() {
            let mut next = Vec::new();

            for context in &contexts {
                let Some(child) = context.child() else {
                    if let Some(command) = context.command() {
                        found_command = true;
                        match command(context) {
                            Ok(value) => {
                                result += value;
                                successful_forks += 1;
                            }
                            Err(error) if forked => {
                                log::debug!("forked command failed for one source: {error}");
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    continue;
                };

                forked |= context.is_forked();

                if !child.has_nodes() {
                    continue;
                }

                match context.redirect_modifier() {
                    None => next.push(child.copy_for(context.source().clone())),
                    Some(modifier) => match modifier(context) {
                        Ok(sources) => {
                            next.extend(sources.into_iter().map(|source| child.copy_for(source)));
                        }
                        Err(error) if forked => {
                            log::debug!("forked redirect modifier failed: {error}");
                        }
                        Err(error) => return Err(error),
                    },
                }
            }

            contexts = next;
        }

        if !found_command {
            return Err(error::DISPATCHER_UNKNOWN_COMMAND
                .create([])
                .with_context(&input, parse.reader.cursor()));
        }

        Ok(if forked { successful_forks } else { result })
    }

    pub fn completion_suggestions(
        &self,
        input: impl Into<String>,
        cursor: usize,
        source: S,
    ) -> Suggestions {
        let parse = self.parse(input, source);
        self.completion_suggestions_for(&parse, cursor)
    }

    pub fn completion_suggestions_for(
        &self,
        parse: &ParseResults<S>,
        cursor: usize,
    ) -> Suggestions {
        let full_input = parse.reader.string();
        let cursor = cursor.min(full_input.len());
        let Some(suggestion_context) = parse.context.find_suggestion_context(cursor) else {
            return Suggestions::empty();
        };

        let start = suggestion_context.start_pos.min(cursor);
        let truncated = &full_input[..cursor];
        let context = parse.context.build(truncated);

        let gathered: Vec<Suggestions> = suggestion_context
            .parent
            .children()
            .iter()
            .filter(|child| child.can_use(context.source()))
            .map(|child| {
                child.list_suggestions(&context, SuggestionsBuilder::new(truncated, start))
            })
            .filter(|suggestions| !suggestions.is_empty())
            .collect();

        Suggestions::merge(full_input, gathered)
    }
}

fn unparsed_error<S>(parse: &ParseResults<S>) -> Error {
    let input = parse.reader.string();
    let cursor = parse.reader.cursor();

    if parse.errors.len() == 1 {
        return parse.errors[0].1.clone();
    }

    if parse.context.range().is_empty() {
        error::DISPATCHER_UNKNOWN_COMMAND
            .create([])
            .with_context(input, cursor)
    } else {
        error::DISPATCHER_UNKNOWN_ARGUMENT
            .create([])
            .with_context(input, cursor)
    }
}

fn parse_nodes<S: Clone>(
    node: &Arc<CommandNode<S>>,
    original_reader: StringReader,
    context_so_far: CommandContextBuilder<S>,
) -> ParseResults<S> {
    let source = context_so_far.source().clone();
    let mut errors: Vec<(Arc<CommandNode<S>>, Error)> = Vec::new();
    let mut potentials: Vec<ParseResults<S>> = Vec::new();
    let cursor = original_reader.cursor();

    for child in node.relevant_nodes(&original_reader) {
        if !child.can_use(&source) {
            continue;
        }

        let mut context = context_so_far.clone();
        let mut reader = original_reader.clone();

        match child.parse(&mut reader, &mut context) {
            Ok(()) => {
                if reader.can_read() && reader.peek() != Some(ARGUMENT_SEPARATOR) {
                    errors.push((
                        child.clone(),
                        error::DISPATCHER_EXPECTED_ARGUMENT_SEPARATOR
                            .create([])
                            .with_context(reader.string(), reader.cursor()),
                    ));
                    continue;
                }
            }
            Err(error) => {
                errors.push((child.clone(), error));
                continue;
            }
        }

        context.with_command(child.command().cloned());
        context.with_node(child.clone(), StringRange::between(cursor, reader.cursor()));

        let lookahead = if child.redirect().is_some() { 1 } else { 2 };
        if reader.can_read_length(lookahead) {
            reader.skip();

            if let Some(redirect) = child.redirect() {
                let child_context =
                    CommandContextBuilder::new(redirect.clone(), source.clone(), reader.cursor());
                let parse = parse_nodes(redirect, reader, child_context);
                context.with_child(parse.context);
                return ParseResults {
                    context,
                    reader: parse.reader,
                    errors: parse.errors,
                };
            }

            potentials.push(parse_nodes(&child, reader, context));
        } else {
            potentials.push(ParseResults {
                context,
                reader,
                errors: Vec::new(),
            });
        }
    }

    if potentials.is_empty() {
        return ParseResults {
            context: context_so_far,
            reader: original_reader,
            errors,
        };
    }

    if potentials.len() > 1 {
        potentials.sort_by(|a, b| {
            let key = |p: &ParseResults<S>| (p.reader.can_read(), !p.errors.is_empty());
            key(a).cmp(&key(b))
        });
    }

    potentials.into_iter().next().unwrap()
}
