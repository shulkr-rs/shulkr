use crate::{
    entity::Hand,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
        types::read_lp_vec3,
    },
};

#[derive(Debug, Clone)]
pub struct InteractPacket {
    pub entity_id: i32,
    pub hand: Hand,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub using_secondary_action: bool,
}

impl Packet for InteractPacket {}
impl ClientPacket for InteractPacket {}

impl Decode for InteractPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let entity_id = r.read_varint()?;
        let hand = Hand::decode(r)?;
        let (target_x, target_y, target_z) = read_lp_vec3(r)?;
        let using_secondary_action = r.read_bool()?;

        Ok(Self {
            entity_id,
            hand,
            target_x,
            target_y,
            target_z,
            using_secondary_action,
        })
    }
}
