use serde::{Deserialize, Serialize};
use shulkr_macros::Enumeration;

use crate::util::Axis;

#[derive(Enumeration, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    #[inline]
    pub fn from_yrot(y_rot: f64) -> Direction {
        match (y_rot / 90.0 + 0.5).floor() as i32 & 3 {
            0 => Direction::South,
            1 => Direction::West,
            2 => Direction::North,
            _ => Direction::East,
        }
    }

    #[inline]
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    #[inline]
    pub fn axis(&self) -> Axis {
        match self {
            Direction::Down => Axis::Y,
            Direction::Up => Axis::Y,
            Direction::North => Axis::Z,
            Direction::South => Axis::Z,
            Direction::West => Axis::X,
            Direction::East => Axis::X,
        }
    }
}
