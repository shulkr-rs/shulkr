use super::BlockState;
use super::property::{Properties, Property};
use crate::registry::{Id, Registries};
use crate::util::Key;
use crate::world::block::BlockEntityType;

include!("../../registry/generated/blocks.rs");

#[derive(Clone)]
pub struct BlockData {
    pub default_state: Id,
    pub min_state_id: Id,
    pub properties: &'static [&'static dyn Property],
    pub block_entity: Option<BlockEntityType>,
}

impl BlockData {
    pub const fn new(
        default_state: Id,
        min_state_id: Id,
        properties: &'static [&'static dyn Property],
        block_entity: Option<BlockEntityType>,
    ) -> Self {
        Self {
            default_state,
            min_state_id,
            properties,
            block_entity,
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

    pub fn from_id(id: Id) -> Option<&'static Block> {
        Registries::BLOCK.by_id(id)
    }

    pub fn from_key(key: impl Into<Key>) -> Option<&'static Block> {
        Registries::BLOCK.by_key(&key.into())
    }
}

impl TryFrom<Id> for Block {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::all().get(value as usize).copied().ok_or(())
    }
}

impl From<Block> for Id {
    #[inline]
    fn from(block: Block) -> Self {
        block as Id
    }
}
