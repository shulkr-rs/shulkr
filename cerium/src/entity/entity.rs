use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, Ordering},
};
use uuid::Uuid;

use crate::{
    entity::{EntityType, Player, meta::entity::EntityMeta},
    protocol::packet::{RemoveEntitiesPacket, SetEntityMetadataPacket, SpawnEntityPacket},
    util::{EntityPose, Position, Viewable, Viewers},
    world::World,
};

#[derive(Clone)]
pub struct Entity(pub(crate) Arc<Inner>);

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::new_with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn new_with_uuid(entity_type: EntityType, uuid: Uuid) -> Self {
        Self(Arc::new(Inner::new(entity_type, uuid)))
    }

    pub fn head_rotation(&self) -> f32 {
        self.0.head_rotation()
    }

    pub fn set_head_rotation(&self, value: f32) {
        self.0.set_head_rotation(value)
    }

    pub fn set_pose(&self, pose: EntityPose) {
        self.0.set_pose(pose)
    }

    pub fn set_position<P>(&self, position: P)
    where
        P: Into<Position>,
    {
        self.0.set_position(position)
    }

    pub fn set_on_fire(&self, value: bool) {
        self.0.set_on_fire(value)
    }

    pub fn is_sneaking(&self) -> bool {
        self.0.is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        self.0.set_sneaking(value)
    }

    pub fn is_swimming(&self) -> bool {
        self.0.is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        self.0.set_swimming(value)
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.0.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        self.0.set_flying_with_elytra(value)
    }

    pub fn is_sprinting(&self) -> bool {
        self.0.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.0.set_sprinting(value)
    }

    pub fn pose(&self) -> EntityPose {
        self.0.pose()
    }

    pub fn is_on_ground(&self) -> bool {
        self.0.is_on_ground()
    }

    pub fn refresh_on_ground(&self, value: bool) {
        self.0.refresh_on_ground(value)
    }

    pub fn despawn(&self) {
        self.0.despawn()
    }
}

impl Viewable for Entity {
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

impl EntityLike for Entity {
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

pub(crate) struct Inner {
    id: i32,
    uuid: Uuid,
    entity_type: EntityType,
    position: Mutex<Position>,
    head_rotation: Mutex<f32>,
    meta: Mutex<EntityMeta>,
    on_ground: AtomicBool,
    viewers: Viewers,
}

impl Inner {
    fn new(entity_type: EntityType, uuid: Uuid) -> Self {
        Self {
            id: Self::generate_id(),
            uuid,
            entity_type,
            position: Mutex::new(Position::ZERO),
            head_rotation: Mutex::new(0.),
            meta: Mutex::new(EntityMeta::new()),
            on_ground: AtomicBool::default(),
            viewers: Viewers::new(),
        }
    }

    /// Generates a new unique entity id.
    fn generate_id() -> i32 {
        static CURRENT_ID: AtomicI32 = AtomicI32::new(1);
        CURRENT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn head_rotation(&self) -> f32 {
        *self.head_rotation.lock()
    }

    pub fn set_head_rotation(&self, value: f32) {
        *self.head_rotation.lock() = value;
    }

    pub fn set_pose(&self, pose: EntityPose) {
        {
            let mut meta = self.meta.lock();
            meta.set_pose(pose);
        }
        self.refresh_meta();
    }

    pub fn set_position<P>(&self, position: P)
    where
        P: Into<Position>,
    {
        *self.position.lock() = position.into();

        // todo: teleport
    }

    pub fn set_on_fire(&self, value: bool) {
        {
            let mut meta = self.meta.lock();
            meta.set_on_fire(value);
        }
        self.refresh_meta();
    }

    pub fn is_sneaking(&self) -> bool {
        self.meta.lock().is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        self.set_sneaking_with(value, false)
    }

    pub(crate) fn set_sneaking_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_sneaking(value));
    }

    pub fn is_swimming(&self) -> bool {
        self.meta.lock().is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        self.set_swimming_with(value, false)
    }

    pub(crate) fn set_swimming_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_swimming(value));
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.meta.lock().is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        self.set_flying_with_elytra_with(value, false)
    }

    pub(crate) fn set_flying_with_elytra_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_flying_with_elytra(value));
    }

    pub(crate) fn resolve_pose(&self, flying: bool) -> EntityPose {
        let meta = self.meta.lock();

        if meta.is_flying_with_elytra() {
            EntityPose::FallFlying
        } else if meta.is_swimming() {
            EntityPose::Swimming
        } else if meta.is_sneaking() && !flying {
            EntityPose::Sneaking
        } else {
            EntityPose::Standing
        }
    }

    pub(crate) fn refresh_pose(&self, flying: bool) -> bool {
        let pose = self.resolve_pose(flying);
        {
            let mut meta = self.meta.lock();
            if meta.get_pose() == pose {
                return false;
            }
            meta.set_pose(pose);
        }
        self.refresh_meta();
        true
    }

    fn update_flag_and_pose<F>(&self, flying: bool, f: F)
    where
        F: FnOnce(&mut EntityMeta),
    {
        {
            let mut meta = self.meta.lock();
            f(&mut meta);
        }

        let pose = self.resolve_pose(flying);
        {
            let mut meta = self.meta.lock();
            meta.set_pose(pose);
        }
        self.refresh_meta();
    }

    fn refresh_meta(&self) {
        self.broadcast_packet(&self.metadata_packet());
    }

    pub fn is_sprinting(&self) -> bool {
        let meta = self.meta.lock();
        meta.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        {
            let mut meta = self.meta.lock();
            meta.set_sprinting(value);
        }
        self.refresh_meta();
    }

    pub(crate) fn metadata_packet(&self) -> SetEntityMetadataPacket {
        SetEntityMetadataPacket {
            entity_id: self.id(),
            entries: self.meta.lock().holder.entries.clone(),
        }
    }

    pub fn pose(&self) -> EntityPose {
        self.meta.lock().get_pose()
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_ground.load(Ordering::Acquire)
    }

    pub fn refresh_on_ground(&self, value: bool) {
        self.on_ground.store(value, Ordering::Release);
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        let position = self.position();

        SpawnEntityPacket {
            id: self.id(),
            uuid: self.uuid(),
            entity_type: self.r#type() as i32,
            position,
            head_yaw: position.yaw(),
            data: 0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
        }
    }

    pub fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }
    }
}

impl Viewable for Inner {
    fn add_viewer(&self, player: Player) {
        self.viewers.add_viewer(player.clone());

        player.send_packet(&self.spawn_packet());
        player.send_packet(&self.metadata_packet());
    }

    fn remove_viewer(&self, player: Player) {
        self.viewers.remove_viewer(player.clone());

        player.send_packet(&RemoveEntitiesPacket {
            entity_ids: vec![self.id()],
        });
    }

    fn viewers(&self) -> &Viewers {
        &self.viewers
    }
}

pub trait EntityLike {
    fn id(&self) -> i32;
    fn uuid(&self) -> Uuid;
    fn r#type(&self) -> EntityType;
    fn world(&self) -> World;
    fn position(&self) -> Position;
}

impl EntityLike for Inner {
    fn id(&self) -> i32 {
        self.id
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn r#type(&self) -> EntityType {
        self.entity_type
    }

    fn world(&self) -> World {
        todo!()
    }

    fn position(&self) -> Position {
        *self.position.lock()
    }
}
