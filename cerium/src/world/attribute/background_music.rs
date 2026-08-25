use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundMusic {
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default_music: Option<Music>,
    #[serde(rename = "creative", skip_serializing_if = "Option::is_none")]
    pub creative_music: Option<Music>,
    #[serde(rename = "underwater", skip_serializing_if = "Option::is_none")]
    pub underwater_music: Option<Music>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Music {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_current_music: Option<bool>,
}
