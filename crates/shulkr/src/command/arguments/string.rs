use crate::{
    command::{arguments::Arg, exceptions::CommandSyntaxException, string_reader::StringReader},
    protocol::encode::{EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringBehaviour {
    /// A single bare word.
    SingleWord,
    /// A bare word, or a `"quoted phrase"`.
    QuotablePhrase,
    /// Everything left in the command, quotes and all. Must be the last argument.
    GreedyPhrase,
}

impl StringBehaviour {
    pub fn examples(&self) -> Vec<String> {
        match self {
            Self::SingleWord => vec!["word", "words_with_underscores"],
            Self::QuotablePhrase => vec!["\"quoted phrase\"", "word", "\"\""],
            Self::GreedyPhrase => vec!["word", "words with spaces", "\"and symbols\""],
        }
        .into_iter()
        .map(str::to_string)
        .collect()
    }
}

impl TryFrom<i32> for StringBehaviour {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::SingleWord,
            1 => Self::QuotablePhrase,
            2 => Self::GreedyPhrase,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringArg {
    behaviour: StringBehaviour,
}

impl StringArg {
    pub fn new(behaviour: StringBehaviour) -> Self {
        Self { behaviour }
    }

    pub fn single_word() -> Self {
        Self::new(StringBehaviour::SingleWord)
    }

    pub fn quotable() -> Self {
        Self::new(StringBehaviour::QuotablePhrase)
    }

    pub fn greedy() -> Self {
        Self::new(StringBehaviour::GreedyPhrase)
    }

    pub fn behaviour(&self) -> StringBehaviour {
        self.behaviour
    }

    pub fn escape_if_required(input: &str) -> String {
        if input.is_empty()
            || !input
                .chars()
                .all(StringReader::is_allowed_in_unquoted_string)
        {
            let mut result = String::with_capacity(input.len() + 2);
            result.push('"');
            for c in input.chars() {
                if c == '\\' || c == '"' {
                    result.push('\\');
                }
                result.push(c);
            }
            result.push('"');
            result
        } else {
            input.to_string()
        }
    }
}

impl Arg for StringArg {
    type Value = String;
    const ID: i32 = 5;

    fn parse(&self, reader: &mut StringReader) -> Result<String, CommandSyntaxException> {
        match self.behaviour {
            StringBehaviour::SingleWord => Ok(reader.read_unquoted_string().to_string()),
            StringBehaviour::QuotablePhrase => reader.read_string(),
            StringBehaviour::GreedyPhrase => {
                let text = reader.remaining().to_string();
                reader.set_cursor(reader.total_length());
                Ok(text)
            }
        }
    }

    fn encode_properties<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
        w.write_varint(self.behaviour as i32)
    }

    fn examples(&self) -> Vec<String> {
        self.behaviour.examples()
    }
}
