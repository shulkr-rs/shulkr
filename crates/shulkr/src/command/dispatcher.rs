use crate::{
    command::{Command, matches::CommandMatches},
    util::RwLock,
};

pub struct CommandDispatcher {
    pub(crate) commands: RwLock<Vec<Command>>,
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandDispatcher {
    pub fn new() -> Self {
        Self {
            commands: RwLock::new(Vec::new()),
        }
    }

    pub fn register(&self, command: Command) {
        let mut commands = self.commands.write();
        commands.retain(|existing| existing.name != command.name);
        commands.push(command);
    }

    pub fn register_all(&self, commands: impl IntoIterator<Item = Command>) {
        let mut cmds = self.commands.write();
        for command in commands {
            cmds.retain(|existing| existing.name != command.name);
            cmds.push(command);
        }
    }

    pub fn unregister(&mut self, command: Command) {
        let mut commands = self.commands.write();
        commands.retain(|existing| existing.name != command.name);
    }

    pub fn parse(&self, input: &str) -> CommandResult {
        let normalized = input.strip_prefix('/').unwrap_or(input).trim();
        let tokens = split_command(normalized)?;
        let Some((name, tail)) = tokens.split_first() else {
            return Err(CommandError::Empty);
        };

        let commands = self.commands.read();
        let Some(command) = commands.iter().find(|command| command.matches_name(name)) else {
            return Err(CommandError::UnknownCommand {
                command: name.clone(),
            });
        };

        command.parse_tokens(tail)
    }
}

// Example:
// let args = split_command("mycommand subcommand \"myvalue\"")?;
// assert_eq!(args, vec!["mycommand", "subcommand", "myvalue"]);
fn split_command(input: &str) -> Result<Vec<String>, CommandError> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut chars = input.chars();
    let mut quote = None;

    while let Some(ch) = chars.next() {
        match (quote, ch) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), '\\') => {
                if let Some(next) = chars.next() {
                    token.push(next);
                }
            }
            (Some(_), c) => token.push(c),
            (None, '"' | '\'') => quote = Some(ch),
            (None, c) if c.is_whitespace() => {
                if !token.is_empty() {
                    tokens.push(std::mem::take(&mut token));
                }
            }
            (None, c) => token.push(c),
        }
    }

    if quote.is_some() {
        return Err(CommandError::UnclosedQuote);
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    Ok(tokens)
}

pub type CommandResult = Result<CommandMatches, CommandError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    UnknownCommand {
        command: String,
    },
    MissingArgument {
        command: String,
        argument: String,
    },
    UnexpectedArgument {
        command: String,
        argument: String,
    },
    InvalidArgument {
        command: String,
        argument: String,
        expected: String,
        value: String,
    },
    UnclosedQuote,
}
