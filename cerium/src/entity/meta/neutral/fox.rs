use uuid::Uuid;

use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::fox::{
            FIRST_UUID, IS_CROUCHING, IS_DEFENDING, IS_FACEPLANTED, IS_INTERESTED, IS_POUNCING,
            IS_SITTING, IS_SLEEPING, SECOND_UUID, VARIANT,
        },
    },
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct FoxMeta {
    holder: MetadataHolder,
}

impl FoxMeta {
    pub fn get_variant(&self) -> FoxVariant {
        FoxVariant::try_from(self.holder.get(VARIANT)).unwrap_or(FoxVariant::Red)
    }

    pub fn set_variant(&self, value: FoxVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn is_sitting(&self) -> bool {
        self.holder.get(IS_SITTING)
    }

    pub fn set_sitting(&self, value: bool) {
        self.holder.set(IS_SITTING, value);
    }

    pub fn is_crouching(&self) -> bool {
        self.holder.get(IS_CROUCHING)
    }

    pub fn set_crouching(&self, value: bool) {
        self.holder.set(IS_CROUCHING, value);
    }

    pub fn is_interested(&self) -> bool {
        self.holder.get(IS_INTERESTED)
    }

    pub fn set_interested(&self, value: bool) {
        self.holder.set(IS_INTERESTED, value);
    }

    pub fn is_pouncing(&self) -> bool {
        self.holder.get(IS_POUNCING)
    }

    pub fn set_pouncing(&self, value: bool) {
        self.holder.set(IS_POUNCING, value);
    }

    pub fn is_sleeping(&self) -> bool {
        self.holder.get(IS_SLEEPING)
    }

    pub fn set_sleeping(&self, value: bool) {
        self.holder.set(IS_SLEEPING, value);
    }

    pub fn is_faceplanted(&self) -> bool {
        self.holder.get(IS_FACEPLANTED)
    }

    pub fn set_faceplanted(&self, value: bool) {
        self.holder.set(IS_FACEPLANTED, value);
    }

    pub fn is_defending(&self) -> bool {
        self.holder.get(IS_DEFENDING)
    }

    pub fn set_defending(&self, value: bool) {
        self.holder.set(IS_DEFENDING, value);
    }

    pub fn get_first_uuid(&self) -> Option<Uuid> {
        self.holder.get(FIRST_UUID)
    }

    pub fn set_first_uuid(&self, value: Option<Uuid>) {
        self.holder.set(FIRST_UUID, value);
    }

    pub fn get_second_uuid(&self) -> Option<Uuid> {
        self.holder.get(SECOND_UUID)
    }

    pub fn set_second_uuid(&self, value: Option<Uuid>) {
        self.holder.set(SECOND_UUID, value);
    }
}

impl MetaAccessor for FoxMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoxVariant {
    Red,
    Snow,
}

impl TryFrom<i32> for FoxVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Red,
            1 => Self::Snow,
            _ => return Err(()),
        })
    }
}

impl DataType for FoxVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        FoxVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid FoxVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
