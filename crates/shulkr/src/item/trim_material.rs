use crate::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimMaterial {
    pub palette_id: String,
    pub description: TextComponent,
}
