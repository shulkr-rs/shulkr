use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::sync::OnceLock;

pub mod property;
use property::*;

mod block;
pub use block::*;

mod block_state;
pub use block_state::*;

mod block_entity;
pub use block_entity::*;

pub static REGISTRY: OnceLock<BlockRegistry> = OnceLock::new();

pub struct BlockRegistry {
    defs: Vec<BlockDef>,
    by_id: Vec<u16>,
    by_key: FxHashMap<String, u16>,
    _state_to_block: Vec<u16>,
}

impl BlockRegistry {
    pub fn load() -> &'static Self {
        static INPUT: &str = include_str!("../../../data/blocks.json");

        let entries: IndexMap<String, serde_json::Value> = serde_json::from_str(INPUT).unwrap();

        let mut defs = Vec::new();
        let mut by_id = Vec::new();
        let mut by_key = FxHashMap::default();
        let mut state_to_block = Vec::new();
        let mut next_state_id = 0u16;

        for (key, block) in &entries {
            let id = block["id"].as_u64().unwrap() as u16;
            let default_state = block["defaultStateId"].as_u64().unwrap() as u16;

            let properties: &'static [DynamicProperty] = {
                let props: Vec<DynamicProperty> = block["properties"]
                    .as_object()
                    .map(|props| {
                        props
                            .iter()
                            .map(|(name, values)| {
                                let name: &'static str = Box::leak(name.clone().into_boxed_str());
                                let count = values.as_array().unwrap().len() as u16;
                                DynamicProperty { name, count }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                Box::leak(props.into_boxed_slice())
            };

            let state_count: u16 = properties
                .iter()
                .map(|p| p.value_count())
                .product::<u16>()
                .max(1);

            let min_state_id = next_state_id;
            next_state_id += state_count;

            let index = defs.len() as u16;
            by_id.push(index);
            by_key.insert(key.clone(), index);

            for _ in 0..state_count {
                state_to_block.push(index);
            }

            let block_entity = block.get("blockEntity").map(|be| {
                let namespace = be["namespace"].as_str().unwrap().to_string();
                let id = be["id"].as_i64().unwrap() as i32;

                BlockEntityInfo { namespace, id }
            });

            defs.push(BlockDef {
                id,
                default_state,
                min_state_id,
                properties,
                block_entity
            });
        }

        REGISTRY.get_or_init(|| Self {
            defs,
            by_id,
            by_key,
            _state_to_block: state_to_block,
        })
    }

    pub fn def_by_index(&self, index: u16) -> &BlockDef {
        &self.defs[index as usize]
    }

    pub fn def_by_state(&self, state_id: u16) -> Option<&BlockDef> {
        let index = self
            .defs
            .partition_point(|def| def.min_state_id <= state_id)
            .saturating_sub(1);
        let def = self.defs.get(index)?;
        // verify state_id is actually within this block's range

        if state_id < def.min_state_id + def.state_count() {
            Some(def)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct BlockDef {
    pub id: u16,
    pub default_state: u16,
    pub min_state_id: u16,
    pub properties: &'static [DynamicProperty],
    pub block_entity: Option<BlockEntityInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl TryFrom<i32> for BlockFace {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Bottom,
            1 => Self::Top,
            2 => Self::North,
            3 => Self::South,
            4 => Self::West,
            5 => Self::East,
            _ => return Err(()),
        })
    }
}

impl BlockDef {
    pub fn state_count(&self) -> u16 {
        self.properties
            .iter()
            .map(|p| p.value_count())
            .product::<u16>()
            .max(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use super::*;

    #[test]
    fn test_properties() {
        let start = Instant::now();
        BlockRegistry::load();
        println!("{:?}", start.elapsed());

        let state = Block::GrassBlock.default_state();

        // false since default state is snowy=false
        assert_eq!(state.get_property::<p![Snowy]>(), Some(false));

        let state = state.with_property::<p![Snowy]>(true);
        assert_eq!(state.get_property::<p![Snowy]>(), Some(true));

        let mut bamboo = Block::Bamboo.default_state();
        println!("leaves: {:?}", bamboo.get_property::<p![BambooLeaves]>());
        bamboo.set_property::<p![BambooLeaves]>(BambooLeaves::Large);
        println!(
            "leaves after: {:?}",
            bamboo.get_property::<p![BambooLeaves]>()
        );
    }
}
