use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),
    #[error("Authentication failed")]
    AuthError,
    #[error("Invalid request")]
    ValidationError,
    #[error("Method not found")]
    MethodNotFound,
    #[error("Invalid parameters")]
    InvalidParams,
}