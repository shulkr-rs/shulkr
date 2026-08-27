pub(crate) mod block;
pub(crate) mod block_entity;
mod block_state;
mod property;

pub use block::*;
pub use block_entity::*;
pub use block_state::*;
pub use property::{BoolProperty, EnumProperty, IntProperty, Property};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl TryFrom<i32> for BlockFace {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Bottom,
            1 => Self::Top,
            2 => Self::North,
            3 => Self::South,
            4 => Self::West,
            5 => Self::East,
            _ => return Err(()),
        })
    }
}
