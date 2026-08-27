use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::blaze::IS_ON_FIRE};

pub struct BlazeMeta {
    holder: MetadataHolder,
}

impl BlazeMeta {
    pub fn is_on_fire(&self) -> bool {
        self.holder.get(IS_ON_FIRE)
    }

    pub fn set_on_fire(&self, value: bool) {
        self.holder.set(IS_ON_FIRE, value);
    }
}

impl MetaAccessor for BlazeMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
