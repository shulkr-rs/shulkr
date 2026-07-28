use crate::{
    auth::{AuthMode, GameProfile, Property},
    network::client::Connection,
    protocol::{
        decode::PacketRead as _,
        packet::{LoginPluginRequestPacket, LoginPluginResponsePacket},
    },
    util::Identifier,
};

const VELOCITY_FORWARDING_VERSION: i32 = 1;
const VELOCITY_PLAYER_INFO_ID: i32 = 1;
const VELOCITY_CHANNEL: Identifier = Identifier::const_new("velocity", "player_info");

pub(super) fn login_velocity(conn: &Connection) {
    conn.send_packet(&LoginPluginRequestPacket {
        message_id: VELOCITY_PLAYER_INFO_ID,
        channel: VELOCITY_CHANNEL,
        data: Box::new([VELOCITY_FORWARDING_VERSION as u8]),
    });
}

pub(crate) fn parse_velocity_response(
    client: &Connection,
    packet: LoginPluginResponsePacket,
) -> anyhow::Result<GameProfile> {
    use anyhow::{Context as _, anyhow, ensure};
    use hmac::{Hmac, KeyInit as _, Mac};
    use sha2::Sha256;

    let secret = match client.server().auth_mode() {
        AuthMode::Velocity(secret) => secret,
        _ => {
            return Err(anyhow!(
                "received velocity response while not in velocity mode"
            ));
        }
    };

    ensure!(
        packet.message_id == VELOCITY_PLAYER_INFO_ID,
        "unexpected plugin response message id: {}",
        packet.message_id
    );

    let data = packet
        .data
        .context("login plugin response had no data (not behind a proxy?)")?;
    ensure!(data.len() >= 32, "velocity response too short");

    let (signature, mut payload) = data.split_at(32);

    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC accepts keys of any length");
    mac.update(payload);
    mac.verify_slice(signature)
        .map_err(|_| anyhow!("velocity forwarding signature mismatch"))?;

    let version = payload.read_varint()?;
    ensure!(
        version == VELOCITY_FORWARDING_VERSION,
        "unsupported velocity forwarding version: {version}"
    );

    let _client_addr = payload.read_string()?;
    let uuid = payload.read_uuid()?;
    let name = payload.read_string()?;
    let properties = payload.read_array(|r| {
        Ok(Property {
            name: r.read_string()?,
            value: r.read_string()?,
            signature: r.read_option(|r| r.read_string())?,
        })
    })?;

    if let Some(expected) = client.game_profile.lock().as_ref() {
        ensure!(expected.name == name, "mismatched usernames");
    }

    Ok(GameProfile {
        uuid,
        name,
        properties,
    })
}
