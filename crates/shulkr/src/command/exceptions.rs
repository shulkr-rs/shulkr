use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandErrorKind {
    // StringReader
    ExpectedStartOfQuote,
    ExpectedEndOfQuote,
    InvalidEscape,
    ExpectedBool,
    InvalidBool,
    ExpectedInt,
    InvalidInt,
    ExpectedLong,
    InvalidLong,
    ExpectedFloat,
    InvalidFloat,
    ExpectedDouble,
    InvalidDouble,
    ExpectedSymbol,
    // Argument types
    IntTooLow,
    IntTooHigh,
    LongTooLow,
    LongTooHigh,
    FloatTooLow,
    FloatTooHigh,
    DoubleTooLow,
    DoubleTooHigh,
    // Dispatcher
    LiteralIncorrect,
    UnknownCommand,
    UnknownArgument,
    ExpectedArgumentSeparator,

    Custom,
}

const CONTEXT_AMOUNT: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSyntaxException {
    kind: CommandErrorKind,
    message: String,
    input: Option<String>,
    cursor: Option<usize>,
}

impl CommandSyntaxException {
    pub fn new(kind: CommandErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            input: None,
            cursor: None,
        }
    }

    pub fn custom(message: impl Into<String>) -> Self {
        Self::new(CommandErrorKind::Custom, message)
    }

    pub fn with_context(mut self, input: &str, cursor: usize) -> Self {
        self.input = Some(input.to_string());
        self.cursor = Some(cursor);
        self
    }

    pub fn kind(&self) -> CommandErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn input(&self) -> Option<&str> {
        self.input.as_deref()
    }

    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }

    pub fn context(&self) -> Option<String> {
        let input = self.input.as_deref()?;
        let cursor = self.cursor?.min(input.len());

        // Back off to a char boundary so slicing a multi-byte input can't panic.
        let mut end = cursor;
        while end > 0 && !input.is_char_boundary(end) {
            end -= 1;
        }
        let mut start = end.saturating_sub(CONTEXT_AMOUNT);
        while start > 0 && !input.is_char_boundary(start) {
            start -= 1;
        }

        let mut context = String::new();
        if start > 0 {
            context.push_str("...");
        }
        context.push_str(&input[start..end]);
        context.push_str("<--[HERE]");
        Some(context)
    }
}

impl fmt::Display for CommandSyntaxException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;
        if let Some(context) = self.context() {
            write!(f, " at position {}: {}", self.cursor.unwrap_or(0), context)?;
        }
        Ok(())
    }
}

impl std::error::Error for CommandSyntaxException {}

impl CommandSyntaxException {
    pub fn expected_start_of_quote() -> Self {
        Self::new(
            CommandErrorKind::ExpectedStartOfQuote,
            "Expected quote to start a string",
        )
    }

    pub fn expected_end_of_quote() -> Self {
        Self::new(
            CommandErrorKind::ExpectedEndOfQuote,
            "Unclosed quoted string",
        )
    }

    pub fn invalid_escape(c: char) -> Self {
        Self::new(
            CommandErrorKind::InvalidEscape,
            format!("Invalid escape sequence '{c}' in quoted string"),
        )
    }

    pub fn expected_bool() -> Self {
        Self::new(CommandErrorKind::ExpectedBool, "Expected bool")
    }

    pub fn invalid_bool(value: &str) -> Self {
        Self::new(
            CommandErrorKind::InvalidBool,
            format!("Invalid bool, expected true or false but found '{value}'"),
        )
    }

    pub fn expected_int() -> Self {
        Self::new(CommandErrorKind::ExpectedInt, "Expected integer")
    }

    pub fn invalid_int(value: &str) -> Self {
        Self::new(
            CommandErrorKind::InvalidInt,
            format!("Invalid integer '{value}'"),
        )
    }

    pub fn expected_long() -> Self {
        Self::new(CommandErrorKind::ExpectedLong, "Expected long")
    }

    pub fn invalid_long(value: &str) -> Self {
        Self::new(
            CommandErrorKind::InvalidLong,
            format!("Invalid long '{value}'"),
        )
    }

    pub fn expected_float() -> Self {
        Self::new(CommandErrorKind::ExpectedFloat, "Expected float")
    }

    pub fn invalid_float(value: &str) -> Self {
        Self::new(
            CommandErrorKind::InvalidFloat,
            format!("Invalid float '{value}'"),
        )
    }

    pub fn expected_double() -> Self {
        Self::new(CommandErrorKind::ExpectedDouble, "Expected double")
    }

    pub fn invalid_double(value: &str) -> Self {
        Self::new(
            CommandErrorKind::InvalidDouble,
            format!("Invalid double '{value}'"),
        )
    }

    pub fn expected_symbol(c: char) -> Self {
        Self::new(CommandErrorKind::ExpectedSymbol, format!("Expected '{c}'"))
    }

    pub fn int_too_low(found: i32, min: i32) -> Self {
        Self::new(
            CommandErrorKind::IntTooLow,
            format!("Integer must not be less than {min}, found {found}"),
        )
    }

    pub fn int_too_high(found: i32, max: i32) -> Self {
        Self::new(
            CommandErrorKind::IntTooHigh,
            format!("Integer must not be more than {max}, found {found}"),
        )
    }

    pub fn long_too_low(found: i64, min: i64) -> Self {
        Self::new(
            CommandErrorKind::LongTooLow,
            format!("Long must not be less than {min}, found {found}"),
        )
    }

    pub fn long_too_high(found: i64, max: i64) -> Self {
        Self::new(
            CommandErrorKind::LongTooHigh,
            format!("Long must not be more than {max}, found {found}"),
        )
    }

    pub fn float_too_low(found: f32, min: f32) -> Self {
        Self::new(
            CommandErrorKind::FloatTooLow,
            format!("Float must not be less than {min}, found {found}"),
        )
    }

    pub fn float_too_high(found: f32, max: f32) -> Self {
        Self::new(
            CommandErrorKind::FloatTooHigh,
            format!("Float must not be more than {max}, found {found}"),
        )
    }

    pub fn double_too_low(found: f64, min: f64) -> Self {
        Self::new(
            CommandErrorKind::DoubleTooLow,
            format!("Double must not be less than {min}, found {found}"),
        )
    }

    pub fn double_too_high(found: f64, max: f64) -> Self {
        Self::new(
            CommandErrorKind::DoubleTooHigh,
            format!("Double must not be more than {max}, found {found}"),
        )
    }

    pub fn literal_incorrect(expected: &str) -> Self {
        Self::new(
            CommandErrorKind::LiteralIncorrect,
            format!("Expected literal {expected}"),
        )
    }

    pub fn unknown_command() -> Self {
        Self::new(CommandErrorKind::UnknownCommand, "Unknown command")
    }

    pub fn unknown_argument() -> Self {
        Self::new(
            CommandErrorKind::UnknownArgument,
            "Incorrect argument for command",
        )
    }

    pub fn expected_argument_separator() -> Self {
        Self::new(
            CommandErrorKind::ExpectedArgumentSeparator,
            "Expected whitespace to end one argument, but found trailing data",
        )
    }
}
