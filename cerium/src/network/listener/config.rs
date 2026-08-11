use std::collections::{HashMap, HashSet};
use std::{io::Cursor, sync::Arc};

use cerium_nbt::NbtCompound;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::entity::{EntityLike as _, MAX_VIEW_DISTANCE, Player};
use crate::event::player::PlayerSpawnEvent;
use crate::protocol::packet::{CommandsPacket, RegistryEntry, Tag, TagRegistry, UpdateTagsPacket};
use crate::registry::{Registries, Registry, RegistryKey};
use crate::util::{Position, TeleportFlags, Viewable};
use crate::world::{DimensionType, chunk::Chunk};
use crate::{event::player::PlayerConfigEvent, network::client::Connection};
use crate::{
    protocol::{
        ProtocolState,
        decode::{Decode as _, DecodeError},
        packet::{
            AcknowledgeFinishConfigPacket, ClientInfoPacket, FeatureFlagsPacket,
            FinishConfigPacket, GameEventPacket, LoginPacket, RegistryDataPacket,
            SetCenterChunkPacket, client, server,
        },
    },
    util::Key,
};

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

#[derive(Debug, Deserialize)]
pub struct TagsFile {
    #[serde(flatten)]
    pub tags: HashMap<String, TagSection>,
}

#[derive(Debug, Deserialize)]
pub struct TagSection {
    pub values: Vec<String>,
}

fn resolve_tag(
    tag_name: &str,
    all_tags: &HashMap<String, TagSection>,
    output: &mut HashSet<String>,
) {
    let section = all_tags
        .get(tag_name)
        .unwrap_or_else(|| panic!("Missing tag: {}", tag_name));

    for value in &section.values {
        if let Some(stripped) = value.strip_prefix('#') {
            resolve_tag(stripped, all_tags, output);
        } else {
            output.insert(value.clone());
        }
    }
}

fn load_tags(_registry: &str, data: &str) -> HashMap<String, Vec<String>> {
    let tags_file: TagsFile = serde_json::from_str(data).unwrap();

    let mut result = HashMap::new();

    for tag_name in tags_file.tags.keys() {
        let mut resolved = HashSet::new();
        resolve_tag(tag_name, &tags_file.tags, &mut resolved);

        result.insert(tag_name.clone(), resolved.into_iter().collect());
    }

    result
}

fn tags<T>(registry: &'static str, reg: &Registry<T>, data: &str) -> TagRegistry
where
    T: Serialize + DeserializeOwned,
{
    let resolved_tags = load_tags("minecraft:timeline", data);

    let packet_tags: Vec<Tag> = resolved_tags
        .into_iter()
        .map(|(name, values)| Tag {
            tag_name: Key::of(name),
            entries: values
                .into_iter()
                .map(|v| reg.get_id(&RegistryKey::new(v)).map(|v| v as i32))
                .flatten()
                .collect(),
        })
        .collect();

    TagRegistry {
        registry: Key::new("minecraft", registry),
        tags: packet_tags,
    }
}

fn block_tags(data: &str) -> TagRegistry {
    let resolved_tags = load_tags("minecraft:block", data);

    let packet_tags: Vec<Tag> = resolved_tags
        .into_iter()
        .map(|(name, values)| Tag {
            tag_name: Key::of(name),
            entries: values
                .into_iter()
                .filter_map(|v| {
                    Registries::BLOCK
                        .get_id(&RegistryKey::of(v))
                        .map(|id| id as i32)
                })
                .collect(),
        })
        .collect();

    TagRegistry {
        registry: Key::vanilla("block"),
        tags: packet_tags,
    }
}

fn handle_client_info(client: Arc<Connection>, packet: ClientInfoPacket) {
    let server = client.server();
    let registries = server.registries();

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

    client.send_packet(&RegistryDataPacket::from(&registries.cat_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.cat_sound_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.chicken_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.chicken_sound_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.cow_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.cow_sound_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.frog_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.painting_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.pig_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.pig_sound_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.wolf_sound_variant));
    client.send_packet(&RegistryDataPacket::from(&registries.wolf_variant));
    client.send_packet(&RegistryDataPacket::from(
        &registries.zombie_nautilus_variant,
    ));
    client.send_packet(&RegistryDataPacket::from(&registries.damage_type));

    client.send_packet(&RegistryDataPacket::from(&registries.biome));
    client.send_packet(&RegistryDataPacket {
        registry_id: Key::vanilla("world_clock"),
        entries: vec![
            RegistryEntry {
                entry_id: Key::new("minecraft", "overworld"),
                data: Some(NbtCompound::new().into()),
            },
            RegistryEntry {
                entry_id: Key::new("minecraft", "the_end"),
                data: Some(NbtCompound::new().into()),
            },
        ],
    });
    client.send_packet(&RegistryDataPacket::from(&registries.timeline));
    client.send_packet(&RegistryDataPacket::from(&registries.dimension_type));
    client.send_packet(&RegistryDataPacket::from(&registries.trim_material));
    client.send_packet(&RegistryDataPacket::from(&registries.jukebox_song));
    client.send_packet(&RegistryDataPacket::from(&registries.banner_pattern));
    client.send_packet(&RegistryDataPacket::from(&registries.instrument));

    client.send_packet(&UpdateTagsPacket {
        registries: vec![
            tags(
                "timeline",
                &registries.timeline,
                include_str!("../../../build_assets/tags/timeline.json"),
            ),
            tags(
                "damage_type",
                &registries.damage_type,
                include_str!("../../../build_assets/tags/damage_type.json"),
            ),
            tags(
                "banner_pattern",
                &registries.banner_pattern,
                include_str!("../../../build_assets/tags/banner_pattern.json"),
            ),
            tags(
                "instrument",
                &registries.instrument,
                include_str!("../../../build_assets/tags/instrument.json"),
            ),
            block_tags(include_str!("../../../build_assets/tags/block.json")),
        ],
    });

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
        player.0.set_world(world);
    } else {
        todo!("no world set");
    }

    let position = if let Some(position) = event.position {
        player.0.set_position(position);
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
        game_mode: 0,
        previous_game_mode: -1,
        is_debug: false,
        is_flat: false,
        death_location: None,
        portal_cooldown: 4,
        sea_level: 64,
        online_mode: true,
        enforces_secure_chat: false,
    });

    player.synchronize_position(position, Position::ZERO, TeleportFlags::empty());

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
            online_player.send_packet(&player.0.add_to_list_packet());
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
        &client.server().command_dispatcher(),
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
