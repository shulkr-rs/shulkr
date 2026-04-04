use crate::{
    entity::Hand,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
};

#[derive(Debug, Clone)]
pub struct UseItemPacket {
    pub hand: Hand,
    pub sequence: i32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Packet for UseItemPacket {}
impl ClientPacket for UseItemPacket {}

impl Decode for UseItemPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hand:     Hand::decode(r)?,
            sequence: r.read_varint()?,
            yaw:      r.read_f32()?,
            pitch:    r.read_f32()?,
        })
    }
}
