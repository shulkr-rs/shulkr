use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Identifier,
};

#[derive(Debug, Clone)]
pub struct PluginMessagePacket {
    pub identifier: Identifier,
    pub data: Vec<u8>,
}

impl Packet for PluginMessagePacket {}
impl ServerPacket for PluginMessagePacket {}

impl Encode for PluginMessagePacket {
    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
