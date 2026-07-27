use crate::{entity::Player, event::Event};

mod player_config;
mod player_spawn;
mod command;

pub use player_config::PlayerConfigEvent;
pub use player_spawn::PlayerSpawnEvent;
pub use command::CommandResultEvent;

pub trait PlayerEvent
where
    Self: Event,
{
    fn get_player(&self) -> &Player;
}
