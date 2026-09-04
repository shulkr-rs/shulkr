use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Position,
};

#[derive(Debug, Clone)]
pub struct EntityPositionRotationPacket {
    pub entity_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}

impl EntityPositionRotationPacket {
    pub fn new(
        entity_id: i32,
        new_position: Position,
        old_position: Position,
        on_ground: bool,
    ) -> Self {
        let delta_x = new_position.x() * 4096. - old_position.x() * 4096.;
        let delta_y = new_position.y() * 4096. - old_position.y() * 4096.;
        let delta_z = new_position.z() * 4096. - old_position.z() * 4096.;

        Self {
            entity_id,
            delta_x: delta_x as i16,
            delta_y: delta_y as i16,
            delta_z: delta_z as i16,
            yaw: (new_position.yaw() * 256. / 360.) as i8,
            pitch: (new_position.pitch() * 256. / 360.) as i8,
            on_ground,
        }
    }
}

impl Packet for EntityPositionRotationPacket {}
impl ServerPacket for EntityPositionRotationPacket {}

impl Encode for EntityPositionRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        w.write_varint(if this.on_ground { 1 } else { 0 })?;
        w.write_i16(this.delta_x)?;
        w.write_i16(this.delta_y)?;
        w.write_i16(this.delta_z)?;
        w.write_i8(this.yaw)?;
        w.write_i8(this.pitch)?;
        Ok(())
    }
}
