use crate::registry::RegistryKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[rustfmt::skip]
impl DimensionType {
    pub const OVERWORLD:        RegistryKey<DimensionType> = RegistryKey::const_vanilla("overworld");
    pub const OVERWORLD_CAVES:  RegistryKey<DimensionType> = RegistryKey::const_vanilla("overworld_caves");
    pub const THE_NETHER:       RegistryKey<DimensionType> = RegistryKey::const_vanilla("the_nether");
    pub const THE_END:          RegistryKey<DimensionType> = RegistryKey::const_vanilla("the_end");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionType {
    pub ambient_light: f32,
    pub attributes: DimensionAttributes,
    pub coordinate_scale: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_clock: Option<String>,
    pub has_ceiling: bool,
    pub has_ender_dragon_fight: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fixed_time: Option<bool>,
    pub has_skylight: bool,
    pub height: i32,
    pub infiniburn: String,
    pub logical_height: i32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: i32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub timelines: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skybox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinal_light: Option<String>,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionAttributes {
    #[serde(rename = "minecraft:audio/ambient_sounds", skip_serializing_if = "Option::is_none")]
    pub ambient_sounds: Option<AmbientSounds>,
    #[serde(rename = "minecraft:audio/background_music", skip_serializing_if = "Option::is_none")]
    pub background_music: Option<HashMap<String, MusicSettings>>,
    #[serde(rename = "minecraft:gameplay/bed_rule", skip_serializing_if = "Option::is_none")]
    pub bed_rule: Option<BedRule>,
    #[serde(rename = "minecraft:gameplay/respawn_anchor_works", skip_serializing_if = "Option::is_none")]
    pub respawn_anchor_works: Option<bool>,
    #[serde(rename = "minecraft:gameplay/nether_portal_spawns_piglin", skip_serializing_if = "Option::is_none")]
    pub nether_portal_spawns_piglin: Option<bool>,
    #[serde(rename = "minecraft:gameplay/can_start_raid", skip_serializing_if = "Option::is_none")]
    pub can_start_raid: Option<bool>,
    #[serde(rename = "minecraft:gameplay/fast_lava", skip_serializing_if = "Option::is_none")]
    pub fast_lava: Option<bool>,
    #[serde(rename = "minecraft:gameplay/piglins_zombify", skip_serializing_if = "Option::is_none")]
    pub piglins_zombify: Option<bool>,
    #[serde(rename = "minecraft:gameplay/sky_light_level", skip_serializing_if = "Option::is_none")]
    pub sky_light_level: Option<f32>,
    #[serde(rename = "minecraft:gameplay/snow_golem_melts", skip_serializing_if = "Option::is_none")]
    pub snow_golem_melts: Option<bool>,
    #[serde(rename = "minecraft:gameplay/water_evaporates", skip_serializing_if = "Option::is_none")]
    pub water_evaporates: Option<bool>,
    #[serde(rename = "minecraft:visual/ambient_light_color", skip_serializing_if = "Option::is_none")]
    pub ambient_light_color: Option<String>,
    #[serde(rename = "minecraft:visual/cloud_color", skip_serializing_if = "Option::is_none")]
    pub cloud_color: Option<String>,
    #[serde(rename = "minecraft:visual/cloud_height", skip_serializing_if = "Option::is_none")]
    pub cloud_height: Option<f32>,
    #[serde(rename = "minecraft:visual/fog_color", skip_serializing_if = "Option::is_none")]
    pub fog_color: Option<String>,
    #[serde(rename = "minecraft:visual/sky_color", skip_serializing_if = "Option::is_none")]
    pub sky_color: Option<String>,
    #[serde(rename = "minecraft:visual/sky_light_color", skip_serializing_if = "Option::is_none")]
    pub sky_light_color: Option<String>,
    #[serde(rename = "minecraft:visual/sky_light_factor", skip_serializing_if = "Option::is_none")]
    pub sky_light_factor: Option<f32>,
    #[serde(rename = "minecraft:visual/fog_start_distance", skip_serializing_if = "Option::is_none")]
    pub fog_start_distance: Option<f32>,
    #[serde(rename = "minecraft:visual/fog_end_distance", skip_serializing_if = "Option::is_none")]
    pub fog_end_distance: Option<f32>,
    #[serde(rename = "minecraft:visual/default_dripstone_particle", skip_serializing_if = "Option::is_none")]
    pub default_dripstone_particle: Option<Particle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientSounds {
    pub mood: Mood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    pub block_search_extent: i32,
    pub offset: f32,
    pub sound: String,
    pub tick_delay: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundMusic {
    pub default: MusicSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSettings {
    pub max_delay: i32,
    pub min_delay: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_current_music: Option<bool>,
    pub sound: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRule {
    pub can_set_spawn: String,
    pub can_sleep: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explodes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Int(i32),
    Uniform {
        #[serde(rename = "type")]
        kind: String,
        max_inclusive: i32,
        min_inclusive: i32,
    },
}
