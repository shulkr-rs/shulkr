use super::*;

#[derive(Clone, Copy)]
pub struct BlockState {
    pub(super) block: Block,
    pub(super) state_id: u16,
}

impl BlockState {
    pub fn block_entity(&self) -> Option<&BlockEntityInfo> {
        self.block().def().block_entity.as_ref()
    }

    pub fn id(&self) -> u16 {
        self.state_id
    }

    pub fn block(&self) -> Block {
        self.block
    }

    pub fn from_id(state_id: u16) -> Option<BlockState> {
        let registry = REGISTRY.get()?;
        let def = registry.def_by_state(state_id)?;
        let block = Block::try_from(registry.by_id.get(def.id as usize).copied()?).ok()?;
        Some(BlockState { block, state_id })
    }

    pub fn from_key(key: impl Into<String>) -> Option<BlockState> {
        Block::from_key(key).map(|b| b.default_state())
    }

    pub fn values() -> Vec<BlockState> {
        let registry = REGISTRY.get().unwrap();
        registry
            .defs
            .iter()
            .flat_map(|def| {
                let block = Block::try_from(*registry.by_id.get(def.id as usize).unwrap()).unwrap();
                (def.min_state_id..def.min_state_id + def.state_count())
                    .map(move |state_id| BlockState { block, state_id })
            })
            .collect()
    }

    pub fn get_property(&self, name: &str) -> Option<&'static str> {
        let def = self.block.def();
        let relative = self.state_id - def.min_state_id;

        let mut stride = 1u16;
        for prop in def.properties.iter().rev() {
            let count = prop.value_count();
            if prop.name == name {
                let index = (relative / stride) % count;
                return Some(prop.values[index as usize]);
            }
            stride *= count;
        }
        None
    }

    pub fn set_property(&mut self, name: &str, value: &str) {
        let def = self.block.def();
        let relative = self.state_id - def.min_state_id;

        let mut stride = 1u16;
        for prop in def.properties.iter().rev() {
            let count = prop.value_count();
            if prop.name == name {
                let old_index = (relative / stride) % count;
                let new_index = prop
                    .values
                    .iter()
                    .position(|v| *v == value)
                    .expect("value not found") as u16;
                self.state_id = self.state_id - old_index * stride + new_index * stride;
                return;
            }
            stride *= count;
        }
        panic!("Property {} not found on block {}", name, def.id);
    }

    pub fn with_property(&self, name: &str, value: &str) -> Option<BlockState> {
        let def = self.block.def();
        let mut stride = 1u16;
        for prop in def.properties.iter().rev() {
            let count = prop.value_count();
            if prop.name == name {
                let new_index = prop.values.iter().position(|v| *v == value)? as u16;
                let relative = self.state_id - def.min_state_id;
                let old_index = (relative / stride) % count;
                return Some(BlockState {
                    block: self.block,
                    state_id: self.state_id - old_index * stride + new_index * stride,
                });
            }
            stride *= count;
        }
        None
    }

    pub fn has_property(&self, name: &str) -> bool {
        self.block
            .def()
            .properties
            .iter()
            .any(|p| p.name == name)
    }
}

impl From<Block> for BlockState {
    fn from(block: Block) -> Self {
        block.default_state()
    }
}
