use crate::world::block::property::Property;

#[derive(Debug, Clone)]
pub struct BoolProperty {
    pub name: &'static str,
}

impl BoolProperty {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn value_count(&self) -> usize {
        2
    }

    pub const fn index_of(&self, value: bool) -> usize {
        // IMPORTANT: true=0, false=1 for Java compatibility
        !value as usize
    }

    pub const fn get_internal_index_const(self, value: bool) -> usize {
        if value { 0 } else { 1 }
    }
}

impl Property for BoolProperty {
    type Value = bool;

    fn len(&self) -> usize {
        2
    }

    fn value_name_from_index(&self, index: usize) -> &str {
        ["true", "false"][index]
    }

    fn get_possible_value_names(&self) -> Box<[&str]> {
        ["true", "false"].into()
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn by_name(&self, value: &str) -> Option<Self::Value> {
        if value == "true" {
            Some(true)
        } else if value == "false" {
            Some(false)
        } else {
            None
        }
    }

    fn possible_values(&self) -> Box<[Self::Value]> {
        [true, false].into()
    }

    fn index_of(&self, value: &Self::Value) -> usize {
        usize::from(!*value)
    }

    fn by_index(&self, index: usize) -> Self::Value {
        index == 0
    }
}

// Instead of million heap allocs we just use 42 bytes of static mem :)
const NUM_STR: [&str; 26] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25",
];

#[derive(Debug, Clone)]
pub struct IntProperty {
    pub min: u8,
    pub max: u8,
    pub name: &'static str,
}

impl IntProperty {
    pub const fn new(name: &'static str, min: u8, max: u8) -> Self {
        Self { min, max, name }
    }

    pub const fn value_count(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    pub const fn get_internal_index_const(self, value: &u8) -> usize {
        if *value <= self.max {
            (*value - self.min) as usize
        } else {
            0
        }
    }
}

impl Property for IntProperty {
    type Value = u8;

    fn len(&self) -> usize {
        IntProperty::value_count(self)
    }

    fn value_name_from_index(&self, index: usize) -> &str {
        NUM_STR[self.min as usize + index]
    }

    fn get_possible_value_names(&self) -> Box<[&str]> {
        NUM_STR[self.min as usize..=self.max as usize].into()
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn by_name(&self, value: &str) -> Option<Self::Value> {
        value
            .parse()
            .ok()
            .filter(|v| v >= &self.min && v <= &self.max)
    }

    fn possible_values(&self) -> Box<[Self::Value]> {
        (self.min..=self.max).collect()
    }

    fn index_of(&self, value: &Self::Value) -> usize {
        if *value <= self.max {
            (*value - self.min) as usize
        } else {
            0
        }
    }

    fn by_index(&self, index: usize) -> Self::Value {
        self.min + index as u8
    }
}

#[derive(Debug, Clone)]
pub struct EnumProperty<T: PropertyEnum + 'static> {
    pub name: &'static str,
    pub possible_values: &'static [T],
}

impl<T: PropertyEnum + 'static> EnumProperty<T> {
    pub const fn new(name: &'static str, possible_values: &'static [T]) -> Self {
        Self {
            name,
            possible_values,
        }
    }

    pub const fn value_count(&self) -> usize {
        self.possible_values.len()
    }
}

impl<T: PropertyEnum + 'static> Property for EnumProperty<T> {
    type Value = T;

    fn len(&self) -> usize {
        EnumProperty::value_count(self)
    }

    fn value_name_from_index(&self, index: usize) -> &str {
        self.possible_values[index].as_str()
    }

    fn get_possible_value_names(&self) -> Box<[&str]> {
        self.possible_values
            .iter()
            .map(PropertyEnum::as_str)
            .collect()
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn by_name(&self, value: &str) -> Option<Self::Value> {
        self.possible_values
            .iter()
            .find(|v| v.as_str() == value)
            .cloned()
    }

    fn possible_values(&self) -> Box<[Self::Value]> {
        self.possible_values.into()
    }

    fn index_of(&self, value: &Self::Value) -> usize {
        self.possible_values
            .iter()
            .position(|v| v == value)
            .unwrap()
    }

    fn by_index(&self, index: usize) -> Self::Value {
        self.possible_values[index].clone()
    }
}

pub trait PropertyEnum: PartialEq + Clone + std::fmt::Debug + Sync + Send {
    fn as_str(&self) -> &str;
}
