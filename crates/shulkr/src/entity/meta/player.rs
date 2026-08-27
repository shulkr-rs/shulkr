use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::player::{
        ADDITIONAL_HEARTS, LEFT_SHOULDER_ENTITY_DATA, RIGHT_SHOULDER_ENTITY_DATA, SCORE,
    },
};

pub struct PlayerMeta {
    holder: MetadataHolder,
}

impl PlayerMeta {
    pub fn get_additional_hearts(&self) -> f32 {
        self.holder.get(ADDITIONAL_HEARTS)
    }

    pub fn set_additional_hearts(&self, value: f32) {
        self.holder.set(ADDITIONAL_HEARTS, value);
    }

    pub fn get_score(&self) -> i32 {
        self.holder.get(SCORE)
    }

    pub fn set_score(&self, value: i32) {
        self.holder.set(SCORE, value);
    }

    pub fn get_left_shoulder_entity_data(&self) -> Option<i32> {
        self.holder.get(LEFT_SHOULDER_ENTITY_DATA)
    }

    pub fn set_left_shoulder_entity_data(&self, value: Option<i32>) {
        self.holder.set(LEFT_SHOULDER_ENTITY_DATA, value);
    }

    pub fn get_right_shoulder_entity_data(&self) -> Option<i32> {
        self.holder.get(RIGHT_SHOULDER_ENTITY_DATA)
    }

    pub fn set_right_shoulder_entity_data(&self, value: Option<i32>) {
        self.holder.set(RIGHT_SHOULDER_ENTITY_DATA, value);
    }
}

impl MetaAccessor for PlayerMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
