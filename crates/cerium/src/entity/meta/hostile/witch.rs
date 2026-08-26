use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::witch::IS_DRINKING_POTION};

pub struct WitchMeta {
    holder: MetadataHolder,
}

impl WitchMeta {
    pub fn is_drinking_potion(&self) -> bool {
        self.holder.get(IS_DRINKING_POTION)
    }

    pub fn set_drinking_potion(&self, value: bool) {
        self.holder.set(IS_DRINKING_POTION, value);
    }
}

impl MetaAccessor for WitchMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
