use crate::auth::Error;
use openssl::{
    pkey::{PKey, Private},
    rsa::{Padding, Rsa},
};

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

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0u8; self.private_key.size() as usize];

        let len = self
            .private_key
            .private_decrypt(data, &mut buf, Padding::PKCS1)
            .map_err(|_| Error::DecryptionError)?;
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
