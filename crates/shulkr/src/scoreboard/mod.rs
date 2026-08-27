pub mod below_name;
pub mod objective;
pub mod sidebar;
pub mod team;

use crate::{
    protocol::encode::{Encode, EncodeError, PacketWrite},
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub enum RenderType {
    Integer,
    Hearts,
}

impl Encode for RenderType {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        match this {
            Self::Integer => w.write_varint(0)?,
            Self::Hearts => w.write_varint(1)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum NumberFormat {
    Blank,
    Styled(TextComponent),
    Fixed(TextComponent),
}

impl Encode for NumberFormat {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        match this {
            Self::Blank => w.write_varint(0)?,
            Self::Styled(c) => {
                w.write_varint(1)?;
                w.write_component(c)?;
            }
            Self::Fixed(c) => {
                w.write_varint(2)?;
                w.write_component(c)?;
            }
        }
        Ok(())
    }
}
