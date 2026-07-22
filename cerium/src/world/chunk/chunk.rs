use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::util::BlockPosition;

use crate::world::{
    block::{BlockEntity, BlockState},
    chunk::ChunkSection,
};

#[derive(Clone)]
pub struct Chunk(Arc<RwLock<Inner>>);

impl Chunk {
    pub(crate) fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
        Self(Arc::new(RwLock::new(Inner::new(chunk_x, chunk_z, min_y))))
    }

    pub fn x(&self) -> i32 {
        self.0.read().x()
    }

    pub fn z(&self) -> i32 {
        self.0.read().z()
    }

    pub fn sections(&self) -> Vec<ChunkSection> {
        self.0.read().sections().clone()
    }

    pub fn block_entites(&self) -> Vec<BlockEntity> {
        self.0
            .read()
            .block_entities
            .values()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        self.0.read().get_block(x, y, z)
    }

    pub fn set_block(&self, x: i32, y: i32, z: i32, block: &BlockState) {
        self.0.write().set_block(x, y, z, block)
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        self.0.read().get_biome(x, y, z)
    }

    pub fn set_biome(&self, x: i32, y: i32, z: i32, biome: i32) {
        self.0.write().set_biome(x, y, z, biome)
    }

    pub fn to_chunk_pos(position: impl Into<BlockPosition>) -> (i32, i32) {
        let position = position.into();
        let chunk_x = position.x().div_euclid(16) as i32;
        let chunk_z = position.z().div_euclid(16) as i32;
        (chunk_x, chunk_z)
    }

    // Port of `ChunkTrackingView.Positioned.contains`
    fn in_view(dx: i32, dz: i32, view_distance: i32) -> bool {
        const BUFFER_RANGE: i32 = 2;
        let dx = (dx.abs() - BUFFER_RANGE).max(0);
        let dz = (dz.abs() - BUFFER_RANGE).max(0);
        dx * dx + dz * dz < view_distance * view_distance
    }

    pub fn chunks_in_range(chunk: (i32, i32), view_distance: i32) -> Vec<(i32, i32)> {
        let (cx, cz) = chunk;
        let bound = view_distance + 2;

        let mut chunks = Vec::new();
        for dz in -bound..=bound {
            for dx in -bound..=bound {
                if Self::in_view(dx, dz, view_distance) {
                    chunks.push((cx + dx, cz + dz));
                }
            }
        }
        chunks
    }

    /// Calulates difference between chunks
    pub fn difference<F>(lhs: (i32, i32), rhs: (i32, i32), view_distance: i32, mut callback: F)
    where
        F: FnMut(i32, i32),
    {
        let bound = view_distance + 2;

        for dz in -bound..=bound {
            for dx in -bound..=bound {
                if !Self::in_view(dx, dz, view_distance) {
                    continue;
                }
                let x = lhs.0 + dx;
                let z = lhs.1 + dz;
                if !Self::in_view(x - rhs.0, z - rhs.1, view_distance) {
                    callback(x, z);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chunk;

    #[test]
    fn in_view_matches_vanilla_shape() {
        assert!(Chunk::in_view(0, 0, 8));
        assert!(Chunk::in_view(2, 0, 8)); // inside bufferRange, always included
        assert!(Chunk::in_view(9, 0, 8)); // dx=7 after buffer, 49 < 64
        assert!(!Chunk::in_view(10, 0, 8)); // dx=8, 64 >= 64
        assert!(!Chunk::in_view(8, 8, 8)); // corner cut by circular falloff: dx=dz=6, 72 >= 64
    }

    #[test]
    fn chunks_in_range_matches_in_view() {
        let view_distance = 4;
        let chunks = Chunk::chunks_in_range((0, 0), view_distance);
        for &(x, z) in &chunks {
            assert!(Chunk::in_view(x, z, view_distance));
        }
        let count = chunks.len();
        let bound = view_distance + 2;
        let expected = (-bound..=bound)
            .flat_map(|dz| (-bound..=bound).map(move |dx| (dx, dz)))
            .filter(|&(dx, dz)| Chunk::in_view(dx, dz, view_distance))
            .count();
        assert_eq!(count, expected);
    }
}

struct Inner {
    chunk_x: i32,
    chunk_z: i32,
    min_y: i32,
    sections: Vec<ChunkSection>,
    block_entities: HashMap<u8, BlockEntity>,
}

impl Inner {
    fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
        let mut sections = vec![];
        for _ in 0..24 {
            sections.push(ChunkSection::new());
        }

        Self {
            chunk_x,
            chunk_z,
            min_y,
            sections,
            block_entities: HashMap::new(),
        }
    }

    fn x(&self) -> i32 {
        self.chunk_x
    }

    fn z(&self) -> i32 {
        self.chunk_z
    }

    fn sections(&self) -> &Vec<ChunkSection> {
        &self.sections
    }

    // pub fn block_entites(&self) -> Vec<&BlockEntity> {
    //     self.block_entities.values().collect::<Vec<_>>()
    // }

    fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        let Some(section) = self.section_at(y) else {
            panic!("Chunk section out of bounds for y: {}", y);
        };

        section.get_block(
            Self::to_relative(x),
            Self::to_relative(y),
            Self::to_relative(z),
        )
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, state: &BlockState) {
        if let Some(info) = state.block_entity() {
            let packed_xz = Self::pack_xz(x, z);
            let block_entity = BlockEntity {
                packed_xz,
                y: y as i16,
                r#type: info.id,
                data: None,
            };

            self.block_entities.insert(packed_xz, block_entity);
        }

        let Some(section) = self.section_at_mut(y) else {
            panic!("Chunk section out of bounds for y: {}", y);
        };

        section.set_block(
            Self::to_relative(x),
            Self::to_relative(y),
            Self::to_relative(z),
            state.id(),
        );
    }

    fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let Some(section) = self.section_at(y) else {
            panic!("Chunk section out of bounds for y: {}", y);
        };

        section.get_biome(
            Self::to_relative(x) / 4,
            Self::to_relative(y) / 4,
            Self::to_relative(z) / 4,
        )
    }

    fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: i32) {
        let Some(section) = self.section_at_mut(y) else {
            panic!("Chunk section out of bounds for y: {}", y);
        };

        section.set_biome(
            Self::to_relative(x) / 4,
            Self::to_relative(y) / 4,
            Self::to_relative(z) / 4,
            biome,
        );
    }

    #[inline]
    fn to_relative(value: i32) -> usize {
        (value & 0x0F) as usize
    }

    #[inline]
    fn section_at(&self, y: i32) -> Option<&ChunkSection> {
        self.sections.get(((y - self.min_y) / 16) as usize)
    }

    #[inline]
    fn section_at_mut(&mut self, y: i32) -> Option<&mut ChunkSection> {
        self.sections.get_mut(((y - self.min_y) / 16) as usize)
    }

    fn pack_xz(world_x: i32, world_z: i32) -> u8 {
        let block_x = world_x & 0xF;
        let block_z = world_z & 0xF;
        ((block_x << 4) | block_z) as u8
    }
}
