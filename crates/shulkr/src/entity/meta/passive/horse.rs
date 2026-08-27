use crate::{
    entity::meta::{MetaAccessor, MetadataHolder, refs::horse::VARIANT},
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct HorseMeta {
    holder: MetadataHolder,
}

impl HorseMeta {
    pub fn get_variant(&self) -> HorseVariant {
        HorseVariant::try_from(self.holder.get(VARIANT)).unwrap_or(HorseVariant::White)
    }

    pub fn set_variant(&self, value: HorseVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for HorseMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HorseVariant {
    White,
    Creamy,
    Chestnut,
    Brown,
    Black,
    Gray,
    DarkBrown,
}

impl TryFrom<i32> for HorseVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::White,
            1 => Self::Creamy,
            2 => Self::Chestnut,
            3 => Self::Brown,
            4 => Self::Black,
            5 => Self::Gray,
            6 => Self::DarkBrown,
            _ => return Err(()),
        })
    }
}

impl DataType for HorseVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        HorseVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid HorseVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
