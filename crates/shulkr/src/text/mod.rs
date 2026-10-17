#[cfg(feature = "minimessage")]
pub mod minimessage;

mod color;
mod component;
mod style;

pub use color::{NamedColor, Rgb, Rgba};
pub use component::*;
#[cfg(feature = "minimessage")]
pub use minimessage::MiniMessage;
pub use style::{ClickEvent, HoverEvent, TextStyle};
