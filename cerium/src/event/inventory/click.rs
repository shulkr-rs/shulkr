use crate::{
    entity::Player,
    event::{Cancellable, Event, inventory::{ClickAction, InventoryEvent}, player::PlayerEvent},
    inventory::Inventory,
    item::ItemStack,
};

pub struct InventoryClickEvent {
    pub(crate) player: Player,
    pub(crate) inventory: Option<Inventory>,
    pub(crate) slot: i16,
    pub(crate) clicked_item: ItemStack,
    pub(crate) click_action: ClickAction,
    pub(crate) cancelled: bool,
}

impl InventoryClickEvent {
    pub fn slot(&self) -> i16 {
        self.slot
    }

    pub fn clicked_item(&self) -> &ItemStack {
        &self.clicked_item
    }

    pub fn click_action(&self) -> ClickAction {
        self.click_action
    }
}

impl Event for InventoryClickEvent {}

impl PlayerEvent for InventoryClickEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl InventoryEvent for InventoryClickEvent {
    fn get_inventory(&self) -> Option<&Inventory> {
        self.inventory.as_ref()
    }
}

impl Cancellable for InventoryClickEvent {
    fn set_cancelled(&mut self, value: bool) {
        self.cancelled = value;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
