use serde::{Deserialize, Serialize};
use shulkr_macros::{DataType, Enumeration};

#[derive(Enumeration, DataType, Serialize, Deserialize)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}
