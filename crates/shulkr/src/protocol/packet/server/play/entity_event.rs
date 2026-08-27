use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct EntityEventPacket {
    pub entity_id: i32,
    pub event: u8,
}

impl Packet for EntityEventPacket {}
impl ServerPacket for EntityEventPacket {}

impl Encode for EntityEventPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(this.entity_id)?;
        w.write_u8(this.event)?;
        Ok(())
    }
}
