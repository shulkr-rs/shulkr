use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PickItemFromEntityPacket {
    pub entity_id: i32,
    pub include_data: bool,
}

impl Packet for PickItemFromEntityPacket {}
impl ClientPacket for PickItemFromEntityPacket {}

impl Decode for PickItemFromEntityPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id:    r.read_varint()?,
            include_data: r.read_bool()?,
        })
    }
}
