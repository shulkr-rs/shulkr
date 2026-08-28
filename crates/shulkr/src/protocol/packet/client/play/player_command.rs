use shulkr_macros::Enumeration;

use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerCommandPacket {
    pub entity_id: i32,
    pub action_id: PlayerCommand,
    pub jump_boost: i32,
}

impl Packet for PlayerCommandPacket {}
impl ClientPacket for PlayerCommandPacket {}

impl Decode for PlayerCommandPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let entity_id = r.read_varint()?;
        Ok(Self {
            entity_id,
            action_id: PlayerCommand::try_from(r.read_varint()?)?,
            jump_boost: r.read_varint()?,
        })
    }
}

#[derive(Enumeration)]
pub enum PlayerCommand {
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenVehicleInventory,
    StartFlyingWithElytra,
}
