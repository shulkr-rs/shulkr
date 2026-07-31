use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

/// Not implemented yet.
pub struct PlayerStartSneakingEvent {
    pub(crate) player: Player,
}

impl Event for PlayerStartSneakingEvent {}

impl PlayerEvent for PlayerStartSneakingEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

/// Not implemented yet.
pub struct PlayerStopSneakingEvent {
    pub(crate) player: Player,
}

impl Event for PlayerStopSneakingEvent {}

impl PlayerEvent for PlayerStopSneakingEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
