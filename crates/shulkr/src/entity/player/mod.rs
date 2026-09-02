use crate::{
    Server,
    auth::GameProfile,
    entity::{Entity, EntityLike, EntityType, GameMode, entity_status},
    inventory::{Inventory, PlayerInventory},
    network::client::Connection,
    protocol::packet::{
        EntityEventPacket, GameEventPacket, Packet, PlayerAction, PlayerEntry, PlayerInfoFlags,
        PlayerInfoRemovePacket, PlayerInfoUpdatePacket, RespawnPacket, ServerPacket,
        SetCenterChunkPacket, SetTablistHeaderFooterPacket, SystemChatMessagePacket,
        server::play::KeepAlivePacket,
    },
    text::TextComponent,
    tickable::Tickable,
    util::{HashMap, HashSet, Mutex, Position, Viewable, Viewers},
    world::{World, chunk::Chunk},
};
use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::sync::Notify;
use uuid::Uuid;

mod abilities;
mod chunks;
mod inventory;
mod meta;
mod movement;
use abilities::Abilities;
use chunks::{ChunkQueue, TrackState};

pub const MAX_VIEW_DISTANCE: i32 = 32;

#[derive(Clone)]
pub struct Player(pub(crate) Arc<imp::Player>);

impl Player {
    pub(crate) fn new(connection: Arc<Connection>, server: Server) -> Self {
        Self(Arc::new(imp::Player::new(connection, server)))
    }

    pub fn addr(&self) -> SocketAddr {
        self.0.connection.addr()
    }

    pub fn name(&self) -> &String {
        &self.0.game_profile.name
    }

    pub fn game_mode(&self) -> GameMode {
        *self.0.game_mode.lock()
    }

    pub fn set_game_mode(&self, game_mode: GameMode) {
        self.update_game_mode(game_mode);

        self.send_packet(&GameEventPacket {
            event: 3,
            value: game_mode as i32 as f32,
        });

        let p = PlayerInfoUpdatePacket {
            players: vec![PlayerEntry {
                uuid: self.uuid(),
                player_actions: vec![PlayerAction::UpdateGameMode { game_mode }],
            }],
            actions: PlayerInfoFlags::UPDATE_GAME_MODE.bits(),
        };
        self.send_packet(&p);
        self.broadcast_packet(&p);

        self.set_allow_flying(game_mode == GameMode::Creative || game_mode == GameMode::Spectator);
        if game_mode != GameMode::Creative && game_mode != GameMode::Spectator {
            self.set_flying(false);
        }

        self.set_insta_break(game_mode == GameMode::Creative);
        self.refresh_abilities();
    }

    fn update_game_mode(&self, game_mode: GameMode) {
        *self.0.game_mode.lock() = game_mode;
    }

    pub fn send_message(&self, message: impl Into<TextComponent>) {
        self.send_packet(&SystemChatMessagePacket {
            content: message.into(),
            overlay: false,
        });
    }

    pub fn kick(&self, reason: impl Into<TextComponent>) {
        self.0.connection.kick(reason.into());
    }

    pub fn send_packet<P>(&self, packet: &P)
    where
        P: Packet + ServerPacket + 'static,
    {
        self.0.connection.send_packet(packet);
    }

    pub fn server(&self) -> &Server {
        &self.0.server
    }

    pub fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }

        let world = self.world();
        self.0.pending_loads.lock().clear();
        for (pos, state) in self.0.tracked_chunks.lock().drain() {
            if state == TrackState::Viewing {
                world.remove_viewer(pos.0, pos.1);
            }
        }
    }

    pub fn change_world(&self, world: World) {
        let tracked: Vec<(i32, i32)> = self.0.tracked_chunks.lock().keys().copied().collect();
        for (cx, cz) in tracked {
            self.untrack_chunk(cx, cz);
        }
        self.0.pending_loads.lock().clear();
        self.0.chunk_queue.lock().queue.clear();
        *self.0.last_view.lock() = None;

        self.set_world(world.clone());

        let dimension = world.dimension();
        let dimension_type = self
            .0
            .server
            .registries()
            .dimension_type
            .get_id(&dimension)
            .unwrap_or(0) as i32;

        self.send_packet(&RespawnPacket {
            dimension_type,
            dimension_name: dimension.into(),
            hashed_seed: 93522819,
            game_mode: self.game_mode(),
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: 4,
            sea_level: 64,
            data_to_keep: 0,
        });

        self.send_packet(&GameEventPacket::START_WAITING_FOR_CHUNKS);

        let (cx, cz) = Chunk::to_chunk_pos(self.position());
        self.send_packet(&SetCenterChunkPacket {
            chunk_x: cx,
            chunk_z: cz,
        });
    }

    pub(crate) fn set_world(&self, world: World) {
        (*self.0.world.lock()) = Some(world)
    }

    /// Changes the tablist header for the player.
    ///
    /// Note: This will clear the footer.
    pub fn set_header(&self, text: impl Into<TextComponent>) {
        self.set_header_and_footer(text, TextComponent::EMPTY)
    }

    /// Changes the tablist footer for the player.
    ///
    /// Note: This will clear the header.
    pub fn set_footer(&self, text: impl Into<TextComponent>) {
        self.set_header_and_footer(TextComponent::EMPTY, text);
    }

    /// Changes both the tablist header and footer for the player.
    pub fn set_header_and_footer(
        &self,
        header: impl Into<TextComponent>,
        footer: impl Into<TextComponent>,
    ) {
        self.send_packet(&SetTablistHeaderFooterPacket {
            header: header.into(),
            footer: footer.into(),
        });
    }

    pub fn set_permission_level(&self, permission_level: u8) {
        self.0
            .permission_level
            .store(permission_level, Ordering::Relaxed);

        let level = entity_status::player::PERMISSION_LEVEL_0 + permission_level;
        self.update_status(level);
    }

    fn update_status(&self, status: u8) {
        let packet = EntityEventPacket {
            entity_id: self.id(),
            event: status,
        };
        self.broadcast_packet(&packet);
        self.send_packet(&packet);
    }

    pub fn permission_level(&self) -> u8 {
        self.0.permission_level.load(Ordering::Relaxed)
    }

    pub(crate) fn add_to_list_packet(&self) -> PlayerInfoUpdatePacket {
        PlayerInfoUpdatePacket {
            actions: (PlayerInfoFlags::ADD_PLAYER
                | PlayerInfoFlags::UPDATE_GAME_MODE
                | PlayerInfoFlags::UPDATE_LISTED)
                .bits(),
            players: vec![PlayerEntry {
                uuid: self.uuid(),
                player_actions: vec![
                    PlayerAction::AddPlayer {
                        name: self.0.game_profile.name.clone(),
                        properties: self.0.game_profile.properties.clone(),
                    },
                    PlayerAction::UpdateGameMode {
                        game_mode: self.game_mode(),
                    },
                    PlayerAction::UpdateListed { listed: true },
                ],
            }],
        }
    }

    fn keep_alive(&self) {
        self.send_packet(&KeepAlivePacket { keep_alive_id: 0 });
    }

    fn tick_keep_alive(&self) {
        const KEEP_ALIVE_INTERVAL: u64 = 20;

        let mut last_keep_alive = self.0.last_keep_alive.lock();
        if last_keep_alive.elapsed() > Duration::from_secs(KEEP_ALIVE_INTERVAL) {
            self.keep_alive();
            *last_keep_alive = Instant::now();
        }
    }
}

impl EntityLike for Player {
    fn id(&self) -> i32 {
        self.0.entity.id()
    }

    fn uuid(&self) -> Uuid {
        self.0.entity.uuid()
    }

    fn r#type(&self) -> EntityType {
        self.0.entity.r#type()
    }

    fn world(&self) -> World {
        self.0.world.lock().clone().unwrap()
    }

    fn position(&self) -> Position {
        self.0.entity.position()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }
}

impl Tickable for Player {
    fn tick(&self) {
        self.process_view_changes();

        // Keep Alive
        self.tick_keep_alive();

        // Chunks
        self.send_pending_chunks();
    }
}

impl Viewable for Player {
    fn add_viewer(&self, player: Player) {
        player.send_packet(&self.add_to_list_packet());

        self.0.entity.add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        player.send_packet(&PlayerInfoRemovePacket {
            uuids: vec![self.uuid()],
        });
        self.0.entity.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        self.0.entity.viewers()
    }
}

mod imp {
    use super::*;

    pub(crate) struct Player {
        pub(super) connection: Arc<Connection>,
        pub(super) game_profile: GameProfile,
        pub(super) entity: Entity,
        pub(super) world: Mutex<Option<World>>,
        pub(super) last_keep_alive: Mutex<Instant>,
        pub(super) game_mode: Mutex<GameMode>,
        pub(crate) chunk_queue: Mutex<ChunkQueue>,
        pub(super) teleport_id: AtomicI32,
        pub(super) permission_level: AtomicU8,

        pub(super) last_view: Mutex<Option<((i32, i32), i32)>>,
        pub(super) tracked_chunks: Mutex<HashMap<(i32, i32), TrackState>>,
        pub(super) pending_loads: Mutex<HashSet<(i32, i32)>>,
        pub(super) pending_notify: Notify,
        pub(super) dispatcher_started: AtomicBool,

        // Player Abilities
        pub(super) abilities: Abilities,

        // Inventory
        pub(super) inventory: Arc<PlayerInventory>,
        pub(super) open_inventory: Mutex<Option<Inventory>>,
        pub(super) held_slot: AtomicU8,

        pub(super) server: Server,
    }

    impl Player {
        pub(super) fn new(connection: Arc<Connection>, server: Server) -> Self {
            let game_profile = connection.game_profile.lock().clone().unwrap();
            Self {
                connection,
                game_profile: game_profile.clone(),
                entity: Entity::with_uuid(EntityType::PLAYER, game_profile.uuid),
                world: Mutex::new(None),
                last_keep_alive: Mutex::new(Instant::now()),
                game_mode: Mutex::new(GameMode::Survival),
                chunk_queue: Mutex::new(ChunkQueue::new()),
                teleport_id: AtomicI32::default(),
                permission_level: AtomicU8::default(),
                last_view: Mutex::new(None),
                tracked_chunks: Mutex::new(HashMap::default()),
                pending_loads: Mutex::new(HashSet::default()),
                pending_notify: Notify::new(),
                dispatcher_started: AtomicBool::new(false),
                abilities: Abilities::new(),
                inventory: Arc::new(PlayerInventory::new()),
                open_inventory: Mutex::new(None),
                held_slot: AtomicU8::default(),
                server,
            }
        }
    }
}
