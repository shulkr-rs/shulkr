use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct CommandSuggestionPacket {
    pub transaction_id: i32,
    pub text: String,
}

impl Packet for CommandSuggestionPacket {}
impl ClientPacket for CommandSuggestionPacket {}

impl Decode for CommandSuggestionPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            transaction_id: r.read_varint()?,
            text: r.read_string_limited::<32500>()?,
        })
    }
}
