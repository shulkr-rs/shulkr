mod events;
pub use events::Events;

pub mod inventory;
pub mod player;
pub mod server_ping;

pub trait Event: Send {}

pub trait Cancellable: Event {
    fn set_cancelled(&mut self, value: bool);

    fn is_cancelled(&self) -> bool;
}
