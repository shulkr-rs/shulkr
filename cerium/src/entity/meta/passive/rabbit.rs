use crate::{
    entity::meta::{MetaAccessor, MetadataHolder, refs::rabbit::VARIANT},
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct RabbitMeta {
    holder: MetadataHolder,
}

impl RabbitMeta {
    pub fn get_variant(&self) -> RabbitVariant {
        RabbitVariant::try_from(self.holder.get(VARIANT)).unwrap_or(RabbitVariant::Brown)
    }

    pub fn set_variant(&self, value: RabbitVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for RabbitMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RabbitVariant {
    Brown,
    White,
    Black,
    WhiteSplotched,
    Gold,
    Salt,
    Evil,
}

impl TryFrom<i32> for RabbitVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Brown,
            1 => Self::White,
            2 => Self::Black,
            3 => Self::WhiteSplotched,
            4 => Self::Gold,
            5 => Self::Salt,
            6 => Self::Evil,
            _ => return Err(()),
        })
    }
}

impl DataType for RabbitVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        RabbitVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid RabbitVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
