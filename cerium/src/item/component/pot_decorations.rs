use crate::{
    item::Material,
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
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
            back: Material::from_id(r.read_varint()?).unwrap_or(Material::Brick),
            left: Material::from_id(r.read_varint()?).unwrap_or(Material::Brick),
            right: Material::from_id(r.read_varint()?).unwrap_or(Material::Brick),
            front: Material::from_id(r.read_varint()?).unwrap_or(Material::Brick),
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(
            &vec![this.back, this.left, this.right, this.front],
            |w, v| w.write_varint(*v as i32),
        )?;
        Ok(())
    }
}
