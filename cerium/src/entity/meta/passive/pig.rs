use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PigVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub baby_asset_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PigSoundVariant {
    adult_sounds: PigSoundSet,
    baby_sounds: PigSoundSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PigSoundSet {
    ambient_sound: String,
    hurt_sound: String,
    eat_sound: String,
    death_sound: String,
    step_sound: String,
}
