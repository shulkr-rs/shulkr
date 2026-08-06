use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::raider::IS_CELEBRATING};

pub struct RaiderMeta {
    holder: MetadataHolder,
}

impl RaiderMeta {
    pub fn is_celebrating(&self) -> bool {
        self.holder.get(IS_CELEBRATING)
    }

    pub fn set_celebrating(&self, value: bool) {
        self.holder.set(IS_CELEBRATING, value);
    }
}

impl MetaAccessor for RaiderMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
