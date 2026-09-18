use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PunchPacket;

impl Packet for PunchPacket {}
impl ClientPacket for PunchPacket {}

impl Decode for PunchPacket {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self)
    }
}
