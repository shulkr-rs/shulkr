use crate::{
    entity::{GameMode, Player},
    event::{Event, player::PlayerEvent},
};

pub struct PlayerRequestGameModeEvent {
    pub(crate) player: Player,
    pub(crate) game_mode: GameMode,
}

impl PlayerRequestGameModeEvent {
    pub fn requested_game_mode(&self) -> GameMode {
        self.game_mode
    }
}

impl Event for PlayerRequestGameModeEvent {}

impl PlayerEvent for PlayerRequestGameModeEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
