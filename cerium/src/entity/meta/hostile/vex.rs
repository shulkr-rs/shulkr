use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::vex::IS_ATTACKING};

pub struct VexMeta {
    holder: MetadataHolder,
}

impl VexMeta {
    pub fn is_attacking(&self) -> bool {
        self.holder.get(IS_ATTACKING)
    }

    pub fn set_attacking(&self, value: bool) {
        self.holder.set(IS_ATTACKING, value);
    }
}

impl MetaAccessor for VexMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
