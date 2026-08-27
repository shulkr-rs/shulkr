use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub baby_asset_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatSoundVariant {
    adult_sounds: CatSoundSet,
    baby_sounds: CatSoundSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatSoundSet {
    ambient_sound: String,
    beg_for_food_sound: String,
    eat_sound: String,
    hiss_sound: String,
    purr_sound: String,
    hurt_sound: String,
    death_sound: String,
    purreow_sound: String,
    stray_ambient_sound: String,
}
