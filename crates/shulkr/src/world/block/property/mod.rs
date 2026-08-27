mod properties;
mod types;

pub use properties::Properties;
pub use std::fmt::Debug;
pub use types::{BoolProperty, EnumProperty, IntProperty};

pub trait Property: Debug + Sync + Send {
    type Value
    where
        Self: Sized;

    fn possible_values(&self) -> Box<[Self::Value]>
    where
        Self: Sized;

    fn index_of(&self, value: &Self::Value) -> usize
    where
        Self: Sized;

    fn by_index(&self, index: usize) -> Self::Value
    where
        Self: Sized;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn value_name_from_index(&self, index: usize) -> &str;
    fn get_possible_value_names(&self) -> Box<[&str]>;

    fn name(&self) -> &'static str;

    fn by_name(&self, value: &str) -> Option<Self::Value>
    where
        Self: Sized;
}
