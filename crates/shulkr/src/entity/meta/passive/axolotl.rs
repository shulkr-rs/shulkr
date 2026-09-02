use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::axolotl::{PLAYING_DEAD, SPAWNED_FROM_BUCKET, VARIANT},
};
use shulkr_macros::{DataType, Enumeration};

pub struct AxolotlMeta {
    holder: MetadataHolder,
}

impl AxolotlMeta {
    pub fn get_variant(&self) -> AxolotlVariant {
        AxolotlVariant::try_from(self.holder.get(VARIANT)).unwrap_or(AxolotlVariant::Lucy)
    }

    pub fn set_variant(&self, value: AxolotlVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn is_playing_dead(&self) -> bool {
        self.holder.get(PLAYING_DEAD)
    }

    pub fn set_playing_dead(&self, value: bool) {
        self.holder.set(PLAYING_DEAD, value);
    }

    pub fn is_from_bucket(&self) -> bool {
        self.holder.get(SPAWNED_FROM_BUCKET)
    }

    pub fn set_from_bucket(&self, value: bool) {
        self.holder.set(SPAWNED_FROM_BUCKET, value);
    }
}

impl MetaAccessor for AxolotlMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum AxolotlVariant {
    Lucy,
    Wild,
    Gold,
    Cyan,
    Blue,
}
