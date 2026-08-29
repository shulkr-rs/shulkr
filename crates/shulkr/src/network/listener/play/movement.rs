use crate::{
    entity::{EntityLike as _, Player},
    protocol::packet::{
        ConfirmTeleportationPacket, PlayerMovementFlagsPacket, PlayerPositionAndRotationPacket,
        PlayerPositionPacket, PlayerRotationPacket,
    },
    util::Position,
};

pub(crate) fn handle_confirm_teleportation(_player: Player, _packet: ConfirmTeleportationPacket) {
    log::warn!("todo: handle_confirm_teleportation");
}

pub(crate) fn handle_player_position(player: Player, packet: PlayerPositionPacket) {
    let new_position = Position::new(
        packet.x,
        packet.feet_y,
        packet.z,
        player.position().yaw(),
        player.position().pitch(),
    );
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

pub(crate) fn handle_player_position_and_rotation(
    player: Player,
    packet: PlayerPositionAndRotationPacket,
) {
    let new_position = Position::new(packet.x, packet.feet_y, packet.z, packet.yaw, packet.pitch);
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

pub(crate) fn handle_player_rotation(player: Player, packet: PlayerRotationPacket) {
    let new_position = Position::new(
        player.position().x(),
        player.position().y(),
        player.position().z(),
        packet.yaw,
        packet.pitch,
    );
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

fn handle_movement(player: Player, new_position: Position, on_ground: bool) {
    let old_position = player.position();
    let new_position = Position::clamp_max(new_position);

    if new_position == old_position {
        return;
    }

    player.refresh_position(new_position);
    player.set_on_ground(on_ground);
}

pub(crate) fn handle_player_movement_flags(_player: Player, _packet: PlayerMovementFlagsPacket) {
    log::warn!("todo: handle_player_movement_flags");
}
