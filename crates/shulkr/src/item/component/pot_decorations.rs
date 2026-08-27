use crate::{
    item::Material,
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
    registry::Id,
};

pub struct PotDecorations {
    pub back: Material,
    pub left: Material,
    pub right: Material,
    pub front: Material,
}

impl DataType for PotDecorations {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let size = r.read_varint()?;
        if size != 4 {
            return Err(DecodeError::Decode("PotDecorations must be of size 4"));
        }

        Ok(Self {
            back: Material::from_id(r.read_varint()? as Id).unwrap_or(Material::BRICK),
            left: Material::from_id(r.read_varint()? as Id).unwrap_or(Material::BRICK),
            right: Material::from_id(r.read_varint()? as Id).unwrap_or(Material::BRICK),
            front: Material::from_id(r.read_varint()? as Id).unwrap_or(Material::BRICK),
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(
            &[this.back, this.left, this.right, this.front],
            |w, v| w.write_varint(Id::from(*v) as i32),
        )?;
        Ok(())
    }
}
