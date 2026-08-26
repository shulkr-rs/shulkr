use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

pub struct PlayerStartSneakingEvent {
    pub(crate) player: Player,
}

impl Event for PlayerStartSneakingEvent {}

impl PlayerEvent for PlayerStartSneakingEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

pub struct PlayerStopSneakingEvent {
    pub(crate) player: Player,
}

impl Event for PlayerStopSneakingEvent {}

impl PlayerEvent for PlayerStopSneakingEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
