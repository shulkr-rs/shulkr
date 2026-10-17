use crate::{
    auth::{GameProfile, Property},
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginSuccessPacket {
    pub game_profile: GameProfile,
    pub session_id: Uuid,
}

impl Packet for LoginSuccessPacket {}
impl ServerPacket for LoginSuccessPacket {}

impl Encode for LoginSuccessPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_uuid(&this.game_profile.uuid)?;
        w.write_string(&this.game_profile.name)?;
        w.write_array(&this.game_profile.properties, Property::encode)?;
        w.write_uuid(&this.session_id)?;
        Ok(())
    }
}
