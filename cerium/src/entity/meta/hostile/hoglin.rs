use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::hoglin::IS_IMMUNE_TO_ZOMBIFICATION};

pub struct HoglinMeta {
    holder: MetadataHolder,
}

impl HoglinMeta {
    pub fn is_immune_to_zombification(&self) -> bool {
        self.holder.get(IS_IMMUNE_TO_ZOMBIFICATION)
    }

    pub fn set_immune_to_zombification(&self, value: bool) {
        self.holder.set(IS_IMMUNE_TO_ZOMBIFICATION, value);
    }
}

impl MetaAccessor for HoglinMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
