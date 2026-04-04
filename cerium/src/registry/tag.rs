use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags {
    #[serde(flatten)]
    pub tags: HashMap<String, Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    values: Vec<String>,
}
