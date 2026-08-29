use super::{Point, Position, Velocity, impl_vector3_ops};
use crate::world::block::BlockFace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl BlockPosition {
    pub const ZERO: BlockPosition = BlockPosition::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn from_long(value: i64) -> Self {
        let x = value >> 38;
        let y = value & 0xFFF;
        let z = (value >> 12) & 0x3FFFFFF;

        Self {
            x: (if x >= 0x2000000 { x - 0x4000000 } else { x }) as i32,
            y: (if y >= 0x800 { y - 0x1000 } else { y }) as i32,
            z: (if z >= 0x2000000 { z - 0x4000000 } else { z }) as i32,
        }
    }

    pub const fn distance_manhattan(&self, other: Self) -> i64 {
        (self.x - other.x).unsigned_abs() as i64
            + (self.y - other.y).unsigned_abs() as i64
            + (self.z - other.z).unsigned_abs() as i64
    }

    pub const fn distance_squared(&self, other: Self) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }

    pub fn distance(&self, other: Self) -> f64 {
        (self.distance_squared(other) as f64).sqrt()
    }

    pub fn relative(&self, face: BlockFace) -> Self {
        let relative = match face {
            BlockFace::Bottom => [0, -1, 0],
            BlockFace::Top => [0, 1, 0],
            BlockFace::North => [0, 0, -1],
            BlockFace::South => [0, 0, 1],
            BlockFace::West => [-1, 0, 0],
            BlockFace::East => [1, 0, 0],
        };
        *self + relative
    }
}

impl_vector3_ops!(BlockPosition, i32);

impl From<Position> for BlockPosition {
    fn from(value: Position) -> Self {
        BlockPosition::new(
            value.x().floor() as i32,
            value.y().floor() as i32,
            value.z().floor() as i32,
        )
    }
}

impl From<Point> for BlockPosition {
    fn from(value: Point) -> Self {
        BlockPosition::new(
            value.x().floor() as i32,
            value.y().floor() as i32,
            value.z().floor() as i32,
        )
    }
}

impl From<Velocity> for BlockPosition {
    fn from(value: Velocity) -> Self {
        BlockPosition::new(
            value.x().floor() as i32,
            value.y().floor() as i32,
            value.z().floor() as i32,
        )
    }
}

impl<A> From<[A; 3]> for BlockPosition
where
    A: Into<i32>,
{
    fn from(value: [A; 3]) -> Self {
        let [x, y, z] = value;
        Self::new(x.into(), y.into(), z.into())
    }
}
