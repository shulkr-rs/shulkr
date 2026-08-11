use std::sync::OnceLock;

use crate::{
    registry::{Id, Registries, Registry},
    util::Key,
    world::block::property::Property,
};

use super::*;

#[derive(Clone, Copy)]
pub struct BlockState {
    pub block: Block,
    pub state_id: u16,
}

impl BlockState {
    pub fn block_entity(&self) -> Option<&BlockEntityType> {
        self.block.data().block_entity.as_ref()
    }

    /// Returns the id of the [BlockState].
    pub fn state_id(&self) -> u16 {
        self.state_id
    }

    /// Returns this [BlockState] back into a [`Block`].
    pub fn as_block(&self) -> Block {
        self.block
    }

    /// Returns the value of a specific [Property].
    ///
    /// If the Block does not have this value [None] is returned.
    pub fn get<P>(&self, property: P) -> Option<P::Value>
    where
        P: Property,
    {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            if prop.name() == property.name() {
                let index = (relative / stride) % count;
                return Some(property.by_index(index as usize));
            }
            stride *= count;
        }
        None
    }

    /// Sets the value of a specific [Property].
    ///
    /// Panics if the block does not have this property.
    pub fn set<P>(&mut self, property: P, value: P::Value)
    where
        P: Property,
    {
        let Some(state_id) = self.set_index(property.name(), property.index_of(&value)) else {
            panic!(
                "Property {} not found on block {}",
                property.name(),
                self.block as u16
            );
        };
        self.state_id = state_id;
    }

    pub fn with<P>(&self, property: P, value: P::Value) -> Option<BlockState>
    where
        P: Property,
    {
        self.set_index(property.name(), property.index_of(&value))
            .map(|state_id| BlockState {
                block: self.block,
                state_id,
            })
    }

    #[cfg(any(feature = "anvil", feature = "polar"))]
    pub(crate) fn with_index(&self, property: &dyn Property, index: usize) -> Option<BlockState> {
        self.set_index(property.name(), index)
            .map(|state_id| BlockState {
                block: self.block,
                state_id,
            })
    }

    fn set_index(&self, name: &str, index: usize) -> Option<u16> {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            if prop.name() == name {
                let old_index = (relative / stride) % count;
                return Some(self.state_id - old_index * stride + index as u16 * stride);
            }
            stride *= count;
        }
        None
    }

    /// Returns a [BlockState] based on the given `state_id`. If no corrosponding BlockState is found [None] is returned.
    pub fn from_id(state_id: u16) -> Option<BlockState> {
        let block_index = block_state_table().block_by_state(state_id)?;
        let block = *Registries::BLOCK.by_id(block_index as Id)?;
        Some(BlockState { block, state_id })
    }

    /// Returns a [BlockState] based on the given `key`. If no corrosponding BlockState is found [None] is returned.
    pub fn from_key(key: impl Into<Key>) -> Option<BlockState> {
        Block::from_key(key).map(|b| b.default_state())
    }

    pub fn all() -> Vec<BlockState> {
        Block::all()
            .iter()
            .flat_map(|block| {
                let min_state_id = block.data().min_state_id;
                (min_state_id..min_state_id + block.data().state_count()).map(move |state_id| {
                    BlockState {
                        block: *block,
                        state_id,
                    }
                })
            })
            .collect()
    }
}

impl From<Block> for BlockState {
    fn from(block: Block) -> Self {
        block.default_state()
    }
}

pub struct BlockStateTable {
    state_to_block: Vec<u16>,
}

impl BlockStateTable {
    pub fn build(blocks: &Registry<Block>) -> Self {
        let mut state_to_block = Vec::new();
        for (index, block) in blocks.values().iter().enumerate() {
            let state_count = block.data().state_count();
            for _ in 0..state_count {
                state_to_block.push(index as u16);
            }
        }
        Self { state_to_block }
    }

    pub fn block_by_state(&self, state_id: u16) -> Option<u16> {
        self.state_to_block.get(state_id as usize).copied()
    }
}

static BLOCK_STATE_TABLE: OnceLock<BlockStateTable> = OnceLock::new();

pub fn block_state_table() -> &'static BlockStateTable {
    BLOCK_STATE_TABLE.get_or_init(|| BlockStateTable::build(Registries::BLOCK))
}
