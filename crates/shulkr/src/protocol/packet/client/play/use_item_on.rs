use crate::{
    entity::Hand,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    world::block::hit_result::BlockHitResult,
};

#[derive(Debug, Clone)]
pub struct UseItemOnPacket {
    pub hand: Hand,
    pub hit_result: BlockHitResult,
    pub sequence: i32,
}

impl Packet for UseItemOnPacket {}
impl ClientPacket for UseItemOnPacket {}

impl Decode for UseItemOnPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let hand = Hand::decode(r)?;

        Ok(Self {
            hand,
            hit_result: BlockHitResult::decode(r)?,
            sequence:   r.read_varint()?,
        })
    }
}
