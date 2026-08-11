use crate::{
    registry::{Id, Registries},
    util::Key,
};

include!("../registry/generated/entity_types.rs");

pub struct EntityTypeData;

impl EntityType {
    pub fn from_id(id: Id) -> Option<EntityType> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<EntityType> {
        Registries::ENTITY_TYPE.by_key(&key).copied()
    }
}

impl TryFrom<Id> for EntityType {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::all().get(value as usize).copied().ok_or(())
    }
}
