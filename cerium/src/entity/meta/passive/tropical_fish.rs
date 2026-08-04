use crate::{
    entity::meta::{MetaAccessor, MetadataHolder, refs::tropical_fish::VARIANT},
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct TropicalFishMeta {
    holder: MetadataHolder,
}

impl TropicalFishMeta {
    pub fn get_pattern(&self) -> TropicalFishPattern {
        TropicalFishPattern::try_from(self.holder.get(VARIANT)).unwrap_or(TropicalFishPattern::Kob)
    }

    pub fn set_pattern(&self, value: TropicalFishPattern) {
        self.holder.set(VARIANT, value as i32);
    }
}

impl MetaAccessor for TropicalFishMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
}

impl TryFrom<i32> for TropicalFishPattern {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Kob,
            1 => Self::Sunstreak,
            2 => Self::Snooper,
            3 => Self::Dasher,
            4 => Self::Brinely,
            5 => Self::Spotty,
            6 => Self::Flopper,
            7 => Self::Stripey,
            8 => Self::Glitter,
            9 => Self::Blockfish,
            10 => Self::Betty,
            11 => Self::Clayfish,
            _ => return Err(()),
        })
    }
}

impl DataType for TropicalFishPattern {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        TropicalFishPattern::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid TropicalFishPattern"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
