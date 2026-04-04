use cerium_nbt::Nbt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityInfo {
    pub namespace: String,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub r#type: i32,
    pub data: Option<Nbt>,
}
