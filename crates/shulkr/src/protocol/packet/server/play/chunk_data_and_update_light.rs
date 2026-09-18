use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    world::{block::BlockEntity, heightmap::Heightmap},
};

#[derive(Debug, Clone)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: ChunkData,
    pub light: LightData,
}

impl Packet for ChunkDataAndUpdateLightPacket {}
impl ServerPacket for ChunkDataAndUpdateLightPacket {}

impl Encode for ChunkDataAndUpdateLightPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(this.chunk_x)?;
        w.write_i32(this.chunk_z)?;
        ChunkData::encode(w, &this.data)?;
        LightData::encode(w, &this.light)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: Vec<Heightmap>,
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
}

impl Encode for ChunkData {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.heightmaps, Heightmap::encode)?;
        w.write_array(&this.data, |b, v| b.write_u8(*v))?;
        w.write_array(&this.block_entities, BlockEntity::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LightData {}

impl Encode for LightData {
    fn encode<W: PacketWrite>(w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        let num_sections: usize = 26;
        let sky_y_mask_bytes = num_sections.div_ceil(8);

        // skyYMask: all 26 sections have sky light data
        w.write_varint(sky_y_mask_bytes as i32)?;
        for i in 0..sky_y_mask_bytes {
            let remaining_bits = num_sections - i * 8;
            let byte = if remaining_bits >= 8 {
                0xFF
            } else {
                (1u16 << remaining_bits) as u8 - 1
            };
            w.write_u8(byte)?;
        }

        // blockYMask: no block light sections
        w.write_varint(0)?; // empty byte array

        // emptySkyYMask: no empty sky sections
        w.write_varint(0)?;

        // emptyBlockYMask: no empty block sections
        w.write_varint(0)?;

        // skyUpdates: one 2048-byte array per set bit in skyYMask (26 total)
        // 0xFF in every nibble = light level 15 everywhere
        w.write_varint(num_sections as i32)?; // list length
        for _ in 0..num_sections {
            w.write_varint(2048)?; // byte array length prefix
            for _ in 0..2048 {
                w.write_u8(0xFF)?;
            }
        }

        // blockUpdates: empty list
        w.write_varint(0)?;

        Ok(())
    }
}
