pub(crate) mod block_entity;
pub mod hit_result;
pub mod placement;

mod block_state;
mod property;

pub use block_entity::*;
pub use block_state::*;
pub use property::*;

use crate::{
    registry::{Id, Ref, registries},
    util::{Aabb, Key, Point, VoxelShape},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(Id);

mod transform;

pub use transform::*;

include!("../../../generated/blocks.rs");

#[derive(Clone)]
pub struct BlockData {
    pub default_state: Id,
    pub min_state_id: Id,
    pub properties: &'static [&'static dyn Property],
    pub block_entity: Option<BlockEntityType>,
    pub shapes: &'static [u64],
    pub fluid_states: &'static [u16],
    pub state_properties: &'static [u32],
    pub offset: Option<BlockOffset>,
    pub transforms: &'static [u16],
}

impl BlockData {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        default_state: Id,
        min_state_id: Id,
        properties: &'static [&'static dyn Property],
        block_entity: Option<BlockEntityType>,
        shapes: &'static [u64],
        fluid_states: &'static [u16],
        state_properties: &'static [u32],
        offset: Option<BlockOffset>,
        transforms: &'static [u16],
    ) -> Self {
        Self {
            default_state,
            min_state_id,
            properties,
            block_entity,
            shapes,
            fluid_states,
            state_properties,
            offset,
            transforms,
        }
    }

    pub fn state_count(&self) -> Id {
        self.properties
            .iter()
            .map(|p| p.len() as Id)
            .product::<Id>()
            .max(1)
    }
}

impl Block {
    pub const fn default_state(&self) -> BlockState {
        BlockState {
            block: *self,
            state_id: self.data().default_state,
        }
    }

    pub fn block_entity(&self) -> Option<&BlockEntityType> {
        self.data().block_entity.as_ref()
    }

    pub fn key(&self) -> &'static Key {
        self.try_key().expect("block is not registered")
    }

    pub fn try_key(&self) -> Option<&'static Key> {
        registries::BLOCK.key_of(self.0)
    }

    pub fn has_tag(&self, tag: &Key) -> bool {
        let registry = &registries::BLOCK;
        registry
            .key_of(self.0)
            .is_some_and(|key| registry.has_tag(key, tag))
    }

    pub fn from_id(id: Id) -> Option<&'static Block> {
        registries::BLOCK.by_id(id).map(Ref::value)
    }

    pub fn from_key(key: impl Into<Key>) -> Option<&'static Block> {
        registries::BLOCK.by_key(&key.into()).map(Ref::value)
    }
}

impl TryFrom<Id> for Block {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        if (value as usize) < registries::BLOCK.len() {
            Ok(Block(value))
        } else {
            Err(())
        }
    }
}

impl From<Block> for Id {
    #[inline]
    fn from(block: Block) -> Self {
        block.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetType {
    Xz,
    Xyz,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockOffset {
    kind: OffsetType,
    max_horizontal: f64,
    max_vertical: f64,
}

impl BlockOffset {
    pub const fn new(kind: OffsetType, max_horizontal: f64, max_vertical: f64) -> Self {
        Self {
            kind,
            max_horizontal,
            max_vertical,
        }
    }

    pub fn at(&self, x: i32, z: i32) -> Point {
        let seed = position_seed(x, 0, z);
        let offset_x = self.horizontal_component(seed & 15);
        let offset_z = self.horizontal_component(seed >> 8 & 15);

        match self.kind {
            OffsetType::Xz => Point::new(offset_x, 0.0, offset_z),
            OffsetType::Xyz => {
                let bits = (seed >> 4 & 15) as f32 / 15.0;
                let offset_y = (f64::from(bits) - 1.0) * self.max_vertical;
                Point::new(offset_x, offset_y, offset_z)
            }
        }
    }

    fn horizontal_component(&self, seed_bits: i64) -> f64 {
        let raw = (f64::from(seed_bits as f32 / 15.0) - 0.5) * 0.5;
        raw.clamp(-self.max_horizontal, self.max_horizontal)
    }
}

const fn position_seed(x: i32, y: i32, z: i32) -> i64 {
    let value =
        (x.wrapping_mul(3_129_871)) as i64 ^ (z as i64).wrapping_mul(116_129_781) ^ y as i64;
    let value = value
        .wrapping_mul(value)
        .wrapping_mul(42_317_861)
        .wrapping_add(value.wrapping_mul(11));
    value >> 16
}

pub struct StateFlags;

impl StateFlags {
    pub const SHIFT: u32 = 8;

    pub const IS_AIR: u32 = 1;
    pub const IS_LIQUID: u32 = 1 << 1;
    pub const IS_SOLID: u32 = 1 << 2;
    pub const IS_SOLID_RENDER: u32 = 1 << 3;
    pub const CAN_OCCLUDE: u32 = 1 << 4;
    pub const CAN_BE_REPLACED: u32 = 1 << 5;
    pub const USE_SHAPE_FOR_LIGHT_OCCLUSION: u32 = 1 << 6;
    pub const PROPAGATES_SKYLIGHT_DOWN: u32 = 1 << 7;
    pub const IS_RANDOMLY_TICKING: u32 = 1 << 8;
    pub const IGNITED_BY_LAVA: u32 = 1 << 9;
    pub const HAS_DYNAMIC_SHAPE: u32 = 1 << 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_know_their_vanilla_tags() {
        crate::registry::registries::register_all();
        let logs = Key::const_vanilla("logs");
        assert!(Block::OAK_LOG.has_tag(&logs));
        assert!(Block::CRIMSON_STEM.has_tag(&logs));
        assert!(!Block::STONE.has_tag(&logs));

        let leaves = Key::const_vanilla("leaves");
        assert!(Block::OAK_LEAVES.has_tag(&leaves));
        assert!(!Block::OAK_LOG.has_tag(&leaves));
    }

    #[test]
    fn tag_membership_is_read_through_the_registry() {
        crate::registry::registries::register_all();
        let dirt = Key::const_vanilla("dirt");
        assert!(registries::BLOCK.has_tag(&Key::const_vanilla("coarse_dirt"), &dirt));
        assert!(!registries::BLOCK.has_tag(&Key::const_vanilla("stone"), &dirt));
    }

    #[test]
    fn iter_tag_walks_the_members() {
        crate::registry::registries::register_all();
        let tag = Key::const_vanilla("logs");
        let members: Vec<&Key> = registries::BLOCK
            .iter_tag(&tag)
            .map(|(key, _)| key)
            .collect();

        assert!(members.len() > 10);
        assert!(members.contains(&&Key::const_vanilla("oak_log")));
    }
}
