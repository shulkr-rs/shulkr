use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::piglin::{IS_BABY, IS_CHARGING_CROSSBOW, IS_DANCING},
};

pub struct PiglinMeta {
    holder: MetadataHolder,
}

impl PiglinMeta {
    pub fn is_baby(&self) -> bool {
        self.holder.get(IS_BABY)
    }

    pub fn set_baby(&self, value: bool) {
        self.holder.set(IS_BABY, value);
    }

    pub fn is_charging_crossbow(&self) -> bool {
        self.holder.get(IS_CHARGING_CROSSBOW)
    }

    pub fn set_charging_crossbow(&self, value: bool) {
        self.holder.set(IS_CHARGING_CROSSBOW, value);
    }

    pub fn is_dancing(&self) -> bool {
        self.holder.get(IS_DANCING)
    }

    pub fn set_dancing(&self, value: bool) {
        self.holder.set(IS_DANCING, value);
    }
}

impl MetaAccessor for PiglinMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
