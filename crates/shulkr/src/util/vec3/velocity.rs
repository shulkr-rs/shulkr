use super::{BlockPosition, Point, Position, impl_vector3_dot_ops, impl_vector3_ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

impl Velocity {
    pub const ZERO: Velocity = Velocity::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl_vector3_ops!(Velocity, f64);
impl_vector3_dot_ops!(Velocity, f64);

impl<A> From<[A; 3]> for Velocity
where
    A: Into<f64>,
{
    fn from(value: [A; 3]) -> Self {
        let [x, y, z] = value;
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl From<Option<Velocity>> for Velocity {
    fn from(value: Option<Velocity>) -> Self {
        value.unwrap_or(Self::ZERO)
    }
}

impl From<Point> for Velocity {
    fn from(value: Point) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<Position> for Velocity {
    fn from(value: Position) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<BlockPosition> for Velocity {
    fn from(value: BlockPosition) -> Self {
        Self::new(value.x() as f64, value.y() as f64, value.z() as f64)
    }
}
