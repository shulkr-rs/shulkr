use serde::{Deserialize, Serialize};

use crate::text::TextComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRule {
    pub can_sleep: BedRuleKind,
    pub can_set_spawn: BedRuleKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explodes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<TextComponent>,
}

impl BedRule {
    pub fn can_sleep_when_dark() -> Self {
        Self {
            can_sleep: BedRuleKind::WhenDark,
            can_set_spawn: BedRuleKind::Always,
            explodes: None,
            error_message: None,
        }
    }

    pub fn explodes() -> Self {
        Self {
            can_sleep: BedRuleKind::Never,
            can_set_spawn: BedRuleKind::Never,
            explodes: Some(true),
            error_message: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BedRuleKind {
    Always,
    WhenDark,
    Never,
}
