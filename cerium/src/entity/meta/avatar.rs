use crate::entity::{
    Hand,
    meta::{
        MetaAccessor, MetadataHolder,
        refs::avatar::{
            CAPE_ENABLED, HAT_ENABLED, JACKET_ENABLED, LEFT_PANTS_LEG_ENABLED, LEFT_SLEEVE_ENABLED,
            MAIN_HAND, RIGHT_PANTS_LEG_ENABLED, RIGHT_SLEEVE_ENABLED,
        },
    },
};

pub struct AvatarMeta {
    holder: MetadataHolder,
}

impl AvatarMeta {
    pub fn get_main_hand(&self) -> Hand {
        self.holder.get(MAIN_HAND)
    }

    pub fn set_main_hand(&self, value: Hand) {
        self.holder.set(MAIN_HAND, value);
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.holder.get(CAPE_ENABLED)
    }

    pub fn set_cape_enabled(&self, value: bool) {
        self.holder.set(CAPE_ENABLED, value);
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.holder.get(JACKET_ENABLED)
    }

    pub fn set_jacket_enabled(&self, value: bool) {
        self.holder.set(JACKET_ENABLED, value);
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.holder.get(LEFT_SLEEVE_ENABLED)
    }

    pub fn set_left_sleeve_enabled(&self, value: bool) {
        self.holder.set(LEFT_SLEEVE_ENABLED, value);
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.holder.get(RIGHT_SLEEVE_ENABLED)
    }

    pub fn set_right_sleeve_enabled(&self, value: bool) {
        self.holder.set(RIGHT_SLEEVE_ENABLED, value);
    }

    pub fn is_left_pants_leg_enabled(&self) -> bool {
        self.holder.get(LEFT_PANTS_LEG_ENABLED)
    }

    pub fn set_left_pants_leg_enabled(&self, value: bool) {
        self.holder.set(LEFT_PANTS_LEG_ENABLED, value);
    }

    pub fn is_right_pants_leg_enabled(&self) -> bool {
        self.holder.get(RIGHT_PANTS_LEG_ENABLED)
    }

    pub fn set_right_pants_leg_enabled(&self, value: bool) {
        self.holder.set(RIGHT_PANTS_LEG_ENABLED, value);
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.holder.get(HAT_ENABLED)
    }

    pub fn set_hat_enabled(&self, value: bool) {
        self.holder.set(HAT_ENABLED, value);
    }
}

impl MetaAccessor for AvatarMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
