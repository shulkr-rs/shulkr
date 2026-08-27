use crate::{
    item::FireworkExplosion,
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct Fireworks {
    pub flight_duration: i32,
    pub explosions: Vec<FireworkExplosion>,
}

impl DataType for Fireworks {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flight_duration: r.read_varint()?,
            explosions: r.read_array(FireworkExplosion::decode)?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.flight_duration)?;
        w.write_array(&this.explosions, FireworkExplosion::encode)?;
        Ok(())
    }
}
