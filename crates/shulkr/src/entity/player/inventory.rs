use std::sync::{Arc, atomic::Ordering};

use super::Player;
use crate::{
    entity::Hand,
    event::{Cancellable, inventory::InventoryOpenEvent},
    inventory::{
        ARMOR_START, DragState, EquipmentSlot, HOTBAR_START, Inventory, OFFHAND_SLOT,
        PlayerInventory,
    },
    item::ItemStack,
    protocol::packet::server::SetHeldItemPacket,
    util::Viewable,
};

impl Player {
    /// Returns the player's inventory.
    ///
    /// Note: this is not the open inventory. Use [`Player#get_open_inventory()`] instead.
    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.0.inventory
    }

    /// Returns the item currently carried on the cursor.
    pub fn carried_item(&self) -> ItemStack {
        self.inventory().carried_item()
    }

    /// Sets the item carried on the cursor.
    pub fn set_carried_item(&self, stack: ItemStack) {
        self.inventory().set_carried_item(stack)
    }

    pub(crate) fn drag_state(&self) -> Option<DragState> {
        self.inventory().drag_state()
    }

    pub(crate) fn set_drag_state(&self, state: Option<DragState>) {
        self.inventory().set_drag_state(state)
    }

    /// Opens an [`Inventory`] for a player.
    pub fn open_inventory(&self, inventory: Inventory) {
        let mut event = InventoryOpenEvent {
            player: self.clone(),
            inventory: inventory.clone(),
            cancelled: false,
        };
        self.server().events().fire(&mut event);

        if event.is_cancelled() {
            return;
        }

        if let Some(inventory) = self.get_open_inventory() {
            inventory.remove_viewer(self.clone());
        }

        inventory.add_viewer(self.clone());
        *self.0.open_inventory.lock() = Some(inventory);
    }

    /// Closes the opened inventory if it is open.
    pub fn close_inventory(&self) {
        let inventory = self.0.open_inventory.lock().take();
        if let Some(inventory) = inventory {
            inventory.remove_viewer(self.clone());
        }
    }

    /// Returns the open inventory.
    pub fn get_open_inventory(&self) -> Option<Inventory> {
        self.0.open_inventory.lock().clone()
    }

    pub fn get_item_in_hand(&self, hand: Hand) -> Option<ItemStack> {
        self.get_equipment(if hand == Hand::Main {
            EquipmentSlot::MainHand
        } else {
            EquipmentSlot::OffHand
        })
    }

    pub fn get_equipment(&self, slot: EquipmentSlot) -> Option<ItemStack> {
        let slot_id = match slot {
            EquipmentSlot::MainHand => {
                HOTBAR_START + self.0.held_slot.load(Ordering::Acquire) as i32
            }
            EquipmentSlot::OffHand => OFFHAND_SLOT,
            EquipmentSlot::Helmet => ARMOR_START,
            EquipmentSlot::Chestplate => ARMOR_START + 1,
            EquipmentSlot::Leggings => ARMOR_START + 2,
            EquipmentSlot::Boots => ARMOR_START + 3,
        };

        self.0.inventory.get_item_stack(slot_id)
    }

    pub fn set_held_slot(&self, slot: u8) {
        self.update_held_slot(slot);
        self.send_packet(&SetHeldItemPacket { slot: slot.into() });
    }

    pub(crate) fn update_held_slot(&self, slot: u8) {
        self.0.held_slot.store(slot, Ordering::Release);
    }

    /// Returns the currently selected hotbar slot (0-8).
    pub fn held_slot(&self) -> u8 {
        self.0.held_slot.load(Ordering::Acquire)
    }
}
