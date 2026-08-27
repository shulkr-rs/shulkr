use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
    util::Position,
};

pub struct PlayerMoveEvent {
    pub(crate) player: Player,
    pub(crate) old_position: Position,
    pub(crate) new_position: Position,
}

impl PlayerMoveEvent {
    pub fn old_position(&self) -> Position {
        self.old_position
    }

    pub fn new_position(&self) -> Position {
        self.new_position
    }
}

impl Event for PlayerMoveEvent {}

impl PlayerEvent for PlayerMoveEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
