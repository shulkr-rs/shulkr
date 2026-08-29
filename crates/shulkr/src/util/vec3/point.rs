use super::{BlockPosition, Position, Velocity, impl_vector3_dot_ops, impl_vector3_ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub const ZERO: Point = Point::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl_vector3_ops!(Point, f64);
impl_vector3_dot_ops!(Point, f64);

impl<A> From<(A, A, A)> for Point
where
    A: Into<f64>,
{
    fn from(value: (A, A, A)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}

impl<A> From<[A; 3]> for Point
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

impl From<Velocity> for Point {
    fn from(value: Velocity) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<Position> for Point {
    fn from(value: Position) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<BlockPosition> for Point {
    fn from(value: BlockPosition) -> Self {
        Self::new(value.x() as f64, value.y() as f64, value.z() as f64)
    }
}
