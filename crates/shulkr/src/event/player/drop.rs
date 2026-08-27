use crate::{
    entity::Player,
    event::{Cancellable, Event, player::PlayerEvent},
    item::ItemStack,
};

/// Not implemented yet.
pub struct ItemDropEvent {
    pub(crate) player: Player,
    pub(crate) item_stack: ItemStack,
    pub(crate) cancelled: bool,
}

impl ItemDropEvent {
    pub fn item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn set_item_stack(&mut self, item_stack: ItemStack) {
        self.item_stack = item_stack;
    }
}

impl Event for ItemDropEvent {}

impl PlayerEvent for ItemDropEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl Cancellable for ItemDropEvent {
    fn set_cancelled(&mut self, value: bool) {
        self.cancelled = value;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
