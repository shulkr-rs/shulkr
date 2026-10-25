use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSoundSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fall_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
}
