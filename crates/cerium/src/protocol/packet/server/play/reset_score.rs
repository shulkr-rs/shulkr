use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct ResetScorePacket {
    pub entity_name: String,
    pub objective_name: String,
}

impl Packet for ResetScorePacket {}
impl ServerPacket for ResetScorePacket {}

impl Encode for ResetScorePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.entity_name)?;
        w.write_string(&this.objective_name)?;
        Ok(())
    }
}
