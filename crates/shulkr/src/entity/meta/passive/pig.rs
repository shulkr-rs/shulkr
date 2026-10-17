use serde::{Deserialize, Serialize};

use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::pig::{BOOST_TIME, SOUND_VARIANT, VARIANT},
    },
    registry::RegistryKey,
};

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

pub struct PigMeta {
    holder: MetadataHolder,
}

impl PigMeta {
    pub fn get_boost_time(&self) -> i32 {
        self.holder.get(BOOST_TIME)
    }

    pub fn set_boost_time(&self, value: i32) {
        self.holder.set(BOOST_TIME, value);
    }

    pub fn get_variant(&self) -> RegistryKey<PigVariant> {
        self.holder.get(VARIANT)
    }

    pub fn set_variant(&self, value: RegistryKey<PigVariant>) {
        self.holder.set(VARIANT, value);
    }

    pub fn get_sound_variant(&self) -> RegistryKey<PigSoundVariant> {
        self.holder.get(SOUND_VARIANT)
    }

    pub fn set_sound_variant(&self, value: RegistryKey<PigSoundVariant>) {
        self.holder.set(SOUND_VARIANT, value);
    }
}

impl MetaAccessor for PigMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
