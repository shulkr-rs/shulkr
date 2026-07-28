use crate::{entity::Player, event::Event};

mod command;
mod player_config;
mod player_spawn;

pub use command::CommandResultEvent;
pub use player_config::PlayerConfigEvent;
pub use player_spawn::PlayerSpawnEvent;

pub trait PlayerEvent
where
    Self: Event,
{
    fn get_player(&self) -> &Player;
}
