use std::{fmt, sync::Arc};

use crate::command::{
    arguments::AnyArg,
    context::{CommandContext, CommandContextBuilder, ParsedArgument},
    error::{self, Error},
    string_reader::StringReader,
    suggestion::{StringRange, Suggestions, SuggestionsBuilder},
};

pub type Command<S> = Arc<dyn Fn(&CommandContext<S>) -> Result<i32, Error> + Send + Sync>;

pub type Requirement<S> = Arc<dyn Fn(&S) -> bool + Send + Sync>;

pub type RedirectModifier<S> =
    Arc<dyn Fn(&CommandContext<S>) -> Result<Vec<S>, Error> + Send + Sync>;

pub type SuggestionProvider<S> =
    Arc<dyn Fn(&CommandContext<S>, SuggestionsBuilder) -> Suggestions + Send + Sync>;

pub enum NodeKind<S> {
    Root,
    Literal {
        literal: String,
    },
    Argument {
        name: String,
        argument_type: Arc<dyn AnyArg>,
        custom_suggestions: Option<SuggestionProvider<S>>,
    },
}

pub struct CommandNode<S> {
    pub(crate) kind: NodeKind<S>,
    pub(crate) children: Vec<Arc<CommandNode<S>>>,
    pub(crate) command: Option<Command<S>>,
    pub(crate) requirement: Option<Requirement<S>>,
    pub(crate) redirect: Option<Arc<CommandNode<S>>>,
    pub(crate) modifier: Option<RedirectModifier<S>>,
    pub(crate) forks: bool,
}

impl<S> fmt::Debug for CommandNode<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandNode")
            .field("name", &self.name())
            .field("children", &self.children.len())
            .field("executable", &self.command.is_some())
            .field("redirect", &self.redirect.is_some())
            .finish()
    }
}

impl<S> CommandNode<S> {
    pub(crate) fn root() -> Self {
        Self {
            kind: NodeKind::Root,
            children: Vec::new(),
            command: None,
            requirement: None,
            redirect: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn name(&self) -> &str {
        match &self.kind {
            NodeKind::Root => "",
            NodeKind::Literal { literal } => literal,
            NodeKind::Argument { name, .. } => name,
        }
    }

    pub fn usage_text(&self) -> String {
        match &self.kind {
            NodeKind::Root => String::new(),
            NodeKind::Literal { literal } => literal.clone(),
            NodeKind::Argument { name, .. } => format!("<{name}>"),
        }
    }

    pub fn children(&self) -> &[Arc<CommandNode<S>>] {
        &self.children
    }

    pub fn command(&self) -> Option<&Command<S>> {
        self.command.as_ref()
    }

    pub fn redirect(&self) -> Option<&Arc<CommandNode<S>>> {
        self.redirect.as_ref()
    }

    pub fn modifier(&self) -> Option<&RedirectModifier<S>> {
        self.modifier.as_ref()
    }

    pub fn is_fork(&self) -> bool {
        self.forks
    }

    pub fn child(&self, name: &str) -> Option<&Arc<CommandNode<S>>> {
        self.children.iter().find(|child| child.name() == name)
    }

    pub fn can_use(&self, source: &S) -> bool {
        match &self.requirement {
            Some(requirement) => requirement(source),
            None => true,
        }
    }

    pub(crate) fn relevant_nodes(&self, reader: &StringReader) -> Vec<Arc<CommandNode<S>>> {
        let has_literals = self
            .children
            .iter()
            .any(|child| matches!(child.kind, NodeKind::Literal { .. }));
        if !has_literals {
            return self.children.clone();
        }

        let mut probe = reader.clone();
        let literal = probe.read_unquoted_string();
        if let Some(matched) = self.children.iter().find(|child| match &child.kind {
            NodeKind::Literal { literal: name } => name == literal,
            _ => false,
        }) {
            return vec![matched.clone()];
        }

        self.children.clone()
    }

    pub(crate) fn parse(
        &self,
        reader: &mut StringReader,
        context: &mut CommandContextBuilder<S>,
    ) -> Result<(), Error> {
        match &self.kind {
            NodeKind::Root => Ok(()),
            NodeKind::Literal { literal } => {
                let start = reader.cursor();
                if !reader.remaining().starts_with(literal.as_str()) {
                    return Err(error::LITERAL_INCORRECT
                        .create([error::arg(literal.clone())])
                        .with_context(reader.string(), start));
                }
                let end = start + literal.len();
                reader.set_cursor(end);
                match reader.peek() {
                    None | Some(' ') => Ok(()),
                    Some(_) => {
                        reader.set_cursor(start);
                        Err(error::LITERAL_INCORRECT
                            .create([error::arg(literal.clone())])
                            .with_context(reader.string(), start))
                    }
                }
            }
            NodeKind::Argument {
                name,
                argument_type,
                ..
            } => {
                let start = reader.cursor();
                let value = argument_type.parse_any(reader)?;
                let parsed =
                    ParsedArgument::new(StringRange::between(start, reader.cursor()), value);
                context.with_argument(name.clone(), parsed);
                Ok(())
            }
        }
    }

    pub(crate) fn list_suggestions(
        &self,
        context: &CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Suggestions {
        match &self.kind {
            NodeKind::Root => Suggestions::empty(),
            NodeKind::Literal { literal } => {
                let mut builder = builder;
                if literal
                    .to_lowercase()
                    .starts_with(builder.remaining_lowercase())
                {
                    builder.suggest(literal.clone());
                }
                builder.build()
            }
            NodeKind::Argument {
                argument_type,
                custom_suggestions,
                ..
            } => match custom_suggestions {
                Some(provider) => provider(context, builder),
                None => argument_type.list_suggestions(builder),
            },
        }
    }
}
