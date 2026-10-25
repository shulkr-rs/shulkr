// This file was auto-generated. Do not edit it manually.

#[rustfmt::skip]
pub static FLUID_STATES: [FluidState; 33usize] = [FluidState::new(Fluid::EMPTY, 0, FluidState::IS_EMPTY), FluidState::new(Fluid::WATER, 1, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 1, 0), FluidState::new(Fluid::WATER, 2, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 2, 0), FluidState::new(Fluid::WATER, 3, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 3, 0), FluidState::new(Fluid::WATER, 4, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 4, 0), FluidState::new(Fluid::WATER, 5, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 5, 0), FluidState::new(Fluid::WATER, 6, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 6, 0), FluidState::new(Fluid::WATER, 7, FluidState::IS_SOURCE), FluidState::new(Fluid::WATER, 7, 0), FluidState::new(Fluid::WATER, 8, FluidState::IS_SOURCE | FluidState::IS_FULL), FluidState::new(Fluid::WATER, 8, FluidState::IS_FULL | FluidState::FALLING), FluidState::new(Fluid::LAVA, 1, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 1, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 2, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 2, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 3, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 3, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 4, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 4, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 5, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 5, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 6, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 6, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 7, FluidState::IS_SOURCE | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 7, FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 8, FluidState::IS_SOURCE | FluidState::IS_FULL | FluidState::IS_RANDOMLY_TICKING), FluidState::new(Fluid::LAVA, 8, FluidState::IS_FULL | FluidState::IS_RANDOMLY_TICKING | FluidState::FALLING)];
use crate::registry::Registry;
use shulkr_macros::static_registry;
#[static_registry]
impl Fluid {
    pub const EMPTY: Fluid = Fluid::new(0);
    pub const WATER: Fluid = Fluid::new(15);
    pub const LAVA: Fluid = Fluid::new(31);
}
pub(crate) fn register_all(registry: &Registry<Fluid>) {
    let register = |key: &'static str, value: Fluid| {
        registry.register(key.into(), value).expect("vanilla registries are filled before freezing");
    };
    register("minecraft:empty", Fluid::EMPTY);
    register("minecraft:water", Fluid::WATER);
    register("minecraft:lava", Fluid::LAVA);
}
