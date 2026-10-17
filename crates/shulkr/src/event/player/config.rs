use crate::{
    entity::Player,
    event::{Event, player::PlayerEvent},
    util::Position,
    world::World,
};

pub struct PlayerConfigEvent {
    pub(crate) player: Player,
    pub(crate) world: Option<World>,
    pub(crate) position: Option<Position>,
}

impl Event for PlayerConfigEvent {}

impl PlayerEvent for PlayerConfigEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl PlayerConfigEvent {
    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn get_world(&self) -> Option<World> {
        self.world.clone()
    }

    pub fn set_position(&mut self, position: impl Into<Position>) {
        self.position = Some(position.into());
    }

    pub fn get_position(&self) -> Option<&Position> {
        self.position.as_ref()
    }
}
