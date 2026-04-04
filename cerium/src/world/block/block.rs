use super::*;

include!("../../registry/generated/blocks.rs");

impl Block {
    pub(crate) fn def(&self) -> &'static BlockDef {
        // SAFETY: REGISTRY is always initialized before any Block is used
        unsafe { REGISTRY.get().unwrap_unchecked() }.def_by_index(*self as u16)
    }

    pub fn default_state(&self) -> BlockState {
       
        BlockState {
            block: *self,
            state_id: self.def().default_state,
        }
    }

    pub fn from_id(id: u16) -> Option<Block> {
        let index = *REGISTRY.get()?.by_id.get(id as usize)?;
        Block::try_from(index).ok()
    }

    pub fn from_key(key: impl Into<String>) -> Option<Block> {
        let index = *REGISTRY.get()?.by_key.get(&key.into())?;
        Block::try_from(index).ok()
    }
}
