#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl EquipmentSlot {
    pub fn slot_id(&self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 0,
            Self::Boots => 0,
            Self::Leggings => 0,
            Self::Chestplate => 0,
            Self::Helmet => 0,
        }
    }
}
