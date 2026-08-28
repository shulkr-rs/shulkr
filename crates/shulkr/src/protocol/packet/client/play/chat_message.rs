use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ChatMessagePacket {
    pub message: String, // 256
    pub timestamp: i64,
    pub salt: i64,
    pub signature: Option<Box<[u8]>>,
    pub message_count: i32,
    pub acknowledged: Vec<u8>,
    pub checksum: u8,
}

impl Packet for ChatMessagePacket {}
impl ClientPacket for ChatMessagePacket {}

impl Decode for ChatMessagePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            message: r.read_string_limited::<256>()?,
            timestamp: r.read_i64()?,
            salt: r.read_i64()?,
            signature: r.read_option(|r| r.read_bytes(256).map(Into::into))?,
            message_count: r.read_varint()?,
            acknowledged: r.read_bytes(3)?,
            checksum: r.read_u8()?,
        })
    }
}
