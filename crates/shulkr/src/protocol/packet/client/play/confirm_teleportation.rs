use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ConfirmTeleportationPacket {
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Packet for ConfirmTeleportationPacket {}
impl ClientPacket for ConfirmTeleportationPacket {}

impl Decode for ConfirmTeleportationPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            teleport_id: r.read_varint()?,
            x: r.read_f64()?,
            y: r.read_f64()?,
            z: r.read_f64()?,
            yaw: r.read_f32()?,
            pitch: r.read_f32()?,
        })
    }
}
