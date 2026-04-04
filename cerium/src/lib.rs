#![feature(const_convert)]
#![feature(const_trait_impl)]

pub mod advancement;
pub mod auth;
pub mod entity;
pub mod event;
pub mod handle;
pub mod inventory;
pub mod item;
pub mod protocol;
pub mod registry;
pub mod scoreboard;
pub mod text;
pub mod tickable;
pub mod util;
pub mod world;

mod server;
pub use server::{NoServerError, Server};

mod network;
