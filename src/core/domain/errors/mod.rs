use thiserror::Error;

/// Domain layer errors
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid address format - must start with 0x")]
    InvalidAddressFormat,

    #[error("Invalid address length - must be 42 characters")]
    InvalidAddressLength,

    #[error("Invalid address characters - must be hexadecimal")]
    InvalidAddressCharacters,

    #[error("Invalid balance")]
    InvalidBalance,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid transaction hash")]
    InvalidTransactionHash,

    #[error("Invalid transaction hash length - must be 66 characters")]
    InvalidTransactionHashLength,

    #[error("Invalid transaction hash characters - must be hexadecimal")]
    InvalidTransactionHashCharacters,

    #[error("Insufficient balance for transfer")]
    InsufficientBalance,

    #[error("Invalid amount - must be greater than zero")]
    InvalidAmount,

    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    #[error("Invalid private key")]
    InvalidPrivateKey,
}

/// Blockchain service errors
#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl From<BlockchainError> for DomainError {
    fn from(err: BlockchainError) -> Self {
        DomainError::BlockchainError(err.to_string())
    }
}
