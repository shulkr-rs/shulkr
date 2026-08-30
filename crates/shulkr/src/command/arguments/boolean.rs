use crate::command::{
    arguments::Arg,
    error::Error,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BoolArg;

impl BoolArg {
    pub fn new() -> Self {
        Self
    }
}

impl Arg for BoolArg {
    type Value = bool;
    const ID: i32 = 0;

    fn parse(&self, reader: &mut StringReader) -> Result<bool, Error> {
        reader.read_boolean()
    }

    fn list_suggestions(&self, mut builder: SuggestionsBuilder) -> Suggestions {
        builder.suggest_matching(["true", "false"]);
        builder.build()
    }

    fn examples(&self) -> Vec<String> {
        vec!["true".to_string(), "false".to_string()]
    }
}
