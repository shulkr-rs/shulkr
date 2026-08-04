use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicI32, Ordering},
};
use uuid::Uuid;

use crate::{
    entity::{
        EntityType, Player,
        meta::{MetaAccessor, MetadataHolder, entity::EntityMeta},
    },
    protocol::packet::{RemoveEntitiesPacket, SetEntityMetadataPacket, SpawnEntityPacket},
    util::{EntityPose, Position, Viewable, Viewers},
    world::World,
};

#[derive(Clone)]
pub struct Entity(pub(crate) Arc<imp::Entity>);

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn with_uuid(entity_type: EntityType, uuid: Uuid) -> Self {
        let entity = imp::Entity::new(entity_type, uuid);
        Self(Arc::new(entity))
    }

    pub fn head_rotation(&self) -> f32 {
        *self.0.head_rotation.lock()
    }

    pub fn set_head_rotation(&self, value: f32) {
        *self.0.head_rotation.lock() = value;
    }

    pub fn set_position<P>(&self, position: P)
    where
        P: Into<Position>,
    {
        *self.0.position.lock() = position.into();
        // todo: teleport
    }

    pub fn meta<T>(&self) -> T
    where
        T: MetaAccessor,
    {
        T::new(self.0.holder.clone())
    }

    fn refresh_meta(&self) {
        self.broadcast_packet(&self.metadata_packet());
    }

    pub fn is_on_fire(&self) -> bool {
        self.meta::<EntityMeta>().is_on_fire()
    }

    pub fn set_on_fire(&self, value: bool) {
        self.meta::<EntityMeta>().set_on_fire(value);
        self.refresh_meta();
    }

    pub fn is_sneaking(&self) -> bool {
        self.meta::<EntityMeta>().is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        self.meta::<EntityMeta>().set_sneaking(value);
        self.refresh_meta();
    }

    pub fn is_swimming(&self) -> bool {
        self.meta::<EntityMeta>().is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        self.meta::<EntityMeta>().set_swimming(value);
        self.refresh_meta();
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.meta::<EntityMeta>().is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        self.meta::<EntityMeta>().set_flying_with_elytra(value);
        self.refresh_meta();
    }

    pub fn is_sprinting(&self) -> bool {
        self.meta::<EntityMeta>().is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.meta::<EntityMeta>().set_sprinting(value);
        self.refresh_meta();
    }

    pub fn get_pose(&self) -> EntityPose {
        self.meta::<EntityMeta>().get_pose()
    }

    pub fn set_pose(&self, value: EntityPose) {
        self.meta::<EntityMeta>().set_pose(value);
        self.refresh_meta();
    }

    pub fn is_on_ground(&self) -> bool {
        self.0.on_ground.load(Ordering::Acquire)
    }

    pub fn set_on_ground(&self, value: bool) {
        self.0.on_ground.store(value, Ordering::Release);
    }

    pub(crate) fn set_sneaking_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_sneaking(value));
    }

    pub(crate) fn set_swimming_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_swimming(value));
    }

    pub(crate) fn set_flying_with_elytra_with(&self, value: bool, flying: bool) {
        self.update_flag_and_pose(flying, |meta| meta.set_flying_with_elytra(value));
    }

    pub(crate) fn resolve_pose(&self, flying: bool) -> EntityPose {
        let meta = self.meta::<EntityMeta>();

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
        let meta = self.meta::<EntityMeta>();

        if meta.get_pose() == pose {
            return false;
        }
        meta.set_pose(pose);

        self.refresh_meta();
        true
    }

    fn update_flag_and_pose<F>(&self, flying: bool, f: F)
    where
        F: FnOnce(&EntityMeta),
    {
        let meta = self.meta::<EntityMeta>();
        f(&meta);

        let pose = self.resolve_pose(flying);
        meta.set_pose(pose);

        self.refresh_meta();
    }

    pub fn despawn(&self) {
        for viewer in self.viewers() {
            self.remove_viewer(viewer);
        }
    }

    pub(crate) fn metadata_packet(&self) -> SetEntityMetadataPacket {
        SetEntityMetadataPacket {
            entity_id: self.id(),
            entries: self.0.holder.entries.lock().clone(),
        }
    }

    pub(crate) fn spawn_packet(&self) -> SpawnEntityPacket {
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
}

impl Viewable for Entity {
    fn add_viewer(&self, player: Player) {
        self.0.viewers.add_viewer(player.clone());

        player.send_packet(&self.spawn_packet());
        player.send_packet(&self.metadata_packet());
    }

    fn remove_viewer(&self, player: Player) {
        self.0.viewers.remove_viewer(player.clone());

        player.send_packet(&RemoveEntitiesPacket {
            entity_ids: vec![self.id()],
        });
    }

    fn viewers(&self) -> &Viewers {
        &self.0.viewers
    }
}

impl EntityLike for Entity {
    fn id(&self) -> i32 {
        self.0.id
    }

    fn uuid(&self) -> Uuid {
        self.0.uuid
    }

    fn r#type(&self) -> EntityType {
        self.0.entity_type
    }

    fn world(&self) -> World {
        todo!()
    }

    fn position(&self) -> Position {
        *self.0.position.lock()
    }
}

mod imp {
    use super::*;

    pub(crate) struct Entity {
        pub(super) id: i32,
        pub(super) uuid: Uuid,
        pub(super) entity_type: EntityType,
        pub(super) position: Mutex<Position>,
        pub(super) head_rotation: Mutex<f32>,
        pub(super) holder: MetadataHolder,
        pub(super) on_ground: AtomicBool,
        pub(super) viewers: Viewers,
    }

    impl Entity {
        pub(super) fn new(entity_type: EntityType, uuid: Uuid) -> Self {
            fn generate_id() -> i32 {
                static CURRENT_ID: AtomicI32 = AtomicI32::new(1);
                CURRENT_ID.fetch_add(1, Ordering::Relaxed)
            }

            Self {
                id: generate_id(),
                uuid,
                entity_type,
                position: Mutex::new(Position::ZERO),
                head_rotation: Mutex::new(0.),
                holder: MetadataHolder::new(),
                on_ground: AtomicBool::default(),
                viewers: Viewers::new(),
            }
        }
    }
}

pub trait EntityLike {
    fn id(&self) -> i32;
    fn uuid(&self) -> Uuid;
    fn r#type(&self) -> EntityType;
    fn world(&self) -> World;
    fn position(&self) -> Position;
}
