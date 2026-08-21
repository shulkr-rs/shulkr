use std::sync::Arc;

use crate::command::{
    dispatcher::CommandDispatcher,
    tree::{CommandNode, NodeKind},
};
use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};
use crate::util::{HashMap, Key};

const NODE_TYPE_ROOT: u8 = 0x00;
const NODE_TYPE_LITERAL: u8 = 0x01;
const NODE_TYPE_ARGUMENT: u8 = 0x02;
const FLAG_EXECUTABLE: u8 = 0x04;
const FLAG_HAS_REDIRECT: u8 = 0x08;
const FLAG_HAS_SUGGESTIONS_TYPE: u8 = 0x10;

const ASK_SERVER: Key = Key::const_vanilla("ask_server");

#[derive(Debug, Clone)]
pub struct CommandsPacket {
    nodes: Vec<WireNode>,
    root_index: i32,
}

impl CommandsPacket {
    pub fn from_dispatcher<S>(dispatcher: &CommandDispatcher<S>, source: &S) -> Self {
        let root = dispatcher.root();
        let mut builder = WireTreeBuilder {
            nodes: vec![WireNode::root()],
            indices: HashMap::default(),
        };

        for child in root.children() {
            if let Some(index) = builder.add(child, source) {
                builder.nodes[0].children.push(index);
            }
        }

        builder.resolve_redirects();

        Self {
            nodes: builder.nodes,
            root_index: 0,
        }
    }
}

impl Packet for CommandsPacket {}
impl ServerPacket for CommandsPacket {}

impl Encode for CommandsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.nodes.len() as i32)?;
        for node in &this.nodes {
            node.encode(w)?;
        }
        w.write_varint(this.root_index)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct WireNode {
    flags: u8,
    children: Vec<i32>,
    name: Option<String>,
    parser: Option<(i32, Vec<u8>)>,
    redirect: Option<i32>,
    suggestions_type: Option<Key>,
}

impl WireNode {
    fn root() -> Self {
        Self {
            flags: NODE_TYPE_ROOT,
            children: Vec::new(),
            name: None,
            parser: None,
            redirect: None,
            suggestions_type: None,
        }
    }

    fn encode<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
        w.write_u8(self.flags)?;

        w.write_varint(self.children.len() as i32)?;
        for child in &self.children {
            w.write_varint(*child)?;
        }

        if let Some(redirect) = self.redirect {
            w.write_varint(redirect)?;
        }

        if let Some(name) = &self.name {
            w.write_string(name)?;
        }

        if let Some((id, properties)) = &self.parser {
            w.write_varint(*id)?;
            for byte in properties {
                w.write_u8(*byte)?;
            }
        }

        if let Some(suggestions_type) = &self.suggestions_type {
            w.write_identifier(suggestions_type)?;
        }

        Ok(())
    }
}

struct WireTreeBuilder<S> {
    nodes: Vec<WireNode>,
    indices: HashMap<NodeKey<S>, i32>,
}

struct NodeKey<S>(Arc<CommandNode<S>>);

impl<S> PartialEq for NodeKey<S> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<S> Eq for NodeKey<S> {}

impl<S> std::hash::Hash for NodeKey<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Arc::as_ptr(&self.0) as *const ()).hash(state);
    }
}

impl<S> Clone for NodeKey<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S> WireTreeBuilder<S> {
    fn add(&mut self, node: &Arc<CommandNode<S>>, source: &S) -> Option<i32> {
        if !node.can_use(source) {
            return None;
        }

        if let Some(index) = self.indices.get(&NodeKey(node.clone())) {
            return Some(*index);
        }

        let mut flags = match &node.kind {
            NodeKind::Root => NODE_TYPE_ROOT,
            NodeKind::Literal { .. } => NODE_TYPE_LITERAL,
            NodeKind::Argument { .. } => NODE_TYPE_ARGUMENT,
        };
        if node.command().is_some() {
            flags |= FLAG_EXECUTABLE;
        }
        if node.redirect().is_some() {
            flags |= FLAG_HAS_REDIRECT;
        }

        let (name, parser, suggestions_type) = match &node.kind {
            NodeKind::Root => (None, None, None),
            NodeKind::Literal { literal } => (Some(literal.clone()), None, None),
            NodeKind::Argument {
                name,
                argument_type,
                custom_suggestions,
            } => (
                Some(name.clone()),
                Some((argument_type.id(), argument_type.properties())),
                custom_suggestions.as_ref().map(|_| ASK_SERVER),
            ),
        };
        if suggestions_type.is_some() {
            flags |= FLAG_HAS_SUGGESTIONS_TYPE;
        }

        let index = self.nodes.len() as i32;
        self.nodes.push(WireNode {
            flags,
            children: Vec::new(),
            name,
            parser,
            suggestions_type,
            redirect: None,
        });
        self.indices.insert(NodeKey(node.clone()), index);

        let mut children = Vec::new();
        for child in node.children() {
            if let Some(child_index) = self.add(child, source) {
                children.push(child_index);
            }
        }
        self.nodes[index as usize].children = children;

        Some(index)
    }

    fn resolve_redirects(&mut self) {
        let pending: Vec<(i32, Arc<CommandNode<S>>)> = self
            .indices
            .iter()
            .filter_map(|(key, index)| key.0.redirect().map(|target| (*index, target.clone())))
            .collect();

        for (index, target) in pending {
            let target_index = match self.indices.get(&NodeKey(target.clone())) {
                Some(target_index) => *target_index,
                None if matches!(target.kind, NodeKind::Root) => 0,
                None => continue,
            };
            self.nodes[index as usize].redirect = Some(target_index);
        }
    }
}
