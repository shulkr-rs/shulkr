use uuid::Uuid;

use crate::{
    auth::{GameProfile, Property},
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
// todo: change to gameprofile
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
    /// Per-login session id. Added in 26.2 (proto 776): `login_finished` now
    /// carries a trailing UUID after the Game Profile.
    pub session_id: Uuid,
}

impl Packet for LoginSuccessPacket {}
impl ServerPacket for LoginSuccessPacket {}

impl Encode for LoginSuccessPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_uuid(&this.uuid)?;
        w.write_string(&this.username)?;
        w.write_array(&this.properties, Property::encode)?;
        w.write_uuid(&this.session_id)?;
        Ok(())
    }
}

impl From<GameProfile> for LoginSuccessPacket {
    fn from(value: GameProfile) -> Self {
        Self {
            uuid: value.uuid,
            username: value.name,
            properties: value.properties,
            session_id: Uuid::new_v4(),
        }
    }
}
