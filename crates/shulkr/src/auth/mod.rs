mod profile;
use openssl::{
    pkey::{PKey, Private},
    rsa::{Padding, Rsa},
};
pub use profile::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AuthMode {
    #[default]
    Online,
    Offline,
    Velocity(String),
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Failed to decrypt secret")]
    DecryptionError,
    #[error("Failed to connect to Mojang session server")]
    FailedRequest,
    #[error("Failed to parse profile json")]
    MalformedJson,
}

#[derive(Debug)]
pub struct KeyStore {
    pub private_key: Rsa<Private>,
    pub public_key: Box<[u8]>,
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyStore {
    pub fn new() -> Self {
        let private_key = Rsa::generate(1024).unwrap();

        let public_key = PKey::from_rsa(private_key.clone())
            .unwrap()
            .public_key_to_der()
            .unwrap()
            .into_boxed_slice();

        Self {
            private_key,
            public_key,
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, AuthError> {
        let mut buf = vec![0u8; self.private_key.size() as usize];

        let len = self
            .private_key
            .private_decrypt(data, &mut buf, Padding::PKCS1)
            .map_err(|_| AuthError::DecryptionError)?;
        buf.truncate(len);

        Ok(buf)
    }

    pub fn digest_secret(&self, secret: &[u8]) -> String {
        use sha1::{Digest as _, Sha1};

        num_bigint::BigInt::from_signed_bytes_be(
            &Sha1::new()
                .chain_update(secret)
                .chain_update(&self.public_key)
                .finalize(),
        )
        .to_str_radix(16)
    }
}

pub type Decryptor = cfb8::Decryptor<aes::Aes128>;
pub type Encryptor = cfb8::Encryptor<aes::Aes128>;

const MOJANG_AUTH_URL: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hash}";

pub fn authenthicate(
    username: &str,
    hash: &str,
    ip: Option<std::net::Ipv4Addr>,
) -> Result<GameProfile, AuthError> {
    let url = if ip.is_some() {
        todo!()
    } else {
        MOJANG_AUTH_URL
            .replace("{username}", username)
            .replace("{hash}", hash)
    };

    let mut response = ureq::get(url)
        .call()
        .map_err(|_| AuthError::FailedRequest)?;

    response
        .body_mut()
        .read_json::<GameProfile>()
        .map_err(|_| AuthError::MalformedJson)
}
