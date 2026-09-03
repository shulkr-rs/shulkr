use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::villager::{DATA, DATA_FINALIZED},
    },
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};
use shulkr_macros::{DataType, Enumeration};

pub struct VillagerMeta {
    holder: MetadataHolder,
}

impl VillagerMeta {
    pub fn get_data(&self) -> VillagerData {
        self.holder.get(DATA)
    }

    pub fn set_data(&self, value: VillagerData) {
        self.holder.set(DATA, value);
    }

    pub fn is_data_finalized(&self) -> bool {
        self.holder.get(DATA_FINALIZED)
    }

    pub fn set_data_finalized(&self, value: bool) {
        self.holder.set(DATA_FINALIZED, value);
    }
}

impl MetaAccessor for VillagerMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType, Default)]
pub enum VillagerVariant {
    Desert,
    Jungle,
    #[default]
    Plains,
    Savanna,
    Snow,
    Swamp,
    Taiga,
}

#[derive(Clone, Copy)]
pub struct VillagerData {
    ty: VillagerVariant,
    profession: i32,
    level: i32,
}

impl Default for VillagerData {
    fn default() -> Self {
        Self::new()
    }
}

impl VillagerData {
    pub const fn new() -> Self {
        Self {
            ty: VillagerVariant::Plains,
            profession: 0,
            level: 0,
        }
    }
}

impl DataType for VillagerData {
    fn decode<R: PacketRead>(r: &mut R) -> Result<VillagerData, DecodeError> {
        Ok(Self {
            ty: VillagerVariant::try_from(r.read_varint()?)?,
            profession: r.read_varint()?,
            level: r.read_varint()?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &VillagerData) -> Result<(), EncodeError> {
        w.write_varint(this.ty as i32)?;
        w.write_varint(this.profession)?;
        w.write_varint(this.level)?;
        Ok(())
    }
}
