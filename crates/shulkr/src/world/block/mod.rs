#[allow(clippy::module_inception)]
pub(crate) mod block;
pub(crate) mod block_entity;
mod block_state;
mod property;

use shulkr_macros::Enumeration;

pub use block::*;
pub use block_entity::*;
pub use block_state::*;
pub use property::{BoolProperty, EnumProperty, IntProperty, Property};

#[derive(Enumeration)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}
