use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
        types::write_lp_vec3,
    },
    util::Position,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SpawnEntityPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub entity_type: i32,
    pub position: Position,
    pub head_yaw: f32,
    pub data: i32,
    /// Spawn velocity in blocks/tick, encoded right after position with the
    /// low-precision `Vec3` codec (26.2 dropped the old three-`i16` layout).
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
}

impl Packet for SpawnEntityPacket {}
impl ServerPacket for SpawnEntityPacket {}

impl Encode for SpawnEntityPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.id)?;
        w.write_uuid(&this.uuid)?;
        w.write_varint(this.entity_type)?;
        w.write_f64(this.position.x())?;
        w.write_f64(this.position.y())?;
        w.write_f64(this.position.z())?;
        write_lp_vec3(w, this.velocity_x, this.velocity_y, this.velocity_z)?;
        w.write_i8((this.position.pitch() * 256. / 360.) as i8)?;
        w.write_i8((this.position.yaw() * 256. / 360.) as i8)?;
        w.write_i8((this.head_yaw * 256. / 360.) as i8)?;
        w.write_varint(this.data)?;
        Ok(())
    }
}
