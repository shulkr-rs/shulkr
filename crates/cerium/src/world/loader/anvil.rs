use std::{
    collections::HashMap,
    fs::File,
    io::{Read as _, Seek as _, SeekFrom},
    path::{Path, PathBuf},
};

use bitfield_struct::bitfield;
use byteorder::{BigEndian, ReadBytesExt as _};
use bytes::Buf as _;
use cerium_nbt::{COMPOUND_ID, Nbt, NbtCompound, NbtTag};
use flate2::bufread::{GzDecoder, ZlibDecoder};
use thiserror::Error;

use crate::{
    registry::{Registry, RegistryKey},
    world::{
        biome::Biome,
        block::{Block, BlockState},
        chunk::Chunk,
    },
};

const SECTOR_SIZE: usize = 4096;
const BLOCKS_PER_SECTION: usize = 16 * 16 * 16;
const BIOMES_PER_SECTION: usize = 4 * 4 * 4;

pub struct AnvilLoader {
    path: PathBuf,
    min_section: i32,
    regions: HashMap<(i32, i32), Region>,
    decompress_buf: Vec<u8>,
    biomes: Registry<Biome>,
}

impl AnvilLoader {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            min_section: -4,
            regions: HashMap::new(),
            decompress_buf: Vec::new(),
            biomes: crate::registry::load_datapack("minecraft:worldgen/biome"),
        }
    }

    pub fn with_min_section(mut self, min_section: i32) -> Self {
        self.min_section = min_section;
        self
    }

    fn region(&mut self, rx: i32, rz: i32) -> Option<&mut Region> {
        if !self.regions.contains_key(&(rx, rz)) {
            let file = File::open(self.path.join(format!("r.{rx}.{rz}.mca"))).ok()?;
            self.regions.insert((rx, rz), Region::open(file)?);
        }
        self.regions.get_mut(&(rx, rz))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load_chunk(&mut self, cx: i32, cz: i32) -> Option<Chunk> {
        let (rx, rz) = (cx.div_euclid(32), cz.div_euclid(32));
        self.region(rx, rz)?;

        let region = self.regions.get_mut(&(rx, rz)).unwrap();
        let raw = region
            .read_chunk(cx, cz, &mut self.decompress_buf, &self.path)
            .ok()??;

        parse_chunk(cx, cz, raw.data, &self.biomes, self.min_section)
    }
}

pub struct Region {
    file: File,
    locations: [Location; 1024],
    timestamps: [u32; 1024],
}

impl Region {
    pub fn open(mut file: File) -> Option<Self> {
        let mut header = [0u8; SECTOR_SIZE * 2];
        file.read_exact(&mut header).ok()?;

        let locations = std::array::from_fn(|i| {
            Location(u32::from_be_bytes(
                header[i * 4..i * 4 + 4].try_into().unwrap(),
            ))
        });
        let timestamps = std::array::from_fn(|i| {
            let off = SECTOR_SIZE + i * 4;
            u32::from_be_bytes(header[off..off + 4].try_into().unwrap())
        });

        Some(Self {
            file,
            locations,
            timestamps,
        })
    }

    fn chunk_idx(cx: i32, cz: i32) -> usize {
        (cx.rem_euclid(32) + cz.rem_euclid(32) * 32) as usize
    }

    fn read_chunk(
        &mut self,
        cx: i32,
        cz: i32,
        decompress_buf: &mut Vec<u8>,
        region_root: &Path,
    ) -> Result<Option<RawChunk>, RegionError> {
        let idx = Self::chunk_idx(cx, cz);
        let location = self.locations[idx];
        let timestamp = self.timestamps[idx];

        if location.is_none() {
            return Ok(None);
        }

        let (sector_offset, sector_count) = location.offset_and_count();
        if sector_offset < 2 {
            return Err(RegionError::InvalidChunkSectorOffset);
        }

        self.file
            .seek(SeekFrom::Start(sector_offset * SECTOR_SIZE as u64))?;

        let exact_size = self.file.read_u32::<BigEndian>()? as usize;
        if exact_size == 0 {
            return Err(RegionError::MissingChunkStream);
        }
        if sector_count * SECTOR_SIZE < exact_size {
            return Err(RegionError::InvalidChunkSize);
        }

        let mut compression = self.file.read_u8()?;

        let data = if compression & 0x80 != 0 {
            compression &= !0x80;
            let mut buf = Vec::new();
            File::open(region_root.join(format!("c.{cx}.{cz}.mcc")))?.read_to_end(&mut buf)?;
            buf
        } else {
            let mut buf = vec![0u8; exact_size - 1];
            self.file.read_exact(&mut buf)?;
            buf
        };

        decompress_buf.clear();
        let mut nbt: &[u8] = match Compression::from_u8(compression) {
            Some(Compression::Gzip) => {
                GzDecoder::new(data.as_slice()).read_to_end(decompress_buf)?;
                decompress_buf
            }
            Some(Compression::Zlib) => {
                ZlibDecoder::new(data.as_slice()).read_to_end(decompress_buf)?;
                decompress_buf
            }
            Some(Compression::None) => data.as_slice(),
            None => return Err(RegionError::InvalidCompressionScheme(compression)),
        };

        if nbt.remaining() < 3 || nbt.get_u8() != COMPOUND_ID {
            return Err(RegionError::MissingChunkStream);
        }
        let name_len = nbt.get_u16() as usize;
        if nbt.remaining() < name_len {
            return Err(RegionError::MissingChunkStream);
        }
        nbt.advance(name_len);

        let data = NbtCompound::deserialize_content(&mut nbt)?;
        if !nbt.is_empty() {
            return Err(RegionError::TrailingNbtData);
        }

        Ok(Some(RawChunk { data, timestamp }))
    }
}

pub struct RawChunk {
    pub data: NbtCompound,
    pub timestamp: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Compression {
    Gzip = 1,
    Zlib = 2,
    None = 3,
}

impl Compression {
    fn from_u8(value: u8) -> Option<Compression> {
        match value {
            1 => Some(Compression::Gzip),
            2 => Some(Compression::Zlib),
            3 => Some(Compression::None),
            _ => None,
        }
    }
}

#[bitfield(u32)]
struct Location {
    count: u8,
    #[bits(24)]
    offset: u32,
}

impl Location {
    fn is_none(self) -> bool {
        self.0 == 0
    }

    fn offset_and_count(self) -> (u64, usize) {
        (u64::from(self.offset()), usize::from(self.count()))
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RegionError {
    #[error("an I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("chunk is allocated, but stream is missing")]
    MissingChunkStream,
    #[error("invalid chunk sector offset")]
    InvalidChunkSectorOffset,
    #[error("invalid chunk size")]
    InvalidChunkSize,
    #[error("invalid compression scheme number of {0}")]
    InvalidCompressionScheme(u8),
    #[error("failed to parse NBT: {0}")]
    Nbt(#[from] cerium_nbt::Error),
    #[error("not all chunk NBT data was read")]
    TrailingNbtData,
}

fn parse_chunk(
    cx: i32,
    cz: i32,
    mut nbt: NbtCompound,
    biomes: &Registry<Biome>,
    min_section: i32,
) -> Option<Chunk> {
    let mut chunk = Chunk::new(cx, cz, min_section * 16);

    let NbtTag::List(sections) = nbt.remove("sections")? else {
        return None;
    };
    if sections.is_empty() {
        return Some(chunk);
    }

    let sections: Vec<NbtCompound> = sections
        .into_iter()
        .map(|s| match s {
            NbtTag::Compound(c) => Some(c),
            _ => None,
        })
        .collect::<Option<_>>()?;

    let section_count = chunk.sections().len() as i32;
    for mut section in sections {
        let NbtTag::Byte(sect_y) = section.remove("Y")? else {
            return None;
        };

        let rel = i32::from(sect_y) - min_section;
        if rel < 0 || rel >= section_count {
            continue;
        }
        let rel = rel as u32;

        parse_block_states(&mut chunk, rel, section.remove("block_states")?)?;
        parse_biomes(&mut chunk, rel, section.remove("biomes")?, biomes)?;
    }

    if let Some(NbtTag::List(entities)) = nbt.remove("block_entities") {
        for entity in entities {
            parse_block_entity(&mut chunk, entity);
        }
    }

    Some(chunk)
}

fn parse_block_entity(chunk: &mut Chunk, entity: NbtTag) {
    let NbtTag::Compound(mut comp) = entity else {
        return;
    };

    let coord = |c: &NbtCompound, key| match c.get(key) {
        Some(NbtTag::Int(v)) => Some(*v),
        _ => None,
    };
    let (Some(x), Some(y), Some(z)) = (coord(&comp, "x"), coord(&comp, "y"), coord(&comp, "z"))
    else {
        return;
    };

    for key in ["x", "y", "z", "id", "keepPacked"] {
        comp.remove(key);
    }

    chunk.set_block_entity_data(
        (x & 15) as usize,
        y,
        (z & 15) as usize,
        y,
        Nbt::new(String::new(), comp),
    );
}

fn parse_block_states(chunk: &mut Chunk, sect_y: u32, tag: NbtTag) -> Option<()> {
    let NbtTag::Compound(mut states) = tag else {
        return None;
    };
    let NbtTag::List(palette) = states.remove("palette")? else {
        return None;
    };
    if !(1..=BLOCKS_PER_SECTION).contains(&palette.len()) {
        return None;
    }

    let mut blocks = Vec::with_capacity(palette.len());
    for entry in palette {
        blocks.push(parse_block(entry)?);
    }

    if blocks.len() == 1 {
        chunk.fill_block_state_section(sect_y, blocks[0].state_id());
        return Some(());
    }

    let NbtTag::LongArray(data) = states.remove("data")? else {
        return None;
    };
    unpack(&data, BLOCKS_PER_SECTION, blocks.len(), 4, |i, idx| {
        let x = i % 16;
        let z = i / 16 % 16;
        let y = i / (16 * 16);
        chunk.set_block(
            x as i32,
            (chunk.min_y() + (sect_y * 16) as i32 + y as i32) as i32,
            z as i32,
            &blocks[idx],
        );
    })
}

fn parse_block(entry: NbtTag) -> Option<BlockState> {
    let NbtTag::Compound(mut comp) = entry else {
        return None;
    };
    let NbtTag::String(name) = comp.remove("Name")? else {
        return None;
    };
    let mut state = Block::from_key(name)?.default_state();

    if let Some(NbtTag::Compound(props)) = comp.remove("Properties") {
        for (key, value) in props.children {
            let NbtTag::String(value) = value else {
                return None;
            };
            state = apply_property(state, &key, &value)?;
        }
    }
    Some(state)
}

fn apply_property(state: BlockState, name: &str, value: &str) -> Option<BlockState> {
    let prop = state
        .as_block()
        .data()
        .properties
        .iter()
        .find(|p| p.name() == name)?;
    let index = (0..prop.len()).find(|&i| prop.value_name_from_index(i) == value)?;
    state.with_index(*prop, index)
}

fn parse_biomes(
    chunk: &mut Chunk,
    sect_y: u32,
    tag: NbtTag,
    biomes: &Registry<Biome>,
) -> Option<()> {
    let NbtTag::Compound(mut comp) = tag else {
        return None;
    };
    let NbtTag::List(palette) = comp.remove("palette")? else {
        return None;
    };
    if !(1..=BIOMES_PER_SECTION).contains(&palette.len()) {
        return None;
    }

    let mut ids = Vec::with_capacity(palette.len());
    for entry in palette {
        let NbtTag::String(name) = entry else {
            return None;
        };

        let lookup = name.strip_prefix("minecraft:").unwrap_or(&name);
        ids.push(biomes.get_id(&RegistryKey::new(lookup.to_owned()))? as i32);
    }

    if ids.len() == 1 {
        chunk.fill_biome_section(sect_y, ids[0] as u16);
        return Some(());
    }

    let NbtTag::LongArray(data) = comp.remove("data")? else {
        return None;
    };
    unpack(&data, BIOMES_PER_SECTION, ids.len(), 1, |i, idx| {
        let x = i % 4;
        let z = i / 4 % 4;
        let y = i / (4 * 4);
        chunk.set_biome(
            (x * 4) as i32,
            (sect_y as usize * 16 + y * 4) as i32,
            (z * 4) as i32,
            ids[idx],
        );
    })
}

fn unpack(
    data: &[i64],
    count: usize,
    palette_len: usize,
    min_bits: usize,
    mut f: impl FnMut(usize, usize),
) -> Option<()> {
    let bits = bit_width(palette_len - 1).max(min_bits);
    let per_long = 64 / bits;
    if data.len() != count.div_ceil(per_long) {
        return None;
    }
    let mask = (1u64 << bits) - 1;

    let mut cell = 0;
    for &long in data {
        let long = long as u64;
        for slot in 0..per_long {
            if cell >= count {
                break;
            }
            let idx = ((long >> (bits * slot)) & mask) as usize;
            if idx >= palette_len {
                return None;
            }
            f(cell, idx);
            cell += 1;
        }
    }
    Some(())
}

fn bit_width(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()) as usize
}
