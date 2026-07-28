use crate::{
    advancement::{Advancement, AdvancementTree},
    item::ItemStack,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Identifier,
};

#[derive(Debug, Clone)]
pub struct UpdateAdvancementsPacket {
    pub reset: bool,
    pub advancements: AdvancementTree,
    pub to_remove: Vec<Identifier>,
    pub progress: Vec<()>,
    pub show_advancements: bool,
}

impl Packet for UpdateAdvancementsPacket {}
impl ServerPacket for UpdateAdvancementsPacket {}

impl Encode for UpdateAdvancementsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_bool(this.reset)?;
        AdvancementTree::encode(w, &this.advancements)?;
        w.write_array(&this.to_remove, PacketWrite::write_identifier)?;
        w.write_array(&this.progress, |_, _| Ok(()))?;
        w.write_bool(this.show_advancements)?;
        Ok(())
    }
}

impl Encode for AdvancementTree {
    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!();
    }
}

impl Encode for Advancement {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_option(&this.parent(), |w, v| w.write_identifier(v))?;

        // Advancement Display
        w.write_component(this.title())?;
        w.write_component(this.description())?;
        ItemStack::encode(w, this.icon())?;
        w.write_varint(this.frame() as i32)?;
        w.write_i32(this.flags())?;
        if let Some(background) = this.background() {
            w.write_identifier(background)?;
        }
        w.write_f32(this.x())?;
        w.write_f32(this.y())?;

        w.write_bool(this.sends_telemetry_data())?;
        todo!()
    }
}
