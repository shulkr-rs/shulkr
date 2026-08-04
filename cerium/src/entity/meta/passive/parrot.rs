use crate::{
    entity::meta::{MetaAccessor, MetadataHolder, refs::parrot::VARIANT},
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct ParrotMeta {
    holder: MetadataHolder,
}

impl ParrotMeta {
    pub fn get_variant(&self) -> ParrotVariant {
        ParrotVariant::try_from(self.holder.get(VARIANT)).unwrap_or(ParrotVariant::RedBlue)
    }

    pub fn set_variant(&self, value: ParrotVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for ParrotMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParrotVariant {
    RedBlue,
    Blue,
    Green,
    YellowBlue,
    Grey,
}

impl TryFrom<i32> for ParrotVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::RedBlue,
            1 => Self::Blue,
            2 => Self::Green,
            3 => Self::YellowBlue,
            4 => Self::Grey,
            _ => return Err(()),
        })
    }
}

impl DataType for ParrotVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        ParrotVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid ParrotVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
