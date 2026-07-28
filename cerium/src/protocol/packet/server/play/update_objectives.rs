use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    scoreboard::{NumberFormat, RenderType},
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub struct UpdateObjectivesPacket {
    pub objective_name: String,
    pub action: UpdateObjectivesAction,
}

impl Packet for UpdateObjectivesPacket {}
impl ServerPacket for UpdateObjectivesPacket {}

impl Encode for UpdateObjectivesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.objective_name)?;
        UpdateObjectivesAction::encode(w, &this.action)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum UpdateObjectivesAction {
    CreateScoreboard {
        value: TextComponent,
        ty: RenderType,
        number_format: Option<NumberFormat>,
    },
    RemoveScoreboard,
    UpdateScoreboard {
        value: TextComponent,
        ty: RenderType,
        number_format: Option<NumberFormat>,
    },
}

impl Encode for UpdateObjectivesAction {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(match this {
            Self::CreateScoreboard { .. } => 0,
            Self::RemoveScoreboard => 1,
            Self::UpdateScoreboard { .. } => 2,
        })?;

        match this {
            Self::CreateScoreboard {
                value,
                ty,
                number_format,
            }
            | Self::UpdateScoreboard {
                value,
                ty,
                number_format,
            } => {
                w.write_component(value)?;
                RenderType::encode(w, ty)?;
                w.write_option(number_format, |w, v| NumberFormat::encode(w, v))?;
            }
            _ => {}
        }

        Ok(())
    }
}
