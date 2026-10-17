use shulkr_macros::{DataType, Enumeration};

#[derive(Enumeration, DataType)]
pub enum SoundCategory {
    Master,
    Music,
    Records,
    Weather,
    Blocks,
    Hostile,
    Neutral,
    Players,
    Ambient,
    Voice,
}
