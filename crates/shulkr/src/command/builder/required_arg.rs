use std::sync::Arc;

use crate::command::{
    arguments::{AnyArg, Arg},
    builder::{IntoCommandNode, ArgumentBuilder, builder_methods},
    context::CommandContext,
    suggestion::{Suggestions, SuggestionsBuilder},
    tree::{CommandNode, NodeKind, SuggestionProvider},
};

pub struct RequiredArgumentBuilder<S> {
    name: String,
    argument_type: Arc<dyn AnyArg>,
    custom_suggestions: Option<SuggestionProvider<S>>,
    builder: ArgumentBuilder<S>,
}

impl<S> RequiredArgumentBuilder<S> {
    builder_methods!();

    pub fn suggests<F>(mut self, provider: F) -> Self
    where
        F: Fn(&CommandContext<S>, SuggestionsBuilder) -> Suggestions + Send + Sync + 'static,
    {
        self.custom_suggestions = Some(Arc::new(provider));
        self
    }

    pub fn build(self) -> CommandNode<S> {
        CommandNode {
            kind: NodeKind::Argument {
                name: self.name,
                argument_type: self.argument_type,
                custom_suggestions: self.custom_suggestions,
            },
            children: self.builder.children,
            command: self.builder.command,
            requirement: self.builder.requirement,
            redirect: self.builder.redirect,
            modifier: self.builder.modifier,
            forks: self.builder.forks,
        }
    }
}

impl<S> IntoCommandNode<S> for RequiredArgumentBuilder<S> {
    fn into_node(self) -> CommandNode<S> {
        self.build()
    }
}

pub fn argument<S, T: Arg>(
    name: impl Into<String>,
    argument_type: T,
) -> RequiredArgumentBuilder<S> {
    RequiredArgumentBuilder {
        name: name.into(),
        argument_type: Arc::new(argument_type),
        custom_suggestions: None,
        builder: ArgumentBuilder::default(),
    }
}
