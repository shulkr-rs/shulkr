use crate::command::error::{self, Error};

const SYNTAX_ESCAPE: char = '\\';
const SYNTAX_DOUBLE_QUOTE: char = '"';
const SYNTAX_SINGLE_QUOTE: char = '\'';

#[derive(Debug, Clone)]
pub struct StringReader {
    string: String,
    cursor: usize,
}

impl StringReader {
    pub fn new(string: impl Into<String>) -> Self {
        Self {
            string: string.into(),
            cursor: 0,
        }
    }

    pub fn string(&self) -> &str {
        &self.string
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        debug_assert!(self.string.is_char_boundary(cursor));
        self.cursor = cursor;
    }

    pub fn total_length(&self) -> usize {
        self.string.len()
    }

    pub fn remaining_length(&self) -> usize {
        self.string.len() - self.cursor
    }

    pub fn read_text(&self) -> &str {
        &self.string[..self.cursor]
    }

    pub fn remaining(&self) -> &str {
        &self.string[self.cursor..]
    }

    pub fn can_read_length(&self, length: usize) -> bool {
        self.remaining().chars().count() >= length
    }

    pub fn can_read(&self) -> bool {
        self.cursor < self.string.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    pub fn peek_offset(&self, offset: usize) -> Option<char> {
        self.remaining().chars().nth(offset)
    }

    pub fn read(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }

    pub fn skip(&mut self) {
        self.read();
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.skip();
        }
    }

    fn is_allowed_number(c: char) -> bool {
        c.is_ascii_digit() || c == '.' || c == '-'
    }

    pub fn is_quoted_string_start(c: char) -> bool {
        c == SYNTAX_DOUBLE_QUOTE || c == SYNTAX_SINGLE_QUOTE
    }

    pub fn is_allowed_in_unquoted_string(c: char) -> bool {
        c.is_ascii_digit()
            || c.is_ascii_uppercase()
            || c.is_ascii_lowercase()
            || c == '_'
            || c == '-'
            || c == '.'
            || c == '+'
    }

    fn read_number_text(&mut self) -> String {
        let start = self.cursor;
        while self.peek().is_some_and(Self::is_allowed_number) {
            self.skip();
        }
        self.string[start..self.cursor].to_string()
    }

    pub fn read_int(&mut self) -> Result<i32, Error> {
        let start = self.cursor;
        let text = self.read_number_text();
        if text.is_empty() {
            return Err(error::READER_EXPECTED_INT
                .create([])
                .with_context(&self.string, start));
        }
        match text.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.cursor = start;
                Err(error::READER_INVALID_INT
                    .create([error::arg(text)])
                    .with_context(&self.string, start))
            }
        }
    }

    pub fn read_long(&mut self) -> Result<i64, Error> {
        let start = self.cursor;
        let text = self.read_number_text();
        if text.is_empty() {
            return Err(error::READER_EXPECTED_LONG
                .create([])
                .with_context(&self.string, start));
        }
        match text.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.cursor = start;
                Err(error::READER_INVALID_LONG
                    .create([error::arg(text)])
                    .with_context(&self.string, start))
            }
        }
    }

    pub fn read_float(&mut self) -> Result<f32, Error> {
        let start = self.cursor;
        let text = self.read_number_text();
        if text.is_empty() {
            return Err(error::READER_EXPECTED_FLOAT
                .create([])
                .with_context(&self.string, start));
        }
        match text.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.cursor = start;
                Err(error::READER_INVALID_FLOAT
                    .create([error::arg(text)])
                    .with_context(&self.string, start))
            }
        }
    }

    pub fn read_double(&mut self) -> Result<f64, Error> {
        let start = self.cursor;
        let text = self.read_number_text();
        if text.is_empty() {
            return Err(error::READER_EXPECTED_DOUBLE
                .create([])
                .with_context(&self.string, start));
        }
        match text.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.cursor = start;
                Err(error::READER_INVALID_DOUBLE
                    .create([error::arg(text)])
                    .with_context(&self.string, start))
            }
        }
    }

    pub fn read_unquoted_string(&mut self) -> &str {
        let start = self.cursor;
        while self.peek().is_some_and(Self::is_allowed_in_unquoted_string) {
            self.skip();
        }
        &self.string[start..self.cursor]
    }

    pub fn read_quoted_string(&mut self) -> Result<String, Error> {
        let Some(next) = self.peek() else {
            return Ok(String::new());
        };
        if !Self::is_quoted_string_start(next) {
            return Err(error::READER_EXPECTED_START_OF_QUOTE
                .create([])
                .with_context(&self.string, self.cursor));
        }
        self.skip();
        self.read_string_until(next)
    }

    fn read_string_until(&mut self, terminator: char) -> Result<String, Error> {
        let mut result = String::new();
        let mut escaped = false;

        while let Some(c) = self.read() {
            if escaped {
                if c == terminator || c == SYNTAX_ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cursor -= c.len_utf8();
                    return Err(error::READER_INVALID_ESCAPE
                        .create([error::arg(c)])
                        .with_context(&self.string, self.cursor));
                }
            } else if c == SYNTAX_ESCAPE {
                escaped = true;
            } else if c == terminator {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        Err(error::READER_EXPECTED_END_OF_QUOTE
            .create([])
            .with_context(&self.string, self.cursor))
    }

    pub fn read_string(&mut self) -> Result<String, Error> {
        match self.peek() {
            Some(next) if Self::is_quoted_string_start(next) => {
                self.skip();
                self.read_string_until(next)
            }
            _ => Ok(self.read_unquoted_string().to_string()),
        }
    }

    pub fn read_boolean(&mut self) -> Result<bool, Error> {
        let start = self.cursor;
        let value = self.read_unquoted_string().to_string();
        match value.as_str() {
            "" => Err(error::READER_EXPECTED_BOOL
                .create([])
                .with_context(&self.string, start)),
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                self.cursor = start;
                Err(error::READER_INVALID_BOOL
                    .create([error::arg(value)])
                    .with_context(&self.string, start))
            }
        }
    }

    pub fn expect(&mut self, c: char) -> Result<(), Error> {
        if self.peek() != Some(c) {
            return Err(error::READER_EXPECTED_SYMBOL
                .create([error::arg(c)])
                .with_context(&self.string, self.cursor));
        }
        self.skip();
        Ok(())
    }
}
