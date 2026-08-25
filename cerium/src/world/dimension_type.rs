use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::world::attribute::EnvironmentAttributeMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionType {
    pub ambient_light: f32,
    pub attributes: EnvironmentAttributeMap,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    #[serde(rename = "type")]
    pub kind: Cow<'static, str>,
    #[serde(flatten, default)]
    pub options: BTreeMap<String, serde_json::Value>,
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
