use crate::{
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::copper_golem::{COPPER_GOLEM_STATE, WEATHERING_COPPER_STATE},
    },
    item::DataType2,
    protocol::{
        decode::{DecodeError, PacketRead},
        encode::{EncodeError, PacketWrite},
    },
};

pub struct CopperGolemMeta {
    holder: MetadataHolder,
}

impl CopperGolemMeta {
    pub fn get_weathering_state(&self) -> WeatheringCopperState {
        self.holder.get(WEATHERING_COPPER_STATE)
    }

    pub fn set_weathering_state(&self, value: WeatheringCopperState) {
        self.holder.set(WEATHERING_COPPER_STATE, value);
    }

    pub fn get_golem_state(&self) -> CopperGolemState {
        self.holder.get(COPPER_GOLEM_STATE)
    }

    pub fn set_golem_state(&self, value: CopperGolemState) {
        self.holder.set(COPPER_GOLEM_STATE, value);
    }
}

impl MetaAccessor for CopperGolemMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CopperGolemState {
    Idle,
    GettingItem,
    GettingNoItem,
    DroppingItem,
    DroppingNoItem,
}

impl TryFrom<i32> for CopperGolemState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => CopperGolemState::Idle,
            1 => CopperGolemState::GettingItem,
            2 => CopperGolemState::GettingNoItem,
            3 => CopperGolemState::DroppingItem,
            4 => CopperGolemState::DroppingNoItem,
            _ => return Err(()),
        };
        Ok(this)
    }
}

impl DataType2<CopperGolemState> for CopperGolemState {
    fn decode<R: PacketRead>(r: &mut R) -> Result<CopperGolemState, DecodeError> {
        CopperGolemState::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &CopperGolemState) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeatheringCopperState {
    Unaffected,
    Exposed,
    Weathered,
    Oxidized,
}

impl TryFrom<i32> for WeatheringCopperState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => Self::Unaffected,
            1 => Self::Exposed,
            2 => Self::Weathered,
            3 => Self::Oxidized,
            _ => return Err(()),
        };
        Ok(this)
    }
}

impl DataType2<WeatheringCopperState> for WeatheringCopperState {
    fn decode<R: PacketRead>(r: &mut R) -> Result<WeatheringCopperState, DecodeError> {
        WeatheringCopperState::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &WeatheringCopperState) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}
