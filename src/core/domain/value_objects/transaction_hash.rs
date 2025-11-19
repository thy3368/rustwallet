use serde::{Deserialize, Serialize};
use std::fmt;

/// Transaction hash (66 characters, starts with 0x)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionHash(String);

impl TransactionHash {
    /// Create new transaction hash with validation
    pub fn new(hash: String) -> Result<Self, crate::core::domain::errors::DomainError> {
        let instance = Self(hash);
        instance.validate()?;
        Ok(instance)
    }

    /// Create transaction hash without validation (use carefully)
    pub fn new_unchecked(hash: String) -> Self {
        Self(hash)
    }

    /// Validate transaction hash format
    pub fn validate(&self) -> Result<(), crate::core::domain::errors::DomainError> {
        use crate::core::domain::errors::DomainError;

        if !self.0.starts_with("0x") {
            return Err(DomainError::InvalidTransactionHash);
        }
        if self.0.len() != 66 {
            return Err(DomainError::InvalidTransactionHashLength);
        }
        // Check hex characters
        if !self.0[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidTransactionHashCharacters);
        }
        Ok(())
    }

    /// Get transaction hash as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TransactionHash {
    fn from(s: String) -> Self {
        Self::new_unchecked(s)
    }
}

impl AsRef<str> for TransactionHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tx_hash() {
        let hash = TransactionHash::new(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        assert!(hash.is_ok());
    }

    #[test]
    fn test_invalid_tx_hash_length() {
        let hash = TransactionHash::new("0x1234".to_string());
        assert!(hash.is_err());
    }

    #[test]
    fn test_invalid_tx_hash_no_prefix() {
        let hash = TransactionHash::new(
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        );
        assert!(hash.is_err());
    }
}
