use crate::{
    entity::{Entity, GameMode, Player},
    event::{Event, player::PlayerEvent},
};

pub struct PlayerAttackEvent {
    pub(crate) player: Player,
    pub(crate) entity: Entity,
}

impl PlayerAttackEvent {
    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}

impl Event for PlayerAttackEvent {}

impl PlayerEvent for PlayerAttackEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
