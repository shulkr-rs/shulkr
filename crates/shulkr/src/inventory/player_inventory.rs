use parking_lot::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::inventory::DragState;
use crate::item::ItemStack;

pub const PLAYER_INVENTORY_SIZE: i32 = 46;

pub const PLAYER_SECTION_SIZE: i32 = 36;

pub const HOTBAR_SLOTS: i32 = 9;
pub const MAIN_INVENTORY_SLOTS: i32 = 27;
pub const ARMOR_SLOTS: i32 = 4;
pub const OFFHAND_SLOTS: i32 = 1;

pub const ARMOR_START: i32 = 5;
pub const MAIN_START: i32 = 9;
pub const HOTBAR_START: i32 = MAIN_START + MAIN_INVENTORY_SLOTS;
pub const OFFHAND_SLOT: i32 = HOTBAR_START + HOTBAR_SLOTS;

#[derive(Debug)]
pub struct PlayerInventory {
    size: i32,
    content: Mutex<Vec<ItemStack>>,
    state: AtomicI32,
    carried: Mutex<ItemStack>,
    drag: Mutex<Option<DragState>>,
}

impl PlayerInventory {
    pub fn new() -> Self {
        let mut content = Vec::with_capacity(PLAYER_INVENTORY_SIZE as usize);
        for _ in 0..PLAYER_INVENTORY_SIZE {
            content.push(ItemStack::EMPTY);
        }

        Self {
            size: PLAYER_INVENTORY_SIZE,
            content: Mutex::new(content),
            state: AtomicI32::new(0),
            carried: Mutex::new(ItemStack::EMPTY),
            drag: Mutex::new(None),
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub(crate) fn next_state(&self) -> i32 {
        self.state.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Returns the item currently carried on the cursor.
    pub fn carried_item(&self) -> ItemStack {
        self.carried.lock().clone()
    }

    /// Sets the item carried on the cursor.
    pub fn set_carried_item(&self, stack: ItemStack) {
        *self.carried.lock() = stack;
    }

    pub(crate) fn drag_state(&self) -> Option<DragState> {
        self.drag.lock().clone()
    }

    pub(crate) fn set_drag_state(&self, state: Option<DragState>) {
        *self.drag.lock() = state;
    }

    pub fn set_item_stack(&self, slot: i32, stack: ItemStack) -> ItemStack {
        let mut content = self.content.lock();
        let Some(current) = content.get_mut(slot as usize) else {
            return ItemStack::EMPTY;
        };
        std::mem::replace(current, stack)
    }

    pub fn take_item_stack(&self, slot: i32) -> ItemStack {
        self.set_item_stack(slot, ItemStack::EMPTY)
    }

    pub fn get_item_stack(&self, slot: i32) -> Option<ItemStack> {
        self.content.lock().get(slot as usize).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Material;

    #[test]
    pub fn test_player_inventory() {
        let inventory = PlayerInventory::new();
        inventory.set_item_stack(9, ItemStack::of(Material::ACACIA_BOAT));

        assert_eq!(
            inventory.get_item_stack(9).map(|v| v.material()),
            Some(Material::ACACIA_BOAT)
        );
        assert_eq!(
            inventory.get_item_stack(10).map(|v| v.material()),
            Some(Material::AIR)
        );
    }

    #[test]
    pub fn test_set_item_stack_returns_old() {
        let inventory = PlayerInventory::new();
        assert_eq!(
            inventory
                .set_item_stack(0, ItemStack::of(Material::STONE))
                .material(),
            Material::AIR
        );
        assert_eq!(
            inventory
                .set_item_stack(0, ItemStack::of(Material::DIRT))
                .material(),
            Material::STONE
        );
        assert_eq!(
            inventory.get_item_stack(0).unwrap().material(),
            Material::DIRT
        );
    }
}
