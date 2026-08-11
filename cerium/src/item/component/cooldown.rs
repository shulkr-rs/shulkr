use crate::{
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
    util::Key,
};

pub struct Cooldown {
    pub seconds: f32,
    pub group: Option<Key>,
}

impl DataType for Cooldown {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            seconds: r.read_f32()?,
            group: r.read_option(R::read_identifier)?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_f32(this.seconds)?;
        w.write_option(&this.group, W::write_identifier)?;
        Ok(())
    }
}
