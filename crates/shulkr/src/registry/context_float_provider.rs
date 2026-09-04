use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextFloatProvider {
    Constant(f32),
    Provider {
        #[serde(rename = "type")]
        kind: String,
        #[serde(flatten)]
        params: Value,
    },
}
