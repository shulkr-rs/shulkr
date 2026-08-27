use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub clock: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_ticks: Option<u64>,
    pub tracks: HashMap<String, Track>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_markers: Option<HashMap<String, TimeMarker>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub keyframes: Vec<Keyframe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease: Option<Ease>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub ticks: u64,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ease {
    Named(String),
    CubicBezier { cubic_bezier: [f32; 4] },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimeMarker {
    Simple(u64),
    Detailed { show_in_commands: bool, ticks: u64 },
}
