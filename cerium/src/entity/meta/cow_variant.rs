use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CowVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub baby_asset_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CowSoundVariant {
    ambient_sound: String,
    hurt_sound: String,
    death_sound: String,
    step_sound: String,
}
