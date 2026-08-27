use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::mob::{IS_AGGRESSIVE, IS_LEFT_HANDED, NO_AI},
};

pub struct MobMeta {
    holder: MetadataHolder,
}

impl MobMeta {
    pub fn has_no_ai(&self) -> bool {
        self.holder.get(NO_AI)
    }

    pub fn set_no_ai(&self, value: bool) {
        self.holder.set(NO_AI, value);
    }

    pub fn is_left_handed(&self) -> bool {
        self.holder.get(IS_LEFT_HANDED)
    }

    pub fn set_left_handed(&self, value: bool) {
        self.holder.set(IS_LEFT_HANDED, value);
    }

    pub fn is_aggressive(&self) -> bool {
        self.holder.get(IS_AGGRESSIVE)
    }

    pub fn set_aggressive(&self, value: bool) {
        self.holder.set(IS_AGGRESSIVE, value);
    }
}

impl MetaAccessor for MobMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
