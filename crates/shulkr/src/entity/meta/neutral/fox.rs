use shulkr_macros::{DataType, Enumeration};
use uuid::Uuid;

use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::fox::{
        FIRST_UUID, IS_CROUCHING, IS_DEFENDING, IS_FACEPLANTED, IS_INTERESTED, IS_POUNCING,
        IS_SITTING, IS_SLEEPING, SECOND_UUID, VARIANT,
    },
};

pub struct FoxMeta {
    holder: MetadataHolder,
}

impl FoxMeta {
    pub fn get_variant(&self) -> FoxVariant {
        FoxVariant::try_from(self.holder.get(VARIANT)).unwrap_or(FoxVariant::Red)
    }

    pub fn set_variant(&self, value: FoxVariant) {
        self.holder.set(VARIANT, value as i32);
    }

    pub fn is_sitting(&self) -> bool {
        self.holder.get(IS_SITTING)
    }

    pub fn set_sitting(&self, value: bool) {
        self.holder.set(IS_SITTING, value);
    }

    pub fn is_crouching(&self) -> bool {
        self.holder.get(IS_CROUCHING)
    }

    pub fn set_crouching(&self, value: bool) {
        self.holder.set(IS_CROUCHING, value);
    }

    pub fn is_interested(&self) -> bool {
        self.holder.get(IS_INTERESTED)
    }

    pub fn set_interested(&self, value: bool) {
        self.holder.set(IS_INTERESTED, value);
    }

    pub fn is_pouncing(&self) -> bool {
        self.holder.get(IS_POUNCING)
    }

    pub fn set_pouncing(&self, value: bool) {
        self.holder.set(IS_POUNCING, value);
    }

    pub fn is_sleeping(&self) -> bool {
        self.holder.get(IS_SLEEPING)
    }

    pub fn set_sleeping(&self, value: bool) {
        self.holder.set(IS_SLEEPING, value);
    }

    pub fn is_faceplanted(&self) -> bool {
        self.holder.get(IS_FACEPLANTED)
    }

    pub fn set_faceplanted(&self, value: bool) {
        self.holder.set(IS_FACEPLANTED, value);
    }

    pub fn is_defending(&self) -> bool {
        self.holder.get(IS_DEFENDING)
    }

    pub fn set_defending(&self, value: bool) {
        self.holder.set(IS_DEFENDING, value);
    }

    pub fn get_first_uuid(&self) -> Option<Uuid> {
        self.holder.get(FIRST_UUID)
    }

    pub fn set_first_uuid(&self, value: Option<Uuid>) {
        self.holder.set(FIRST_UUID, value);
    }

    pub fn get_second_uuid(&self) -> Option<Uuid> {
        self.holder.get(SECOND_UUID)
    }

    pub fn set_second_uuid(&self, value: Option<Uuid>) {
        self.holder.set(SECOND_UUID, value);
    }
}

impl MetaAccessor for FoxMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, DataType)]
pub enum FoxVariant {
    Red,
    Snow,
}
