#![allow(unused)]

mod attack;
mod command;
mod config;
mod drop;
mod gamemode;
mod input;
mod mv;
mod pick;
mod sneak;
mod spawn;
mod teleport;

pub use attack::PlayerAttackEvent;
pub use command::CommandResultEvent;
pub use config::PlayerConfigEvent;
pub use gamemode::PlayerRequestGameModeEvent;
pub use input::PlayerInputEvent;
pub use mv::PlayerMoveEvent;
pub use pick::{PlayerPickBlockEvent, PlayerPickEntityEvent};
pub use sneak::{PlayerStartSneakingEvent, PlayerStopSneakingEvent};
pub use spawn::PlayerSpawnEvent;
pub use teleport::EntityTeleportEvent;

use crate::{entity::Player, event::Event};

pub trait PlayerEvent
where
    Self: Event,
{
    fn get_player(&self) -> &Player;
}
