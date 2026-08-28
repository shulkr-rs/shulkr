use shulkr_macros::{DataType, Enumeration};

#[derive(Enumeration, DataType)]
pub enum Hand {
    Main,
    Off,
}

#[derive(Enumeration, DataType)]
pub enum MainHand {
    Left,
    Right,
}
