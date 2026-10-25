use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct OptionsAppliedPacket {
    // Empty
}

impl Packet for OptionsAppliedPacket {}
impl ClientPacket for OptionsAppliedPacket {}

impl Decode for OptionsAppliedPacket {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
