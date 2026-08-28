use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct AttackPacket {
    pub entity_id: i32,
}

impl Packet for AttackPacket {}
impl ClientPacket for AttackPacket {}

impl Decode for AttackPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id: r.read_varint()?,
        })
    }
}
