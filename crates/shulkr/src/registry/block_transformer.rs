use crate::util::Direction;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTransformer {
    pub block_state_provider: BlockStateProvider,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disallowed_faces: Vec<Direction>,
    pub item_damage_per_use: i32,
    pub sound: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStateProvider {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(flatten)]
    pub params: Value,
}
