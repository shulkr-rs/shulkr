use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::llama::{STRENGTH, VARIANT},
    },
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct LlamaMeta {
    holder: MetadataHolder,
}

impl LlamaMeta {
    pub fn get_variant(&self) -> LlamaVariant {
        LlamaVariant::try_from(self.holder.get(VARIANT)).unwrap_or(LlamaVariant::Creamy)
    }

    pub fn set_variant(&self, value: LlamaVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn get_strength(&self) -> i32 {
        self.holder.get(STRENGTH)
    }

    pub fn set_strength(&self, value: i32) {
        self.holder.set(STRENGTH, value);
    }
}

impl MetaAccessor for LlamaMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LlamaVariant {
    Creamy,
    White,
    Brown,
    Gray,
}

impl TryFrom<i32> for LlamaVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Creamy,
            1 => Self::White,
            2 => Self::Brown,
            3 => Self::Gray,
            _ => return Err(()),
        })
    }
}

impl DataType for LlamaVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        LlamaVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid LlamaVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
