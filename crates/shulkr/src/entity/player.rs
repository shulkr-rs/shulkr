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
        entity_status,
    },
    event::{Cancellable, inventory::InventoryOpenEvent, player::PlayerMoveEvent},
    inventory::{
        ARMOR_START, DragState, EquipmentSlot, HOTBAR_START, Inventory, OFFHAND_SLOT,
        PlayerInventory,
    },
    item::ItemStack,
    network::client::Connection,
    protocol::packet::{
        ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket,
        EntityEventPacket, EntityPositionRotationPacket, EntityRotationPacket, GameEventPacket,
        Packet, PlayerAbilities, PlayerAction, PlayerEntry, PlayerInfoFlags,
        PlayerInfoRemovePacket, PlayerInfoUpdatePacket, RespawnPacket, ServerPacket,
        SetCenterChunkPacket, SetHeadRotationPacket, SetTablistHeaderFooterPacket,
        SyncPlayerPositionPacket, SystemChatMessagePacket, UnloadChunkPacket,
        server::{PlayerAbilitiesPacket, SetHeldItemPacket, play::KeepAlivePacket},
    },
    text::TextComponent,
    tickable::Tickable,
    util::{EntityPose, Position, TeleportFlags, Viewable, Viewers},
    world::{World, chunk::Chunk},
};

pub const MAX_VIEW_DISTANCE: i32 = 32;

#[derive(Clone, PartialEq)]
pub struct Player(pub(crate) Arc<PlayerData>);

impl Player {
    pub(crate) fn new(connection: Arc<Connection>, server: Server) -> Self {
        Self(Arc::new(PlayerData::new(connection, server)))
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
        self.0.server()
    }

    pub fn despawn(&self) {
        self.0.despawn()
    }

    pub fn change_world(&self, world: World) {
        self.0.change_world(world)
    }

    /// Returns the player's inventory.
    ///
    /// Note: this is not the open inventory. Use [`Player#get_open_inventory()`] instead.
    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        self.0.inventory()
    }

    /// Returns the item currently carried on the cursor.
    pub fn carried_item(&self) -> ItemStack {
        self.inventory().carried_item()
    }

    /// Sets the item carried on the cursor.
    pub fn set_carried_item(&self, stack: ItemStack) {
        self.inventory().set_carried_item(stack)
    }

    pub(crate) fn drag_state(&self) -> Option<DragState> {
        self.inventory().drag_state()
    }

    pub(crate) fn set_drag_state(&self, state: Option<DragState>) {
        self.inventory().set_drag_state(state)
    }

    /// Opens an [`Inventory`] for a player.
    pub fn open_inventory(&self, inventory: Inventory) {
        PlayerData::open_inventory(self.clone(), inventory);
    }

    /// Closes the opened inventory if it is open.
    pub fn close_inventory(&self) {
        PlayerData::close_inventory(self.clone());
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

    /// Returns the currently selected hotbar slot (0-8).
    pub fn held_slot(&self) -> u8 {
        self.0.held_slot.load(Ordering::Acquire)
    }

    pub fn refresh_position(&self, new_position: Position) {
        let old_position = self.position();

        let mut event = PlayerMoveEvent {
            player: self.clone(),
            new_position,
            old_position,
        };
        self.server().events().fire(&mut event);

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

    pub fn set_on_ground(&self, value: bool) {
        self.0.set_on_ground(value)
    }

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

    /// Returns if the player is swimming.
    pub fn is_swimming(&self) -> bool {
        self.0.is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        self.0.set_swimming(value)
    }

    /// Returns if the player is gliding with an elytra.
    pub fn is_flying_with_elytra(&self) -> bool {
        self.0.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        self.0.set_flying_with_elytra(value)
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
        self.0.set_header_and_footer(header.into(), footer.into())
    }

    pub fn set_permission_level(&self, permission_level: u8) {
        self.0
            .permission_level
            .store(permission_level, Ordering::Relaxed);

        let level = entity_status::player::PERMISSION_LEVEL_0 + permission_level;
        self.0.update_status(level);
    }

    pub fn permission_level(&self) -> u8 {
        self.0.permission_level.load(Ordering::Relaxed)
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
                    let chunk = loop {
                        let chunk = world.load_chunk_async(pos.0, pos.1).await;

                        let mut tracked = this.0.tracked_chunks.lock();
                        if !matches!(tracked.get(&pos), Some(TrackState::Pending)) {
                            return;
                        }

                        if world.add_viewer(pos.0, pos.1) {
                            tracked.insert(pos, TrackState::Viewing);
                            break chunk;
                        }
                    };

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

type TrackedDiff = (Vec<(i32, i32)>, Vec<(i32, i32)>);

fn diff_tracked(
    tracked: &mut HashMap<(i32, i32), TrackState>,
    desired: &HashSet<(i32, i32)>,
) -> TrackedDiff {
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

pub(crate) struct PlayerData {
    connection: Arc<Connection>,
    game_profile: GameProfile,
    entity: Entity,
    world: Mutex<Option<World>>,
    last_keep_alive: Mutex<Instant>,
    game_mode: Mutex<GameMode>,
    pub(crate) chunk_queue: Mutex<ChunkQueue>,
    teleport_id: AtomicI32,
    permission_level: AtomicU8,

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

impl PlayerData {
    fn new(connection: Arc<Connection>, server: Server) -> Self {
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

    fn update_status(&self, status: u8) {
        let packet = EntityEventPacket {
            entity_id: self.id(),
            event: status,
        };
        self.broadcast_packet(&packet);
        self.send_packet(&packet);
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
        let inventory = this.0.open_inventory.lock().take();
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
            EquipmentSlot::MainHand => HOTBAR_START + self.held_slot.load(Ordering::Acquire) as i32,
            EquipmentSlot::OffHand => OFFHAND_SLOT,
            EquipmentSlot::Helmet => ARMOR_START,
            EquipmentSlot::Chestplate => ARMOR_START + 1,
            EquipmentSlot::Leggings => ARMOR_START + 2,
            EquipmentSlot::Boots => ARMOR_START + 3,
        };

        self.inventory.get_item_stack(slot_id)
    }

    fn set_held_slot(&self, slot: u8) {
        self.update_held_slot(slot);
        self.send_packet(&SetHeldItemPacket { slot: slot.into() });
    }

    pub fn update_held_slot(&self, slot: u8) {
        self.held_slot.store(slot, Ordering::Release);
    }

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

    fn send_pending_chunks(&self) {
        let mut queue = self.chunk_queue.lock();

        if queue.queue.is_empty() || queue.lead >= queue.max_lead {
            return;
        }

        let per_tick = queue.target_cpt;
        queue.pending_chunks = (queue.pending_chunks + per_tick).min(per_tick.max(1.));
        if queue.pending_chunks < 1. {
            return;
        }

        let quota = queue.pending_chunks as usize;
        let batch_size = if self.is_local() {
            queue.queue.len()
        } else {
            queue.queue.len().min(quota)
        };

        let center = Chunk::to_chunk_pos(self.position());
        queue.sort_by_distance_to(center);

        self.send_packet(&ChunkBatchStartPacket {});

        let mut sent = 0;
        while sent < batch_size
            && let Some(chunk) = queue.dequeue()
        {
            let packet: ChunkDataAndUpdateLightPacket = (&chunk).into();
            self.send_packet(&packet);
            sent += 1;
        }

        queue.pending_chunks -= sent as f32;
        self.send_packet(&ChunkBatchFinishedPacket {
            batch_size: sent as i32,
        });
        queue.lead += 1;
    }

    fn is_local(&self) -> bool {
        self.addr().ip().is_loopback()
    }

    pub(crate) fn set_world(&self, world: World) {
        (*self.world.lock()) = Some(world)
    }

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

    fn set_on_ground(&self, value: bool) {
        self.entity.set_on_ground(value);
    }

    fn next_teleport_id(&self) -> i32 {
        self.teleport_id.fetch_add(1, Ordering::Release)
    }

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
        let changed = self.flying() != value;
        self.abilities.flying.store(value, Ordering::Release);

        if changed {
            self.entity.refresh_pose(value);
        }
    }

    fn set_flying(&self, value: bool) {
        self.update_flying(value);
        self.send_packet(&self.entity.metadata_packet());
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
        self.send_packet(&self.entity.metadata_packet());
    }

    fn is_sprinting(&self) -> bool {
        self.entity.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.entity.set_sprinting(value);
        self.send_packet(&self.entity.metadata_packet());
    }

    fn is_sneaking(&self) -> bool {
        self.entity.is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        if self.is_sneaking() == value {
            return;
        }

        self.entity.set_sneaking_with(value, self.flying());
        self.send_packet(&self.entity.metadata_packet());
    }

    fn is_swimming(&self) -> bool {
        self.entity.is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        if self.is_swimming() == value {
            return;
        }

        self.entity.set_swimming_with(value, self.flying());
        self.send_packet(&self.entity.metadata_packet());
    }

    fn is_flying_with_elytra(&self) -> bool {
        self.entity.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        if self.is_flying_with_elytra() == value {
            return;
        }

        self.entity
            .set_flying_with_elytra_with(value, self.flying());
        self.send_packet(&self.entity.metadata_packet());
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

    fn change_world(&self, world: World) {
        let tracked: Vec<(i32, i32)> = self.tracked_chunks.lock().keys().copied().collect();
        for (cx, cz) in tracked {
            self.untrack_chunk(cx, cz);
        }
        self.pending_loads.lock().clear();
        self.chunk_queue.lock().queue.clear();
        *self.last_view.lock() = None;

        self.set_world(world.clone());

        let dimension = world.dimension();
        let dimension_type = self
            .server
            .registries()
            .dimension_type
            .get_id(&dimension)
            .unwrap_or(0) as i32;

        self.send_packet(&RespawnPacket {
            dimension_type,
            dimension_name: dimension.into(),
            hashed_seed: 93522819,
            game_mode: self.game_mode().into(),
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

    fn set_header_and_footer(&self, header: TextComponent, footer: TextComponent) {
        self.send_packet(&SetTablistHeaderFooterPacket { header, footer });
    }
}

impl Tickable for PlayerData {
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

impl Viewable for PlayerData {
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

impl PartialEq for PlayerData {
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }
}

impl EntityLike for PlayerData {
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
