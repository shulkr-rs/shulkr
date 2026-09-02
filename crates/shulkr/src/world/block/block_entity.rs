use crate::registry::Id;
use shulkr_nbt::Nbt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockEntityType(Id);

include!("../../../generated/block_entity_types.rs");

impl From<BlockEntityType> for Id {
    #[inline]
    fn from(block_entity_type: BlockEntityType) -> Self {
        block_entity_type.0
    }
}

pub struct BlockEntityTypeData;

impl Default for BlockEntityTypeData {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockEntityTypeData {
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub r#type: i32,
    pub data: Option<Nbt>,
}
