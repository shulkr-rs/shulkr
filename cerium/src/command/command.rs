use std::todo;

use crate::{
    command::{
        arg::{
            Arg,
            kind::{ArgKind, StringBehaviour},
        },
        dispatcher::CommandError,
        matches::{CommandMatches, CommandValue, FloatRange, IntRange},
    },
    entity::GameMode,
    util::HashMap,
};

pub struct Command {
    pub(crate) name: String,
    pub(crate) aliases: Vec<String>,
    pub(crate) args: Vec<Arg>,
    pub(crate) subcommands: Vec<Command>,
}

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            name,
            aliases: Vec::new(),
            args: Vec::new(),
            subcommands: Vec::new(),
        }
    }

    pub fn alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }

    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases.extend(aliases);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn args(mut self, args: Vec<Arg>) -> Self {
        self.args.extend(args);
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    pub fn subcommands(mut self, subcommands: Vec<Command>) -> Self {
        self.subcommands.extend(subcommands);
        self
    }

    pub(crate) fn matches_name(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|alias| alias == name)
    }

    pub(crate) fn parse_tokens(&self, tokens: &[String]) -> Result<CommandMatches, CommandError> {
        if let Some((head, tail)) = tokens.split_first() {
            if let Some(subcommand) = self
                .subcommands
                .iter()
                .find(|command| command.matches_name(head))
            {
                let matches = subcommand.parse_tokens(tail)?;
                return Ok(CommandMatches::new(
                    self.name.clone(),
                    HashMap::default(),
                    Some(matches),
                ));
            }
        }

        let mut args = HashMap::default();
        let mut cursor = 0;

        for arg in &self.args {
            match tokens.get(cursor) {
                Some(_) => {
                    let parsed = parse_arg_value(&self.name, arg, &tokens[cursor..])?;
                    cursor += parsed.consumed;
                    args.insert(arg.name.clone(), parsed.value);
                }
                None if arg.required => {
                    return Err(CommandError::MissingArgument {
                        command: self.name.clone(),
                        argument: arg.name.clone(),
                    });
                }
                None => {}
            }
        }

        if let Some(unexpected) = tokens.get(cursor) {
            return Err(CommandError::UnexpectedArgument {
                command: self.name.clone(),
                argument: unexpected.clone(),
            });
        }

        Ok(CommandMatches::new(self.name.clone(), args, None))
    }
}

struct ParsedArg {
    consumed: usize,
    value: CommandValue,
}

fn parse_arg_value(command: &str, arg: &Arg, tokens: &[String]) -> Result<ParsedArg, CommandError> {
    let value = tokens.first().expect("parse_arg_value requires one token");

    let parsed = match arg.kind {
        ArgKind::Bool => ParsedArg {
            consumed: 1,
            value: CommandValue::Bool {
                raw: value.clone(),
                value: match value.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return invalid_arg(command, arg, "bool", value),
                },
            },
        },
        ArgKind::Integer { min, max } => {
            let parsed =
                parse_bounded(command, value, "integer", min, max, str::parse::<i32>, arg)?;
            ParsedArg {
                consumed: 1,
                value: CommandValue::Integer {
                    raw: value.clone(),
                    value: parsed,
                },
            }
        }
        ArgKind::Long { min, max } => {
            let parsed = parse_bounded(command, value, "long", min, max, str::parse::<i64>, arg)?;
            ParsedArg {
                consumed: 1,
                value: CommandValue::Long {
                    raw: value.clone(),
                    value: parsed,
                },
            }
        }
        ArgKind::Float { min, max } => {
            let parsed = parse_bounded(command, value, "float", min, max, str::parse::<f32>, arg)?;
            ParsedArg {
                consumed: 1,
                value: CommandValue::Float {
                    raw: value.clone(),
                    value: parsed,
                },
            }
        }
        ArgKind::Double { min, max } => {
            let parsed = parse_bounded(command, value, "double", min, max, str::parse::<f64>, arg)?;
            ParsedArg {
                consumed: 1,
                value: CommandValue::Double {
                    raw: value.clone(),
                    value: parsed,
                },
            }
        }
        ArgKind::String(StringBehaviour::SingleWord | StringBehaviour::QuotablePhrase) => {
            ParsedArg {
                consumed: 1,
                value: CommandValue::String { raw: value.clone() },
            }
        }
        ArgKind::String(StringBehaviour::GreedyPhrase) => ParsedArg {
            consumed: tokens.len(),
            value: CommandValue::String {
                raw: tokens.join(" "),
            },
        },
        ArgKind::IntRange => ParsedArg {
            consumed: 1,
            value: CommandValue::IntRange {
                raw: value.clone(),
                value: parse_range(command, value, "int range", str::parse::<i32>, arg)
                    .map(|(min, max)| IntRange { min, max })?,
            },
        },
        ArgKind::FloatRange => ParsedArg {
            consumed: 1,
            value: CommandValue::FloatRange {
                raw: value.clone(),
                value: parse_range(command, value, "float range", str::parse::<f32>, arg)
                    .map(|(min, max)| FloatRange { min, max })?,
            },
        },
        ArgKind::GameMode => ParsedArg {
            consumed: 1,
            value: CommandValue::GameMode {
                raw: value.clone(),
                value: match parse_game_mode(value) {
                    Some(mode) => mode,
                    None => return invalid_arg(command, arg, "gamemode", value),
                },
            },
        },
        ArgKind::Entity { .. }
        | ArgKind::ScoreHolder { .. }
        | ArgKind::Time { .. }
        | ArgKind::Resource { .. }
        | ArgKind::ResourceKey { .. }
        | ArgKind::ResourceOrTag { .. }
        | ArgKind::ResourceOrTagKey { .. }
        | ArgKind::ResourceSelector { .. } => todo!(),
    };

    Ok(parsed)
}

fn parse_game_mode(value: &str) -> Option<GameMode> {
    let mode = match value.to_ascii_lowercase().as_str() {
        "survival" => GameMode::Survival,
        "creative" => GameMode::Creative,
        "adventure" => GameMode::Adventure,
        "spectator" => GameMode::Spectator,
        _ => return None,
    };
    Some(mode)
}

fn parse_bounded<T, F>(
    command: &str,
    raw: &str,
    expected: &str,
    min: Option<T>,
    max: Option<T>,
    parse: F,
    arg: &Arg,
) -> Result<T, CommandError>
where
    T: PartialOrd + Copy,
    F: FnOnce(&str) -> Result<T, <T as std::str::FromStr>::Err>,
    T: std::str::FromStr + Copy,
{
    let value = parse(raw).map_err(|_| invalid_arg_error(command, arg, expected, raw))?;

    if min.is_some_and(|min| value < min) || max.is_some_and(|max| value > max) {
        return Err(invalid_arg_error(command, arg, expected, raw));
    }

    Ok(value)
}

fn parse_range<T, F>(
    command: &str,
    raw: &str,
    expected: &str,
    parse: F,
    arg: &Arg,
) -> Result<(Option<T>, Option<T>), CommandError>
where
    F: Fn(&str) -> Result<T, <T as std::str::FromStr>::Err>,
    T: std::str::FromStr + Copy,
{
    if let Some((min, max)) = raw.split_once("..") {
        let min = if min.is_empty() {
            None
        } else {
            Some(parse(min).map_err(|_| invalid_arg_error(command, arg, expected, raw))?)
        };
        let max = if max.is_empty() {
            None
        } else {
            Some(parse(max).map_err(|_| invalid_arg_error(command, arg, expected, raw))?)
        };

        if min.is_none() && max.is_none() {
            return Err(invalid_arg_error(command, arg, expected, raw));
        }

        return Ok((min, max));
    }

    let exact = parse(raw).map_err(|_| invalid_arg_error(command, arg, expected, raw))?;
    Ok((Some(exact), Some(exact)))
}

fn invalid_arg<T>(
    command: &str,
    arg: &Arg,
    expected: &str,
    value: &str,
) -> Result<T, CommandError> {
    Err(invalid_arg_error(command, arg, expected, value))
}

fn invalid_arg_error(command: &str, arg: &Arg, expected: &str, value: &str) -> CommandError {
    CommandError::InvalidArgument {
        command: command.to_string(),
        argument: arg.name.clone(),
        expected: expected.to_string(),
        value: value.to_string(),
    }
}
