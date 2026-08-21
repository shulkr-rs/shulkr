use std::{any::Any, sync::Arc};

use crate::{
    command::{
        arguments::Arg,
        exceptions::CommandSyntaxException,
        suggestion::StringRange,
        tree::{Command, CommandNode, RedirectModifier},
    },
    util::HashMap,
};

#[derive(Debug, Clone)]
pub struct ParsedArgument {
    range: StringRange,
    result: Arc<dyn Any + Send + Sync>,
}

impl ParsedArgument {
    pub(crate) fn new(range: StringRange, result: Arc<dyn Any + Send + Sync>) -> Self {
        Self { range, result }
    }

    pub fn range(&self) -> StringRange {
        self.range
    }
}

pub struct ParsedCommandNode<S> {
    pub node: Arc<CommandNode<S>>,
    pub range: StringRange,
}

impl<S> Clone for ParsedCommandNode<S> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            range: self.range,
        }
    }
}

pub struct CommandContext<S> {
    source: S,
    input: String,
    arguments: HashMap<String, ParsedArgument>,
    command: Option<Command<S>>,
    root_node: Arc<CommandNode<S>>,
    nodes: Vec<ParsedCommandNode<S>>,
    range: StringRange,
    child: Option<Box<CommandContext<S>>>,
    modifier: Option<RedirectModifier<S>>,
    forks: bool,
}

impl<S: Clone> Clone for CommandContext<S> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            input: self.input.clone(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range,
            child: self.child.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S> CommandContext<S> {
    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn range(&self) -> StringRange {
        self.range
    }

    pub fn nodes(&self) -> &[ParsedCommandNode<S>] {
        &self.nodes
    }

    pub fn root_node(&self) -> &Arc<CommandNode<S>> {
        &self.root_node
    }

    pub fn command(&self) -> Option<&Command<S>> {
        self.command.as_ref()
    }

    pub fn child(&self) -> Option<&CommandContext<S>> {
        self.child.as_deref()
    }

    pub fn last_child(&self) -> &CommandContext<S> {
        let mut context = self;
        while let Some(child) = context.child.as_deref() {
            context = child;
        }
        context
    }

    pub fn redirect_modifier(&self) -> Option<&RedirectModifier<S>> {
        self.modifier.as_ref()
    }

    pub fn is_forked(&self) -> bool {
        self.forks
    }

    pub fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }

    pub fn has(&self, name: &str) -> bool {
        self.arguments.contains_key(name)
    }

    pub fn get<T: Arg>(&self, name: &str) -> Result<T::Value, CommandSyntaxException> {
        let parsed = self.arguments.get(name).ok_or_else(|| {
            CommandSyntaxException::custom(format!(
                "No such argument '{name}' exists on this command"
            ))
        })?;

        parsed
            .result
            .downcast_ref::<T::Value>()
            .cloned()
            .ok_or_else(|| {
                CommandSyntaxException::custom(format!(
                    "Argument '{name}' is not of the requested type"
                ))
            })
    }

    pub fn get_raw(&self, name: &str) -> Option<&str> {
        let parsed = self.arguments.get(name)?;
        Some(parsed.range.get(&self.input))
    }

    pub(crate) fn copy_for(&self, source: S) -> Self
    where
        S: Clone,
    {
        let mut copy = self.clone();
        copy.source = source;
        copy
    }
}

pub struct CommandContextBuilder<S> {
    arguments: HashMap<String, ParsedArgument>,
    root_node: Arc<CommandNode<S>>,
    nodes: Vec<ParsedCommandNode<S>>,
    source: S,
    command: Option<Command<S>>,
    child: Option<Box<CommandContextBuilder<S>>>,
    range: StringRange,
    modifier: Option<RedirectModifier<S>>,
    forks: bool,
}

impl<S: Clone> Clone for CommandContextBuilder<S> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            source: self.source.clone(),
            command: self.command.clone(),
            child: self.child.clone(),
            range: self.range,
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S: Clone> CommandContextBuilder<S> {
    pub(crate) fn new(root_node: Arc<CommandNode<S>>, source: S, start: usize) -> Self {
        Self {
            arguments: HashMap::default(),
            root_node,
            nodes: Vec::new(),
            source,
            command: None,
            child: None,
            range: StringRange::at(start),
            modifier: None,
            forks: false,
        }
    }

    pub(crate) fn build(&self, input: &str) -> CommandContext<S> {
        CommandContext {
            source: self.source.clone(),
            input: input.to_string(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            root_node: self.root_node.clone(),
            nodes: self.nodes.clone(),
            range: self.range,
            child: self.child.as_ref().map(|c| Box::new(c.build(input))),
            modifier: self.modifier.clone(),
            forks: self.forks,
        }
    }
}

impl<S> CommandContextBuilder<S> {
    pub(crate) fn source(&self) -> &S {
        &self.source
    }

    pub(crate) fn range(&self) -> StringRange {
        self.range
    }

    pub(crate) fn with_argument(&mut self, name: String, argument: ParsedArgument) {
        self.arguments.insert(name, argument);
    }

    pub(crate) fn with_command(&mut self, command: Option<Command<S>>) {
        self.command = command;
    }

    pub(crate) fn with_node(&mut self, node: Arc<CommandNode<S>>, range: StringRange) {
        self.range = StringRange::encompassing(self.range, range);
        self.modifier = node.modifier().cloned();
        self.forks = node.is_fork();
        self.nodes.push(ParsedCommandNode { node, range });
    }

    pub(crate) fn with_child(&mut self, child: CommandContextBuilder<S>) {
        self.child = Some(Box::new(child));
    }

    pub(crate) fn find_suggestion_context(&self, cursor: usize) -> Option<SuggestionContext<S>> {
        if self.range.start() > cursor {
            return None;
        }

        if self.range.end() < cursor {
            if let Some(child) = self.child.as_deref() {
                return child.find_suggestion_context(cursor);
            }
            return Some(match self.nodes.last() {
                Some(last) => SuggestionContext {
                    parent: last.node.clone(),
                    start_pos: last.range.end() + 1,
                },
                None => SuggestionContext {
                    parent: self.root_node.clone(),
                    start_pos: self.range.start(),
                },
            });
        }

        let mut previous = self.root_node.clone();
        for parsed in &self.nodes {
            if parsed.range.start() <= cursor && cursor <= parsed.range.end() {
                return Some(SuggestionContext {
                    parent: previous,
                    start_pos: parsed.range.start(),
                });
            }
            previous = parsed.node.clone();
        }

        Some(SuggestionContext {
            parent: previous,
            start_pos: self.range.start(),
        })
    }
}

pub(crate) struct SuggestionContext<S> {
    pub(crate) parent: Arc<CommandNode<S>>,
    pub(crate) start_pos: usize,
}
