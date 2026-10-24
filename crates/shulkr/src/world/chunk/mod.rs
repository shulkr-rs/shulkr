mod async_dedup;
mod chunk_tracker;
mod graph;
mod section;
mod ticket;
mod ticket_storage;
mod view_tracker;

pub use async_dedup::AsyncDedup;
pub use chunk_tracker::{ChunkGraph, ChunkGraphSource};
pub use graph::{DynamicGraph, GraphState};
pub use section::*;
pub use ticket::{Ticket, TicketType};
pub use ticket_storage::TicketStorage;
pub use view_tracker::ViewTracker;

use crate::{
    registry::{Id, Ref, registries},
    util::{BlockPosition, HashMap, RwLock},
    world::{
        biome::Biome,
        block::{BlockEntity, BlockState},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Chunk(Arc<RwLock<imp::Chunk>>);

impl Chunk {
    pub(crate) fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
        Self(Arc::new(RwLock::new(imp::Chunk::new(
            chunk_x, chunk_z, min_y,
        ))))
    }

    pub fn x(&self) -> i32 {
        self.0.read().chunk_x
    }

    pub fn z(&self) -> i32 {
        self.0.read().chunk_z
    }

    pub fn min_y(&self) -> i32 {
        self.0.read().min_y
    }

    pub fn sections(&self) -> Vec<ChunkSection> {
        self.0.read().sections.clone()
    }

    pub fn block_entites(&self) -> Vec<BlockEntity> {
        self.0
            .read()
            .block_entities
            .values()
            .cloned()
            .collect::<Vec<_>>()
    }

    /// Returns the [BlockState] id at `position`.
    ///
    /// Panics if `position` is outside this chunk's height range.
    pub fn get_block(&self, position: impl Into<BlockPosition>) -> u16 {
        let [x, y, z] = Into::<BlockPosition>::into(position).into();

        let chunk = self.0.read();
        let section = chunk.expect_section(y);

        section.get_block(to_relative(x), to_relative(y), to_relative(z))
    }

    /// Changes the [BlockState] at `position` to `block`.
    ///
    /// Panics if `position` is outside this chunk's height range.
    pub fn set_block(&self, position: impl Into<BlockPosition>, block: &BlockState) {
        let [x, y, z] = Into::<BlockPosition>::into(position).into();

        let mut chunk = self.0.write();

        let section = chunk.expect_section_mut(y);
        section.set_block(
            to_relative(x),
            to_relative(y),
            to_relative(z),
            block.state_id(),
        );

        let packed_xz = pack_xz(x, z);
        let key = (packed_xz, y as i16);
        if let Some(info) = block.block_entity() {
            let block_entity = BlockEntity {
                packed_xz,
                y: y as i16,
                r#type: Id::from(*info) as i32,
                data: None,
            };

            chunk.block_entities.insert(key, block_entity);
        } else {
            chunk.block_entities.remove(&key);
        }
    }

    /// Returns the [Biome] at `position`.
    ///
    /// Panics if `position` is outside this chunk's height range.
    pub fn get_biome(&self, position: impl Into<BlockPosition>) -> &'static Ref<Biome> {
        let [x, y, z] = Into::<BlockPosition>::into(position).into();

        let chunk = self.0.read();
        let section = chunk.expect_section(y);

        let id = section.get_biome(to_relative(x) / 4, to_relative(y) / 4, to_relative(z) / 4);
        registries::BIOME
            .by_id(id)
            .unwrap_or_else(|| panic!("Unknown biome id: {}", id))
    }

    /// Changes the [Biome] at `position` to `biome`.
    ///
    /// Panics if `position` is outside this chunk's height range.
    pub fn set_biome(&self, position: impl Into<BlockPosition>, biome: &Ref<Biome>) {
        let [x, y, z] = Into::<BlockPosition>::into(position).into();

        let mut chunk = self.0.write();
        let section = chunk.expect_section_mut(y);

        section.set_biome(
            to_relative(x) / 4,
            to_relative(y) / 4,
            to_relative(z) / 4,
            biome.id(),
        );
    }

    pub fn fill_blocks(&self, blocks: impl IntoIterator<Item = (i32, i32, i32, BlockState)>) {
        for (x, y, z, state) in blocks {
            self.set_block([x, y, z], &state);
        }
    }

    pub fn fill_biomes<'a>(
        &self,
        biomes: impl IntoIterator<Item = (i32, i32, i32, &'a Ref<Biome>)>,
    ) {
        for (x, y, z, biome) in biomes {
            self.set_biome([x, y, z], biome);
        }
    }

    pub fn to_chunk_pos(position: impl Into<BlockPosition>) -> (i32, i32) {
        let position = position.into();
        (position.x().div_euclid(16), position.z().div_euclid(16))
    }

    pub(crate) fn in_view(dx: i32, dz: i32, view_distance: i32) -> bool {
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

    #[cfg(any(feature = "anvil", feature = "polar"))]
    pub(crate) fn fill_block_state_section(&mut self, sect_y: u32, block: u16) {
        use crate::world::palette::Palette;

        let mut palette = Palette::blocks();
        palette.fill(block);
        self.0.write().sections[sect_y as usize].set_block_states(palette);
    }

    #[cfg(any(feature = "anvil", feature = "polar"))]
    pub(crate) fn fill_biome_section(&mut self, sect_y: u32, biome: u16) {
        use crate::world::palette::Palette;

        let mut palette = Palette::biomes();
        palette.fill(biome);
        self.0.write().sections[sect_y as usize].set_biomes(palette);
    }

    pub fn set_block_entity_data(
        &mut self,
        x: usize,
        _local_y: i32,
        z: usize,
        world_y: i32,
        data: shulkr_nbt::Nbt,
    ) -> bool {
        let key = (pack_xz(x as i32, z as i32), world_y as i16);
        if let Some(block_entity) = self.0.write().block_entities.get_mut(&key) {
            block_entity.data = Some(data);
            true
        } else {
            false
        }
    }
}

#[inline]
fn to_relative(value: i32) -> usize {
    (value & 0x0F) as usize
}

fn pack_xz(world_x: i32, world_z: i32) -> u8 {
    let block_x = world_x & 0xF;
    let block_z = world_z & 0xF;
    ((block_x << 4) | block_z) as u8
}

mod imp {
    use super::*;

    pub(super) struct Chunk {
        pub(super) chunk_x: i32,
        pub(super) chunk_z: i32,
        pub(super) min_y: i32,
        pub(super) sections: Vec<ChunkSection>,
        pub(super) block_entities: HashMap<(u8, i16), BlockEntity>,
    }

    impl Chunk {
        pub(super) fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
            let mut sections = vec![];
            for _ in 0..24 {
                sections.push(ChunkSection::new());
            }

            Self {
                chunk_x,
                chunk_z,
                min_y,
                sections,
                block_entities: HashMap::default(),
            }
        }

        // Returns the section containing `y`, panicking if it is out of bounds.
        #[inline]
        pub(super) fn expect_section(&self, y: i32) -> &ChunkSection {
            self.section_index(y)
                .and_then(|index| self.sections.get(index))
                .unwrap_or_else(|| panic!("Chunk section out of bounds for y: {}", y))
        }

        // Returns the section containing `y` mutably, panicking if it is out of bounds.
        #[inline]
        pub(super) fn expect_section_mut(&mut self, y: i32) -> &mut ChunkSection {
            self.section_index(y)
                .and_then(|index| self.sections.get_mut(index))
                .unwrap_or_else(|| panic!("Chunk section out of bounds for y: {}", y))
        }

        // Returns the index of the section containing `y`, or `None` if `y` is below `min_y`.
        #[inline]
        fn section_index(&self, y: i32) -> Option<usize> {
            usize::try_from((y - self.min_y).div_euclid(16)).ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chunk;
    use crate::world::block::Block;

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

    #[test]
    fn stacked_block_entities_are_kept_apart() {
        crate::registry::registries::register_all();
        let chunk = Chunk::new(0, 0, -64);
        let chest = Block::from_key("minecraft:chest").unwrap().default_state();
        chunk.set_block([1, 10, 1], &chest);
        chunk.set_block([1, 11, 1], &chest);

        let mut ys = chunk
            .block_entites()
            .iter()
            .map(|e| e.y)
            .collect::<Vec<_>>();
        ys.sort();
        assert_eq!(ys, [10, 11]);
    }

    #[test]
    fn replacing_a_block_entity_removes_it() {
        crate::registry::registries::register_all();
        let chunk = Chunk::new(0, 0, -64);
        let chest = Block::from_key("minecraft:chest").unwrap().default_state();
        let stone = Block::from_key("minecraft:stone").unwrap().default_state();
        chunk.set_block([1, 10, 1], &chest);
        chunk.set_block([1, 10, 1], &stone);

        assert!(chunk.block_entites().is_empty());
    }

    #[test]
    #[should_panic(expected = "Chunk section out of bounds")]
    fn a_position_just_below_min_y_is_out_of_bounds() {
        crate::registry::registries::register_all();
        let chunk = Chunk::new(0, 0, -64);
        chunk.get_block([0, -65, 0]);
    }

    #[test]
    fn an_out_of_bounds_set_block_leaves_block_entities_untouched() {
        crate::registry::registries::register_all();
        let chunk = Chunk::new(0, 0, -64);
        let chest = Block::from_key("minecraft:chest").unwrap().default_state();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            chunk.set_block([0, 320, 0], &chest)
        }));

        assert!(result.is_err());
        assert!(chunk.block_entites().is_empty());
    }
}
