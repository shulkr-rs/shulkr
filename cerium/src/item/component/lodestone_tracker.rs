use crate::{
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
    util::WorldPos,
};

pub struct LodestoneTracker {
    pub world_pos: Option<WorldPos>,
    pub tracked: bool,
}

impl DataType for LodestoneTracker {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            world_pos: r.read_option(WorldPos::decode)?,
            tracked: r.read_bool()?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_option(&this.world_pos, WorldPos::encode)?;
        w.write_bool(this.tracked)?;
        Ok(())
    }
}
