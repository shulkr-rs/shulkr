use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::rabbit::VARIANT};
use shulkr_macros::{DataType, Enumeration};

pub struct RabbitMeta {
    holder: MetadataHolder,
}

impl RabbitMeta {
    pub fn get_variant(&self) -> RabbitVariant {
        RabbitVariant::try_from(self.holder.get(VARIANT)).unwrap_or_default()
    }

    pub fn set_variant(&self, value: RabbitVariant) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for RabbitMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType, Default)]
pub enum RabbitVariant {
    #[default]
    Brown,
    White,
    Black,
    WhiteSplotched,
    Gold,
    Salt,
    Evil,
}
