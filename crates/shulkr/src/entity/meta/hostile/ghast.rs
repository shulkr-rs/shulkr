use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::ghast::IS_ATTACKING};

pub struct GhastMeta {
    holder: MetadataHolder,
}

impl GhastMeta {
    pub fn is_attacking(&self) -> bool {
        self.holder.get(IS_ATTACKING)
    }

    pub fn set_attacking(&self, value: bool) {
        self.holder.set(IS_ATTACKING, value);
    }
}

impl MetaAccessor for GhastMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
