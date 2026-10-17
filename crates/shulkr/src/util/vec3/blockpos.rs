use super::{Point, Position, Velocity, impl_vector3_ops};
use crate::util::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

    pub fn relative(&self, direction: Direction) -> Self {
        let relative = match direction {
            Direction::Down => [0, -1, 0],
            Direction::Up => [0, 1, 0],
            Direction::North => [0, 0, -1],
            Direction::South => [0, 0, 1],
            Direction::West => [-1, 0, 0],
            Direction::East => [1, 0, 0],
        };
        *self + relative
    }

    pub const fn offset(&self, dx: i32, dy: i32, dz: i32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }

    pub const fn above(&self) -> Self {
        self.offset(0, 1, 0)
    }

    pub const fn above_n(&self, n: i32) -> Self {
        self.offset(0, n, 0)
    }

    pub const fn below(&self) -> Self {
        self.offset(0, -1, 0)
    }

    pub const fn below_n(&self, n: i32) -> Self {
        self.offset(0, -n, 0)
    }

    pub const fn north(&self) -> Self {
        self.offset(0, 0, -1)
    }

    pub const fn south(&self) -> Self {
        self.offset(0, 0, 1)
    }

    pub const fn west(&self) -> Self {
        self.offset(-1, 0, 0)
    }

    pub const fn east(&self) -> Self {
        self.offset(1, 0, 0)
    }

    pub const fn relative_n(&self, direction: Direction, n: i32) -> Self {
        let (dx, dy, dz) = direction.offset();
        self.offset(dx * n, dy * n, dz * n)
    }

    pub const fn at_y(&self, y: i32) -> Self {
        Self::new(self.x, y, self.z)
    }

    pub const fn within_manhattan(
        self,
        reach_x: i32,
        reach_y: i32,
        reach_z: i32,
    ) -> ManhattanBlockPositions {
        ManhattanBlockPositions {
            origin: self,
            reach_x,
            reach_y,
            reach_z,
            max_depth: reach_x + reach_y + reach_z,
            current_depth: 0,
            max_x: 0,
            max_y: 0,
            x: 0,
            y: 0,
            pending_z_mirror: None,
            done: false,
        }
    }
}

impl<'de> serde::Deserialize<'de> for BlockPosition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let [x, y, z] = <[i32; 3]>::deserialize(deserializer)?;
        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug, Clone)]
pub struct ManhattanBlockPositions {
    origin: BlockPosition,
    reach_x: i32,
    reach_y: i32,
    reach_z: i32,
    max_depth: i32,
    current_depth: i32,
    max_x: i32,
    max_y: i32,
    x: i32,
    y: i32,
    pending_z_mirror: Option<BlockPosition>,
    done: bool,
}

impl Iterator for ManhattanBlockPositions {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.pending_z_mirror.take() {
            return Some(pos);
        }
        if self.done {
            return None;
        }

        loop {
            if self.y > self.max_y {
                self.x += 1;
                if self.x > self.max_x {
                    self.current_depth += 1;
                    if self.current_depth > self.max_depth {
                        self.done = true;
                        return None;
                    }

                    self.max_x = self.reach_x.min(self.current_depth);
                    self.x = -self.max_x;
                }

                self.max_y = self.reach_y.min(self.current_depth - self.x.abs());
                self.y = -self.max_y;
            }

            let x = self.x;
            let y = self.y;
            let z = self.current_depth - x.abs() - y.abs();
            self.y += 1;
            if z > self.reach_z {
                continue;
            }

            let pos = self.origin.offset(x, y, z);
            if z != 0 {
                self.pending_z_mirror = Some(self.origin.offset(x, y, -z));
            }
            return Some(pos);
        }
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

impl From<BlockPosition> for [i32; 3] {
    fn from(position: BlockPosition) -> Self {
        [position.x, position.y, position.z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_offsets_are_unit_steps() {
        let origin = BlockPosition::ZERO;
        assert_eq!(origin.above(), BlockPosition::new(0, 1, 0));
        assert_eq!(origin.below(), BlockPosition::new(0, -1, 0));
        assert_eq!(origin.north(), BlockPosition::new(0, 0, -1));
        assert_eq!(origin.south(), BlockPosition::new(0, 0, 1));
        assert_eq!(origin.west(), BlockPosition::new(-1, 0, 0));
        assert_eq!(origin.east(), BlockPosition::new(1, 0, 0));
    }

    #[test]
    fn within_manhattan_visits_every_position_in_reach_once() {
        let origin = BlockPosition::new(10, 64, -5);
        let positions: Vec<_> = origin.within_manhattan(2, 1, 2).collect();
        let unique: crate::util::HashSet<_> = positions.iter().copied().collect();
        assert_eq!(positions.len(), unique.len());
        assert_eq!(positions[0], origin);
        assert!(positions.iter().all(|pos| {
            let delta = *pos - origin;
            delta.x().abs() <= 2 && delta.y().abs() <= 1 && delta.z().abs() <= 2
        }));
    }
}
