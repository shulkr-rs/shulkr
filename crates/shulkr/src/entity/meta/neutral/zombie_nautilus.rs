use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZombieNautilusVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
