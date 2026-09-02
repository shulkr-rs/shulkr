use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct TickingStepPacket {
    pub tick_steps: i32,
}

impl Packet for TickingStepPacket {}
impl ServerPacket for TickingStepPacket {}

impl Encode for TickingStepPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.tick_steps)?;
        Ok(())
    }
}
