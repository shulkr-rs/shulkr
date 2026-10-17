use super::login::DeathLocation;
use crate::{
    entity::GameMode,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Key,
};

pub const KEEP_ATTRIBUTE_MODIFIERS: u8 = 1;
pub const KEEP_ENTITY_DATA: u8 = 2;
pub const KEEP_ALL_DATA: u8 = 3;

#[derive(Debug, Clone)]
pub struct RespawnPacket {
    pub dimension_type: i32,
    pub dimension_name: Key,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: Option<GameMode>,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
    pub portal_cooldown: i32,
    pub sea_level: i32,
    pub data_to_keep: u8,
}

impl Packet for RespawnPacket {}
impl ServerPacket for RespawnPacket {}

impl Encode for RespawnPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.dimension_type)?;
        w.write_key(&this.dimension_name)?;
        w.write_i64(this.hashed_seed)?;
        w.write_varint(this.game_mode as i32)?;
        w.write_varint(this.previous_game_mode.map_or(0, |gm| gm as i32 + 1))?;
        w.write_bool(this.is_debug)?;
        w.write_bool(this.is_flat)?;
        w.write_option(&this.death_location, DeathLocation::encode)?;
        w.write_varint(this.portal_cooldown)?;
        w.write_varint(this.sea_level)?;
        w.write_u8(this.data_to_keep)?;
        Ok(())
    }
}
