use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::util::{Axis, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rotation {
    #[default]
    None,
    #[serde(rename = "clockwise_90")]
    Clockwise90,
    #[serde(rename = "180")]
    Clockwise180,
    #[serde(rename = "counterclockwise_90")]
    CounterClockwise90,
}

impl Rotation {
    pub const ALL: [Rotation; 4] = [
        Rotation::None,
        Rotation::Clockwise90,
        Rotation::Clockwise180,
        Rotation::CounterClockwise90,
    ];

    pub const fn opposite(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Clockwise90 => Self::CounterClockwise90,
            Self::Clockwise180 => Self::Clockwise180,
            Self::CounterClockwise90 => Self::Clockwise90,
        }
    }

    pub const fn then(self, other: Self) -> Self {
        let steps = (self.steps() + other.steps()) % 4;
        match steps {
            0 => Self::None,
            1 => Self::Clockwise90,
            2 => Self::Clockwise180,
            _ => Self::CounterClockwise90,
        }
    }

    pub const fn steps(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Clockwise90 => 1,
            Self::Clockwise180 => 2,
            Self::CounterClockwise90 => 3,
        }
    }
}

/// A reflection across one horizontal axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mirror {
    #[default]
    None,
    LeftRight,
    FrontBack,
}

impl Mirror {
    pub const ALL: [Mirror; 3] = [Mirror::None, Mirror::LeftRight, Mirror::FrontBack];
}

/// The transforms stored per state, in the order the generated table holds them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    Clockwise90,
    Clockwise180,
    CounterClockwise90,
    LeftRight,
    FrontBack,
}

impl Transform {
    pub const COUNT: usize = 5;

    pub const fn index(self) -> usize {
        match self {
            Self::Clockwise90 => 0,
            Self::Clockwise180 => 1,
            Self::CounterClockwise90 => 2,
            Self::LeftRight => 3,
            Self::FrontBack => 4,
        }
    }

    pub const fn of_rotation(rotation: Rotation) -> Option<Self> {
        match rotation {
            Rotation::None => None,
            Rotation::Clockwise90 => Some(Self::Clockwise90),
            Rotation::Clockwise180 => Some(Self::Clockwise180),
            Rotation::CounterClockwise90 => Some(Self::CounterClockwise90),
        }
    }

    pub const fn of_mirror(mirror: Mirror) -> Option<Self> {
        match mirror {
            Mirror::None => None,
            Mirror::LeftRight => Some(Self::LeftRight),
            Mirror::FrontBack => Some(Self::FrontBack),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::block::{Block, Properties};

    #[test]
    fn a_symmetric_block_is_unchanged() {
        let stone = Block::STONE.default_state();

        for rotation in Rotation::ALL {
            assert_eq!(stone.rotate(rotation), stone);
        }
        for mirror in Mirror::ALL {
            assert_eq!(stone.mirror(mirror), stone);
        }
    }

    #[test]
    fn a_pillar_swaps_its_axis_on_a_quarter_turn() {
        let x = Block::OAK_LOG
            .default_state()
            .with(Properties::AXIS, Axis::X);
        let z = Block::OAK_LOG
            .default_state()
            .with(Properties::AXIS, Axis::Z);
        let y = Block::OAK_LOG
            .default_state()
            .with(Properties::AXIS, Axis::Y);

        assert_eq!(x.rotate(Rotation::Clockwise90), z);
        assert_eq!(z.rotate(Rotation::Clockwise90), x);
        assert_eq!(y.rotate(Rotation::Clockwise90), y);
        assert_eq!(x.rotate(Rotation::Clockwise180), x);
    }

    #[test]
    fn stairs_turn_and_reflect() {
        let north = Block::OAK_STAIRS
            .default_state()
            .with(Properties::HORIZONTAL_FACING, Direction::North);

        let east = north.rotate(Rotation::Clockwise90);
        assert_eq!(east.try_get(Properties::HORIZONTAL_FACING), Direction::East);

        let south = north.rotate(Rotation::Clockwise180);
        assert_eq!(
            south.try_get(Properties::HORIZONTAL_FACING),
            Direction::South
        );

        assert_eq!(east.mirror(Mirror::LeftRight), east);
        assert_eq!(
            east.mirror(Mirror::FrontBack)
                .try_get(Properties::HORIZONTAL_FACING),
            Direction::West
        );
    }

    #[test]
    fn rotations_compose_and_invert() {
        let north = Block::OAK_STAIRS
            .default_state()
            .with(Properties::HORIZONTAL_FACING, Direction::North);

        assert_eq!(
            north
                .rotate(Rotation::Clockwise90)
                .rotate(Rotation::CounterClockwise90),
            north
        );
        assert_eq!(
            Rotation::Clockwise90.then(Rotation::Clockwise90),
            Rotation::Clockwise180
        );
        assert_eq!(
            Rotation::Clockwise90.opposite(),
            Rotation::CounterClockwise90
        );
    }

    #[test]
    fn none_is_identity_for_every_state() {
        for state in crate::world::block::BlockState::all() {
            assert_eq!(state.rotate(Rotation::None), state);
            assert_eq!(state.mirror(Mirror::None), state);
        }
    }
}
