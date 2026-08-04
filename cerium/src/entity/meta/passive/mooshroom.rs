use crate::{
    entity::meta::{MetaAccessor, MetadataHolder, refs::mooshroom::VARIANT},
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct MooshroomMeta {
    holder: MetadataHolder,
}

impl MooshroomMeta {
    pub fn get_variant(&self) -> MooshroomVariant {
        MooshroomVariant::try_from(self.holder.get(VARIANT)).unwrap_or(MooshroomVariant::Red)
    }

    pub fn set_variant(&self, value: MooshroomVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for MooshroomMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MooshroomVariant {
    Red,
    Brown,
}

impl TryFrom<i32> for MooshroomVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Red,
            1 => Self::Brown,
            _ => return Err(()),
        })
    }
}

impl DataType for MooshroomVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        MooshroomVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid MooshroomVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
