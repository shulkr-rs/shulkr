use std::{io::Cursor, sync::Arc};

use crate::entity::{EntityLike as _, GameMode, MAX_VIEW_DISTANCE, Player};
use crate::event::player::PlayerSpawnEvent;
use crate::protocol::packet::CommandsPacket;
use crate::util::{TeleportFlags, Velocity, Viewable};
use crate::world::{DimensionType, chunk::Chunk};
use crate::{event::player::PlayerConfigEvent, network::client::Connection};
use crate::{
    protocol::{
        ProtocolState,
        decode::{Decode as _, DecodeError},
        packet::{
            AcknowledgeFinishConfigPacket, ClientInfoPacket, FeatureFlagsPacket,
            FinishConfigPacket, GameEventPacket, LoginPacket, SetCenterChunkPacket, client, server,
        },
    },
    util::Key,
};

mod registries;

#[rustfmt::skip]
pub fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_client_info(client, ClientInfoPacket::decode(data)?),
        0x01 => handle_cookie_response(client),
        0x02 => handle_plugin_message(client, client::config::PluginMessagePacket::decode(data)?),
        0x03 => handle_acknowledge_finish_config(client, AcknowledgeFinishConfigPacket::decode(data)?),
        0x04 => handle_keep_alive(client),
        0x05 => handle_pong(client),
        0x06 => handle_resource_pack_response(client),
        0x07 => handle_client_known_packs(client, client::config::KnownPacksPacket::decode(data)?),
        0x08 => handle_custom_click_action(client),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

fn handle_client_info(client: Arc<Connection>, packet: ClientInfoPacket) {
    let server = client.server();

    client.set_view_distance(packet.view_distance as i32);

    client.send_packet(&server::config::PluginMessagePacket {
        identifier: Key::vanilla("brand"),
        data: server.brand().into_bytes().into_boxed_slice(),
    });

    client.send_packet(&server::config::KnownPacksPacket {
        known_packs: Vec::new(),
    });

    client.send_packet(&FeatureFlagsPacket {
        feature_flags: vec![Key::vanilla("vanilla")],
    });

    let registries = server.registries();
    registries::send_registries(&client, registries);
    registries::send_registry_tags(&client, registries);

    client.send_packet(&FinishConfigPacket {});
}

fn handle_cookie_response(_client: Arc<Connection>) {}

fn handle_plugin_message(_client: Arc<Connection>, _packet: client::config::PluginMessagePacket) {}

fn handle_acknowledge_finish_config(
    client: Arc<Connection>,
    _packet: AcknowledgeFinishConfigPacket,
) {
    client.set_state(ProtocolState::Play);

    let player = Player::new(client.clone(), client.server().clone());
    {
        let mut players = client.server().players().lock();
        players.push(player.clone());

        let mut guard = client.player.lock();
        *guard = Some(player.clone());
    }

    let mut event = PlayerConfigEvent {
        player: player.clone(),
        world: None,
        position: None,
    };
    client.server().events().fire(&mut event);

    if let Some(world) = event.world {
        player.set_world(world);
    } else {
        todo!("no world set");
    }

    let position = if let Some(position) = event.position {
        player.set_position(position);
        position
    } else {
        todo!("no position set");
    };

    let dimension = DimensionType::OVERWORLD;

    let registries = client.server().registries();

    client.send_packet(&LoginPacket {
        entity_id: player.id(),
        is_hardcore: false,
        dimension_names: vec![dimension.clone().into()],
        max_players: 20,
        view_distance: MAX_VIEW_DISTANCE,
        simulation_distance: 8,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: registries.dimension_type.get_id(&dimension).unwrap_or(0) as i32,
        dimension_name: dimension.into(),
        hashed_seed: 93522819,
        game_mode: GameMode::Survival,
        previous_game_mode: -1,
        is_debug: false,
        is_flat: false,
        death_location: None,
        portal_cooldown: 4,
        sea_level: 64,
        online_mode: true,
        enforces_secure_chat: false,
    });

    player.synchronize_position(position, Velocity::ZERO, TeleportFlags::empty());

    client.send_packet(&GameEventPacket::START_WAITING_FOR_CHUNKS);

    let (cx, cy) = Chunk::to_chunk_pos(position);
    client.send_packet(&SetCenterChunkPacket {
        chunk_x: cx,
        chunk_z: cy,
    });

    {
        let online_players = &*client.server().players().lock();

        // Add player to tab for already playing players.
        for online_player in online_players {
            online_player.send_packet(&player.add_to_list_packet());
            if *online_player != player {
                player.add_viewer(online_player.clone());
            }
        }

        // Add already playing player to tab for player.
        for online_player in online_players {
            if *online_player == player {
                continue;
            }
            online_player.add_viewer(player.clone());
        }
    }

    client.send_packet(&CommandsPacket::from_dispatcher(
        client.server().command_dispatcher(),
    ));

    client.server().events().fire(&mut PlayerSpawnEvent {
        player: player.clone(),
    });
}

fn handle_keep_alive(_client: Arc<Connection>) {}

fn handle_pong(_client: Arc<Connection>) {}

fn handle_resource_pack_response(_client: Arc<Connection>) {}

fn handle_client_known_packs(_client: Arc<Connection>, _packet: client::config::KnownPacksPacket) {}

fn handle_custom_click_action(_client: Arc<Connection>) {}
