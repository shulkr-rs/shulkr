mod error;
mod key_store;
mod profile;

pub use error::Error;
pub use key_store::KeyStore;
pub use profile::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AuthMode {
    #[default]
    Online,
    Offline,
    Velocity(String),
}

pub type Decryptor = cfb8::Decryptor<aes::Aes128>;
pub type Encryptor = cfb8::Encryptor<aes::Aes128>;

const MOJANG_AUTH_URL: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hash}";

pub fn authenthicate(
    username: &str,
    hash: &str,
    ip: Option<std::net::Ipv4Addr>,
) -> Result<GameProfile, Error> {
    let url = if ip.is_some() {
        todo!()
    } else {
        MOJANG_AUTH_URL
            .replace("{username}", username)
            .replace("{hash}", hash)
    };

    let mut response = ureq::get(url).call().map_err(|_| Error::FailedRequest)?;

    response
        .body_mut()
        .read_json::<GameProfile>()
        .map_err(|_| Error::MalformedJson)
}
