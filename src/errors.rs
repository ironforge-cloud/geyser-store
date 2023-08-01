use thiserror::Error;

pub type GeyserStoreResult<T> = Result<T, GeyserStoreError>;

#[derive(Error, Debug)]
pub enum GeyserStoreError {
    #[error("SqliteError ({0})")]
    SqliteError(#[from] rusqlite::Error),

    #[error("ConvertInfallible ({0})")]
    ConvertInfallibleError(#[from] std::convert::Infallible),

    #[error("Base64DecodeError ({0})")]
    Base64DecodeError(#[from] base64::DecodeError),
}
