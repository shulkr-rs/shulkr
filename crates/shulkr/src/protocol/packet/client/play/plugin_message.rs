use crate::{
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    util::Key,
};

#[derive(Debug, Clone)]
pub struct PluginMessagePacket {
    pub identifier: Key,
    pub data: Vec<u8>,
}

impl Packet for PluginMessagePacket {}
impl ClientPacket for PluginMessagePacket {}

impl Decode for PluginMessagePacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            identifier: r.read_key()?,
            data:       r.read_bytes(-1)?,
        })
    }
}
