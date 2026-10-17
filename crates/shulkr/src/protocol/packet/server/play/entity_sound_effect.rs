use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    sound::{SoundCategory, SoundEvent},
};

#[derive(Debug, Clone)]
pub struct EntitySoundEffectPacket {
    pub sound: SoundEvent,
    pub category: SoundCategory,
    pub entity_id: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

impl Packet for EntitySoundEffectPacket {}
impl ServerPacket for EntitySoundEffectPacket {}

impl Encode for EntitySoundEffectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(u16::from(this.sound) as i32 + 1)?;
        SoundCategory::encode(w, &this.category)?;
        w.write_varint(this.entity_id)?;
        w.write_f32(this.volume)?;
        w.write_f32(this.pitch)?;
        w.write_i64(this.seed)?;
        Ok(())
    }
}
