use std::{any::Any, sync::Arc};

mod inventory;
pub use inventory::*;

mod inventory_type;
pub use inventory_type::*;

mod player_inventory;
pub use player_inventory::PlayerInventory;

use crate::util::HashMap;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: HashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}
