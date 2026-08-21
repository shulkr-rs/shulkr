use crate::command::{
    builder::{IntoCommandNode, ArgumentBuilder, builder_methods},
    tree::{CommandNode, NodeKind},
};

pub struct LiteralArgumentBuilder<S> {
    literal: String,
    builder: ArgumentBuilder<S>,
}

impl<S> LiteralArgumentBuilder<S> {
    builder_methods!();

    pub fn build(self) -> CommandNode<S> {
        CommandNode {
            kind: NodeKind::Literal {
                literal: self.literal,
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

impl<S> IntoCommandNode<S> for LiteralArgumentBuilder<S> {
    fn into_node(self) -> CommandNode<S> {
        self.build()
    }
}

pub fn literal<S>(name: impl Into<String>) -> LiteralArgumentBuilder<S> {
    LiteralArgumentBuilder {
        literal: name.into(),
        builder: ArgumentBuilder::default(),
    }
}
