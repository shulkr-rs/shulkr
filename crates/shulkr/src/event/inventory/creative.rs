use crate::{
    entity::Player,
    event::{Cancellable, Event, player::PlayerEvent},
    item::ItemStack,
};

pub struct CreativeInventoryActionEvent {
    pub(crate) player: Player,
    pub(crate) slot: i16,
    pub(crate) clicked_item: ItemStack,
    pub(crate) cancelled: bool,
}

impl CreativeInventoryActionEvent {
    pub fn slot(&self) -> i16 {
        self.slot
    }

    pub fn clicked_item(&self) -> &ItemStack {
        &self.clicked_item
    }

    pub fn set_clicked_item(&mut self, item: ItemStack) {
        self.clicked_item = item;
    }
}

impl Event for CreativeInventoryActionEvent {}

impl PlayerEvent for CreativeInventoryActionEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl Cancellable for CreativeInventoryActionEvent {
    fn set_cancelled(&mut self, value: bool) {
        self.cancelled = value;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
