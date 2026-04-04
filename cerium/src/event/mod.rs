mod events;
pub use events::Events;

use crate::PingResponse;

pub mod inventory;
pub mod player;

pub trait Event: Send {}

pub struct ServerListPingEvent {
    pub(crate) response: PingResponse,
}

impl ServerListPingEvent {
    pub fn new(response: PingResponse) -> Self {
        Self { response }
    }

    pub fn get_response(&self) -> &PingResponse {
        &self.response
    }

    pub fn set_response(&mut self, response: PingResponse) {
        self.response = response;
    }
}

impl Event for ServerListPingEvent {}

pub trait Cancellable: Event {
    fn set_cancelled(&mut self, value: bool);

    fn is_cancelled(&self) -> bool;
}
