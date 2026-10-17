use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratedPotPattern {
    pub asset_id: String,
}
