use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    sound::SoundCategory,
    util::Key,
};

#[derive(Debug, Clone)]
pub struct StopSoundPacket {
    pub category: Option<SoundCategory>,
    pub sound: Option<Key>,
}

impl Packet for StopSoundPacket {}
impl ServerPacket for StopSoundPacket {}

impl Encode for StopSoundPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        let flags = this.category.is_some() as u8 | (this.sound.is_some() as u8) << 1;

        w.write_u8(flags)?;
        if let Some(category) = &this.category {
            SoundCategory::encode(w, category)?;
        }
        if let Some(sound) = &this.sound {
            w.write_key(sound)?;
        }
        Ok(())
    }
}
