pub mod kind;

use crate::command::arg::kind::ArgKind;

#[derive(Debug, Clone)]
pub struct Arg {
    pub(crate) name: String,
    pub(crate) required: bool,
    pub(crate) kind: ArgKind,
}

impl Arg {
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}
