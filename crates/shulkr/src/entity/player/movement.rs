use std::sync::atomic::Ordering;

use super::Player;
use crate::{
    entity::EntityLike,
    event::player::{EntityTeleportEvent, PlayerMoveEvent},
    protocol::packet::{
        EntityPositionRotationPacket, EntityRotationPacket, SetCenterChunkPacket,
        SetHeadRotationPacket, SyncPlayerPositionPacket,
    },
    util::{Position, TeleportFlags, Velocity, Viewable},
    world::chunk::Chunk,
};

impl Player {
    pub(crate) fn set_position(&self, position: Position) {
        self.0.entity.set_position(position);
    }

    pub fn refresh_position(&self, new_position: Position) {
        let old_position = self.position();

        let mut event = PlayerMoveEvent {
            player: self.clone(),
            new_position,
            old_position,
        };
        self.server().events().fire(&mut event);

        self.update_position(new_position);
    }

    fn update_position(&self, new_position: Position) -> bool {
        let old_position = self.position();

        self.0.entity.set_position(new_position);
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

    pub fn synchronize_position(
        &self,
        position: Position,
        velocity: Velocity,
        flags: TeleportFlags,
    ) {
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

    fn next_teleport_id(&self) -> i32 {
        self.0.teleport_id.fetch_add(1, Ordering::Release)
    }

    pub fn head_roation(&self) -> f32 {
        self.0.entity.head_rotation()
    }

    pub fn set_head_roation(&self, value: f32) {
        self.0.entity.set_head_rotation(value);
    }

    pub fn is_on_ground(&self) -> bool {
        self.0.entity.is_on_ground()
    }

    pub fn set_on_ground(&self, value: bool) {
        self.0.entity.set_on_ground(value);
    }

    /// Teleports the player to a `position`, zeroing velocity and teleportation flags.
    ///
    /// Fires an [`EntityTeleportEvent`] before moving the player.
    ///
    /// # Examples
    /// Teleport the player to a fixed position.
    /// ```
    /// # use shulkr::entity::{EntityLike, Player};
    /// # use shulkr::util::Position;
    /// # fn example(player: &Player) {
    /// player.teleport_to([0.5, 75.0, 0.5]);
    /// assert_eq!(player.position(), Position::from([0.5, 75.0, 0.5]));
    /// # }
    /// ```
    pub fn teleport_to(&self, position: impl Into<Position>) {
        self.teleport(position, Velocity::ZERO, TeleportFlags::empty());
    }

    /// Teleports the player to a specified position, applying the given velocity and resolving
    /// relative axes/rotation per flags (see [`TeleportFlags`]) against the player's current
    /// position.
    ///
    /// Fires an [`EntityTeleportEvent`] before moving the player.
    ///
    /// # Examples
    /// Teleport the player to a fixed location: with no flags set, `position` is an
    /// absolute coordinate, not a delta.
    /// ```
    /// # use shulkr::entity::Player;
    /// # use shulkr::util::{TeleportFlags, Velocity};
    /// # fn example(player: &Player) {
    /// player.teleport([0.5, 64.0, 0.5], Velocity::ZERO, TeleportFlags::empty());
    /// # }
    /// ```
    ///
    /// Teleport the player 1 block up and send them upward with some velocity,
    /// leaving x/z and rotation where they are.
    /// ```
    /// # use shulkr::entity::Player;
    /// # use shulkr::util::{TeleportFlags, Velocity};
    /// # fn example(player: &Player) {
    /// player.teleport(
    ///     [0.0, 1.0, 0.0],
    ///     Velocity::new(0.0, 1.0, 0.0),
    ///     TeleportFlags::X
    ///         | TeleportFlags::Y
    ///         | TeleportFlags::Z
    ///         | TeleportFlags::YAW
    ///         | TeleportFlags::PITCH,
    /// );
    /// # }
    /// ```
    pub fn teleport(
        &self,
        position: impl Into<Position>,
        velocity: impl Into<Velocity>,
        flags: TeleportFlags,
    ) {
        let position = position.into().relative_to(self.position(), &flags);
        let position = Position::clamp_max(position);
        let velocity = velocity.into();

        let event = &mut EntityTeleportEvent {
            player: self.clone(),
        };
        self.server().events().fire(event);

        self.0.entity.set_position(position.clone());
        self.synchronize_position(position, velocity, TeleportFlags::empty());
    }
}
