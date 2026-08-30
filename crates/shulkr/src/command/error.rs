use std::fmt;

use crate::text::{NamedColor, TextComponent};

const CONTEXT_AMOUNT: usize = 10;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: Box<TextComponent>,
    input: Option<String>,
    cursor: Option<usize>,
}

impl Error {
    fn new(message: TextComponent) -> Self {
        Self {
            message: Box::new(message),
            input: None,
            cursor: None,
        }
    }

    pub fn custom(message: impl Into<TextComponent>) -> Self {
        Self::new(message.into())
    }

    pub fn with_context(mut self, input: &str, cursor: usize) -> Self {
        self.input = Some(input.to_string());
        self.cursor = Some(cursor);
        self
    }

    pub fn message(&self) -> &TextComponent {
        &self.message
    }

    pub fn input(&self) -> Option<&str> {
        self.input.as_deref()
    }

    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }

    pub fn context(&self) -> Option<TextComponent> {
        let input = self.input.as_deref()?;
        let cursor = self.cursor?.min(input.len());

        let mut end = cursor;
        while end > 0 && !input.is_char_boundary(end) {
            end -= 1;
        }
        let mut start = end.saturating_sub(CONTEXT_AMOUNT);
        while start > 0 && !input.is_char_boundary(start) {
            start -= 1;
        }

        let prefix = if start > 0 {
            format!("...{}", &input[start..end])
        } else {
            input[start..end].to_string()
        };
        let mut context = TextComponent::text(prefix).color(NamedColor::Gray);

        if end < input.len() {
            context = context.child(
                TextComponent::text(input[end..].to_string())
                    .color(NamedColor::Red)
                    .underlined(),
            );
        }

        context = context.child(
            TextComponent::translatable("command.context.here", Some("<--[HERE]"), Vec::new())
                .color(NamedColor::Red)
                .italic(),
        );

        Some(context)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message.plain_text())?;
        if let Some(context) = self.context() {
            write!(
                f,
                " at position {}: {}",
                self.cursor.unwrap_or(0),
                context.plain_text()
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

pub fn arg(value: impl fmt::Display) -> TextComponent {
    TextComponent::text(value.to_string())
}

pub struct ErrorKind {
    key: &'static str,
}

impl ErrorKind {
    pub const fn new(key: &'static str) -> Self {
        Self { key }
    }

    pub fn create(&self, args: impl IntoIterator<Item = TextComponent>) -> Error {
        Error::new(TextComponent::translatable(
            self.key,
            None::<&str>,
            args.into_iter().collect(),
        ))
    }
}

pub static READER_EXPECTED_START_OF_QUOTE: ErrorKind =
    ErrorKind::new("parsing.quote.expected.start");
pub static READER_EXPECTED_END_OF_QUOTE: ErrorKind = ErrorKind::new("parsing.quote.expected.end");
pub static READER_INVALID_ESCAPE: ErrorKind = ErrorKind::new("parsing.quote.escape");
pub static READER_EXPECTED_BOOL: ErrorKind = ErrorKind::new("parsing.bool.expected");
pub static READER_INVALID_BOOL: ErrorKind = ErrorKind::new("parsing.bool.invalid");
pub static READER_EXPECTED_INT: ErrorKind = ErrorKind::new("parsing.int.expected");
pub static READER_INVALID_INT: ErrorKind = ErrorKind::new("parsing.int.invalid");
pub static READER_EXPECTED_LONG: ErrorKind = ErrorKind::new("parsing.long.expected");
pub static READER_INVALID_LONG: ErrorKind = ErrorKind::new("parsing.long.invalid");
pub static READER_EXPECTED_FLOAT: ErrorKind = ErrorKind::new("parsing.float.expected");
pub static READER_INVALID_FLOAT: ErrorKind = ErrorKind::new("parsing.float.invalid");
pub static READER_EXPECTED_DOUBLE: ErrorKind = ErrorKind::new("parsing.double.expected");
pub static READER_INVALID_DOUBLE: ErrorKind = ErrorKind::new("parsing.double.invalid");
pub static READER_EXPECTED_SYMBOL: ErrorKind = ErrorKind::new("parsing.expected");

pub static INTEGER_TOO_LOW: ErrorKind = ErrorKind::new("argument.integer.low");
pub static INTEGER_TOO_HIGH: ErrorKind = ErrorKind::new("argument.integer.big");
pub static LONG_TOO_LOW: ErrorKind = ErrorKind::new("argument.long.low");
pub static LONG_TOO_HIGH: ErrorKind = ErrorKind::new("argument.long.big");
pub static FLOAT_TOO_LOW: ErrorKind = ErrorKind::new("argument.float.low");
pub static FLOAT_TOO_HIGH: ErrorKind = ErrorKind::new("argument.float.big");
pub static DOUBLE_TOO_LOW: ErrorKind = ErrorKind::new("argument.double.low");
pub static DOUBLE_TOO_HIGH: ErrorKind = ErrorKind::new("argument.double.big");

pub static LITERAL_INCORRECT: ErrorKind = ErrorKind::new("argument.literal.incorrect");
pub static DISPATCHER_UNKNOWN_COMMAND: ErrorKind = ErrorKind::new("command.unknown.command");
pub static DISPATCHER_UNKNOWN_ARGUMENT: ErrorKind = ErrorKind::new("command.unknown.argument");
pub static DISPATCHER_EXPECTED_ARGUMENT_SEPARATOR: ErrorKind =
    ErrorKind::new("command.expected.separator");
