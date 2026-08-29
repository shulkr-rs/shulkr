use super::Player;
use crate::util::EntityPose;

impl Player {
    pub fn set_pose(&self, pose: EntityPose) {
        self.0.entity.set_pose(pose);
        self.send_packet(&self.0.entity.metadata_packet());
    }

    /// Returns if the player is sprinting.
    pub fn is_sprinting(&self) -> bool {
        self.0.entity.is_sprinting()
    }

    pub fn set_sprinting(&self, value: bool) {
        self.0.entity.set_sprinting(value);
        self.send_packet(&self.0.entity.metadata_packet());
    }

    /// Returns if the player is sneaking.
    pub fn is_sneaking(&self) -> bool {
        self.0.entity.is_sneaking()
    }

    pub fn set_sneaking(&self, value: bool) {
        if self.is_sneaking() == value {
            return;
        }

        self.0.entity.set_sneaking_with(value, self.flying());
        self.send_packet(&self.0.entity.metadata_packet());
    }

    /// Returns if the player is swimming.
    pub fn is_swimming(&self) -> bool {
        self.0.entity.is_swimming()
    }

    pub fn set_swimming(&self, value: bool) {
        if self.is_swimming() == value {
            return;
        }

        self.0.entity.set_swimming_with(value, self.flying());
        self.send_packet(&self.0.entity.metadata_packet());
    }

    /// Returns if the player is gliding with an elytra.
    pub fn is_flying_with_elytra(&self) -> bool {
        self.0.entity.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&self, value: bool) {
        if self.is_flying_with_elytra() == value {
            return;
        }

        self.0
            .entity
            .set_flying_with_elytra_with(value, self.flying());
        self.send_packet(&self.0.entity.metadata_packet());
    }
}
