use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::strider::{IS_SHAKING, TIME_TO_BOOST},
};

pub struct StriderMeta {
    holder: MetadataHolder,
}

impl StriderMeta {
    pub fn get_time_to_boost(&self) -> i32 {
        self.holder.get(TIME_TO_BOOST)
    }

    pub fn set_time_to_boost(&self, value: i32) {
        self.holder.set(TIME_TO_BOOST, value);
    }

    pub fn is_shaking(&self) -> bool {
        self.holder.get(IS_SHAKING)
    }

    pub fn set_shaking(&self, value: bool) {
        self.holder.set(IS_SHAKING, value);
    }
}

impl MetaAccessor for StriderMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
