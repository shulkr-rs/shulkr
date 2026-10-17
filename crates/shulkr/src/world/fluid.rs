use crate::{
    registry::{Id, registries},
    util::Key,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fluid(Id);

include!("../../generated/fluids.rs");

#[derive(Clone)]
pub struct FluidData {
    pub default_state: u16,
}

impl FluidData {
    pub const fn new(default_state: u16) -> Self {
        Self { default_state }
    }
}

impl Fluid {
    pub fn key(&self) -> &'static Key {
        self.try_key().expect("fluid is not registered")
    }

    pub fn try_key(&self) -> Option<&'static Key> {
        registries::FLUID.key_of(self.0)
    }

    pub fn has_tag(&self, tag: &Key) -> bool {
        let registry = &registries::FLUID;
        registry
            .key_of(self.0)
            .is_some_and(|key| registry.has_tag(key, tag))
    }

    pub const fn default_state(&self) -> FluidState {
        FLUID_STATES[self.data().default_state as usize]
    }

    pub const fn is_empty(&self) -> bool {
        self.default_state().is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidState {
    fluid: Fluid,
    amount: u8,
    flags: u8,
}

impl FluidState {
    pub const IS_EMPTY: u8 = 1 << 0;
    pub const IS_SOURCE: u8 = 1 << 1;
    pub const IS_FULL: u8 = 1 << 2;
    pub const IS_RANDOMLY_TICKING: u8 = 1 << 3;
    pub const FALLING: u8 = 1 << 4;

    pub const fn new(fluid: Fluid, amount: u8, flags: u8) -> Self {
        Self {
            fluid,
            amount,
            flags,
        }
    }

    pub const EMPTY: Self = FLUID_STATES[0];

    pub const fn fluid(&self) -> Fluid {
        self.fluid
    }

    pub const fn amount(&self) -> u8 {
        self.amount
    }

    pub const fn is_empty(&self) -> bool {
        self.flags & Self::IS_EMPTY != 0
    }

    pub const fn is_source(&self) -> bool {
        self.flags & Self::IS_SOURCE != 0
    }

    pub const fn is_full(&self) -> bool {
        self.flags & Self::IS_FULL != 0
    }

    pub const fn is_randomly_ticking(&self) -> bool {
        self.flags & Self::IS_RANDOMLY_TICKING != 0
    }

    pub const fn falling(&self) -> bool {
        self.flags & Self::FALLING != 0
    }

    pub fn own_height(&self) -> f32 {
        if self.is_empty() {
            0.0
        } else {
            f32::from(self.amount) / 9.0
        }
    }

    pub fn is_water(&self) -> bool {
        self.fluid == Fluid::WATER || self.fluid == Fluid::FLOWING_WATER
    }

    pub fn is_lava(&self) -> bool {
        self.fluid == Fluid::LAVA || self.fluid == Fluid::FLOWING_LAVA
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::block::{Block, BlockState, Properties};

    #[test]
    fn a_block_without_a_fluid_is_empty() {
        let state = Block::STONE.default_state();

        assert_eq!(state.get_fluid_state(), FluidState::EMPTY);
        assert!(!state.has_fluid());
        assert!(state.get_fluid_state().is_empty());
    }

    #[test]
    fn a_water_source_block_is_a_full_source() {
        let fluid = Block::WATER.default_state().get_fluid_state();

        assert_eq!(fluid.fluid(), Fluid::WATER);
        assert!(fluid.is_source());
        assert!(fluid.is_full());
        assert!(!fluid.is_empty());
        assert_eq!(fluid.amount(), 8);
        assert!(fluid.is_water());
        assert!(!fluid.is_lava());
    }

    #[test]
    fn water_levels_map_to_flowing_amounts() {
        let level_1 = Block::WATER
            .default_state()
            .with(Properties::LEVEL, 1)
            .get_fluid_state();

        assert_eq!(level_1.fluid(), Fluid::FLOWING_WATER);
        assert!(!level_1.is_source());
        assert_eq!(level_1.amount(), 7);
    }

    #[test]
    fn a_waterlogged_block_holds_a_water_source() {
        let dry = Block::OAK_SLAB
            .default_state()
            .with(Properties::WATERLOGGED, false);
        let wet = Block::OAK_SLAB
            .default_state()
            .with(Properties::WATERLOGGED, true);

        assert!(!dry.has_fluid());
        assert!(wet.has_fluid());

        let fluid = wet.get_fluid_state();
        assert_eq!(fluid.fluid(), Fluid::WATER);
        assert!(fluid.is_source());
        assert!(fluid.is_water());
    }

    #[test]
    fn plants_that_live_in_water_hold_a_water_source() {
        for block in [Block::KELP_PLANT, Block::SEAGRASS, Block::BUBBLE_COLUMN] {
            let fluid = block.default_state().get_fluid_state();

            assert!(fluid.is_water(), "{:?} should hold water", block.key());
            assert!(fluid.is_source());
        }
    }

    #[test]
    fn lava_is_randomly_ticking_and_not_water() {
        let fluid = Block::LAVA.default_state().get_fluid_state();

        assert_eq!(fluid.fluid(), Fluid::LAVA);
        assert!(fluid.is_lava());
        assert!(!fluid.is_water());
        assert!(fluid.is_randomly_ticking());
    }

    #[test]
    fn fluid_keys_come_from_the_registry() {
        crate::registry::registries::register_all();
        assert_eq!(Fluid::WATER.key().to_string(), "minecraft:water");
        assert_eq!(
            Fluid::FLOWING_LAVA.key().to_string(),
            "minecraft:flowing_lava"
        );
        assert!(Fluid::EMPTY.is_empty());
    }

    #[test]
    fn own_height_follows_the_amount() {
        assert_eq!(FluidState::EMPTY.own_height(), 0.0);
        assert_eq!(
            Block::WATER.default_state().get_fluid_state().own_height(),
            8.0 / 9.0
        );
    }

    #[test]
    fn every_block_state_resolves_a_fluid_state() {
        for state in BlockState::all() {
            let fluid = state.get_fluid_state();
            assert_eq!(fluid.is_empty(), !state.has_fluid());
        }
    }
}
