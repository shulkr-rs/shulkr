pub mod advancement;
pub mod auth;
pub mod command;
pub mod entity;
pub mod event;
pub mod inventory;
pub mod item;
pub mod protocol;
pub mod registry;
pub mod scoreboard;
pub mod text;
pub mod tickable;
pub mod util;
pub mod world;

pub(crate) mod assets;
mod network;
mod server;
mod server_ping;

pub use server::Server;
pub use server_ping::*;

// Minecraft Constants

/// The current protocol version.
pub const PROTOCOL_VERSION: i32 = 776;

/// The name of the current protocol version.
pub const PROTOCOL_NAME: &str = "26.2";
