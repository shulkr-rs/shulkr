use crate::{
    auth::{self, AuthMode, GameProfile},
    network::{
        client::Connection,
        listener::login::velocity::{login_velocity, parse_velocity_response},
    },
    protocol::{
        ProtocolState,
        decode::{Decode as _, DecodeError},
        packet::{
            EncryptionRequestPacket, EncryptionResponsePacket, LoginAcknowledgePacket,
            LoginPluginResponsePacket, LoginStartPacket, LoginSuccessPacket, SetCompressionPacket,
        },
    },
};
use std::{io::Cursor, sync::Arc};

mod velocity;

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_login_start(client, LoginStartPacket::decode(data)?).await,
        0x01 => handle_encryption_response(client, EncryptionResponsePacket::decode(data)?).await,
        0x02 => handle_login_plugin_response(client, LoginPluginResponsePacket::decode(data)?).await,
        0x03 => handle_login_acknowledged(client, LoginAcknowledgePacket::decode(data)?),
        0x04 => handle_cookie_response(client),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_login_start(client: Arc<Connection>, packet: LoginStartPacket) {
    let game_profile = GameProfile {
        uuid: packet.uuid,
        name: packet.name,
        properties: vec![],
    };
    *client.game_profile.lock() = Some(game_profile.clone());

    match client.server().auth_mode() {
        AuthMode::Offline => login_offline(client, game_profile).await,
        AuthMode::Online => login_online(&client),
        AuthMode::Velocity(_) => login_velocity(&client),
    }
}

async fn login_offline(client: Arc<Connection>, mut game_profile: GameProfile) {
    use uuid::Uuid;

    game_profile.uuid = Uuid::new_v3(
        &Uuid::NAMESPACE_OID,
        format!("OfflinePlayer:{}", game_profile.name).as_bytes(),
    );
    finish_login(client, game_profile).await;
}

fn login_online(conn: &Connection) {
    let verify_token: [u8; 4] = rand::random();
    *conn.verify_token.lock() = verify_token;

    conn.send_packet(&EncryptionRequestPacket {
        server_id: String::new(),
        public_key: conn.key_store.public_key.clone(),
        verify_token: Box::new(verify_token),
        should_authenticate: true,
    });
}

async fn finish_login(client: Arc<Connection>, game_profile: GameProfile) {
    *client.game_profile.lock() = Some(game_profile.clone());
    enable_compression(&client).await;
    client.send_packet(&LoginSuccessPacket {
        game_profile,
        session_id: uuid::Uuid::new_v4(),
    });
}

async fn enable_compression(client: &Connection) {
    const COMPRESSION_THRESHOLD: i32 = 256;

    client
        .send_packet_now(&SetCompressionPacket {
            threshold: COMPRESSION_THRESHOLD,
        })
        .await;
    client.set_compression(COMPRESSION_THRESHOLD).await;
}

async fn handle_encryption_response(client: Arc<Connection>, packet: EncryptionResponsePacket) {
    let shared_secret = client.key_store.decrypt(&packet.shared_secret).unwrap();
    client.set_encryption(&shared_secret).await;

    let username = client.game_profile.lock().as_ref().unwrap().name.clone();
    let hash = client.key_store.digest_secret(&shared_secret);
    let game_profile = auth::authenthicate(&username, &hash, None).unwrap();

    finish_login(client, game_profile).await;
}

fn handle_login_acknowledged(client: Arc<Connection>, _packet: LoginAcknowledgePacket) {
    client.set_state(ProtocolState::Config);
}

fn handle_cookie_response(_client: Arc<Connection>) {}

async fn handle_login_plugin_response(client: Arc<Connection>, packet: LoginPluginResponsePacket) {
    if !matches!(client.server().auth_mode(), AuthMode::Velocity(_)) {
        return;
    }

    match parse_velocity_response(&client, packet) {
        Ok(game_profile) => finish_login(client, game_profile).await,
        Err(err) => {
            log::warn!("Velocity forwarding failed: {err:#}");
            client.kick("Failed to verify proxy connection.");
        }
    }
}
