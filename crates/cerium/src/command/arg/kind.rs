use crate::util::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringBehaviour {
    /// A single whitespace-delimited word.
    SingleWord,
    /// A single word, or a `"quoted phrase"`.
    QuotablePhrase,
    /// The entire remaining input.
    GreedyPhrase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgKind {
    /// `brigadier:bool`.
    Bool,
    /// `brigadier:integer`.
    Integer { min: Option<i32>, max: Option<i32> },
    /// `brigadier:long`.
    Long { min: Option<i64>, max: Option<i64> },
    /// `brigadier:float`.
    Float { min: Option<f32>, max: Option<f32> },
    /// `brigadier:double`.
    Double { min: Option<f64>, max: Option<f64> },
    /// `brigadier:string`.
    String(StringBehaviour),
    /// `minecraft:int_range`.
    IntRange,
    /// `minecraft:float_range`.
    FloatRange,
    /// `minecraft:game_mode`.
    GameMode,
    /// `minecraft:entity`
    Entity { single: bool, players_only: bool },
    /// `minecraft:score_holder`.
    ScoreHolder { multiple: bool },
    /// `minecraft:time`.
    Time { min: i32 },
    /// `minecraft:resource`
    Resource { registry: Key },
    /// `minecraft:resource_key`.
    ResourceKey { registry: Key },
    /// `minecraft:resource_or_tag`.
    ResourceOrTag { registry: Key },
    /// `minecraft:resource_or_tag_key`.
    ResourceOrTagKey { registry: Key },
    /// `minecraft:resource_selector`.
    ResourceSelector { registry: Key },
}

impl ArgKind {
    pub fn parser_id(&self) -> i32 {
        match self {
            ArgKind::Bool => 0,
            ArgKind::Float { .. } => 1,
            ArgKind::Double { .. } => 2,
            ArgKind::Integer { .. } => 3,
            ArgKind::Long { .. } => 4,
            ArgKind::String(_) => 5,
            ArgKind::Entity { .. } => 6,
            ArgKind::ScoreHolder { .. } => 31,
            ArgKind::IntRange => 39,
            ArgKind::FloatRange => 40,
            ArgKind::GameMode => 42,
            ArgKind::Time { .. } => 43,
            ArgKind::ResourceOrTag { .. } => 44,
            ArgKind::ResourceOrTagKey { .. } => 45,
            ArgKind::Resource { .. } => 46,
            ArgKind::ResourceKey { .. } => 47,
            ArgKind::ResourceSelector { .. } => 48,
        }
    }
}
