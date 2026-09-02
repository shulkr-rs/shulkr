use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::llama::{STRENGTH, VARIANT},
};
use shulkr_macros::{DataType, Enumeration};

pub struct LlamaMeta {
    holder: MetadataHolder,
}

impl LlamaMeta {
    pub fn get_variant(&self) -> LlamaVariant {
        LlamaVariant::try_from(self.holder.get(VARIANT)).unwrap_or(LlamaVariant::Creamy)
    }

    pub fn set_variant(&self, value: LlamaVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn get_strength(&self) -> i32 {
        self.holder.get(STRENGTH)
    }

    pub fn set_strength(&self, value: i32) {
        self.holder.set(STRENGTH, value);
    }
}

impl MetaAccessor for LlamaMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum LlamaVariant {
    Creamy,
    White,
    Brown,
    Gray,
}
