use std::sync::atomic::{AtomicBool, Ordering};

use parking_lot::Mutex;

use super::Player;
use crate::protocol::packet::{PlayerAbilities, server::PlayerAbilitiesPacket};

pub struct Abilities {
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

impl Player {
    /// Returns if the player is invurnable.
    pub fn invurnable(&self) -> bool {
        self.0.abilities.invurnable.load(Ordering::Acquire)
    }

    pub fn set_invurnable(&self, value: bool) {
        self.0.abilities.invurnable.store(value, Ordering::Release);
        self.refresh_abilities();
    }

    /// Returns the flying speed of the player.
    pub fn flying_speed(&self) -> f32 {
        *self.0.abilities.flying_speed.lock()
    }

    pub fn set_flying_speed(&self, value: f32) {
        *self.0.abilities.flying_speed.lock() = value;
        self.refresh_abilities();
    }

    /// Returns the fov modifier of the player.
    pub fn fov_modifier(&self) -> f32 {
        *self.0.abilities.fov_modifier.lock()
    }

    pub fn set_fov_modifier(&self, value: f32) {
        *self.0.abilities.fov_modifier.lock() = value;
        self.refresh_abilities();
    }

    /// Returns if the player is allowed to fly.
    pub fn allow_flying(&self) -> bool {
        self.0.abilities.allow_flying.load(Ordering::Acquire)
    }

    pub fn set_allow_flying(&self, value: bool) {
        self.0
            .abilities
            .allow_flying
            .store(value, Ordering::Release);
        self.refresh_abilities();
    }

    /// Returns if the player is currently flying.
    pub fn flying(&self) -> bool {
        self.0.abilities.flying.load(Ordering::Acquire)
    }

    pub fn set_flying(&self, value: bool) {
        let changed = self.flying() != value;
        self.0.abilities.flying.store(value, Ordering::Release);

        if changed {
            self.0.entity.refresh_pose(value);
        }
        self.send_packet(&self.0.entity.metadata_packet());
        self.refresh_abilities();
    }

    fn insta_break(&self) -> bool {
        self.0.abilities.insta_break.load(Ordering::Acquire)
    }

    pub(super) fn set_insta_break(&self, value: bool) {
        self.0.abilities.insta_break.store(value, Ordering::Release);
    }

    pub fn refresh_abilities(&self) {
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
            flying_speed: *self.0.abilities.flying_speed.lock(),
            fov_modifier: *self.0.abilities.fov_modifier.lock(),
        });
    }
}
