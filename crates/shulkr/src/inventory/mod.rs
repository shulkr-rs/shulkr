mod drag;
mod equipment_slot;
mod inventory_type;
mod player_inventory;

pub use equipment_slot::EquipmentSlot;
pub use inventory_type::*;
pub use player_inventory::{
    ARMOR_SLOTS, ARMOR_START, HOTBAR_SLOTS, HOTBAR_START, MAIN_INVENTORY_SLOTS, MAIN_START,
    OFFHAND_SLOT, OFFHAND_SLOTS, PLAYER_INVENTORY_SIZE, PLAYER_SECTION_SIZE, PlayerInventory,
};

pub(crate) use drag::{DragAction, DragState};

use crate::{
    entity::Player,
    item::ItemStack,
    protocol::packet::{
        OpenScreenPacket, SetContainerContentPacket, SetContainerSlotPacket,
        server::CloseContainerPacket,
    },
    text::TextComponent,
    util::{HashMap, Mutex, Viewable, Viewers},
};
use std::{
    any::Any,
    sync::{
        Arc,
        atomic::{AtomicI32, Ordering},
    },
};

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: i32,
    pub item_id: Option<i32>,
    pub to_add: HashMap<i32, Arc<dyn Any + Send + Sync>>,
    pub to_remove: Vec<i32>,
}

#[derive(Clone)]
pub struct Inventory(Arc<imp::Inventory>);

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Inventory {}

impl Inventory {
    pub fn new(ty: InventoryType, title: impl Into<TextComponent>) -> Self {
        let inventory = imp::Inventory::new(ty, title);
        Self(Arc::new(inventory))
    }

    /// Returns the id of the inventory.
    pub fn id(&self) -> i32 {
        self.0.id()
    }

    /// Returns the type of the inventory.
    pub fn r#type(&self) -> InventoryType {
        self.0.r#type()
    }

    /// Returns the title of the inventory.
    pub fn title(&self) -> &TextComponent {
        self.0.title()
    }

    /// Returns the size of the inventory.
    pub fn size(&self) -> i32 {
        self.r#type().size()
    }

    /// Adds an [`ItemStack`] to the first available slot in the inventory.
    pub fn add_item_stack(&self, stack: ItemStack) {
        self.0.add_item_stack(stack)
    }

    /// Inserts an [`ItemStack`] into a given slot and overwrites the previous data.
    pub fn set_item_stack(&self, slot: i32, stack: ItemStack) {
        self.0.set_item_stack(slot, stack)
    }

    /// Returns the [`ItemStack`] in the current slot.
    pub fn get_item_stack(&self, slot: i32) -> ItemStack {
        self.0.get_item_stack(slot)
    }
}

impl Viewable for Inventory {
    fn add_viewer(&self, player: Player) {
        self.0.add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        self.0.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        self.0.viewers()
    }
}

mod imp {
    use super::*;

    pub struct Inventory {
        id: i32,
        ty: InventoryType,
        title: TextComponent,
        content: Mutex<Vec<ItemStack>>,
        viewers: Viewers,
        state: AtomicI32,
    }

    impl Inventory {
        pub fn new(ty: InventoryType, title: impl Into<TextComponent>) -> Self {
            let size = ty.size();
            let content = vec![ItemStack::EMPTY; size as usize];

            Self {
                id: Self::generate_id(),
                ty,
                title: title.into(),
                content: Mutex::new(content),
                viewers: Viewers::new(),
                state: AtomicI32::new(0),
            }
        }

        fn next_state(&self) -> i32 {
            self.state.fetch_add(1, Ordering::Relaxed) + 1
        }

        fn generate_id() -> i32 {
            static CURRENT_ID: AtomicI32 = AtomicI32::new(1);
            CURRENT_ID
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| {
                    Some(if i + 1 >= 128 { 1 } else { i + 1 })
                })
                .unwrap()
        }

        pub fn id(&self) -> i32 {
            self.id
        }

        pub fn r#type(&self) -> InventoryType {
            self.ty
        }

        pub fn title(&self) -> &TextComponent {
            &self.title
        }

        pub fn add_item_stack(&self, stack: ItemStack) {
            let mut content = self.content.lock();
            let Some(ix) = content.iter().position(|s| s.is_empty()) else {
                return;
            };
            content[ix] = stack.clone();

            self.broadcast_packet(&SetContainerSlotPacket {
                window_id: self.id(),
                state_id: self.next_state(),
                slot: ix as i16,
                slot_data: stack.into(),
            });
        }

        pub fn set_item_stack(&self, slot: i32, stack: ItemStack) {
            let mut content = self.content.lock();
            let Some(current) = content.get_mut(slot as usize) else {
                return;
            };
            *current = stack.clone();
            drop(content);

            self.broadcast_packet(&SetContainerSlotPacket {
                window_id: self.id(),
                state_id: self.next_state(),
                slot: slot as i16,
                slot_data: stack.into(),
            });
        }

        pub fn get_item_stack(&self, slot: i32) -> ItemStack {
            self.content
                .lock()
                .get(slot as usize)
                .cloned()
                .unwrap_or(ItemStack::EMPTY)
        }

        pub fn refresh_contents(&self, player: Player) {
            let content = self.content.lock().clone();
            player.send_packet(&SetContainerContentPacket {
                window_id: self.id(),
                state_id: self.state.load(Ordering::Relaxed),
                slot_data: content.into_iter().map(|s| s.into()).collect(),
                carried_item: ItemStack::EMPTY.into(),
            });
        }
    }

    impl Viewable for Inventory {
        fn add_viewer(&self, player: Player) {
            self.viewers.add_viewer(player.clone());

            player.send_packet(&OpenScreenPacket {
                window_id: self.id(),
                window_type: self.r#type().id(),
                window_title: self.title().clone(),
            });
            self.refresh_contents(player);
        }

        fn remove_viewer(&self, player: Player) {
            self.viewers.remove_viewer(player.clone());

            player.send_packet(&CloseContainerPacket {
                window_id: self.id(),
            });
        }

        fn viewers(&self) -> &Viewers {
            &self.viewers
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Material;

    #[test]
    #[rustfmt::skip]
    fn test_set_item_stack() {
        let inventory = Inventory::new(InventoryType::Generic9x6, "");
        inventory.set_item_stack(1, ItemStack::EMPTY);
        inventory.set_item_stack(22, ItemStack::new(Material::ACACIA_BOAT, 1));

        assert_eq!(inventory.get_item_stack(0).material(), Material::AIR);
        assert_eq!(inventory.get_item_stack(1).material(), Material::AIR);
        assert_eq!(inventory.get_item_stack(22).material(), Material::ACACIA_BOAT);
    }

    #[test]
    #[rustfmt::skip]
    fn test_add_item_stack() {
        let inventory = Inventory::new(InventoryType::Generic9x6, "");
        inventory.add_item_stack(ItemStack::EMPTY);
        inventory.add_item_stack(ItemStack::new(Material::GRANITE_STAIRS, 1));
        inventory.add_item_stack(ItemStack::new(Material::RED_CANDLE, 1));

        assert_eq!(inventory.get_item_stack(0).material(), Material::GRANITE_STAIRS);
        assert_eq!(inventory.get_item_stack(1).material(), Material::RED_CANDLE);
    }
}
