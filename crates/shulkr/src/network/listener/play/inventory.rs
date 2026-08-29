use crate::{
    entity::{GameMode, Player},
    event::{
        Cancellable,
        inventory::{ClickAction, CreativeInventoryActionEvent, InventoryClickEvent},
    },
    inventory::{
        DragAction, DragState, HOTBAR_SLOTS, HOTBAR_START, Inventory, MAIN_START, OFFHAND_SLOT,
        PLAYER_INVENTORY_SIZE, PLAYER_SECTION_SIZE,
    },
    item::ItemStack,
    protocol::packet::{
        ClickContainerPacket, SetContainerSlotPacket, SetCreativeModeSlotPacket,
        SetCursorItemPacket,
        client::{CloseContainerPacket, play::SetHeldItemPacket},
    },
};

#[derive(Clone)]
pub(crate) struct Window {
    container: Option<Inventory>,
    player: Player,
}

impl Window {
    pub(crate) fn for_player(player: &Player, window_id: i32) -> Option<Self> {
        if window_id == 0 {
            return Some(Self {
                container: None,
                player: player.clone(),
            });
        }

        let container = player.get_open_inventory()?;
        if container.id() != window_id {
            return None;
        }

        Some(Self {
            container: Some(container),
            player: player.clone(),
        })
    }

    fn container_size(&self) -> Option<i32> {
        self.container.as_ref().map(|c| c.size())
    }

    pub(crate) fn size(&self) -> i32 {
        match self.container_size() {
            None => PLAYER_INVENTORY_SIZE,
            Some(size) => size + PLAYER_SECTION_SIZE,
        }
    }

    // Splits a raw packet slot into (is_container, local index into that storage).
    fn resolve(&self, slot: i32) -> (bool, i32) {
        match self.container_size() {
            None => (false, slot),
            Some(size) if (0..size).contains(&slot) => (true, slot),
            Some(size) => (false, slot - size + MAIN_START),
        }
    }

    fn get_player(&self, local: i32) -> ItemStack {
        self.player
            .inventory()
            .get_item_stack(local)
            .unwrap_or(ItemStack::EMPTY)
    }

    fn set_player(&self, local: i32, stack: ItemStack) {
        self.player.inventory().set_item_stack(local, stack.clone());
        self.player.send_packet(&SetContainerSlotPacket {
            window_id: 0,
            state_id: self.player.inventory().next_state(),
            slot: local as i16,
            slot_data: stack.into(),
        });
    }

    fn get_container(&self, local: i32) -> ItemStack {
        self.container.as_ref().unwrap().get_item_stack(local)
    }

    fn set_container(&self, local: i32, stack: ItemStack) {
        self.container
            .as_ref()
            .unwrap()
            .set_item_stack(local, stack);
    }

    fn get_in(&self, in_container: bool, local: i32) -> ItemStack {
        if in_container {
            self.get_container(local)
        } else {
            self.get_player(local)
        }
    }

    fn set_in(&self, in_container: bool, local: i32, stack: ItemStack) {
        if in_container {
            self.set_container(local, stack);
        } else {
            self.set_player(local, stack);
        }
    }

    pub(crate) fn get(&self, slot: i32) -> ItemStack {
        let (in_container, local) = self.resolve(slot);
        self.get_in(in_container, local)
    }

    pub(crate) fn set(&self, slot: i32, stack: ItemStack) {
        let (in_container, local) = self.resolve(slot);
        self.set_in(in_container, local, stack);
    }
}

pub(crate) fn click(window: &Window, slot: i32, button: i8, carried: &mut ItemStack) {
    if button != 0 && button != 1 {
        return;
    }

    if slot == -1 || slot == -999 {
        if button == 0 {
            *carried = ItemStack::EMPTY;
        } else {
            *carried = if carried.is_empty() {
                ItemStack::EMPTY
            } else {
                carried.clone().with_amount(carried.amount() - 1)
            };
        }
        return;
    }

    let slot_stack = window.get(slot);

    if carried.is_empty() {
        if button == 0 {
            window.set(slot, ItemStack::EMPTY);
            *carried = slot_stack;
        } else {
            let take = (slot_stack.amount() + 1) / 2;
            if take > 0 {
                let left = slot_stack.amount() - take;
                window.set(
                    slot,
                    if left == 0 {
                        ItemStack::EMPTY
                    } else {
                        slot_stack.clone().with_amount(left)
                    },
                );
                *carried = slot_stack.with_amount(take);
            }
        }
    } else if slot_stack.is_empty() {
        if button == 0 {
            window.set(slot, carried.clone());
            *carried = ItemStack::EMPTY;
        } else {
            let amount = carried.amount();
            window.set(slot, carried.clone().with_amount(1));
            *carried = carried.clone().with_amount(amount - 1);
        }
    } else if carried.can_stack_with(&slot_stack) {
        if button == 0 {
            let max = slot_stack.max_stack_size();
            let moved = (max - slot_stack.amount()).min(carried.amount());
            window.set(
                slot,
                slot_stack.clone().with_amount(slot_stack.amount() + moved),
            );
            let remaining = carried.amount() - moved;
            *carried = if remaining == 0 {
                ItemStack::EMPTY
            } else {
                carried.clone().with_amount(remaining)
            };
        } else if slot_stack.amount() < slot_stack.max_stack_size() {
            window.set(
                slot,
                slot_stack.clone().with_amount(slot_stack.amount() + 1),
            );
            *carried = carried.clone().with_amount(carried.amount() - 1);
        }
    } else {
        window.set(slot, carried.clone());
        *carried = slot_stack;
    }
}

pub(crate) fn shift_click(window: &Window, slot: i32) {
    let moving = window.get(slot);
    if moving.is_empty() {
        return;
    }

    let from_container = window
        .container_size()
        .is_some_and(|size| (0..size).contains(&slot));
    let targets = quick_move_targets(window, from_container, slot);
    let mut remaining = moving.amount();

    for &(in_container, local) in &targets {
        if remaining == 0 {
            break;
        }
        let target = window.get_in(in_container, local);
        if target.is_empty() || !target.can_stack_with(&moving) {
            continue;
        }
        let room = target.max_stack_size() - target.amount();
        if room > 0 {
            let moved = room.min(remaining);
            window.set_in(
                in_container,
                local,
                target.clone().with_amount(target.amount() + moved),
            );
            remaining -= moved;
        }
    }

    for &(in_container, local) in &targets {
        if remaining == 0 {
            break;
        }
        let target = window.get_in(in_container, local);
        if !target.is_empty() {
            continue;
        }
        let moved = remaining.min(moving.max_stack_size());
        window.set_in(in_container, local, moving.clone().with_amount(moved));
        remaining -= moved;
    }

    if remaining == moving.amount() {
        return;
    }

    let left = if remaining == 0 {
        ItemStack::EMPTY
    } else {
        moving.with_amount(remaining)
    };
    window.set(slot, left);
}

// `slot` is only used as a player-local index, and only when there is no open container.
fn quick_move_targets(window: &Window, from_container: bool, slot: i32) -> Vec<(bool, i32)> {
    let mut targets = Vec::new();

    match window.container_size() {
        None => {
            let local = slot;
            if (HOTBAR_START..HOTBAR_START + HOTBAR_SLOTS).contains(&local) {
                for l in MAIN_START..HOTBAR_START {
                    targets.push((false, l));
                }
            } else if (MAIN_START..HOTBAR_START).contains(&local) {
                for l in HOTBAR_START..HOTBAR_START + HOTBAR_SLOTS {
                    targets.push((false, l));
                }
            } else {
                for l in MAIN_START..HOTBAR_START + HOTBAR_SLOTS {
                    targets.push((false, l));
                }
            }
        }
        Some(size) => {
            if from_container {
                for l in MAIN_START..HOTBAR_START + HOTBAR_SLOTS {
                    targets.push((false, l));
                }
            } else {
                for l in 0..size {
                    targets.push((true, l));
                }
            }
        }
    }

    targets
}

pub(crate) fn number_click(window: &Window, slot: i32, button: i8) {
    if !(0..HOTBAR_SLOTS).contains(&(button as i32)) {
        return;
    }

    let hotbar_slot = HOTBAR_START + button as i32;

    let (in_container, local) = window.resolve(slot);
    if !in_container && local == hotbar_slot {
        return;
    }

    let clicked = window.get_in(in_container, local);
    let hotbar = window.get_player(hotbar_slot);

    window.set_in(in_container, local, hotbar);
    window.set_player(hotbar_slot, clicked);
}

pub(crate) fn offhand_swap(window: &Window, slot: i32, carried: &ItemStack) {
    if !carried.is_empty() {
        return;
    }

    let (in_container, local) = window.resolve(slot);
    let clicked = window.get_in(in_container, local);
    let off = window.get_player(OFFHAND_SLOT);

    window.set_in(in_container, local, off);
    window.set_player(OFFHAND_SLOT, clicked);
}

pub(crate) fn middle_click(
    window: &Window,
    slot: i32,
    button: i8,
    carried: &mut ItemStack,
    creative: bool,
) {
    if button != 2 || !creative {
        return;
    }

    let clicked = window.get(slot);
    if !clicked.is_empty() {
        *carried = clicked.clone().with_amount(clicked.max_stack_size());
    }
}

pub(crate) fn drop_click(window: &Window, slot: i32, button: i8) {
    if slot == -1 {
        return;
    }

    let stack = window.get(slot);
    if stack.is_empty() {
        return;
    }

    match button {
        0 => {
            let left = stack.amount() - 1;
            window.set(
                slot,
                if left == 0 {
                    ItemStack::EMPTY
                } else {
                    stack.with_amount(left)
                },
            );
        }
        1 => window.set(slot, ItemStack::EMPTY),
        _ => {}
    }
}

pub(crate) fn drag_click(
    window: &Window,
    drag: &mut Option<DragState>,
    slot: i32,
    button: i8,
    carried: &mut ItemStack,
    creative: bool,
) {
    let action = match button >> 2 {
        0 => DragAction::Left,
        1 => DragAction::Right,
        2 => DragAction::Middle,
        _ => {
            *drag = None;
            return;
        }
    };

    match button & 3 {
        0 => {
            *drag = Some(DragState {
                action,
                slots: Vec::new(),
            })
        }
        1 => {
            if let Some(state) = drag.as_mut()
                && state.action == action
                && slot != -1
                && !state.slots.contains(&(slot as i16))
            {
                let target = window.get(slot);
                if target.is_empty() || carried.can_stack_with(&target) {
                    state.slots.push(slot as i16);
                }
            }
        }
        2 => {
            if let Some(state) = drag.as_ref()
                && state.action == action
            {
                end_drag(window, state, carried, creative);
            }
            *drag = None;
        }
        _ => {}
    }
}

pub(crate) fn end_drag(
    window: &Window,
    state: &DragState,
    carried: &mut ItemStack,
    creative: bool,
) {
    match state.action {
        DragAction::Left => {
            let mut slots = Vec::new();
            for &slot in &state.slots {
                let slot = slot as i32;
                let target = window.get(slot);
                if target.is_empty()
                    || (target.can_stack_with(carried) && target.amount() < target.max_stack_size())
                {
                    slots.push(slot);
                }
            }

            if slots.is_empty() {
                return;
            }

            let total = carried.amount();
            let per = total / slots.len() as i32;
            let remainder = total % slots.len() as i32;
            let mut placed = 0;

            for (i, &slot) in slots.iter().enumerate() {
                if placed >= total {
                    break;
                }
                let target = window.get(slot);
                let (base, current) = if target.is_empty() {
                    (carried.clone(), 0)
                } else {
                    (target.clone(), target.amount())
                };
                let extra = if (i as i32) < remainder { 1 } else { 0 };
                let share = per + extra;
                let add = (base.max_stack_size() - current).max(0).min(share);
                if add > 0 {
                    window.set(slot, base.with_amount(current + add));
                    placed += add;
                }
            }

            let remaining = total - placed;
            *carried = if remaining == 0 {
                ItemStack::EMPTY
            } else {
                carried.clone().with_amount(remaining)
            };
        }
        DragAction::Right => {
            for &slot in &state.slots {
                if carried.is_empty() {
                    break;
                }
                let slot = slot as i32;
                let target = window.get(slot);
                if target.is_empty() {
                    window.set(slot, carried.clone().with_amount(1));
                    *carried = carried.clone().with_amount(carried.amount() - 1);
                } else if target.can_stack_with(carried)
                    && target.amount() < target.max_stack_size()
                {
                    window.set(slot, target.clone().with_amount(target.amount() + 1));
                    *carried = carried.clone().with_amount(carried.amount() - 1);
                }
            }
        }
        DragAction::Middle => {
            if creative && !carried.is_empty() {
                for &slot in &state.slots {
                    let slot = slot as i32;
                    let target = window.get(slot);
                    if target.is_empty() {
                        window.set(slot, carried.clone().with_amount(1));
                    }
                }
            }
        }
    }
}

pub(crate) fn double_click(window: &Window, _slot: i32, carried: &mut ItemStack) {
    if carried.is_empty() {
        return;
    }

    let max = carried.max_stack_size();
    for slot in 0..window.size() {
        if carried.amount() >= max {
            break;
        }

        let stack = window.get(slot);
        if stack.is_empty() || !stack.can_stack_with(carried) {
            continue;
        }

        let moved = (max - carried.amount()).min(stack.amount());
        if moved > 0 {
            let left = stack.amount() - moved;
            window.set(
                slot,
                if left == 0 {
                    ItemStack::EMPTY
                } else {
                    stack.with_amount(left)
                },
            );
            *carried = carried.clone().with_amount(carried.amount() + moved);
        }
    }
}

pub fn handle_click_container(player: Player, packet: ClickContainerPacket) {
    let Some(window) = Window::for_player(&player, packet.window_id) else {
        return;
    };

    let slot = packet.slot as i32;
    let click_action = ClickAction::from_raw(packet.mode, packet.button);
    let clicked_item = window.get(slot);

    let mut event = InventoryClickEvent {
        inventory: window.container.clone(),
        player: player.clone(),
        slot: packet.slot,
        clicked_item,
        click_action,
        cancelled: false,
    };

    player.server().events().fire(&mut event);

    if event.is_cancelled() {
        return;
    }

    let mut carried = player.carried_item();
    let creative = player.game_mode() == GameMode::Creative;
    let mut drag = player.drag_state();

    match packet.mode {
        0 => click(&window, slot, packet.button, &mut carried),
        1 => shift_click(&window, slot),
        2 if packet.button == 40 => offhand_swap(&window, slot, &carried),
        2 => number_click(&window, slot, packet.button),
        3 => middle_click(&window, slot, packet.button, &mut carried, creative),
        4 => drop_click(&window, slot, packet.button),
        5 => drag_click(
            &window,
            &mut drag,
            slot,
            packet.button,
            &mut carried,
            creative,
        ),
        6 => double_click(&window, slot, &mut carried),
        _ => {}
    }

    let mid_drag = packet.mode == 5 && packet.button & 3 != 2;
    if !mid_drag {
        player.send_packet(&SetCursorItemPacket {
            carried_item: carried.clone().into(),
        });
    }

    player.set_carried_item(carried);
    player.set_drag_state(drag);
}

pub fn handle_close_container(player: Player, packet: CloseContainerPacket) {
    let _ = packet;
    player.close_inventory();
}

pub(crate) fn handle_set_held_item(player: Player, packet: SetHeldItemPacket) {
    player.update_held_slot(packet.slot as u8);
}

pub(crate) fn handle_set_creative_mode_slot(player: Player, packet: SetCreativeModeSlotPacket) {
    if player.game_mode() != GameMode::Creative {
        return;
    }

    let item_stack = ItemStack::from(packet.clicked_item);
    let previous_item = player.inventory().get_item_stack(packet.slot as i32);

    let mut event = CreativeInventoryActionEvent {
        player: player.clone(),
        slot: packet.slot,
        clicked_item: item_stack,
        cancelled: false,
    };
    player.server().events().fire(&mut event);

    if event.is_cancelled() {
        if let Some(previous_item) = previous_item {
            player.send_packet(&SetContainerSlotPacket {
                window_id: 0,
                state_id: player.inventory().next_state(),
                slot: packet.slot,
                slot_data: previous_item.into(),
            });
        }
        return;
    }

    let item_stack = event.clicked_item;

    if packet.slot == -1 {
        return;
    }

    if !(1..=OFFHAND_SLOT).contains(&(packet.slot as i32)) {
        return;
    }

    let inventory = player.inventory();
    inventory.set_item_stack(packet.slot as i32, item_stack.clone());

    player.send_packet(&SetContainerSlotPacket {
        window_id: 0,
        state_id: player.inventory().next_state(),
        slot: packet.slot,
        slot_data: item_stack.into(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{AuthMode, GameProfile};
    use crate::inventory::InventoryType;
    use crate::item::Material;
    use crate::network::client::Connection;
    use crate::protocol::ProtocolState;
    use uuid::Uuid;

    fn test_server() -> crate::Server {
        static SERVER: std::sync::OnceLock<crate::Server> = std::sync::OnceLock::new();
        SERVER
            .get_or_init(|| crate::Server::new(AuthMode::Offline))
            .clone()
    }

    fn test_player() -> Player {
        let server = test_server();

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let client_stream = std::net::TcpStream::connect(addr).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        client_stream.set_nonblocking(true).ok();
        server_stream.set_nonblocking(true).ok();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (connection, _rx) = runtime.block_on(async {
            Connection::new(
                addr,
                tokio::net::TcpStream::from_std(server_stream).unwrap(),
            )
        });

        {
            let mut profile = connection.game_profile.lock();
            *profile = Some(GameProfile {
                uuid: Uuid::new_v4(),
                name: "test".to_string(),
                properties: vec![],
            });
        }
        connection.set_state(ProtocolState::Play);

        Player::new(connection.clone(), server.clone())
    }

    fn player_window() -> Window {
        let player = test_player();
        Window::for_player(&player, 0).unwrap()
    }

    fn container_window(size: i32) -> Window {
        assert_eq!(size, 9, "test helper only wires up a 9-slot container");
        let player = test_player();
        let inventory = Inventory::new(InventoryType::Generic9x1, "test");
        player.open_inventory(inventory.clone());
        Window::for_player(&player, inventory.id()).unwrap()
    }

    fn stone(n: i32) -> ItemStack {
        ItemStack::new(Material::STONE, n)
    }

    fn dirt(n: i32) -> ItemStack {
        ItemStack::new(Material::DIRT, n)
    }

    #[test]
    fn left_click_picks_up_and_places() {
        let window = player_window();
        window.set(9, stone(10));

        let mut carried = ItemStack::EMPTY;
        click(&window, 9, 0, &mut carried);

        assert_eq!(carried.material(), Material::STONE);
        assert_eq!(carried.amount(), 10);
        assert!(window.get(9).is_empty());

        click(&window, 10, 0, &mut carried);

        assert!(carried.is_empty());
        assert_eq!(window.get(10).amount(), 10);
    }

    #[test]
    fn left_click_merges_into_max_stack_size() {
        let window = player_window();
        window.set(9, stone(60));

        let mut carried = stone(10);
        click(&window, 9, 0, &mut carried);

        assert_eq!(window.get(9).amount(), 64);
        assert_eq!(carried.amount(), 6);
    }

    #[test]
    fn left_click_swaps_different_items() {
        let window = player_window();
        window.set(9, stone(3));

        let mut carried = dirt(2);
        click(&window, 9, 0, &mut carried);

        assert_eq!(window.get(9).material(), Material::DIRT);
        assert_eq!(window.get(9).amount(), 2);
        assert_eq!(carried.material(), Material::STONE);
        assert_eq!(carried.amount(), 3);
    }

    #[test]
    fn right_click_picks_up_half() {
        let window = player_window();
        window.set(9, stone(5));

        let mut carried = ItemStack::EMPTY;
        click(&window, 9, 1, &mut carried);

        assert_eq!(carried.amount(), 3);
        assert_eq!(window.get(9).amount(), 2);
    }

    #[test]
    fn right_click_places_one() {
        let window = player_window();
        let mut carried = stone(5);
        click(&window, 9, 1, &mut carried);

        assert_eq!(carried.amount(), 4);
        assert_eq!(window.get(9).amount(), 1);
    }

    #[test]
    fn shift_click_quick_moves_to_other_side() {
        let window = container_window(9);
        window.set(0, stone(64));
        window.set(1, stone(1));

        shift_click(&window, 0);

        assert!(window.get(0).is_empty());
        assert_eq!(window.get_player(MAIN_START).amount(), 64);
        assert!(window.get_player(MAIN_START + 1).is_empty());
        assert_eq!(window.get(1).amount(), 1);
    }

    #[test]
    fn shift_click_merges_into_existing_stacks_first() {
        let window = container_window(9);
        window.set(0, stone(30));
        window.set_player(MAIN_START, stone(40));

        shift_click(&window, 0);

        assert!(window.get(0).is_empty());
        assert_eq!(window.get_player(MAIN_START).amount(), 64);
        assert_eq!(window.get_player(MAIN_START + 1).amount(), 6);
    }

    #[test]
    fn number_key_swaps_with_hotbar() {
        let window = player_window();
        window.set(MAIN_START, stone(5));
        window.set(HOTBAR_START + 3, dirt(1));

        number_click(&window, MAIN_START, 3);

        assert_eq!(window.get(HOTBAR_START + 3).material(), Material::STONE);
        assert_eq!(window.get(MAIN_START).material(), Material::DIRT);
    }

    #[test]
    fn drop_removes_items() {
        let window = player_window();
        window.set(9, stone(5));

        drop_click(&window, 9, 0);
        assert_eq!(window.get(9).amount(), 4);

        drop_click(&window, 9, 1);
        assert!(window.get(9).is_empty());
    }

    #[test]
    fn offhand_swap_swaps_hovered_slot_with_offhand() {
        let window = player_window();
        window.set(MAIN_START, stone(1));
        window.set(OFFHAND_SLOT, dirt(2));

        offhand_swap(&window, MAIN_START, &ItemStack::EMPTY);

        assert_eq!(window.get(MAIN_START).material(), Material::DIRT);
        assert_eq!(window.get(OFFHAND_SLOT).material(), Material::STONE);
    }

    #[test]
    fn offhand_swap_does_nothing_with_cursor_full() {
        let window = player_window();
        window.set(MAIN_START, stone(1));
        window.set(OFFHAND_SLOT, dirt(2));

        offhand_swap(&window, MAIN_START, &stone(1));

        assert_eq!(window.get(MAIN_START).material(), Material::STONE);
        assert_eq!(window.get(OFFHAND_SLOT).material(), Material::DIRT);
    }

    #[test]
    fn double_click_collects_all_matching() {
        let window = player_window();
        window.set(9, stone(10));
        window.set(10, stone(20));
        window.set(11, dirt(20));

        let mut carried = stone(1);
        double_click(&window, 9, &mut carried);

        assert_eq!(carried.amount(), 31);
        assert_eq!(window.get(9).amount(), 0);
        assert_eq!(window.get(10).amount(), 0);
        assert_eq!(window.get(11).amount(), 20);
    }

    #[test]
    fn left_drag_distributes_evenly() {
        let window = player_window();
        let mut drag = None;
        let mut carried = stone(9);

        drag_click(&window, &mut drag, 9, 0, &mut carried, false);
        drag_click(&window, &mut drag, 10, 1, &mut carried, false);
        drag_click(&window, &mut drag, 11, 1, &mut carried, false);
        drag_click(&window, &mut drag, 12, 2, &mut carried, false);

        assert!(window.get(9).is_empty());
        assert_eq!(window.get(10).material(), Material::STONE);
        assert_eq!(window.get(10).amount(), 5);
        assert_eq!(window.get(11).material(), Material::STONE);
        assert_eq!(window.get(11).amount(), 4);
        assert!(window.get(12).is_empty());
        assert!(carried.is_empty());
    }
}
