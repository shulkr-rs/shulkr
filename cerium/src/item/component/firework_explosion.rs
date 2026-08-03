use crate::{
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
    text::Rgb,
};

pub struct FireworkExplosion {
    pub shape: ExplosionShape,
    pub colors: Vec<Rgb>,
    pub fade_colors: Vec<Rgb>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}

impl DataType for FireworkExplosion {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            shape: ExplosionShape::try_from(r.read_varint()?)
                .map_err(|_| DecodeError::Decode("Invalid ExplosionShape"))?,
            colors: r.read_array(|r| r.read_varint().map(|v| Rgb::of(v as u32)))?,
            fade_colors: r.read_array(|r| r.read_varint().map(|v| Rgb::of(v as u32)))?,
            has_trail: r.read_bool()?,
            has_twinkle: r.read_bool()?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.shape as i32)?;
        w.write_array(&this.colors, |w, v| w.write_i32(v.to_hex() as i32))?;
        w.write_array(&this.fade_colors, |w, v| w.write_i32(v.to_hex() as i32))?;
        w.write_bool(this.has_trail)?;
        w.write_bool(this.has_twinkle)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl TryFrom<i32> for ExplosionShape {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SmallBall),
            1 => Ok(Self::LargeBall),
            2 => Ok(Self::Star),
            3 => Ok(Self::Creeper),
            4 => Ok(Self::Burst),
            _ => Err(()),
        }
    }
}
