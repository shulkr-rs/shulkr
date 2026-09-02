use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct TickingStatePacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}

impl Packet for TickingStatePacket {}
impl ServerPacket for TickingStatePacket {}

impl Encode for TickingStatePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_f32(this.tick_rate)?;
        w.write_bool(this.is_frozen)?;
        Ok(())
    }
}
