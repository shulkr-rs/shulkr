use crate::{
    registry::{Id, Registries},
    util::Key,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityType(Id);

include!("../../generated/entity_types.rs");

pub struct EntityTypeData;

impl EntityTypeData {
    pub const fn new() -> Self {
        Self
    }
}

impl EntityType {
    pub fn from_id(id: Id) -> Option<EntityType> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<EntityType> {
        Registries::ENTITY_TYPE.by_key(&key).copied()
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
        if (value as usize) < Registries::ENTITY_TYPE.len() {
            Ok(EntityType(value))
        } else {
            Err(())
        }
    }
}
