use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Key,
};

#[derive(Debug, Clone)]
pub struct LoginPluginRequestPacket {
    pub message_id: i32,
    pub channel: Key,
    pub data: Box<[u8]>,
}

impl Packet for LoginPluginRequestPacket {}
impl ServerPacket for LoginPluginRequestPacket {}

impl Encode for LoginPluginRequestPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.message_id)?;
        w.write_key(&this.channel)?;
        w.write_boxed_slice(&this.data)?;
        Ok(())
    }
}
