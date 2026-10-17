use super::{Aabb, BlockPosition, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundingBox {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

impl BoundingBox {
    pub const INFINITE: Self = Self {
        min_x: i32::MIN,
        min_y: i32::MIN,
        min_z: i32::MIN,
        max_x: i32::MAX,
        max_y: i32::MAX,
        max_z: i32::MAX,
    };

    pub const fn new(
        min_x: i32,
        min_y: i32,
        min_z: i32,
        max_x: i32,
        max_y: i32,
        max_z: i32,
    ) -> Self {
        let (min_x, max_x) = ordered(min_x, max_x);
        let (min_y, max_y) = ordered(min_y, max_y);
        let (min_z, max_z) = ordered(min_z, max_z);
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    pub const fn from_corners(a: BlockPosition, b: BlockPosition) -> Self {
        Self::new(a.x(), a.y(), a.z(), b.x(), b.y(), b.z())
    }

    pub const fn from_point(pos: BlockPosition) -> Self {
        Self::new(pos.x(), pos.y(), pos.z(), pos.x(), pos.y(), pos.z())
    }

    #[expect(clippy::too_many_arguments)]
    pub const fn orient_box(
        foot_x: i32,
        foot_y: i32,
        foot_z: i32,
        off_x: i32,
        off_y: i32,
        off_z: i32,
        width: i32,
        height: i32,
        depth: i32,
        direction: Direction,
    ) -> Self {
        match direction {
            Direction::North => Self::new(
                foot_x + off_x,
                foot_y + off_y,
                foot_z - depth + 1 + off_z,
                foot_x + width - 1 + off_x,
                foot_y + height - 1 + off_y,
                foot_z + off_z,
            ),
            Direction::West => Self::new(
                foot_x - depth + 1 + off_z,
                foot_y + off_y,
                foot_z + off_x,
                foot_x + off_z,
                foot_y + height - 1 + off_y,
                foot_z + width - 1 + off_x,
            ),
            Direction::East => Self::new(
                foot_x + off_z,
                foot_y + off_y,
                foot_z + off_x,
                foot_x + depth - 1 + off_z,
                foot_y + height - 1 + off_y,
                foot_z + width - 1 + off_x,
            ),
            _ => Self::new(
                foot_x + off_x,
                foot_y + off_y,
                foot_z + off_z,
                foot_x + width - 1 + off_x,
                foot_y + height - 1 + off_y,
                foot_z + depth - 1 + off_z,
            ),
        }
    }

    pub const fn min_x(self) -> i32 {
        self.min_x
    }

    pub const fn min_y(self) -> i32 {
        self.min_y
    }

    pub const fn min_z(self) -> i32 {
        self.min_z
    }

    pub const fn max_x(self) -> i32 {
        self.max_x
    }

    pub const fn max_y(self) -> i32 {
        self.max_y
    }

    pub const fn max_z(self) -> i32 {
        self.max_z
    }

    pub const fn x_span(self) -> i32 {
        self.max_x - self.min_x + 1
    }

    pub const fn y_span(self) -> i32 {
        self.max_y - self.min_y + 1
    }

    pub const fn z_span(self) -> i32 {
        self.max_z - self.min_z + 1
    }

    pub const fn center(self) -> BlockPosition {
        BlockPosition::new(
            self.min_x + (self.max_x - self.min_x + 1) / 2,
            self.min_y + (self.max_y - self.min_y + 1) / 2,
            self.min_z + (self.max_z - self.min_z + 1) / 2,
        )
    }

    pub const fn is_inside(self, pos: BlockPosition) -> bool {
        pos.x() >= self.min_x
            && pos.x() <= self.max_x
            && pos.y() >= self.min_y
            && pos.y() <= self.max_y
            && pos.z() >= self.min_z
            && pos.z() <= self.max_z
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.max_x >= other.min_x
            && self.min_x <= other.max_x
            && self.max_z >= other.min_z
            && self.min_z <= other.max_z
            && self.max_y >= other.min_y
            && self.min_y <= other.max_y
    }

    pub const fn intersects_xz(self, min_x: i32, min_z: i32, max_x: i32, max_z: i32) -> bool {
        self.max_x >= min_x && self.min_x <= max_x && self.max_z >= min_z && self.min_z <= max_z
    }

    pub const fn encapsulate(self, pos: BlockPosition) -> Self {
        Self {
            min_x: min(self.min_x, pos.x()),
            min_y: min(self.min_y, pos.y()),
            min_z: min(self.min_z, pos.z()),
            max_x: max(self.max_x, pos.x()),
            max_y: max(self.max_y, pos.y()),
            max_z: max(self.max_z, pos.z()),
        }
    }

    pub const fn encapsulate_box(self, other: Self) -> Self {
        Self {
            min_x: min(self.min_x, other.min_x),
            min_y: min(self.min_y, other.min_y),
            min_z: min(self.min_z, other.min_z),
            max_x: max(self.max_x, other.max_x),
            max_y: max(self.max_y, other.max_y),
            max_z: max(self.max_z, other.max_z),
        }
    }

    pub const fn inflated_by(self, amount: i32) -> Self {
        self.inflated_by_xyz(amount, amount, amount)
    }

    pub const fn inflated_by_xyz(self, dx: i32, dy: i32, dz: i32) -> Self {
        Self {
            min_x: self.min_x - dx,
            min_y: self.min_y - dy,
            min_z: self.min_z - dz,
            max_x: self.max_x + dx,
            max_y: self.max_y + dy,
            max_z: self.max_z + dz,
        }
    }

    pub const fn moved(self, dx: i32, dy: i32, dz: i32) -> Self {
        Self {
            min_x: self.min_x + dx,
            min_y: self.min_y + dy,
            min_z: self.min_z + dz,
            max_x: self.max_x + dx,
            max_y: self.max_y + dy,
            max_z: self.max_z + dz,
        }
    }

    pub fn for_all_corners(self, mut consumer: impl FnMut(BlockPosition)) {
        for z in [self.min_z, self.max_z] {
            for y in [self.min_y, self.max_y] {
                for x in [self.min_x, self.max_x] {
                    consumer(BlockPosition::new(x, y, z));
                }
            }
        }
    }
}

impl From<BoundingBox> for Aabb {
    fn from(bounds: BoundingBox) -> Self {
        Self::new(
            f64::from(bounds.min_x),
            f64::from(bounds.min_y),
            f64::from(bounds.min_z),
            f64::from(bounds.max_x + 1),
            f64::from(bounds.max_y + 1),
            f64::from(bounds.max_z + 1),
        )
    }
}

const fn ordered(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }
}

const fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

const fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_in_either_order_give_the_same_box() {
        let a = BoundingBox::new(5, 5, 5, 0, 0, 0);
        let b = BoundingBox::new(0, 0, 0, 5, 5, 5);
        assert_eq!(a, b);
        assert_eq!((a.min_x(), a.min_y(), a.min_z()), (0, 0, 0));
        assert_eq!((a.max_x(), a.max_y(), a.max_z()), (5, 5, 5));
    }

    #[test]
    fn aabb_covers_the_max_block() {
        let aabb = Aabb::from(BoundingBox::new(-2, 0, 3, 1, 4, 3));
        assert_eq!((aabb.min_x(), aabb.min_y(), aabb.min_z()), (-2.0, 0.0, 3.0));
        assert_eq!((aabb.max_x(), aabb.max_y(), aabb.max_z()), (2.0, 5.0, 4.0));
    }
}
