use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::zoglin::IS_BABY};

pub struct ZoglinMeta {
    holder: MetadataHolder,
}

impl ZoglinMeta {
    pub fn is_baby(&self) -> bool {
        self.holder.get(IS_BABY)
    }

    pub fn set_baby(&self, value: bool) {
        self.holder.set(IS_BABY, value);
    }
}

impl MetaAccessor for ZoglinMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
