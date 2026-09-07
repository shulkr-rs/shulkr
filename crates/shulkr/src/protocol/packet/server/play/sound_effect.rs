use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    sound::{SoundCategory, SoundEvent},
    util::Point,
};

#[derive(Debug, Clone)]
pub struct SoundEffectPacket {
    pub sound: SoundEvent,
    pub category: SoundCategory,
    pub position: Point,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

impl Packet for SoundEffectPacket {}
impl ServerPacket for SoundEffectPacket {}

impl Encode for SoundEffectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(u16::from(this.sound) as i32 + 1)?;
        SoundCategory::encode(w, &this.category)?;
        w.write_i32((this.position.x() * 8.0) as i32)?;
        w.write_i32((this.position.y() * 8.0) as i32)?;
        w.write_i32((this.position.z() * 8.0) as i32)?;
        w.write_f32(this.volume)?;
        w.write_f32(this.pitch)?;
        w.write_i64(this.seed)?;
        Ok(())
    }
}
