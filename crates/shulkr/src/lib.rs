#[rustfmt::skip]
#[path = "../generated/version.rs"]
pub mod version;
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
pub mod server_ping;
pub mod text;
pub mod tickable;
pub mod util;
pub mod world;

mod network;
mod server;

pub use server::Server;
