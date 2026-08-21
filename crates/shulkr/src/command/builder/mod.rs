mod literal;
mod required_arg;

pub use literal::{LiteralArgumentBuilder, literal};
pub use required_arg::{RequiredArgumentBuilder, argument};

use std::sync::Arc;

use crate::command::tree::{Command, CommandNode, NodeKind, RedirectModifier, Requirement};

pub trait IntoCommandNode<S> {
    fn into_node(self) -> CommandNode<S>;
}

impl<S> IntoCommandNode<S> for CommandNode<S> {
    fn into_node(self) -> CommandNode<S> {
        self
    }
}

pub(super) struct ArgumentBuilder<S> {
    pub(super) children: Vec<Arc<CommandNode<S>>>,
    pub(super) command: Option<Command<S>>,
    pub(super) requirement: Option<Requirement<S>>,
    pub(super) redirect: Option<Arc<CommandNode<S>>>,
    pub(super) modifier: Option<RedirectModifier<S>>,
    pub(super) forks: bool,
}

impl<S> Default for ArgumentBuilder<S> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            command: None,
            requirement: None,
            redirect: None,
            modifier: None,
            forks: false,
        }
    }
}

impl<S> ArgumentBuilder<S> {
    pub(super) fn add_child(&mut self, node: CommandNode<S>) {
        let is_literal = matches!(node.kind, NodeKind::Literal { .. });
        let node = Arc::new(node);
        if is_literal {
            let insert_at = self
                .children
                .iter()
                .position(|child| !matches!(child.kind, NodeKind::Literal { .. }))
                .unwrap_or(self.children.len());
            self.children.insert(insert_at, node);
        } else {
            self.children.push(node);
        }
    }
}

macro_rules! builder_methods {
    () => {
        pub fn then(mut self, child: impl $crate::command::builder::IntoCommandNode<S>) -> Self {
            self.builder.add_child(child.into_node());
            self
        }

        pub fn executes<F>(mut self, command: F) -> Self
        where
            F: Fn(
                    &$crate::command::context::CommandContext<S>,
                ) -> Result<i32, $crate::command::exceptions::CommandSyntaxException>
                + Send
                + Sync
                + 'static,
        {
            self.builder.command = Some(std::sync::Arc::new(command));
            self
        }

        pub fn requires<F>(mut self, requirement: F) -> Self
        where
            F: Fn(&S) -> bool + Send + Sync + 'static,
        {
            self.builder.requirement = Some(std::sync::Arc::new(requirement));
            self
        }

        pub fn redirect(
            self,
            target: std::sync::Arc<$crate::command::tree::CommandNode<S>>,
        ) -> Self {
            self.forward(target, None, false)
        }

        pub fn redirect_modified<F>(
            self,
            target: std::sync::Arc<$crate::command::tree::CommandNode<S>>,
            modifier: F,
        ) -> Self
        where
            F: Fn(
                    &$crate::command::context::CommandContext<S>,
                ) -> Result<Vec<S>, $crate::command::exceptions::CommandSyntaxException>
                + Send
                + Sync
                + 'static,
        {
            let modifier: $crate::command::tree::RedirectModifier<S> =
                std::sync::Arc::new(modifier);
            self.forward(target, Some(modifier), false)
        }

        pub fn fork<F>(
            self,
            target: std::sync::Arc<$crate::command::tree::CommandNode<S>>,
            modifier: F,
        ) -> Self
        where
            F: Fn(
                    &$crate::command::context::CommandContext<S>,
                ) -> Result<Vec<S>, $crate::command::exceptions::CommandSyntaxException>
                + Send
                + Sync
                + 'static,
        {
            let modifier: $crate::command::tree::RedirectModifier<S> =
                std::sync::Arc::new(modifier);
            self.forward(target, Some(modifier), true)
        }

        fn forward(
            mut self,
            target: std::sync::Arc<$crate::command::tree::CommandNode<S>>,
            modifier: Option<$crate::command::tree::RedirectModifier<S>>,
            forks: bool,
        ) -> Self {
            assert!(
                self.builder.children.is_empty(),
                "cannot forward a node that already has children"
            );
            self.builder.redirect = Some(target);
            self.builder.modifier = modifier;
            self.builder.forks = forks;
            self
        }
    };
}

pub(super) use builder_methods;
