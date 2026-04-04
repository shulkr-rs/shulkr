use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerPattern {
    pub asset_id: String,
    pub translation_key: String,
}
