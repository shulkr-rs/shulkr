use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct PlayerChatMessagePacket {}

impl Packet for PlayerChatMessagePacket {}
impl ServerPacket for PlayerChatMessagePacket {}

impl Encode for PlayerChatMessagePacket {
    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}
