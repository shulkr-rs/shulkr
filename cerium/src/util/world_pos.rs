use crate::{
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
    util::{BlockPosition, Identifier},
};

pub struct WorldPos {
    pub dimension: Identifier,
    pub position: BlockPosition,
}

impl DataType for WorldPos {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            dimension: r.read_identifier()?,
            position: r.read_position()?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_identifier(&this.dimension)?;
        w.write_position(&this.position)?;
        Ok(())
    }
}
