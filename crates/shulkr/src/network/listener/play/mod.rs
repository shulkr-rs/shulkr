use std::io::Cursor;

use crate::{
    entity::{GameMode, Player},
    event::player::{
        CommandResultEvent, PlayerInputEvent, PlayerRequestGameModeEvent, PlayerStartSneakingEvent,
        PlayerStopSneakingEvent,
    },
    protocol::{
        decode::{Decode as _, DecodeError},
        packet::{
            AttackPacket, ChangeRecipeBookSettingsPacket, ChatCommandPacket, ChatMessagePacket,
            ChunkBatchReceivedPacket, ClickContainerPacket, ClientInfoPacket, ClientTickEndPacket,
            ConfirmTeleportationPacket, InteractPacket, PickItemFromBlockPacket,
            PickItemFromEntityPacket, PlayerActionPacket, PlayerCommand, PlayerCommandPacket,
            PlayerInputFlags, PlayerInputPacket, PlayerLoadedPacket, PlayerMovementFlagsPacket,
            PlayerPositionAndRotationPacket, PlayerPositionPacket, PlayerRequestGameModePacket,
            PlayerRotationPacket, PlayerSessionPacket, SeenAdvancementsPacket,
            SetCreativeModeSlotPacket, SwingArmPacket, UseItemOnPacket, UseItemPacket,
            client::{
                self,
                play::{
                    CloseContainerPacket, KeepAlivePacket, PingRequestPacket,
                    PlayerAbilitiesPacket, SetHeldItemPacket,
                },
            },
        },
    },
};

mod interaction;
mod inventory;
mod movement;

use interaction::*;
use inventory::*;
use movement::*;

#[rustfmt::skip]
pub fn handle_packet(player: Player, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_confirm_teleportation(player, ConfirmTeleportationPacket::decode(data)?),
        0x01 => handle_attack(player, AttackPacket::decode(data)?),
        0x05 => handle_request_game_mode(player, PlayerRequestGameModePacket::decode(data)?),
        0x07 => handle_chat_command(player, ChatCommandPacket::decode(data)?),
        0x09 => handle_chat_message(player, ChatMessagePacket::decode(data)?),
        0x0A => handle_player_session(player, PlayerSessionPacket::decode(data)?),
        0x0B => handle_chunk_batch_received(player, ChunkBatchReceivedPacket::decode(data)?),
        0x0D => handle_client_tick_end(player, ClientTickEndPacket::decode(data)?),
        0x0E => handle_client_info(player, ClientInfoPacket::decode(data)?),
        0x12 => handle_click_container(player, ClickContainerPacket::decode(data)?),
        0x13 => handle_close_container(player, CloseContainerPacket::decode(data)?),
        0x16 => handle_plugin_message(player, client::play::PluginMessagePacket::decode(data)?),
        0x1A => handle_interact(player, InteractPacket::decode(data)?),
        0x1C => handle_keep_alive(player, KeepAlivePacket::decode(data)?),
        0x1E => handle_player_position(player, PlayerPositionPacket::decode(data)?),
        0x1F => handle_player_position_and_rotation(player, PlayerPositionAndRotationPacket::decode(data)?),
        0x20 => handle_player_rotation(player, PlayerRotationPacket::decode(data)?),
        0x21 => handle_player_movement_flags(player, PlayerMovementFlagsPacket::decode(data)?),
        0x24 => handle_pick_item_from_block(player, PickItemFromBlockPacket::decode(data)?),
        0x25 => handle_pick_item_from_entity(player, PickItemFromEntityPacket::decode(data)?),
        0x26 => handle_ping_request(player, PingRequestPacket::decode(data)?),
        0x28 => handle_player_abilities(player, PlayerAbilitiesPacket::decode(data)?),
        0x29 => handle_player_action(player, PlayerActionPacket::decode(data)?),
        0x2A => handle_player_command(player, PlayerCommandPacket::decode(data)?),
        0x2B => handle_player_input(player, PlayerInputPacket::decode(data)?),
        0x2C => handle_player_loaded(player, PlayerLoadedPacket::decode(data)?),
        0x2E => hande_change_recipe_book_settings(player, ChangeRecipeBookSettingsPacket::decode(data)?),
        0x32 => handle_seen_advancements(player, SeenAdvancementsPacket::decode(data)?),
        0x35 => handle_set_held_item(player, SetHeldItemPacket::decode(data)?),
        0x38 => handle_set_creative_mode_slot(player, SetCreativeModeSlotPacket::decode(data)?),
        0x3F => handle_swing_arm(player, SwingArmPacket::decode(data)?),
        0x42 => handle_use_item_on(player, UseItemOnPacket::decode(data)?),
        0x43 => handle_use_item(player, UseItemPacket::decode(data)?),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

fn handle_request_game_mode(player: Player, packet: PlayerRequestGameModePacket) {
    let server = player.server();

    server.events().fire(&mut PlayerRequestGameModeEvent {
        player: player.clone(),
        game_mode: packet.game_mode,
    });
}

fn handle_chat_command(player: Player, packet: ChatCommandPacket) {
    let server = player.server();
    let input = &packet.command;

    let result = server.command_dispatcher().parse(input);

    server
        .events()
        .fire(&mut CommandResultEvent::new(player.clone(), input, result));
}

fn handle_chat_message(player: Player, packet: ChatMessagePacket) {
    for player in player.server().players().lock().iter() {
        // Instead of sending a real PlayerChatMessagePacket we send a system chat message
        // to avoid player reporting.
        player.send_message(format!("<{}> {}", player.name(), packet.message));
    }
}

fn handle_player_session(_player: Player, _packet: PlayerSessionPacket) {
    log::warn!("todo: handle_player_session");
}

fn handle_chunk_batch_received(player: Player, packet: ChunkBatchReceivedPacket) {
    let mut queue = player.0.chunk_queue.lock();
    queue.lead = (queue.lead - 1).max(0);
    queue.target_cpt = if packet.chunks_per_tick.is_nan() {
        0.01
    } else {
        packet.chunks_per_tick.clamp(0.01, 64.)
    };

    if queue.lead == 0 {
        queue.pending_chunks = 1.;
    }
    queue.max_lead = 10;
}

fn handle_client_tick_end(_player: Player, _packet: ClientTickEndPacket) {
    // ignored
}

fn handle_client_info(player: Player, packet: ClientInfoPacket) {
    player.set_view_distance(packet.view_distance as i32);
}

fn handle_plugin_message(_player: Player, _packet: client::play::PluginMessagePacket) {
    log::warn!("todo: handle_plugin_message");
}

fn handle_keep_alive(_player: Player, _packet: KeepAlivePacket) {
    log::warn!("todo: handle_keep_alive");
}

fn handle_ping_request(_player: Player, _packet: PingRequestPacket) {
    log::warn!("todo: handle_ping_request");
}

fn handle_player_abilities(player: Player, packet: PlayerAbilitiesPacket) {
    let can_fly = player.allow_flying() || player.game_mode() == GameMode::Creative;

    if can_fly {
        let flying = (packet.flags & 0x02) != 0;
        player.set_flying(flying);
    }
}

fn handle_player_command(player: Player, packet: PlayerCommandPacket) {
    match packet.action_id {
        PlayerCommand::StartSprinting => player.0.set_sprinting(true),
        PlayerCommand::StopSprinting => player.0.set_sprinting(false),
        _ => todo!(),
    }
}

fn handle_player_input(player: Player, packet: PlayerInputPacket) {
    let server = player.server();

    let sneaking = packet.flags.contains(PlayerInputFlags::SNEAK);
    let was_sneaking = player.is_sneaking();

    server.events().fire(&mut PlayerInputEvent {
        player: player.clone(),
        flags: packet.flags.clone(),
    });

    if sneaking != was_sneaking {
        player.0.set_sneaking(sneaking);

        if sneaking {
            server.events().fire(&mut PlayerStartSneakingEvent {
                player: player.clone(),
            });
        } else {
            server.events().fire(&mut PlayerStopSneakingEvent {
                player: player.clone(),
            });
        }
    }
}

fn handle_player_loaded(_player: Player, _packet: PlayerLoadedPacket) {
    log::warn!("todo: handle_player_loaded");
}

fn hande_change_recipe_book_settings(_player: Player, _packet: ChangeRecipeBookSettingsPacket) {
    log::warn!("todo: hande_change_recipe_book_settings");
}

fn handle_seen_advancements(_player: Player, _packet: SeenAdvancementsPacket) {}
