use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::copper_golem::{COPPER_GOLEM_STATE, WEATHERING_COPPER_STATE},
};
use shulkr_macros::{DataType, Enumeration};

pub struct CopperGolemMeta {
    holder: MetadataHolder,
}

impl CopperGolemMeta {
    pub fn get_weathering_state(&self) -> WeatheringCopperState {
        self.holder.get(WEATHERING_COPPER_STATE)
    }

    pub fn set_weathering_state(&self, value: WeatheringCopperState) {
        self.holder.set(WEATHERING_COPPER_STATE, value);
    }

    pub fn get_golem_state(&self) -> CopperGolemState {
        self.holder.get(COPPER_GOLEM_STATE)
    }

    pub fn set_golem_state(&self, value: CopperGolemState) {
        self.holder.set(COPPER_GOLEM_STATE, value);
    }
}

impl MetaAccessor for CopperGolemMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum CopperGolemState {
    Idle,
    GettingItem,
    GettingNoItem,
    DroppingItem,
    DroppingNoItem,
}

#[derive(Enumeration, DataType)]
pub enum WeatheringCopperState {
    Unaffected,
    Exposed,
    Weathered,
    Oxidized,
}
