use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(flatten)]
    pub params: Value,
}
