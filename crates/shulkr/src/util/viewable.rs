use parking_lot::RwLock;

use crate::{
    entity::Player,
    protocol::packet::{Packet, ServerPacket},
};

pub trait Viewable {
    fn viewers(&self) -> &Viewers;

    fn add_viewer(&self, player: Player) {
        self.viewers().add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        self.viewers().remove_viewer(player);
    }

    fn broadcast_packet<P>(&self, packet: &P)
    where
        P: Packet + ServerPacket + 'static,
    {
        for viewer in self.viewers() {
            viewer.send_packet(packet);
        }
    }
}

pub struct Viewers {
    viewers: RwLock<Vec<Player>>,
}

impl Viewers {
    pub fn new() -> Self {
        Self {
            viewers: RwLock::new(vec![]),
        }
    }

    pub fn add_viewer(&self, player: Player) {
        self.viewers.write().push(player);
    }

    pub fn remove_viewer(&self, player: Player) {
        self.viewers.write().retain(|other| *other != player);
    }

    pub fn iter(&self) -> Vec<Player> {
        self.viewers.read().clone()
    }

    pub fn len(&self) -> usize {
        self.viewers.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.viewers.read().is_empty()
    }

    pub fn broadcast_packet<P>(&self, packet: &P)
    where
        P: Packet + ServerPacket + 'static,
    {
        let viewers = self.viewers.read();
        for viewer in viewers.iter() {
            viewer.send_packet(packet);
        }
    }
}

impl Clone for Viewers {
    fn clone(&self) -> Self {
        Self {
            viewers: self.viewers.read().clone().into(),
        }
    }
}

impl IntoIterator for Viewers {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.into_inner().into_iter()
    }
}

impl<'a> IntoIterator for &'a Viewers {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.read().clone().into_iter()
    }
}
