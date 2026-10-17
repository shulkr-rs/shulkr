#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to decrypt secret")]
    DecryptionError,
    #[error("Failed to connect to Mojang session server")]
    FailedRequest,
    #[error("Failed to parse profile json")]
    MalformedJson,
}
