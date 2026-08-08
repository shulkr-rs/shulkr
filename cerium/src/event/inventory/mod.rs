use crate::{event::player::PlayerEvent, inventory::Inventory};

mod click;
pub use click::InventoryClickEvent;

mod open;
pub use open::InventoryOpenEvent;

mod close;
pub use close::InventoryCloseEvent;

mod creative;
pub use creative::CreativeInventoryActionEvent;

pub trait InventoryEvent: PlayerEvent {
    /// Returns the open inventory, or `None` when the click happened in the
    /// player's own inventory (window id 0).
    fn get_inventory(&self) -> Option<&Inventory>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAction {
    /// Plain left click.
    Left,
    /// Plain right click.
    Right,
    /// Middle click.
    Middle,
    /// Shift + left click.
    ShiftLeft,
    /// Shift + right click.
    ShiftRight,
    Other,
}

impl ClickAction {
    pub fn from_raw(mode: i32, button: i8) -> Self {
        match (mode, button) {
            (0, 0) => Self::Left,
            (0, 1) => Self::Right,
            (1, 0) => Self::ShiftLeft,
            (1, 1) => Self::ShiftRight,
            (3, _) => Self::Middle,
            _ => Self::Other,
        }
    }
}
