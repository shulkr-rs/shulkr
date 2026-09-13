use crate::entity::EntityDimensions;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VoxelShape {
    boxes: &'static [&'static Aabb],
}

impl VoxelShape {
    pub const EMPTY: VoxelShape = VoxelShape::new(&[]);
    pub const FULL: VoxelShape = VoxelShape::new(&[&Aabb::FULL]);

    pub const fn new(boxes: &'static [&'static Aabb]) -> Self {
        Self { boxes }
    }

    pub fn intersects(&self, entity: &EntityDimensions, x: f32, y: f32, z: f32) -> bool {
        let entity_aabb = entity.aabb(x, y, z);

        self.boxes.iter().any(|aabb| entity_aabb.intersects(aabb))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Aabb {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
}

impl Aabb {
    pub const EMPTY: Self = Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    pub const FULL: Self = Self::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    pub const fn new(
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
    ) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    pub const fn min_x(&self) -> f32 {
        self.min_x
    }

    pub const fn min_y(&self) -> f32 {
        self.min_y
    }

    pub const fn min_z(&self) -> f32 {
        self.min_z
    }

    pub const fn max_x(&self) -> f32 {
        self.max_x
    }

    pub const fn max_y(&self) -> f32 {
        self.max_y
    }

    pub const fn max_z(&self) -> f32 {
        self.max_z
    }

    pub const fn intersects(&self, other: &Aabb) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
            && self.min_z < other.max_z
            && self.max_z > other.min_z
    }
}
