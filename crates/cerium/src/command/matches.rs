use crate::{entity::GameMode, util::HashMap};

#[derive(Debug, Clone)]
pub struct CommandMatches {
    pub(crate) command: String,
    pub(crate) args: HashMap<String, CommandValue>,
    pub(crate) subcommand: Option<Box<CommandMatches>>,
}

impl CommandMatches {
    pub(crate) fn new(
        command: impl Into<String>,
        args: HashMap<String, CommandValue>,
        subcommand: Option<CommandMatches>,
    ) -> Self {
        Self {
            command: command.into(),
            args,
            subcommand: subcommand.map(Box::new),
        }
    }

    pub fn command_name(&self) -> &str {
        &self.command
    }

    pub fn get_raw(&self, name: &str) -> Option<&str> {
        self.args.get(name).map(CommandValue::raw)
    }

    pub fn get<T: FromCommandValue>(&self, name: &str) -> Option<T> {
        T::from_command_value(self.args.get(name)?)
    }

    pub fn get_value(&self, name: &str) -> Option<&CommandValue> {
        self.args.get(name)
    }

    pub fn subcommand(&self) -> Option<(&str, &CommandMatches)> {
        self.subcommand
            .as_deref()
            .map(|matches| (matches.command_name(), matches))
    }
}

pub trait FromCommandValue: Sized {
    fn from_command_value(value: &CommandValue) -> Option<Self>;
}

impl FromCommandValue for bool {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::Bool { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for i32 {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::Integer { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for i64 {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::Long { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for f32 {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::Float { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for f64 {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::Double { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for String {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::String { raw } => Some(raw.clone()),
            _ => None,
        }
    }
}

impl FromCommandValue for IntRange {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::IntRange { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for FloatRange {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::FloatRange { value, .. } => Some(*value),
            _ => None,
        }
    }
}

impl FromCommandValue for GameMode {
    fn from_command_value(value: &CommandValue) -> Option<Self> {
        match value {
            CommandValue::GameMode { value, .. } => Some(*value),
            _ => None,
        }
    }
}

// impl FromCommandValue for EntitySelector {
//     fn from_command_value(value: &CommandValue) -> Option<Self> {
//         match value {
//             CommandValue::Entity { value, .. } => Some(value.clone()),
//             _ => None,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum CommandValue {
    Bool { raw: String, value: bool },
    Integer { raw: String, value: i32 },
    Long { raw: String, value: i64 },
    Float { raw: String, value: f32 },
    Double { raw: String, value: f64 },
    String { raw: String },
    IntRange { raw: String, value: IntRange },
    FloatRange { raw: String, value: FloatRange },
    GameMode { raw: String, value: GameMode },
    // Entity { raw: String, value: EntitySelector },
}

impl CommandValue {
    pub fn raw(&self) -> &str {
        match self {
            CommandValue::Bool { raw, .. }
            | CommandValue::Integer { raw, .. }
            | CommandValue::Long { raw, .. }
            | CommandValue::Float { raw, .. }
            | CommandValue::Double { raw, .. }
            | CommandValue::String { raw }
            | CommandValue::IntRange { raw, .. }
            | CommandValue::FloatRange { raw, .. }
            | CommandValue::GameMode { raw, .. } => raw,
            // | CommandValue::Entity { raw, .. } => raw,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IntRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatRange {
    pub min: Option<f32>,
    pub max: Option<f32>,
}
