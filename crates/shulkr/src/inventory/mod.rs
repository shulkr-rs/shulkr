use std::{any::Any, sync::Arc};

#[allow(clippy::module_inception)]
mod inventory;
pub use inventory::*;

mod inventory_type;
pub use inventory_type::*;

mod player_inventory;
pub use player_inventory::{
    ARMOR_SLOTS, ARMOR_START, HOTBAR_SLOTS, HOTBAR_START, MAIN_INVENTORY_SLOTS, MAIN_START,
    OFFHAND_SLOT, OFFHAND_SLOTS, PLAYER_INVENTORY_SIZE, PLAYER_SECTION_SIZE, PlayerInventory,
};

mod equipment_slot;
pub use equipment_slot::EquipmentSlot;

mod drag;
pub(crate) use drag::{DragAction, DragState};

use crate::util::HashMap;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: HashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}
