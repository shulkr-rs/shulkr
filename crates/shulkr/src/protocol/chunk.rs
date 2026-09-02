use crate::{
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        encode::{Encode, EncodeError, PacketWrite},
        packet::{ChunkData, ChunkDataAndUpdateLightPacket, LightData},
    },
    world::{
        block::{Block, BlockEntity, BlockState},
        chunk::{Chunk, ChunkSection},
        heightmap::Heightmap,
        palette::{Palette, PaletteFormat},
    },
};
use bytes::BytesMut;

impl Decode for BlockEntity {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            packed_xz: r.read_u8()?,
            y: r.read_i16()?,
            r#type: r.read_varint()?,
            data: Some(r.read_nbt()?),
        })
    }
}

impl Encode for BlockEntity {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.packed_xz)?;
        w.write_i16(this.y)?;
        w.write_varint(this.r#type)?;
        if let Some(data) = &this.data {
            w.write_nbt(data)?;
        } else {
            w.write_u8(0)?; // End Tag (temporary solution! todo!)
        }
        Ok(())
    }
}

impl Decode for Heightmap {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: r.read_varint()?,
            data: r.read_array(|r| r.read_i64())?,
        })
    }
}

impl Encode for Heightmap {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.kind)?;
        w.write_array(&this.data, |w, v| w.write_i64(*v))?;
        Ok(())
    }
}

impl Encode for Palette {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        let (bpe, format, values) = this.compute();
        w.write_u8(bpe)?;
        PaletteFormat::encode(w, &format)?;
        for value in &values {
            w.write_i64(*value)?;
        }
        Ok(())
    }
}

impl Encode for PaletteFormat {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        match this {
            PaletteFormat::SingleValued { value } => {
                w.write_varint(*value as i32)?;
            }
            PaletteFormat::Indirect { values } => {
                w.write_array(values, |buffer, value| buffer.write_varint(*value as i32))?;
            }
            PaletteFormat::Direct => {}
        }
        Ok(())
    }
}

impl ChunkSection {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self, fluid_count: i16) -> Result<(), EncodeError> {
        w.write_i16(this.block_states.count() as i16)?;
        w.write_i16(fluid_count)?;
        Palette::encode(w, &this.block_states)?;
        Palette::encode(w, &this.biomes)?;
        Ok(())
    }
}

impl From<&Chunk> for ChunkDataAndUpdateLightPacket {
    fn from(value: &Chunk) -> Self {
        let mut data = BytesMut::new();
        for section in value.sections() {
            let fluid_count = match section.block_states.format {
                PaletteFormat::SingleValued { value } => {
                    if is_fluid(value) {
                        section.block_states.count()
                    } else {
                        0
                    }
                }
                _ => section
                    .block_states
                    .count
                    .iter()
                    .filter(|(id, _)| is_fluid(**id))
                    .map(|(_, count)| *count)
                    .sum::<i32>(),
            };

            ChunkSection::encode(&mut data, &section, fluid_count as i16).unwrap()
        }

        let chunk_x = value.x();
        let chunk_z = value.z();

        let data = ChunkData {
            heightmaps: vec![],
            data: data.to_vec(),
            block_entities: value.block_entites(),
        };
        let light = LightData {};

        ChunkDataAndUpdateLightPacket {
            chunk_x,
            chunk_z,
            data,
            light,
        }
    }
}

fn is_fluid(state_id: u16) -> bool {
    BlockState::from_id(state_id)
        .is_some_and(|state| matches!(state.as_block(), Block::WATER | Block::LAVA))
}
