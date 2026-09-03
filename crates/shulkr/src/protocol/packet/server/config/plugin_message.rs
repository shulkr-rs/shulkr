use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Key,
};

#[derive(Debug, Clone)]
pub struct PluginMessagePacket {
    pub identifier: Key,
    pub data: Box<[u8]>,
}

impl Packet for PluginMessagePacket {}
impl ServerPacket for PluginMessagePacket {}

impl Encode for PluginMessagePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_key(&this.identifier)?;
        w.write_boxed_slice(&this.data)?;
        Ok(())
    }
}
