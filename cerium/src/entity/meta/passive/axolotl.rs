use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::axolotl::{PLAYING_DEAD, SPAWNED_FROM_BUCKET, VARIANT},
    },
    protocol::{
        DataType,
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct AxolotlMeta {
    holder: MetadataHolder,
}

impl AxolotlMeta {
    pub fn get_variant(&self) -> AxolotlVariant {
        AxolotlVariant::try_from(self.holder.get(VARIANT)).unwrap_or(AxolotlVariant::Lucy)
    }

    pub fn set_variant(&self, value: AxolotlVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn is_playing_dead(&self) -> bool {
        self.holder.get(PLAYING_DEAD)
    }

    pub fn set_playing_dead(&self, value: bool) {
        self.holder.set(PLAYING_DEAD, value);
    }

    pub fn is_from_bucket(&self) -> bool {
        self.holder.get(SPAWNED_FROM_BUCKET)
    }

    pub fn set_from_bucket(&self, value: bool) {
        self.holder.set(SPAWNED_FROM_BUCKET, value);
    }
}

impl MetaAccessor for AxolotlMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AxolotlVariant {
    Lucy,
    Wild,
    Gold,
    Cyan,
    Blue,
}

impl TryFrom<i32> for AxolotlVariant {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Lucy,
            1 => Self::Wild,
            2 => Self::Gold,
            3 => Self::Cyan,
            4 => Self::Blue,
            _ => return Err(()),
        })
    }
}

impl DataType for AxolotlVariant {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        AxolotlVariant::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid AxolotlVariant"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
