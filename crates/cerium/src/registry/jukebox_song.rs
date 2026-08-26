use serde::{Deserialize, Serialize};

use crate::text::TextComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JukeboxSong {
    pub comparator_output: i32,
    pub description: TextComponent,
    pub length_in_seconds: f32,
    pub sound_event: String,
}
