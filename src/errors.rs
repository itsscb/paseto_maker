use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClaimError {
    #[error("invalid value")]
    InvalidValue,
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("paseto error: {0}")]
    PasetoError(#[from] rusty_paseto::generic::PasetoError),
    #[error("invalid claim: {0}")]
    InvalidClaim(#[from] rusty_paseto::generic::PasetoClaimError),
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Invalid claim: {0}")]
    InvalidClaim(String),
    #[error("Token expired")]
    Expired,
    #[error("Token not valid")]
    Invalid,
    #[error("Token validation failed")]
    Validation,
    #[error("Token malformed")]
    Format,
    #[error("Claim error: {0}")]
    ClaimError(#[from] ClaimError),
    #[error("Token creation failed: {0}")]
    TokenCreationFailed(String),
}

#[derive(Error, Debug)]
pub enum MakerError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}
