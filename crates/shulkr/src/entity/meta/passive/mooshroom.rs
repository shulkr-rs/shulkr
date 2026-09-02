use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::mooshroom::VARIANT};
use shulkr_macros::{DataType, Enumeration};

pub struct MooshroomMeta {
    holder: MetadataHolder,
}

impl MooshroomMeta {
    pub fn get_variant(&self) -> MooshroomVariant {
        MooshroomVariant::try_from(self.holder.get(VARIANT)).unwrap_or(MooshroomVariant::Red)
    }

    pub fn set_variant(&self, value: MooshroomVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for MooshroomMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum MooshroomVariant {
    Red,
    Brown,
}
