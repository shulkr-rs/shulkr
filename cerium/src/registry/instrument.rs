use serde::{Deserialize, Serialize};

use crate::text::TextComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub description: TextComponent,
    pub range: f32,
    pub sound_event: String,
    pub use_duration: f32,
}
