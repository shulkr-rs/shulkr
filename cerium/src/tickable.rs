use std::time::Duration;
use tokio::time::{Interval, interval};

use crate::Server;

pub trait Tickable {
    fn tick(&self);
}

pub struct Ticker {
    server: Server,
    interval: Interval,
}

impl Ticker {
    pub fn new() -> Self {
        Self {
            server: Server::current(),
            interval: interval(Duration::from_millis(50)),
        }
    }

    pub async fn tick(&mut self) {
        self.interval.tick().await;

        let server = self.server.clone();

        for player in &*server.players().lock() {
            player.tick();
        }
    }
}
