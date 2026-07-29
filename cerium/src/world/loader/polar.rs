//! Loader for the [Polar](https://github.com/hollow-cube/polar) single-file
//! world format.
//!
//! The whole file is read and decoded up front into owned [`Chunk`]s; pull them
//! out with [`PolarLoader::load_chunk`] (each chunk is yielded once).

use std::collections::HashMap;
use std::path::Path;

use bytes::Buf as _;
use cerium_nbt::{Nbt, NbtCompound};
use thiserror::Error;

use crate::{
    registry::{DynamicRegistry, RegistryKey},
    world::{
        biome::Biome,
        block::{Block, BlockState},
        chunk::Chunk,
    },
};

const MAGIC: u32 = 0x506F_6C72; // "Polr"

// Supported format revisions. Behaviour differs across a handful of them; see
// https://github.com/hollow-cube/polar.
const VERSION_USERDATA_OPT_BLOCK_ENT_NBT: u16 = 2;
const VERSION_MINESTOM_NBT_READ_BREAK: u16 = 3;
const VERSION_WORLD_USERDATA: u16 = 4;
const VERSION_DATA_CONVERTER: u16 = 6;
const VERSION_UNIFIED_LIGHT: u16 = 1;
const VERSION_IMPROVED_LIGHT: u16 = 7;
const LATEST_VERSION: u16 = 7;

const BLOCKS_PER_SECTION: usize = 16 * 16 * 16;
const BIOMES_PER_SECTION: usize = 4 * 4 * 4;
const LIGHT_BYTES: usize = 2048;

const LIGHT_PRESENT: u8 = 3;

type LoadHook = Box<dyn FnMut(&Chunk, &[u8])>;
type SaveHook = Box<dyn FnMut(&Chunk, &mut Vec<u8>)>;

pub struct PolarLoader {
    min_section: i8,
    max_section: i8,
    user_data: Vec<u8>,
    chunks: HashMap<(i32, i32), Chunk>,
    chunk_user_data: HashMap<(i32, i32), Vec<u8>>,
    on_load: Option<LoadHook>,
    on_save: Option<SaveHook>,
}

impl PolarLoader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, PolarError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PolarError> {
        let mut r = bytes;

        if read_u32(&mut r)? != MAGIC {
            return Err(PolarError::BadMagic);
        }
        let version = read_u16(&mut r)?;
        if version == 0 || version > LATEST_VERSION {
            return Err(PolarError::UnsupportedVersion(version));
        }

        if version >= VERSION_DATA_CONVERTER {
            let _data_version = read_varint(&mut r)?;
        }

        let compression = read_u8(&mut r)?;
        let data_len = read_varint(&mut r)? as usize;

        let decompressed;
        let mut body: &[u8] = match compression {
            0 => {
                ensure(r, data_len)?;
                &r[..data_len]
            }
            1 => {
                decompressed = zstd::stream::decode_all(r).map_err(|_| PolarError::Decompress)?;
                &decompressed
            }
            other => return Err(PolarError::UnknownCompression(other)),
        };

        let biomes = crate::registry::load(
            "minecraft:worldgen/biome".into(),
            include_str!("../../../data/worldgen/biome.json"),
        );

        let min_section = read_i8(&mut body)?;
        let max_section = read_i8(&mut body)?;
        if max_section < min_section {
            return Err(PolarError::Malformed);
        }
        let section_count = (max_section as i32 - min_section as i32 + 1) as usize;

        let user_data = if version >= VERSION_WORLD_USERDATA {
            read_byte_array(&mut body)?
        } else {
            Vec::new()
        };

        let chunk_count = read_varint(&mut body)?;
        if chunk_count < 0 {
            return Err(PolarError::Malformed);
        }

        let mut chunks = HashMap::with_capacity(chunk_count as usize);
        let mut chunk_user_data = HashMap::new();
        for _ in 0..chunk_count {
            let (chunk, chunk_data) =
                read_chunk(&mut body, version, min_section, section_count, &biomes)?;
            let key = (chunk.x(), chunk.z());
            if !chunk_data.is_empty() {
                chunk_user_data.insert(key, chunk_data);
            }
            chunks.insert(key, chunk);
        }

        Ok(Self {
            min_section,
            max_section,
            user_data,
            chunks,
            chunk_user_data,
            on_load: None,
            on_save: None,
        })
    }

    pub fn on_load(&mut self, hook: impl FnMut(&Chunk, &[u8]) + 'static) -> &mut Self {
        self.on_load = Some(Box::new(hook));
        self
    }

    pub fn on_save(&mut self, hook: impl FnMut(&Chunk, &mut Vec<u8>) + 'static) -> &mut Self {
        self.on_save = Some(Box::new(hook));
        self
    }

    pub fn save_chunk_data(&mut self, chunk: &Chunk) -> Vec<u8> {
        let mut buf = Vec::new();
        if let Some(hook) = &mut self.on_save {
            hook(chunk, &mut buf);
        }
        buf
    }

    pub fn load_chunk(&mut self, cx: i32, cz: i32) -> Option<Chunk> {
        let chunk = self.chunks.remove(&(cx, cz))?;
        if let Some(hook) = &mut self.on_load {
            let user_data = self
                .chunk_user_data
                .get(&(cx, cz))
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            hook(&chunk, user_data);
        }
        Some(chunk)
    }

    pub fn min_section(&self) -> i8 {
        self.min_section
    }

    pub fn max_section(&self) -> i8 {
        self.max_section
    }

    pub fn user_data(&self) -> &[u8] {
        &self.user_data
    }
}

fn read_chunk(
    body: &mut &[u8],
    version: u16,
    min_section: i8,
    section_count: usize,
    biomes: &DynamicRegistry<Biome>,
) -> Result<(Chunk, Vec<u8>), PolarError> {
    let cx = read_varint(body)?;
    let cz = read_varint(body)?;
    let mut chunk = Chunk::new(cx, cz, min_section as i32 * 16);

    for section in 0..section_count {
        read_section(body, version, section as u32, &mut chunk, biomes)?;
    }

    let block_entity_count = read_varint(body)?;
    if block_entity_count < 0 {
        return Err(PolarError::Malformed);
    }
    for _ in 0..block_entity_count {
        read_block_entity(body, version, &mut chunk)?;
    }

    let mask = read_u32(body)?;
    for _ in 0..mask.count_ones() {
        skip_long_array(body)?;
    }

    let user_data = if version > VERSION_USERDATA_OPT_BLOCK_ENT_NBT {
        read_byte_array(body)?
    } else {
        Vec::new()
    };

    Ok((chunk, user_data))
}

fn read_section(
    body: &mut &[u8],
    version: u16,
    section: u32,
    chunk: &mut Chunk,
    biomes: &DynamicRegistry<Biome>,
) -> Result<(), PolarError> {
    if read_bool(body)? {
        // Empty section: nothing else encoded, leave it as air.
        return Ok(());
    }

    let in_range = (section as usize) < chunk.sections().len();

    // blocks
    let palette_len = read_varint(body)? as usize;
    if !(1..=BLOCKS_PER_SECTION).contains(&palette_len) {
        return Err(PolarError::Malformed);
    }
    let mut blocks = Vec::with_capacity(palette_len);
    for _ in 0..palette_len {
        blocks.push(parse_block(&read_string(body)?)?);
    }

    if palette_len == 1 {
        if in_range {
            chunk.fill_block_state_section(section, blocks[0].id() as i32);
        }
    } else {
        let data = read_long_array(body)?;
        unpack(&data, BLOCKS_PER_SECTION, palette_len, |cell, idx| {
            if in_range {
                let x = cell % 16;
                let z = cell / 16 % 16;
                let y = cell / (16 * 16);
                chunk.set_block(
                    x as i32,
                    chunk.min_y() + (section * 16) as i32 + y as i32,
                    z as i32,
                    &blocks[idx],
                );
            }
        })?;
    }

    // biomes
    let palette_len = read_varint(body)? as usize;
    if palette_len < 1 {
        return Err(PolarError::Malformed);
    }
    let mut ids = Vec::with_capacity(palette_len);
    for _ in 0..palette_len {
        let name = read_string(body)?;
        let id = biomes
            .get_id(&RegistryKey::new(name))
            .ok_or(PolarError::Malformed)? as i32;
        ids.push(id);
    }

    if palette_len == 1 {
        if in_range {
            chunk.fill_biome_section(section, ids[0]);
        }
    } else {
        let data = read_long_array(body)?;
        unpack(&data, BIOMES_PER_SECTION, palette_len, |cell, idx| {
            if in_range {
                let x = cell % 4;
                let z = cell / 4 % 4;
                let y = cell / (4 * 4);
                chunk.set_biome(
                    (x * 4) as i32,
                    (section as usize * 16 + y * 4) as i32,
                    (z * 4) as i32,
                    ids[idx],
                );
            }
        })?;
    }

    // light
    if version >= VERSION_IMPROVED_LIGHT {
        if read_u8(body)? == LIGHT_PRESENT {
            skip(body, LIGHT_BYTES)?;
        }
        if read_u8(body)? == LIGHT_PRESENT {
            skip(body, LIGHT_BYTES)?;
        }
    } else if version > VERSION_UNIFIED_LIGHT {
        if read_bool(body)? {
            skip(body, LIGHT_BYTES)?;
        }
        if read_bool(body)? {
            skip(body, LIGHT_BYTES)?;
        }
    } else if read_bool(body)? {
        skip(body, LIGHT_BYTES * 2)?;
    }

    Ok(())
}

fn read_block_entity(body: &mut &[u8], version: u16, chunk: &mut Chunk) -> Result<(), PolarError> {
    let pos = read_u32(body)?;
    let x = (pos & 0xF) as usize;
    let z = ((pos >> 28) & 0xF) as usize;
    let mut y = ((pos & 0x07FF_FFF0) >> 4) as i32;
    if pos & 0x0800_0000 != 0 {
        y = -y;
    }

    if read_bool(body)? {
        let _ = read_string(body)?;
    }

    let has_nbt = if version <= VERSION_USERDATA_OPT_BLOCK_ENT_NBT {
        true
    } else {
        read_bool(body)?
    };
    if has_nbt {
        let data = read_nbt(body, version)?;
        chunk.set_block_entity_data(x, y, z, y, Nbt::new(String::new(), data));
    }
    Ok(())
}

fn parse_block(entry: &str) -> Result<BlockState, PolarError> {
    let (name, props) = match entry.split_once('[') {
        Some((name, rest)) => (name, Some(rest.trim_end_matches(']'))),
        None => (entry, None),
    };

    let mut state = Block::from_key(name)
        .ok_or(PolarError::Malformed)?
        .default_state();

    if let Some(props) = props.filter(|p| !p.is_empty()) {
        for pair in props.split(',') {
            let (key, value) = pair.split_once('=').ok_or(PolarError::Malformed)?;
            state = state
                .with_property(key.trim(), value.trim())
                .ok_or(PolarError::Malformed)?;
        }
    }

    Ok(state)
}

fn unpack(
    data: &[i64],
    count: usize,
    palette_len: usize,
    mut f: impl FnMut(usize, usize),
) -> Result<(), PolarError> {
    let bits = bit_width(palette_len - 1).max(1);
    let per_long = 64 / bits;
    if data.len() != count.div_ceil(per_long) {
        return Err(PolarError::Malformed);
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
                return Err(PolarError::Malformed);
            }
            f(cell, idx);
            cell += 1;
        }
    }
    Ok(())
}

fn bit_width(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()) as usize
}

fn ensure(body: &[u8], n: usize) -> Result<(), PolarError> {
    if body.len() < n {
        Err(PolarError::UnexpectedEof)
    } else {
        Ok(())
    }
}

fn read_u8(body: &mut &[u8]) -> Result<u8, PolarError> {
    ensure(body, 1)?;
    Ok(body.get_u8())
}

fn read_i8(body: &mut &[u8]) -> Result<i8, PolarError> {
    ensure(body, 1)?;
    Ok(body.get_i8())
}

fn read_bool(body: &mut &[u8]) -> Result<bool, PolarError> {
    Ok(read_u8(body)? != 0)
}

fn read_u16(body: &mut &[u8]) -> Result<u16, PolarError> {
    ensure(body, 2)?;
    Ok(body.get_u16())
}

fn read_u32(body: &mut &[u8]) -> Result<u32, PolarError> {
    ensure(body, 4)?;
    Ok(body.get_u32())
}

fn read_varint(body: &mut &[u8]) -> Result<i32, PolarError> {
    let mut value: i32 = 0;
    for shift in (0..35).step_by(7) {
        let byte = read_u8(body)?;
        value |= ((byte & 0x7F) as i32) << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(PolarError::VarIntTooLong)
}

fn read_string(body: &mut &[u8]) -> Result<String, PolarError> {
    let len = read_varint(body)? as usize;
    ensure(body, len)?;
    let (s, rest) = body.split_at(len);
    *body = rest;
    String::from_utf8(s.to_vec()).map_err(|_| PolarError::Malformed)
}

fn read_long_array(body: &mut &[u8]) -> Result<Vec<i64>, PolarError> {
    let len = read_varint(body)? as usize;
    ensure(body, len * 8)?;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(body.get_i64());
    }
    Ok(out)
}

fn skip(body: &mut &[u8], n: usize) -> Result<(), PolarError> {
    ensure(body, n)?;
    body.advance(n);
    Ok(())
}

fn read_byte_array(body: &mut &[u8]) -> Result<Vec<u8>, PolarError> {
    let len = read_varint(body)? as usize;
    ensure(body, len)?;
    let (bytes, rest) = body.split_at(len);
    *body = rest;
    Ok(bytes.to_vec())
}

fn skip_long_array(body: &mut &[u8]) -> Result<(), PolarError> {
    let len = read_varint(body)? as usize;
    skip(body, len * 8)
}

fn read_nbt(body: &mut &[u8], version: u16) -> Result<NbtCompound, PolarError> {
    if read_u8(body)? != cerium_nbt::COMPOUND_ID {
        return Err(PolarError::Malformed);
    }

    if version <= VERSION_MINESTOM_NBT_READ_BREAK {
        let name_len = read_u16(body)? as usize;
        skip(body, name_len)?;
    }
    NbtCompound::deserialize_content(body).map_err(|_| PolarError::Malformed)
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PolarError {
    #[error("an I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("not a Polar file (bad magic)")]
    BadMagic,
    #[error("unsupported Polar version: {0}")]
    UnsupportedVersion(u16),
    #[error("unknown compression scheme: {0}")]
    UnknownCompression(u8),
    #[error("failed to decompress world data")]
    Decompress,
    #[error("varint is too long")]
    VarIntTooLong,
    #[error("unexpected end of data")]
    UnexpectedEof,
    #[error("malformed Polar data")]
    Malformed,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_world() -> Vec<u8> {
        let mut body = Vec::new();
        body.push(0i8 as u8); // min_section
        body.push(0i8 as u8); // max_section
        push_byte_array(&mut body, b"world-meta"); // world user data (v>=4)
        push_varint(&mut body, 1); // chunk count

        // chunk (0, 0)
        push_varint(&mut body, 0); // chunk x
        push_varint(&mut body, 0); // chunk z

        // single section
        body.push(0); // not empty
        push_varint(&mut body, 1); // block palette size
        push_string(&mut body, "minecraft:stone");
        push_varint(&mut body, 1); // biome palette size
        push_string(&mut body, "minecraft:plains");
        body.push(0); // block light: absent
        body.push(0); // sky light: absent

        push_varint(&mut body, 0); // block entity count
        body.extend_from_slice(&0u32.to_be_bytes()); // heightmap mask
        push_byte_array(&mut body, b"chunk-meta"); // chunk user data (v>2)

        // Header (version 7, uncompressed).
        let mut w = Vec::new();
        w.extend_from_slice(b"Polr"); // magic
        w.extend_from_slice(&7u16.to_be_bytes()); // version
        push_varint(&mut w, 0); // data version (v>=6)
        w.push(0); // compression: none
        push_varint(&mut w, body.len() as i32); // payload length
        w.extend_from_slice(&body);

        w
    }

    fn push_varint(buf: &mut Vec<u8>, mut value: i32) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn push_string(buf: &mut Vec<u8>, s: &str) {
        push_varint(buf, s.len() as i32);
        buf.extend_from_slice(s.as_bytes());
    }

    fn push_byte_array(buf: &mut Vec<u8>, bytes: &[u8]) {
        push_varint(buf, bytes.len() as i32);
        buf.extend_from_slice(bytes);
    }

    #[test]
    fn loads_solid_stone_chunk() {
        let bytes = sample_world();
        let mut loader = PolarLoader::from_bytes(&bytes).expect("parse");
        assert_eq!(loader.min_section(), 0);
        assert_eq!(loader.max_section(), 0);

        assert_eq!(loader.user_data(), b"world-meta");

        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = seen.clone();
        loader.on_load(move |_chunk, user_data| sink.borrow_mut().push(user_data.to_vec()));

        let chunk = loader.load_chunk(0, 0).expect("chunk present");
        let stone = Block::from_key("minecraft:stone").unwrap().default_state();
        assert_eq!(chunk.get_block(0, 0, 0), stone.id());
        assert_eq!(chunk.get_block(15, 15, 15), stone.id());
        assert_eq!(seen.borrow().as_slice(), &[b"chunk-meta".to_vec()]);

        assert!(loader.load_chunk(0, 0).is_none());
    }

    #[test]
    fn rejects_bad_magic() {
        assert!(matches!(
            PolarLoader::from_bytes(b"nope...."),
            Err(PolarError::BadMagic)
        ));
    }
}
