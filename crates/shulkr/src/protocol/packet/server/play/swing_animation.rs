use crate::{
    entity::Hand,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};
use shulkr_macros::{DataType, Enumeration};

#[derive(Debug, Clone)]
pub struct SwingAnimationPacket {
    pub entity_id: i32,
    pub hand: Hand,
    pub animation: SwingAnimation,
}

impl Packet for SwingAnimationPacket {}
impl ServerPacket for SwingAnimationPacket {}

impl Encode for SwingAnimationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        Hand::encode(w, &this.hand)?;
        SwingAnimationType::encode(w, &this.animation.kind)?;
        w.write_varint(this.animation.duration)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SwingAnimation {
    pub kind: SwingAnimationType,
    pub duration: i32,
}

impl SwingAnimation {
    pub const DEFAULT: Self = Self {
        kind: SwingAnimationType::Whack,
        duration: 6,
    };
}

#[derive(Enumeration, DataType)]
pub enum SwingAnimationType {
    None,
    Whack,
    Stab,
}
