use crate::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub description: TextComponent,
    pub range: f32,
    pub sound_event: String,
    pub use_duration: f32,
}
