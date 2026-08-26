use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::wither::{CENTER_TARGET, INVULNERABLE_TIME, LEFT_TARGET, RIGHT_TARGET},
};

pub struct WitherMeta {
    holder: MetadataHolder,
}

impl WitherMeta {
    pub fn get_center_target(&self) -> i32 {
        self.holder.get(CENTER_TARGET)
    }

    pub fn set_center_target(&self, value: i32) {
        self.holder.set(CENTER_TARGET, value);
    }

    pub fn get_left_target(&self) -> i32 {
        self.holder.get(LEFT_TARGET)
    }

    pub fn set_left_target(&self, value: i32) {
        self.holder.set(LEFT_TARGET, value);
    }

    pub fn get_right_target(&self) -> i32 {
        self.holder.get(RIGHT_TARGET)
    }

    pub fn set_right_target(&self, value: i32) {
        self.holder.set(RIGHT_TARGET, value);
    }

    pub fn get_invulnerable_time(&self) -> i32 {
        self.holder.get(INVULNERABLE_TIME)
    }

    pub fn set_invulnerable_time(&self, value: i32) {
        self.holder.set(INVULNERABLE_TIME, value);
    }
}

impl MetaAccessor for WitherMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
