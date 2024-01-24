use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleError {
    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Data serialization error")]
    SerializationError,

    #[error("Data load error: {0}")]
    DataLoadError(String),

    #[error("Data deserialization error")]
    DeserializationError,

    #[error("Invalid pricing record data")]
    InvalidPricingRecord,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Timestamp is too far in the future")]
    TimestampTooFarInFuture,

    #[error("Timestamp is too old")]
    TimestampTooOld,

    #[error("Missing rates in pricing record")]
    MissingRates,
}
