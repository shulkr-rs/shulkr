use std::{fmt::Debug, sync::Arc};

use rustc_hash::FxBuildHasher;

use crate::{
    inventory::Slot,
    item::{ComponentMap, DataComponent, Material},
    registry::Id,
    util::HashMap,
};

#[derive(Debug, Clone)]
pub struct ItemStack {
    material: Material,
    amount: i32,
    components: ComponentMap,
}

impl ItemStack {
    pub const EMPTY: ItemStack = ItemStack::new(Material::AIR, 0);

    pub const fn new(material: Material, amount: i32) -> Self {
        Self {
            material,
            amount,
            components: HashMap::with_hasher(FxBuildHasher),
        }
    }

    pub const fn of(material: Material) -> Self {
        Self {
            material,
            amount: 1,
            components: HashMap::with_hasher(FxBuildHasher),
        }
    }

    pub fn with_material(self, material: Material) -> Self {
        Self {
            material,
            amount: self.amount,
            components: self.components,
        }
    }

    pub fn with_amount(self, amount: i32) -> Self {
        Self {
            material: self.material,
            amount,
            components: self.components,
        }
    }

    pub fn with<T>(mut self, component: DataComponent<T>, value: T) -> Self
    where
        T: 'static + Sync + Send,
    {
        self.components.insert(component.id(), Arc::new(value));
        self
    }

    pub fn get<T>(&self, component: DataComponent<T>) -> Option<&T>
    where
        T: 'static,
    {
        self.components
            .get(&component.id())
            .and_then(|v| v.downcast_ref::<T>())
    }

    pub fn set<T>(mut self, component: DataComponent<T>, value: T)
    where
        T: 'static + Send + Sync,
    {
        self.components.insert(component.id(), Arc::new(value));
    }

    pub fn has<T>(&self, component: DataComponent<T>) -> bool
    where
        T: 'static,
    {
        self.get(component).is_some()
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    /// Returns true if this stack is empty (`Material::AIR`).
    pub fn is_empty(&self) -> bool {
        self.material == Material::AIR || self.amount <= 0
    }

    /// Returns the maximum amount this stack can hold, taking a custom
    /// `MAX_STACK_SIZE` component into account.
    pub fn max_stack_size(&self) -> i32 {
        self.get(DataComponent::MAX_STACK_SIZE)
            .copied()
            .unwrap_or_else(|| self.material.max_stack_size())
    }

    /// Returns true if two stacks can be merged into one, e.g. same item.
    pub fn can_stack_with(&self, other: &ItemStack) -> bool {
        !self.is_empty() && self.material == other.material
    }
}

impl From<Material> for ItemStack {
    fn from(value: Material) -> Self {
        Self::of(value)
    }
}

impl From<Slot> for ItemStack {
    fn from(value: Slot) -> Self {
        if let Some(item_id) = value.item_id {
            Self {
                material: Material::from_id(item_id as Id).unwrap(),
                amount: value.item_count,
                components: value.to_add,
            }
        } else {
            ItemStack::EMPTY
        }
    }
}

impl Into<Slot> for ItemStack {
    fn into(self) -> Slot {
        Slot {
            item_count: self.amount,
            item_id: Some(Id::from(self.material) as i32),
            to_add: self.components,
            to_remove: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_components() {
        let stack = ItemStack::of(Material::STONE)
            .with(DataComponent::MAX_STACK_SIZE, 16)
            .with(DataComponent::MAX_DAMAGE, 99)
            .with(DataComponent::UNBREAKABLE, ());

        assert_eq!(stack.get(DataComponent::MAX_STACK_SIZE), Some(&16));
        assert_eq!(stack.get(DataComponent::MAX_DAMAGE), Some(&99));
        assert_eq!(stack.get(DataComponent::UNBREAKABLE), Some(&()));
        assert_eq!(stack.get(DataComponent::DAMAGE), None);
    }
}
