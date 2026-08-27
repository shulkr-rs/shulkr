use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
    protocol::packet::PlayerInputFlags,
};

pub struct PlayerInputEvent {
    pub(crate) player: Player,
    pub(crate) flags: PlayerInputFlags,
}

impl PlayerInputEvent {
    pub fn is_holding_forward_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::FORWARD)
    }

    pub fn is_holding_backward_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::BACKWARD)
    }

    pub fn is_holding_left_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::LEFT)
    }

    pub fn is_holding_right_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::RIGHT)
    }

    pub fn is_holding_jump_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::JUMP)
    }

    pub fn is_holding_sneak_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::SNEAK)
    }

    pub fn is_holding_sprint_key(&self) -> bool {
        self.flags.contains(PlayerInputFlags::SPRINT)
    }
}

impl Event for PlayerInputEvent {}

impl PlayerEvent for PlayerInputEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
