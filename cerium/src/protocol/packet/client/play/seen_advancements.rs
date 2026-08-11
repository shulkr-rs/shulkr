use crate::{
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    util::Key,
};

#[derive(Debug, Clone)]
pub struct SeenAdvancementsPacket {
    pub action: i32,
    pub tab_id: Option<Key>,
}

impl Packet for SeenAdvancementsPacket {}
impl ClientPacket for SeenAdvancementsPacket {}

impl Decode for SeenAdvancementsPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let action = r.read_varint()?;
        Ok(Self {
            action,
            tab_id: if action == 0 {
                Some(r.read_identifier()?)
            } else {
                None
            },
        })
    }
}
