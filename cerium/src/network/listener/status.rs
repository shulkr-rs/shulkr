use crate::{
    PingResponse, Version,
    protocol::{
        decode::{Decode as _, DecodeError},
        packet::{
            PongResponsePacket, StatusRequestPacket, StatusResponsePacket,
            client::status::PingRequestPacket,
        },
    },
    text::TextComponent,
};
use crate::{event::server_ping::ServerListPingEvent, network::client::Connection};

use std::{io::Cursor, sync::Arc};

#[rustfmt::skip]
pub fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_status_request(client, StatusRequestPacket::decode(data)?),
        0x01 => handle_ping_request(client, PingRequestPacket::decode(data)?),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

fn handle_status_request(client: Arc<Connection>, _packet: StatusRequestPacket) {
    use crate::{PROTOCOL_NAME, PROTOCOL_VERSION};
    use uuid::Uuid;

    let version = Version::new(PROTOCOL_VERSION, PROTOCOL_NAME);
    let default_ping = PingResponse::builder(version)
        .with_max_players(100)
        .with_online_players(5)
        .with_player(("Steve", Uuid::new_v4()).into())
        .with_description(TextComponent::text("Example Description"))
        .build();

    let mut event = ServerListPingEvent::new(default_ping);
    client.server().events().fire(&mut event);

    client.send_packet(&StatusResponsePacket {
        json_response: serde_json::to_string(event.get_response()).unwrap(),
    });
}

fn handle_ping_request(client: Arc<Connection>, packet: PingRequestPacket) {
    client.send_packet(&PongResponsePacket {
        timestamp: packet.timestamp,
    });
}
