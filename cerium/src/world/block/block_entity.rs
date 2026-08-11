use cerium_nbt::Nbt;

include!("../../registry/generated/block_entity_types.rs");

pub struct BlockEntityTypeData;

#[derive(Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub r#type: i32,
    pub data: Option<Nbt>,
}
