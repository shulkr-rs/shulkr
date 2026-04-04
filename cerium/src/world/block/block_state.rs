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

    pub fn get_property<P: Property>(&self) -> Option<P::Value> {
        let def = self.block.def();
        let relative = self.state_id - def.min_state_id;

        let mut stride = 1u16;
        for prop in def.properties.iter().rev() {
            if prop.name() == P::NAME {
                let index = (relative / stride) % prop.value_count();
                return P::from_index(index as usize);
            }
            stride *= prop.value_count();
        }
        None
    }

    pub fn set_property<P: Property>(&mut self, value: P::Value) {
        let def = self.block.def();
        let relative = self.state_id - def.min_state_id;

        let mut stride = 1u16;
        for prop in def.properties.iter().rev() {
            if prop.name() == P::NAME {
                let old_index = (relative / stride) % prop.value_count();
                let new_index = P::to_index(&value) as u16;
                self.state_id = self.state_id - old_index * stride + new_index * stride;
                return;
            }
            stride *= prop.value_count();
        }
        panic!("Property {} not found on block {}", P::NAME, def.id);
    }

    pub fn with_property<P>(&self, value: P::Value) -> Self
    where
        P: Property,
    {
        let mut new = *self;
        new.set_property::<P>(value);
        new
    }

    pub fn has_property<P: Property>(&self) -> bool {
        self.block
            .def()
            .properties
            .iter()
            .any(|p| p.name() == P::NAME)
    }
}

impl From<Block> for BlockState {
    fn from(block: Block) -> Self {
        block.default_state()
    }
}
