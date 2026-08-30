mod boolean;
mod numeric;
mod string;

pub use boolean::*;
pub use numeric::*;
pub use string::*;

use std::{any::Any, fmt::Debug, sync::Arc};

use crate::{
    command::{
        error::Error,
        string_reader::StringReader,
        suggestion::{Suggestions, SuggestionsBuilder},
    },
    protocol::encode::{EncodeError, PacketWrite},
};

pub trait Arg: Send + Sync + Debug + 'static {
    type Value: Send + Sync + Debug + Clone + 'static;

    const ID: i32;

    fn parse(&self, reader: &mut StringReader) -> Result<Self::Value, Error>;

    fn list_suggestions(&self, _builder: SuggestionsBuilder) -> Suggestions {
        Suggestions::empty()
    }

    fn encode_properties<W: PacketWrite>(&self, _w: &mut W) -> Result<(), EncodeError> {
        Ok(())
    }

    fn examples(&self) -> Vec<String> {
        Vec::new()
    }
}

pub trait AnyArg: Send + Sync + Debug {
    fn id(&self) -> i32;

    fn parse_any(&self, reader: &mut StringReader) -> Result<Arc<dyn Any + Send + Sync>, Error>;

    fn list_suggestions(&self, builder: SuggestionsBuilder) -> Suggestions;

    fn properties(&self) -> Vec<u8>;

    fn examples(&self) -> Vec<String>;
}

impl<T: Arg> AnyArg for T {
    fn id(&self) -> i32 {
        T::ID
    }

    fn parse_any(&self, reader: &mut StringReader) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        let value = self.parse(reader)?;
        Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
    }

    fn list_suggestions(&self, builder: SuggestionsBuilder) -> Suggestions {
        Arg::list_suggestions(self, builder)
    }

    fn properties(&self) -> Vec<u8> {
        let mut properties = Vec::new();
        self.encode_properties(&mut properties)
            .expect("encoding parser properties into a byte buffer is infallible");
        properties
    }

    fn examples(&self) -> Vec<String> {
        Arg::examples(self)
    }
}
