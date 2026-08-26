use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::living_entity::{
            ACTIVE_HAND, HEALTH, IS_HAND_ACTIVE, IS_IN_RIPTIDE_SPIN_ATTACK,
            IS_POTION_EFFECT_AMBIENT, LOCATION_OF_BED, NUMBER_OF_ARROWS, NUMBER_OF_BEE_STINGERS,
        },
    },
    util::BlockPosition,
};

pub struct LivingEntityMeta {
    holder: MetadataHolder,
}

impl LivingEntityMeta {
    pub fn is_hand_active(&self) -> bool {
        self.holder.get(IS_HAND_ACTIVE)
    }

    pub fn set_hand_active(&self, value: bool) {
        self.holder.set(IS_HAND_ACTIVE, value);
    }

    pub fn get_active_hand(&self) -> bool {
        self.holder.get(ACTIVE_HAND)
    }

    pub fn set_active_hand(&self, value: bool) {
        self.holder.set(ACTIVE_HAND, value);
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.holder.get(IS_IN_RIPTIDE_SPIN_ATTACK)
    }

    pub fn set_in_riptide_spin_attack(&self, value: bool) {
        self.holder.set(IS_IN_RIPTIDE_SPIN_ATTACK, value);
    }

    pub fn get_health(&self) -> f32 {
        self.holder.get(HEALTH)
    }

    pub fn set_health(&self, value: f32) {
        self.holder.set(HEALTH, value);
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        self.holder.get(IS_POTION_EFFECT_AMBIENT)
    }

    pub fn set_potion_effect_ambient(&self, value: bool) {
        self.holder.set(IS_POTION_EFFECT_AMBIENT, value);
    }

    pub fn get_number_of_arrows(&self) -> i32 {
        self.holder.get(NUMBER_OF_ARROWS)
    }

    pub fn set_number_of_arrows(&self, value: i32) {
        self.holder.set(NUMBER_OF_ARROWS, value);
    }

    pub fn get_number_of_bee_stingers(&self) -> i32 {
        self.holder.get(NUMBER_OF_BEE_STINGERS)
    }

    pub fn set_number_of_bee_stingers(&self, value: i32) {
        self.holder.set(NUMBER_OF_BEE_STINGERS, value);
    }

    pub fn get_bed_location(&self) -> Option<BlockPosition> {
        self.holder.get(LOCATION_OF_BED)
    }

    pub fn set_bed_location(&self, value: Option<BlockPosition>) {
        self.holder.set(LOCATION_OF_BED, value);
    }
}

impl MetaAccessor for LivingEntityMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
