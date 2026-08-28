use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("Invalid {name}: {value}")]
pub struct InvalidEnumVariant {
    pub name: &'static str,
    pub value: i32,
}

impl InvalidEnumVariant {
    pub const fn new(name: &'static str, value: i32) -> Self {
        Self { name, value }
    }
}
