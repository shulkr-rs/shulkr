use crate::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JukeboxSong {
    pub comparator_output: i32,
    pub description: TextComponent,
    pub length_in_seconds: f32,
    pub sound_event: String,
}
