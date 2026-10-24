use crate::world::{
    block::{Block, BlockState},
    palette::Palette,
};

pub const SECTION_SIZE: usize = 16;
pub const SECTION_VOLUME: usize = SECTION_SIZE * SECTION_SIZE * SECTION_SIZE;

const AIR_STATE_ID: u16 = Block::AIR.default_state().state_id();

fn state_from_id(state_id: u16) -> Option<BlockState> {
    BlockState::from_id(state_id)
}

#[inline]
const fn block_index(x: usize, y: usize, z: usize) -> usize {
    (y * SECTION_SIZE + z) * SECTION_SIZE + x
}

/// A 16x16x16 slice of a [`crate::world::chunk::Chunk`].
///
/// The block states of an all-air section are not stored at all, so a chunk of
/// mostly empty sections costs almost nothing.
#[derive(Debug, Clone)]
pub struct ChunkSection {
    block_states: Option<Palette>,
    biomes: Palette,
    non_empty_block_count: u16,
    fluid_count: u16,
    counts_stale: bool,
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            block_states: None,
            biomes: Palette::biomes(),
            non_empty_block_count: 0,
            fluid_count: 0,
            counts_stale: false,
        }
    }

    pub fn block_states(&self) -> Option<&Palette> {
        self.block_states.as_ref()
    }

    pub fn set_block_states(&mut self, block_states: Palette) {
        self.block_states = Some(block_states);
        self.counts_stale = true;
    }

    pub fn biomes(&self) -> &Palette {
        &self.biomes
    }

    pub fn biomes_mut(&mut self) -> &mut Palette {
        &mut self.biomes
    }

    pub fn set_biomes(&mut self, biomes: Palette) {
        self.biomes = biomes;
    }

    pub fn block_states_mut(&mut self) -> &mut [u16] {
        self.counts_stale = true;
        self.block_states
            .get_or_insert_with(Palette::blocks)
            .data_mut()
    }

    // Block

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u16 {
        match &self.block_states {
            None => AIR_STATE_ID,
            Some(block_states) => block_states.get(x, y, z),
        }
    }

    #[inline]
    pub fn get_block_at_index(&self, index: usize) -> u16 {
        match &self.block_states {
            None => AIR_STATE_ID,
            Some(block_states) => block_states.get_index(index),
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: u16) -> u16 {
        if self.counts_stale {
            self.recalculate_counts();
        }

        let old_block = self.get_block(x, y, z);
        if old_block == block {
            return old_block;
        }

        self.block_states
            .get_or_insert_with(Palette::blocks)
            .set_index(block_index(x, y, z), block);

        let was_air = is_air(old_block);
        let is_air = is_air(block);
        if was_air != is_air {
            self.non_empty_block_count = if is_air {
                self.non_empty_block_count - 1
            } else {
                self.non_empty_block_count + 1
            };
        }

        let had_fluid = has_fluid(old_block);
        let has_fluid = has_fluid(block);
        if had_fluid != has_fluid {
            self.fluid_count = if has_fluid {
                self.fluid_count + 1
            } else {
                self.fluid_count - 1
            };
        }

        old_block
    }

    pub fn distinct_blocks(&self) -> impl Iterator<Item = u16> + '_ {
        self.block_states
            .as_ref()
            .map(|block_states| block_states.distinct())
            .into_iter()
            .flatten()
            .chain(self.block_states.is_none().then_some(AIR_STATE_ID))
    }

    pub fn has_only_air(&self) -> bool {
        match &self.block_states {
            None => true,
            Some(block_states) => block_states.uniform().is_some_and(is_air),
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.non_empty_block_count == 0
    }

    pub const fn non_empty_block_count(&self) -> u16 {
        self.non_empty_block_count
    }

    pub const fn fluid_count(&self) -> u16 {
        self.fluid_count
    }

    pub const fn has_fluid(&self) -> bool {
        self.fluid_count > 0
    }

    pub const fn counts_are_stale(&self) -> bool {
        self.counts_stale
    }

    pub fn recalculate_counts(&mut self) {
        self.counts_stale = false;

        let Some(block_states) = self.block_states.as_mut() else {
            self.non_empty_block_count = 0;
            self.fluid_count = 0;
            return;
        };

        block_states.recount();

        if block_states.uniform() == Some(AIR_STATE_ID) {
            self.block_states = None;
            self.non_empty_block_count = 0;
            self.fluid_count = 0;
            return;
        }

        let mut non_empty: u16 = 0;
        let mut fluid: u16 = 0;
        for (state_id, count) in block_states.counts() {
            let count = count as u16;
            if !is_air(state_id) {
                non_empty += count;
            }
            if has_fluid(state_id) {
                fluid += count;
            }
        }

        self.non_empty_block_count = non_empty;
        self.fluid_count = fluid;
    }

    // Biome

    #[inline]
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: u16) {
        self.biomes.set(x, y, z, biome);
    }

    #[inline]
    pub fn get_biome(&self, x: usize, y: usize, z: usize) -> u16 {
        self.biomes.get(x, y, z)
    }
}

fn is_air(state_id: u16) -> bool {
    state_id == AIR_STATE_ID || state_from_id(state_id).is_some_and(|state| state.is_air())
}

fn has_fluid(state_id: u16) -> bool {
    state_from_id(state_id).is_some_and(|state| state.has_fluid())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stone() -> u16 {
        Block::STONE.default_state().state_id()
    }

    fn water() -> u16 {
        Block::WATER.default_state().state_id()
    }

    #[test]
    fn a_new_section_stores_no_block_states() {
        let section = ChunkSection::new();

        assert!(section.block_states().is_none());
        assert!(section.has_only_air());
        assert!(section.is_empty());
        assert_eq!(section.get_block(0, 0, 0), AIR_STATE_ID);
    }

    #[test]
    fn set_block_keeps_the_counts_up_to_date() {
        crate::registry::registries::register_all();
        let mut section = ChunkSection::new();

        assert_eq!(section.set_block(1, 2, 3, stone()), AIR_STATE_ID);
        assert_eq!(section.non_empty_block_count(), 1);
        assert_eq!(section.fluid_count(), 0);

        assert_eq!(section.set_block(1, 2, 3, water()), stone());
        assert_eq!(section.non_empty_block_count(), 1);
        assert_eq!(section.fluid_count(), 1);

        assert_eq!(section.set_block(1, 2, 3, AIR_STATE_ID), water());
        assert_eq!(section.non_empty_block_count(), 0);
        assert_eq!(section.fluid_count(), 0);
    }

    #[test]
    fn recalculate_counts_reads_the_palette_after_a_bulk_write() {
        crate::registry::registries::register_all();
        let mut section = ChunkSection::new();

        let entries = section.block_states_mut();
        entries[block_index(0, 0, 0)] = stone();
        entries[block_index(1, 0, 0)] = water();
        entries[block_index(2, 0, 0)] = water();
        assert!(section.counts_are_stale());

        section.recalculate_counts();

        assert!(!section.counts_are_stale());
        assert_eq!(section.non_empty_block_count(), 3);
        assert_eq!(section.fluid_count(), 2);
    }

    #[test]
    fn a_section_bulk_written_back_to_air_releases_its_palette() {
        let mut section = ChunkSection::new();

        section.block_states_mut()[0] = stone();
        assert!(section.block_states().is_some());

        section.block_states_mut()[0] = AIR_STATE_ID;
        section.recalculate_counts();

        assert!(section.block_states().is_none());
        assert!(section.has_only_air());
    }

    #[test]
    fn a_counter_aware_write_recounts_a_stale_section_first() {
        crate::registry::registries::register_all();
        let mut section = ChunkSection::new();

        section.block_states_mut()[block_index(0, 0, 0)] = stone();
        assert_eq!(section.non_empty_block_count(), 0);

        assert_eq!(section.set_block(0, 0, 0, water()), stone());
        assert_eq!(section.non_empty_block_count(), 1);
        assert_eq!(section.fluid_count(), 1);
    }

    #[test]
    fn blocks_use_y_z_x_linear_order() {
        let mut section = ChunkSection::new();
        section.set_block(3, 4, 5, stone());

        assert_eq!(
            section.get_block_at_index(3 + 5 * 16 + 4 * 16 * 16),
            stone()
        );
    }
}
