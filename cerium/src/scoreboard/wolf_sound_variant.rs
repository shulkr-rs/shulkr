use cerium_nbt::{Nbt, NbtCompound, ToNbt};
use serde::{Deserialize, Serialize};

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
