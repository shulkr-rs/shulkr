use crate::{text::TextComponent, util::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimMaterial {
    pub asset_name: String,
    pub description: TextComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_armor_assets: Option<HashMap<String, String>>,
}
