use crate::{PingResponse, event::Event};

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
