use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    pub assets: WolfAssets,
    pub baby_assets: WolfAssets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfAssets {
    wild: String,
    tame: String,
    angry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfSoundVariant {
    pub adult_sounds: WolfSounds,
    pub baby_sounds: WolfSounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfSounds {
    pub hurt_sound: String,
    pub pant_sound: String,
    pub whine_sound: String,
    pub ambient_sound: String,
    pub death_sound: String,
    pub growl_sound: String,
    pub step_sound: String,
}
