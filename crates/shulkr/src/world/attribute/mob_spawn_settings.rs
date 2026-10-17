use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MobSpawnSettings {
    #[serde(default)]
    pub spawns_by_category: BTreeMap<String, Vec<SpawnerData>>,
    #[serde(default)]
    pub spawn_costs: BTreeMap<String, SpawnCost>,
}

impl MobSpawnSettings {
    pub const fn empty() -> Self {
        Self {
            spawns_by_category: BTreeMap::new(),
            spawn_costs: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnerData {
    #[serde(rename = "type")]
    pub kind: String,
    pub count: SpawnCount,
    pub weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpawnCount {
    Int(i32),
    Uniform {
        #[serde(rename = "type")]
        kind: String,
        max_inclusive: i32,
        min_inclusive: i32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnCost {
    pub energy_budget: f32,
    pub charge: f32,
}
