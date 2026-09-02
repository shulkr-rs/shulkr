use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::horse::VARIANT};
use shulkr_macros::{DataType, Enumeration};

pub struct HorseMeta {
    holder: MetadataHolder,
}

impl HorseMeta {
    pub fn get_variant(&self) -> HorseVariant {
        HorseVariant::try_from(self.holder.get(VARIANT)).unwrap_or(HorseVariant::White)
    }

    pub fn set_variant(&self, value: HorseVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for HorseMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum HorseVariant {
    White,
    Creamy,
    Chestnut,
    Brown,
    Black,
    Gray,
    DarkBrown,
}
