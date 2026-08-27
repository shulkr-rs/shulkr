use crate::{
    inventory::Slot,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
pub struct SetCursorItemPacket {
    pub carried_item: Slot,
}

impl Packet for SetCursorItemPacket {}
impl ServerPacket for SetCursorItemPacket {}

impl Encode for SetCursorItemPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        Slot::encode(w, &this.carried_item)?;
        Ok(())
    }
}
