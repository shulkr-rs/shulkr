use crate::{
    entity::GameMode,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
};

#[derive(Debug, Clone)]
pub struct PlayerRequestGameModePacket {
    pub game_mode: GameMode,
}

impl Packet for PlayerRequestGameModePacket {}
impl ClientPacket for PlayerRequestGameModePacket {}

impl Decode for PlayerRequestGameModePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            game_mode: GameMode::try_from(r.read_varint()?)?,
        })
    }
}
