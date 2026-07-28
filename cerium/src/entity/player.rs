use parking_lot::Mutex;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::sync::Notify;
use uuid::Uuid;

use crate::{
    Server,
    auth::GameProfile,
    entity::{
        EntityType, GameMode, Hand,
        entity::{Entity, EntityLike},
    },
    event::{Cancellable, inventory::InventoryOpenEvent},
    inventory::{Inventory, PlayerInventory},
    item::ItemStack,
    network::client::Connection,
    protocol::packet::{
        ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket,
        EntityPositionRotationPacket, EntityRotationPacket, GameEventPacket, Packet,
        PlayerAbilities, PlayerAction, PlayerEntry, PlayerInfoFlags, PlayerInfoRemovePacket,
        PlayerInfoUpdatePacket, ServerPacket, SetCenterChunkPacket, SetHeadRotationPacket,
        SetTablistHeaderFooterPacket, SyncPlayerPositionPacket, SystemChatMessagePacket,
        UnloadChunkPacket,
        server::{PlayerAbilitiesPacket, SetHeldItemPacket, play::KeepAlivePacket},
    },
    text::TextComponent,
    tickable::Tickable,
    util::{EntityPose, Position, TeleportFlags, Viewable, Viewers},
    world::{World, chunk::Chunk},
};

pub const MAX_VIEW_DISTANCE: i32 = 32;

#[derive(Clone, PartialEq)]
pub struct Player(pub(crate) Arc<Inner>);

impl Player {
    pub(crate) fn new(connection: Arc<Connection>, server: Server) -> Self {
        Self(Arc::new(Inner::new(connection, server)))
    }

    pub fn addr(&self) -> SocketAddr {
        self.0.addr()
    }

    pub fn name(&self) -> &String {
        self.0.name()
    }

    pub fn game_mode(&self) -> GameMode {
        self.0.game_mode()
    }

    pub fn set_game_mode(&self, game_mode: GameMode) {
        self.0.set_game_mode(game_mode)
    }

    pub fn send_message(&self, message: impl Into<TextComponent>) {
        self.0.send_message(message)
    }

    pub fn kick(&self, reason: impl Into<TextComponent>) {
        self.0.kick(reason)
    }

    pub fn send_packet<P>(&self, packet: &P)
    where
        P: Packet + ServerPacket + 'static,
    {
        self.0.send_packet(packet)
    }

    pub fn server(&self) -> &Server {
        &self.0.server()
    }

    pub fn despawn(&self) {
        self.0.despawn()
    }

    // ===== Inventory ======

    /// Returns the player's inventory.
    ///
    /// Note: this is not the open inventory. Use [`Player#get_open_inventory()`] instead.
    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        self.0.inventory()
    }

    /// Opens an [`Inventory`] for a player.
    pub fn open_inventory(&self, inventory: Inventory) {
        Inner::open_inventory(self.clone(), inventory);
    }

    /// Closes the opened inventory if it is open.
    pub fn close_inventory(&self) {
        Inner::close_inventory(self.clone());
    }

    /// Returns the open inventory.
    pub fn get_open_inventory(&self) -> Option<Inventory> {
        self.0.get_open_inventory()
    }

    pub fn get_item_in_hand(&self, hand: Hand) -> Option<ItemStack> {
        self.0.get_item_in_hand(hand)
    }

    pub fn get_equipment(&self, slot: EquipmentSlot) -> Option<ItemStack> {
        self.0.get_equipment(slot)
    }

    pub fn set_held_slot(&self, slot: u8) {
        self.0.set_held_slot(slot)
    }

    // ===== Position & Movement ======

    pub fn refresh_position(&self, new_position: Position) {
        self.0.update_position(new_position);
    }

    pub fn synchronize_position(
        &self,
        position: Position,
        velocity: Position,
        flags: TeleportFlags,
    ) {
        self.0.synchronize_position(position, velocity, flags)
    }

    pub fn head_roation(&self) -> f32 {
        self.0.head_roation()
    }

    pub fn set_head_roation(&self, value: f32) {
        self.0.set_head_roation(value)
    }

    pub fn is_on_ground(&self) -> bool {
        self.0.is_on_ground()
    }

    pub fn refresh_on_ground(&self, value: bool) {
        self.0.refresh_on_ground(value)
    }

    // ===== Abilities ======

    /// Returns if the player is invurnable.
    pub fn invurnable(&self) -> bool {
        self.0.invurnable()
    }

    pub fn set_invurnable(&self, value: bool) {
        self.0.update_invurnable(value);
        self.0.refresh_abilities();
    }

    /// Returns the flying speed of the player.
    pub fn flying_speed(&self) -> f32 {
        self.0.flying_speed()
    }

    pub fn set_flying_speed(&self, value: f32) {
        self.0.update_flying_speed(value);
        self.0.refresh_abilities();
    }

    /// Returns the fov modifier of the player.
    pub fn fov_modifier(&self) -> f32 {
        self.0.fov_modifier()
    }

    pub fn set_fov_modifier(&self, value: f32) {
        self.0.update_fov_modifier(value);
        self.0.refresh_abilities();
    }

    /// Returns if the player is allowed to fly.
    pub fn allow_flying(&self) -> bool {
        self.0.allow_flying()
    }

    pub fn set_allow_flying(&self, value: bool) {
        self.0.update_allow_flying(value);
        self.0.refresh_abilities();
    }

    /// Returns if the player is currently flying.
    pub fn flying(&self) -> bool {
        self.0.flying()
    }

    pub fn set_flying(&self, value: bool) {
        self.0.set_flying(value)
    }

    pub fn refresh_abilities(&self) {
        self.0.refresh_abilities()
    }

    pub fn set_pose(&self, pose: EntityPose) {
        self.0.set_pose(pose)
    }

    /// Returns if the player is sprinting.
    pub fn is_sprinting(&self) -> bool {
        self.0.is_sprinting()
    }

    /// Returns if the player is sneaking.
    pub fn is_sneaking(&self) -> bool {
        self.0.is_sneaking()
    }

    // ===== Scoreboard =====

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
        self.0.set_header_and_footer(header.into(), footer.into())
    }
}

impl EntityLike for Player {
    fn id(&self) -> i32 {
        self.0.id()
    }

    fn uuid(&self) -> Uuid {
        self.0.uuid()
    }

    fn r#type(&self) -> EntityType {
        self.0.r#type()
    }

    fn world(&self) -> World {
        self.0.world()
    }

    fn position(&self) -> Position {
        self.0.position()
    }
}

impl Tickable for Player {
    fn tick(&self) {
        self.process_view_changes();
        self.0.tick()
    }
}

impl Player {
    fn process_view_changes(&self) {
        let center = Chunk::to_chunk_pos(self.position());
        let view_distance = self.0.view_distance();

        {
            let mut last_view = self.0.last_view.lock();
            if *last_view == Some((center, view_distance)) {
                return;
            }
            *last_view = Some((center, view_distance));
        }

        let desired: HashSet<(i32, i32)> = Chunk::chunks_in_range(center, view_distance)
            .into_iter()
            .collect();

        let (added, stale) = diff_tracked(&mut self.0.tracked_chunks.lock(), &desired);

        for pos in stale {
            self.0.untrack_chunk(pos.0, pos.1);
        }

        if !added.is_empty() {
            self.0.pending_loads.lock().extend(added);
            self.0.pending_notify.notify_one();
        }

        self.ensure_load_dispatcher();
    }

    fn take_nearest_pending(&self) -> Option<(i32, i32)> {
        let (center_x, center_z) = Chunk::to_chunk_pos(self.position());
        let mut pending = self.0.pending_loads.lock();
        let nearest = pending.iter().copied().min_by_key(|&(cx, cz)| {
            let (dx, dz) = ((cx - center_x) as i64, (cz - center_z) as i64);
            dx * dx + dz * dz
        })?;
        pending.remove(&nearest);
        Some(nearest)
    }

    pub fn set_view_distance(&self, view_distance: i32) {
        self.0.connection.set_view_distance(view_distance);
    }

    fn ensure_load_dispatcher(&self) {
        if self.0.dispatcher_started.swap(true, Ordering::AcqRel) {
            return;
        }

        let this = self.clone();
        tokio::spawn(async move {
            while !this.0.connection.closed() {
                let notified = this.0.pending_notify.notified();

                let Some(pos) = this.take_nearest_pending() else {
                    let _ = tokio::time::timeout(Duration::from_millis(500), notified).await;
                    continue;
                };

                let permit = this.world().acquire_load_permit().await;

                let this = this.clone();
                tokio::spawn(async move {
                    let world = this.world();
                    let chunk = world.load_chunk_async(pos.0, pos.1).await;

                    {
                        let mut tracked = this.0.tracked_chunks.lock();
                        match tracked.get_mut(&pos) {
                            Some(state @ TrackState::Pending) => {
                                *state = TrackState::Viewing;
                                world.add_viewer(pos.0, pos.1);
                            }
                            _ => return,
                        }
                    }

                    while this.0.chunk_queue.lock().queue.len() > 256 {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                    drop(permit);
                    this.0.send_chunk(chunk);
                });
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrackState {
    Pending,
    Viewing,
}

fn diff_tracked(
    tracked: &mut HashMap<(i32, i32), TrackState>,
    desired: &HashSet<(i32, i32)>,
) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut added = Vec::new();
    for &pos in desired {
        if let std::collections::hash_map::Entry::Vacant(entry) = tracked.entry(pos) {
            entry.insert(TrackState::Pending);
            added.push(pos);
        }
    }
    let stale = tracked
        .keys()
        .copied()
        .filter(|pos| !desired.contains(pos))
        .collect();
    (added, stale)
}

impl Viewable for Player {
    fn add_viewer(&self, player: Player) {
        self.0.add_viewer(player)
    }

    fn remove_viewer(&self, player: Player) {
        self.0.remove_viewer(player)
    }

    fn viewers(&self) -> &Viewers {
        self.0.viewers()
    }
}

type SyncChunk = Chunk;

pub struct ChunkQueue {
    pub queue: Vec<SyncChunk>,
    pub target_cpt: f32,
    pub pending_chunks: f32,
    pub max_lead: i32,
    pub lead: i32,
}

impl ChunkQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            target_cpt: 9.,
            pending_chunks: 0.,
            max_lead: 1,
            lead: 0,
        }
    }

    pub fn enqueue(&mut self, chunk: SyncChunk) {
        self.queue.push(chunk);
    }

    pub fn cancel(&mut self, cx: i32, cz: i32) -> bool {
        let before = self.queue.len();
        self.queue
            .retain(|chunk| chunk.x() != cx || chunk.z() != cz);
        self.queue.len() != before
    }

    pub fn sort_by_distance_to(&mut self, from: (i32, i32)) {
        self.queue.sort_by_key(|chunk| {
            let dx = chunk.x() - from.0;
            let dz = chunk.z() - from.1;
            std::cmp::Reverse(dx * dx + dz * dz)
        });
    }

    pub fn dequeue(&mut self) -> Option<SyncChunk> {
        self.queue.pop()
    }
}

struct Abilities {
    flying: AtomicBool,
    allow_flying: AtomicBool,
    invurnable: AtomicBool,
    insta_break: AtomicBool,
    flying_speed: Mutex<f32>,
    fov_modifier: Mutex<f32>,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            flying: AtomicBool::default(),
            allow_flying: AtomicBool::default(),
            invurnable: AtomicBool::default(),
            insta_break: AtomicBool::default(),
            flying_speed: Mutex::new(0.05),
            fov_modifier: Mutex::new(0.1),
        }
    }
}

pub(crate) struct Inner {
    connection: Arc<Connection>,
    game_profile: GameProfile,
    entity: Entity,
    world: Mutex<Option<World>>,
    last_keep_alive: Mutex<Instant>,
    game_mode: Mutex<GameMode>,
    pub(crate) chunk_queue: Mutex<ChunkQueue>,
    teleport_id: AtomicI32,

    last_view: Mutex<Option<((i32, i32), i32)>>,
    tracked_chunks: Mutex<HashMap<(i32, i32), TrackState>>,
    pending_loads: Mutex<HashSet<(i32, i32)>>,
    pending_notify: Notify,
    dispatcher_started: AtomicBool,

    // Player Abilities
    abilities: Abilities,

    // Inventory
    inventory: Arc<PlayerInventory>,
    open_inventory: Mutex<Option<Inventory>>,
    held_slot: AtomicU8,

    server: Server,
}

impl Inner {
    fn new(connection: Arc<Connection>, server: Server) -> Self {
        let game_profile = connection.game_profile.lock().clone().unwrap();
        Self {
            connection,
            game_profile: game_profile.clone(),
            entity: Entity::new_with_uuid(EntityType::Player, game_profile.uuid),
            world: Mutex::new(None),
            last_keep_alive: Mutex::new(Instant::now()),
            game_mode: Mutex::new(GameMode::Survival),
            chunk_queue: Mutex::new(ChunkQueue::new()),
            teleport_id: AtomicI32::default(),
            last_view: Mutex::new(None),
            tracked_chunks: Mutex::new(HashMap::new()),
            pending_loads: Mutex::new(HashSet::new()),
            pending_notify: Notify::new(),
            dispatcher_started: AtomicBool::new(false),
            abilities: Abilities::new(),
            inventory: Arc::new(PlayerInventory::new()),
            open_inventory: Mutex::new(None),
            held_slot: AtomicU8::default(),
            server,
        }
    }

    fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    fn view_distance(&self) -> i32 {
        self.connection.view_distance()
    }

    fn name(&self) -> &String {
        &self.game_profile.name
    }

    fn game_mode(&self) -> GameMode {
        *self.game_mode.lock()
    }

    pub(crate) fn set_position(&self, position: Position) {
        self.entity.set_position(position);
    }

    fn set_game_mode(&self, game_mode: GameMode) {
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

        self.update_allow_flying(
            game_mode == GameMode::Creative || game_mode == GameMode::Spectator,
        );
        if game_mode != GameMode::Creative && game_mode != GameMode::Spectator {
            self.set_flying(false);
        }

        self.update_insta_break(game_mode == GameMode::Creative);

        self.refresh_abilities();
    }

    fn update_game_mode(&self, game_mode: GameMode) {
        *self.game_mode.lock() = game_mode;
    }

    fn send_message(&self, message: impl Into<TextComponent>) {
        self.send_packet(&SystemChatMessagePacket {
            content: message.into(),
            overlay: false,
        });
    }

    fn kick(&self, reason: impl Into<TextComponent>) {
        self.connection.kick(reason.into());
    }

    fn send_packet<P>(&self, packet: &P)
    where
        P: Packet + ServerPacket + 'static,
    {
        self.connection.send_packet(packet);
    }

    fn keep_alive(&self) {
        self.send_packet(&KeepAlivePacket { keep_alive_id: 0 });
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
                        name: self.game_profile.name.clone(),
                        properties: self.game_profile.properties.clone(),
                    },
                    PlayerAction::UpdateGameMode {
                        game_mode: self.game_mode(),
                    },
                    PlayerAction::UpdateListed { listed: true },
                ],
            }],
        }
    }

    fn server(&self) -> &Server {
        &self.server
    }

    // ===== Inventory ======

    fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.inventory
    }

    fn open_inventory(this: Player, inventory: Inventory) {
        let mut event = InventoryOpenEvent {
            player: this.clone(),
            inventory: inventory.clone(),
            cancelled: false,
        };
        this.server().events().fire(&mut event);

        if event.is_cancelled() {
            return;
        }

        if let Some(inventory) = this.get_open_inventory() {
            inventory.remove_viewer(this.clone());
        }

        inventory.add_viewer(this.clone());
        *this.0.open_inventory.lock() = Some(inventory);
    }

    fn close_inventory(this: Player) {
        let inventory = this.0.open_inventory.lock().clone();
        if let Some(inventory) = inventory {
            inventory.remove_viewer(this);
        }
    }

    fn get_open_inventory(&self) -> Option<Inventory> {
        self.open_inventory.lock().clone()
    }

    fn get_item_in_hand(&self, hand: Hand) -> Option<ItemStack> {
        self.get_equipment(if hand == Hand::MainHand {
            EquipmentSlot::MainHand
        } else {
            EquipmentSlot::OffHand
        })
    }

    fn get_equipment(&self, slot: EquipmentSlot) -> Option<ItemStack> {
        let slot_id = match slot {
            EquipmentSlot::MainHand => self.held_slot.load(Ordering::Acquire) + 36,
            EquipmentSlot::OffHand => 45,
            EquipmentSlot::Boots => 44,
            EquipmentSlot::Leggings => 43,
            EquipmentSlot::Chestplate => 42,
            EquipmentSlot::Helmet => 41,
        };

        self.inventory.get_item_stack(slot_id as i32)
    }

    fn set_held_slot(&self, slot: u8) {
        self.update_held_slot(slot);
        self.send_packet(&SetHeldItemPacket { slot: slot.into() });
    }

    pub fn update_held_slot(&self, slot: u8) {
        self.held_slot.store(slot, Ordering::Release);
    }

    // ===== World ======

    fn untrack_chunk(&self, cx: i32, cz: i32) {
        let Some(state) = self.tracked_chunks.lock().remove(&(cx, cz)) else {
            return;
        };

        if state == TrackState::Pending {
            self.pending_loads.lock().remove(&(cx, cz));
            return;
        }

        if !self.chunk_queue.lock().cancel(cx, cz) {
            self.send_packet(&UnloadChunkPacket {
                chunk_x: cx,
                chunk_z: cz,
            });
        }
        self.world().remove_viewer(cx, cz);
    }

    fn send_chunk(&self, chunk: SyncChunk) {
        let mut queue = self.chunk_queue.lock();
        queue.enqueue(chunk);
    }

    fn is_loopback(&self) -> bool {
        self.addr().ip().is_loopback()
    }

    fn send_pending_chunks(&self) {
        const MAX_CHUNKS_PER_TICK: f32 = 64.;

        let mut queue = self.chunk_queue.lock();

        if queue.queue.is_empty() || queue.lead >= queue.max_lead {
            return;
        }

        let per_tick = if self.is_loopback() {
            MAX_CHUNKS_PER_TICK
        } else {
            queue.target_cpt
        };
        queue.pending_chunks = (queue.pending_chunks + per_tick).min(64.);
        if queue.pending_chunks < 1. {
            return;
        }

        let center = Chunk::to_chunk_pos(self.position());
        queue.sort_by_distance_to(center);

        self.send_packet(&ChunkBatchStartPacket {});

        let mut batch_size = 0;
        while queue.pending_chunks >= 1.
            && let Some(chunk) = queue.dequeue()
        {
            let packet: ChunkDataAndUpdateLightPacket = (&chunk).into();
            self.send_packet(&packet);

            queue.pending_chunks -= 1.;
            batch_size += 1;
        }

        self.send_packet(&ChunkBatchFinishedPacket { batch_size });
        queue.lead += 1;
    }

    pub(crate) fn set_world(&self, world: World) {
        (*self.world.lock()) = Some(world)
    }

    // ===== Position & Movement ======

    fn update_position(&self, new_position: Position) -> bool {
        let old_position = self.position();

        self.set_position(new_position);
        self.set_head_roation(new_position.yaw());

        let old_chunk = Chunk::to_chunk_pos(old_position);
        let new_chunk = Chunk::to_chunk_pos(new_position);

        let chunk_changed = old_chunk != new_chunk;
        if chunk_changed {
            self.send_packet(&SetCenterChunkPacket {
                chunk_x: new_chunk.0,
                chunk_z: new_chunk.1,
            });
        }

        let head_rotation = new_position.yaw();

        let distance_x = (new_position.x() - old_position.x()).abs();
        let distance_y = (new_position.y() - old_position.y()).abs();
        let distance_z = (new_position.z() - old_position.z()).abs();

        let position_changed = (distance_x + distance_y + distance_z) > 0.;
        let rotation_changed = (new_position.yaw() != old_position.yaw())
            || (new_position.pitch() != old_position.pitch());

        let on_ground = self.is_on_ground();
        match () {
            _ if distance_x > 8. || distance_y > 8. || distance_z > 8. => {
                log::warn!("todo: teleport player because he moved more than 8 blocks.")
            }
            _ if position_changed && rotation_changed => {
                self.broadcast_packet(&EntityPositionRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
                self.broadcast_packet(&SetHeadRotationPacket::new(self.id(), head_rotation));
            }
            _ if position_changed => {
                self.broadcast_packet(&EntityPositionRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
            }
            _ if rotation_changed => {
                self.broadcast_packet(&EntityRotationPacket::new(
                    self.id(),
                    new_position,
                    old_position,
                    on_ground,
                ));
                self.broadcast_packet(&SetHeadRotationPacket::new(self.id(), head_rotation));
            }
            _ => {}
        }

        chunk_changed
    }

    fn synchronize_position(&self, position: Position, velocity: Position, flags: TeleportFlags) {
        let teleport_id = self.next_teleport_id();
        self.send_packet(&SyncPlayerPositionPacket {
            teleport_id,
            position,
            velocity_x: velocity.x(),
            velocity_y: velocity.y(),
            velocity_z: velocity.z(),
            yaw: position.yaw(),
            pitch: position.pitch(),
            flags,
        });
    }

    fn head_roation(&self) -> f32 {
        self.entity.head_rotation()
    }

    fn set_head_roation(&self, value: f32) {
        self.entity.set_head_rotation(value);
    }

    fn is_on_ground(&self) -> bool {
        self.entity.is_on_ground()
    }

    fn refresh_on_ground(&self, value: bool) {
        self.entity.refresh_on_ground(value);
    }

    fn next_teleport_id(&self) -> i32 {
        self.teleport_id.fetch_add(1, Ordering::Release)
    }

    // ===== Abilities ======

    fn insta_break(&self) -> bool {
        self.abilities.insta_break.load(Ordering::Acquire)
    }

    fn update_insta_break(&self, value: bool) {
        self.abilities.insta_break.store(value, Ordering::Release);
    }

    fn invurnable(&self) -> bool {
        self.abilities.invurnable.load(Ordering::Acquire)
    }

    fn update_invurnable(&self, value: bool) {
        self.abilities.invurnable.store(value, Ordering::Release);
    }

    fn flying_speed(&self) -> f32 {
        *self.abilities.flying_speed.lock()
    }

    fn update_flying_speed(&self, value: f32) {
        *self.abilities.flying_speed.lock() = value;
    }

    fn fov_modifier(&self) -> f32 {
        *self.abilities.fov_modifier.lock()
    }

    fn update_fov_modifier(&self, value: f32) {
        *self.abilities.fov_modifier.lock() = value;
    }

    fn allow_flying(&self) -> bool {
        self.abilities.allow_flying.load(Ordering::Acquire)
    }

    fn update_allow_flying(&self, value: bool) {
        self.abilities.allow_flying.store(value, Ordering::Release);
    }

    fn flying(&self) -> bool {
        self.abilities.flying.load(Ordering::Acquire)
    }

    fn update_flying(&self, value: bool) {
        self.abilities.flying.store(value, Ordering::Release);

        if self.flying() != value {
            let pose = self.entity.pose();
            match () {
                _ if self.is_sneaking() && pose == EntityPose::Standing => {
                    self.update_pose(EntityPose::Sneaking);
                }
                _ if pose == EntityPose::Sneaking => {
                    self.update_pose(EntityPose::Standing);
                }
                _ => {}
            }
        }
    }

    fn set_flying(&self, value: bool) {
        self.update_flying(value);
        self.send_packet(&self.entity.0.metadata_packet());
        self.refresh_abilities();
    }

    fn refresh_abilities(&self) {
        let mut flags = PlayerAbilities::empty();
        if self.invurnable() {
            flags |= PlayerAbilities::INVURNABLE;
        }
        if self.flying() {
            flags |= PlayerAbilities::FLYING;
        }
        if self.allow_flying() {
            flags |= PlayerAbilities::ALLOW_FLYING;
        }
        if self.insta_break() {
            flags |= PlayerAbilities::CREATIVE_MODE;
        }

        self.send_packet(&PlayerAbilitiesPacket {
            flags,
            flying_speed: *self.abilities.flying_speed.lock(),
            fov_modifier: *self.abilities.fov_modifier.lock(),
        });
    }

    fn update_pose(&self, pose: EntityPose) {
        self.entity.set_pose(pose);
    }

    fn set_pose(&self, pose: EntityPose) {
        self.update_pose(pose);
        self.send_packet(&self.entity.0.metadata_packet());
    }

    fn is_sprinting(&self) -> bool {
        self.entity.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.entity.set_sprinting(value);
        self.send_packet(&self.entity.0.metadata_packet());
    }

    fn is_sneaking(&self) -> bool {
        self.entity.is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        self.entity.set_sneaking(value);
        self.send_packet(&self.entity.0.metadata_packet());
    }

    fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }

        let world = self.world();
        self.pending_loads.lock().clear();
        for (pos, state) in self.tracked_chunks.lock().drain() {
            if state == TrackState::Viewing {
                world.remove_viewer(pos.0, pos.1);
            }
        }
    }

    // ===== Scoreboard =====

    fn set_header_and_footer(&self, header: TextComponent, footer: TextComponent) {
        self.send_packet(&SetTablistHeaderFooterPacket { header, footer });
    }
}

impl Tickable for Inner {
    fn tick(&self) {
        // Keep Alive
        {
            let mut last_keep_alive = self.last_keep_alive.lock();
            if last_keep_alive.elapsed() > Duration::from_secs(20) {
                self.keep_alive();
                *last_keep_alive = Instant::now();
            }
        }

        // Chunks
        self.send_pending_chunks();
    }
}

impl Viewable for Inner {
    fn add_viewer(&self, player: Player) {
        player.send_packet(&self.add_to_list_packet());

        self.entity.add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        player.send_packet(&PlayerInfoRemovePacket {
            uuids: vec![self.uuid()],
        });
        self.entity.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        self.entity.viewers()
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }
}

impl EntityLike for Inner {
    fn id(&self) -> i32 {
        self.entity.id()
    }

    fn uuid(&self) -> Uuid {
        self.entity.uuid()
    }

    fn r#type(&self) -> EntityType {
        self.entity.r#type()
    }

    fn world(&self) -> World {
        self.world.lock().clone().unwrap()
    }

    fn position(&self) -> Position {
        self.entity.position()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl EquipmentSlot {
    pub fn slot_id(&self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 0,
            Self::Boots => 0,
            Self::Leggings => 0,
            Self::Chestplate => 0,
            Self::Helmet => 0,
        }
    }
}
