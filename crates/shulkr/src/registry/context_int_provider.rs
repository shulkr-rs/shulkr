use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextIntProvider {
    Constant(i32),
    Provider {
        #[serde(rename = "type")]
        kind: String,
        #[serde(flatten)]
        params: Value,
    },
}
