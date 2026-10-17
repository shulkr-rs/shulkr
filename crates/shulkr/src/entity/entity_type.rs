use super::EntityDimensions;
use crate::{
    registry::{Id, registries},
    util::Key,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityType(Id);

include!("../../generated/entity_types.rs");

pub struct EntityTypeData {
    dimensions: EntityDimensions,
}

impl EntityTypeData {
    pub const fn new(dimensions: EntityDimensions) -> Self {
        Self { dimensions }
    }
}

impl EntityType {
    pub const fn dimensions(&self) -> EntityDimensions {
        self.data().dimensions
    }

    pub const fn width(&self) -> f32 {
        self.dimensions().width()
    }

    pub const fn height(&self) -> f32 {
        self.dimensions().height()
    }

    pub const fn eye_height(&self) -> f32 {
        self.dimensions().eye_height()
    }

    pub fn from_id(id: Id) -> Option<EntityType> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<EntityType> {
        registries::ENTITY_TYPE.by_key(&key).map(|entry| **entry)
    }
}

impl From<EntityType> for Id {
    #[inline]
    fn from(entity_type: EntityType) -> Self {
        entity_type.0
    }
}

impl TryFrom<Id> for EntityType {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        if (value as usize) < registries::ENTITY_TYPE.len() {
            Ok(EntityType(value))
        } else {
            Err(())
        }
    }
}
