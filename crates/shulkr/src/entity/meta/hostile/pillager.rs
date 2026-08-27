use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::pillager::IS_CHARGING};

pub struct PillagerMeta {
    holder: MetadataHolder,
}

impl PillagerMeta {
    pub fn is_charging(&self) -> bool {
        self.holder.get(IS_CHARGING)
    }

    pub fn set_charging(&self, value: bool) {
        self.holder.set(IS_CHARGING, value);
    }
}

impl MetaAccessor for PillagerMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
