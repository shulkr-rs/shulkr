use shulkr_macros::Enumeration;

use crate::{
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    util::BlockPosition,
    world::block::BlockFace,
};

#[derive(Debug, Clone)]
pub struct PlayerActionPacket {
    pub status: PlayerDiggingState,
    pub position: BlockPosition,
    pub face: BlockFace,
    pub sequence: i32,
}

impl Packet for PlayerActionPacket {}
impl ClientPacket for PlayerActionPacket {}

impl Decode for PlayerActionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let status = PlayerDiggingState::try_from(r.read_varint()?)?;
        let position = r.read_position()?;
        let face = BlockFace::try_from(i32::from(r.read_u8()?))?;

        Ok(Self {
            status,
            position,
            face,
            sequence: r.read_varint()?,
        })
    }
}

#[derive(Enumeration)]
pub enum PlayerDiggingState {
    StartDigging,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropItem,
    ItemUpdated,
    SwapItemInHand,
}
