use shulkr_macros::{DataType, Enumeration};

use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::tropical_fish::VARIANT};

pub struct TropicalFishMeta {
    holder: MetadataHolder,
}

impl TropicalFishMeta {
    pub fn get_pattern(&self) -> TropicalFishPattern {
        TropicalFishPattern::try_from(self.holder.get(VARIANT)).unwrap_or(TropicalFishPattern::Kob)
    }

    pub fn set_pattern(&self, value: TropicalFishPattern) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for TropicalFishMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum TropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
}
