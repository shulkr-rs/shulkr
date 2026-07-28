use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub data: Option<Box<[u8]>>,
}

impl Packet for LoginPluginResponsePacket {}
impl ClientPacket for LoginPluginResponsePacket {}

impl Decode for LoginPluginResponsePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            message_id: r.read_varint()?,
            data: r.read_option(|r| r.read_bytes(-1).map(Vec::into_boxed_slice))?,
        })
    }
}
