use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::parrot::VARIANT};
use shulkr_macros::{DataType, Enumeration};

pub struct ParrotMeta {
    holder: MetadataHolder,
}

impl ParrotMeta {
    pub fn get_variant(&self) -> ParrotVariant {
        ParrotVariant::try_from(self.holder.get(VARIANT)).unwrap_or_default()
    }

    pub fn set_variant(&self, value: ParrotVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for ParrotMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType, Default)]
pub enum ParrotVariant {
    #[default]
    RedBlue,
    Blue,
    Green,
    YellowBlue,
    Grey,
}
