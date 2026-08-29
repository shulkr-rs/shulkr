use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
};

pub struct EntityTeleportEvent {
    pub(crate) player: Player,
}

impl Event for EntityTeleportEvent {}

impl PlayerEvent for EntityTeleportEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
