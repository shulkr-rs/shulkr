use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::entity::{FLYING_WITH_ELYTRA, ON_FIRE, POSE, SNEAKING, SPRINTING, SWIMMING},
    },
    util::EntityPose,
};

pub struct EntityMeta {
    pub holder: MetadataHolder,
}

impl EntityMeta {
    pub fn is_on_fire(&self) -> bool {
        self.holder.get(ON_FIRE)
    }

    pub fn set_on_fire(&self, value: bool) {
        self.holder.set(ON_FIRE, value);
    }

    pub fn is_sneaking(&self) -> bool {
        self.holder.get(SNEAKING)
    }

    pub fn set_sneaking(&self, value: bool) {
        self.holder.set(SNEAKING, value);
    }

    pub fn is_swimming(&self) -> bool {
        self.holder.get(SWIMMING)
    }

    pub fn set_swimming(&self, value: bool) {
        self.holder.set(SWIMMING, value);
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.holder.get(FLYING_WITH_ELYTRA)
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        self.holder.set(FLYING_WITH_ELYTRA, value);
    }

    pub fn is_sprinting(&self) -> bool {
        self.holder.get(SPRINTING)
    }

    pub fn set_sprinting(&self, value: bool) {
        self.holder.set(SPRINTING, value);
    }

    pub fn set_pose(&self, pose: EntityPose) {
        self.holder.set(POSE, pose);
    }

    pub fn get_pose(&self) -> EntityPose {
        self.holder.get(POSE)
    }
}

impl MetaAccessor for EntityMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
