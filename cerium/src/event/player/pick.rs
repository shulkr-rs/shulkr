use crate::{
    entity::{EntityType, Player},
    event::{Event, player::PlayerEvent},
    util::BlockPosition,
    world::block::Block,
};

pub struct PlayerPickBlockEvent {
    pub(crate) player: Player,
    pub(crate) position: BlockPosition,
    pub(crate) include_data: bool,
}

impl PlayerPickBlockEvent {
    pub fn position(&self) -> BlockPosition {
        self.position
    }

    pub fn include_data(&self) -> bool {
        self.include_data
    }
}

impl Event for PlayerPickBlockEvent {}

impl PlayerEvent for PlayerPickBlockEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

pub struct PlayerPickEntityEvent {
    pub(crate) player: Player,
    pub(crate) entity_type: EntityType,
    pub(crate) include_data: bool,
}

impl PlayerPickEntityEvent {
    pub fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn include_data(&self) -> bool {
        self.include_data
    }
}

impl Event for PlayerPickEntityEvent {}

impl PlayerEvent for PlayerPickEntityEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
