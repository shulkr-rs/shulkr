use crate::command::arg::kind::ArgKind;
use crate::command::{
    Command, arg::Arg, arg::kind::StringBehaviour, dispatcher::CommandDispatcher,
};
use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

const NODE_TYPE_ROOT: u8 = 0x00;
const NODE_TYPE_LITERAL: u8 = 0x01;
const NODE_TYPE_ARGUMENT: u8 = 0x02;
const FLAG_EXECUTABLE: u8 = 0x04;

#[derive(Debug, Clone)]
pub struct CommandsPacket {
    nodes: Vec<CommandNode>,
    root_index: i32,
}

impl CommandsPacket {
    pub fn from_dispatcher(dispatcher: &CommandDispatcher) -> Self {
        let mut builder = CommandTreeBuilder::default();
        let commands = dispatcher.commands.read();
        for command in commands.iter() {
            builder.add_command(command);
        }
        builder.finish()
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
struct CommandNode {
    flags: u8,
    children: Vec<i32>,
    name: Option<String>,
    parser: Option<ArgKind>,
}

impl CommandNode {
    fn root() -> Self {
        Self {
            flags: NODE_TYPE_ROOT,
            children: Vec::new(),
            name: None,
            parser: None,
        }
    }

    fn literal(name: impl Into<String>, executable: bool) -> Self {
        Self {
            flags: NODE_TYPE_LITERAL | if executable { FLAG_EXECUTABLE } else { 0 },
            children: Vec::new(),
            name: Some(name.into()),
            parser: None,
        }
    }

    fn argument(arg: &Arg, executable: bool) -> Self {
        Self {
            flags: NODE_TYPE_ARGUMENT | if executable { FLAG_EXECUTABLE } else { 0 },
            children: Vec::new(),
            name: Some(arg.name.to_string()),
            parser: Some(arg.kind.clone()),
        }
    }

    fn encode<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
        w.write_u8(self.flags)?;
        w.write_varint(self.children.len() as i32)?;
        for child in &self.children {
            w.write_varint(*child)?;
        }

        if let Some(name) = &self.name {
            w.write_string(name)?;
        }

        if let Some(kind) = &self.parser {
            w.write_varint(kind.parser_id())?;
            encode_parser_properties(w, kind)?;
        }

        Ok(())
    }
}

fn encode_parser_properties<W: PacketWrite>(w: &mut W, kind: &ArgKind) -> Result<(), EncodeError> {
    match kind {
        ArgKind::Float { min, max } => encode_number_bounds(w, *min, *max, PacketWrite::write_f32),
        ArgKind::Double { min, max } => encode_number_bounds(w, *min, *max, PacketWrite::write_f64),
        ArgKind::Integer { min, max } => {
            encode_number_bounds(w, *min, *max, PacketWrite::write_i32)
        }
        ArgKind::Long { min, max } => encode_number_bounds(w, *min, *max, PacketWrite::write_i64),
        ArgKind::String(behaviour) => encode_string_behaviour(w, *behaviour),
        ArgKind::Entity {
            single,
            players_only,
        } => w.write_u8(u8::from(*single) | (u8::from(*players_only) << 1)),
        ArgKind::ScoreHolder { multiple } => w.write_u8(u8::from(*multiple)),
        ArgKind::Time { min } => w.write_i32(*min),
        ArgKind::Resource { registry }
        | ArgKind::ResourceKey { registry }
        | ArgKind::ResourceOrTag { registry }
        | ArgKind::ResourceOrTagKey { registry }
        | ArgKind::ResourceSelector { registry } => w.write_identifier(registry),
        ArgKind::Bool | ArgKind::IntRange | ArgKind::FloatRange | ArgKind::GameMode => Ok(()),
    }
}

fn encode_string_behaviour<W: PacketWrite>(
    w: &mut W,
    behaviour: StringBehaviour,
) -> Result<(), EncodeError> {
    match behaviour {
        StringBehaviour::SingleWord => w.write_varint(0),
        StringBehaviour::QuotablePhrase => w.write_varint(1),
        StringBehaviour::GreedyPhrase => w.write_varint(2),
    }
}

fn encode_number_bounds<W, T, F>(
    w: &mut W,
    min: Option<T>,
    max: Option<T>,
    mut write: F,
) -> Result<(), EncodeError>
where
    W: PacketWrite,
    F: FnMut(&mut W, T) -> Result<(), EncodeError>,
{
    let flags = u8::from(min.is_some()) | (u8::from(max.is_some()) << 1);
    w.write_u8(flags)?;
    if let Some(min) = min {
        write(w, min)?;
    }
    if let Some(max) = max {
        write(w, max)?;
    }
    Ok(())
}

struct CommandTreeBuilder {
    nodes: Vec<CommandNode>,
}

impl Default for CommandTreeBuilder {
    fn default() -> Self {
        Self {
            nodes: vec![CommandNode::root()],
        }
    }
}

impl CommandTreeBuilder {
    fn add_command(&mut self, command: &Command) {
        let index = self.push_command(command);
        self.nodes[0].children.push(index);

        for alias in &command.aliases {
            let alias_index = self.push_command_named(command, alias);
            self.nodes[0].children.push(alias_index);
        }
    }

    fn push_command(&mut self, command: &Command) -> i32 {
        self.push_command_named(command, &command.name)
    }

    fn push_command_named(&mut self, command: &Command, name: &str) -> i32 {
        let executable =
            command.args.iter().all(|arg| !arg.required) && command.subcommands.is_empty();
        let index = self.push_node(CommandNode::literal(name, executable));

        self.append_args(index, &command.args);
        for subcommand in &command.subcommands {
            let child = self.push_command(subcommand);
            self.nodes[index as usize].children.push(child);
        }

        index
    }

    fn append_args(&mut self, parent: i32, args: &[Arg]) {
        let mut current = parent;
        for (idx, arg) in args.iter().enumerate() {
            let executable = args[idx + 1..].iter().all(|a| !a.required);
            let child = self.push_node(CommandNode::argument(arg, executable));
            self.nodes[current as usize].children.push(child);
            current = child;
        }
    }

    fn push_node(&mut self, node: CommandNode) -> i32 {
        let index = self.nodes.len() as i32;
        self.nodes.push(node);
        index
    }

    fn finish(self) -> CommandsPacket {
        CommandsPacket {
            nodes: self.nodes,
            root_index: 0,
        }
    }
}
