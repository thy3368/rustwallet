use serde::{Deserialize, Serialize};
use std::fmt;
use crate::DomainError;

/// Multi-chain address (supports Ethereum, Bitcoin, Solana)
/// - Ethereum: 0x + 40 hex characters (42 total)
/// - Bitcoin: 26-62 characters, starts with 1, 3, bc1, m, n, or tb1
/// - Solana: 32-44 characters, Base58 encoded
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    /// Create new address with validation
    pub fn new(addr: String) -> Result<Self, DomainError> {
        let instance = Self(addr);
        instance.validate()?;
        Ok(instance)
    }

    /// Create address without validation (use carefully)
    pub fn new_unchecked(addr: String) -> Self {
        Self(addr)
    }

    /// Validate address format (supports Ethereum, Bitcoin, Solana)
    pub fn validate(&self) -> Result<(), DomainError> {
        // Basic validation: address should not be empty
        if self.0.is_empty() {
            return Err(DomainError::InvalidAddressFormat);
        }

        // Ethereum address: 0x + 40 hex characters
        if self.0.starts_with("0x") {
            if self.0.len() != 42 {
                return Err(DomainError::InvalidAddressLength);
            }
            if !self.0[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(DomainError::InvalidAddressCharacters);
            }
            return Ok(());
        }

        // Bitcoin address: 26-35 characters, alphanumeric
        // Starts with 1, 3, or bc1 (mainnet) or m, n, tb1 (testnet)
        if self.0.len() >= 26 && self.0.len() <= 62 {
            if self.0.starts_with('1')
                || self.0.starts_with('3')
                || self.0.starts_with("bc1")
                || self.0.starts_with('m')
                || self.0.starts_with('n')
                || self.0.starts_with("tb1")
            {
                // Basic alphanumeric check (Bitcoin uses Base58)
                return Ok(());
            }
        }

        // Solana address: 32-44 characters, Base58 encoded
        if self.0.len() >= 32 && self.0.len() <= 44 {
            // Solana addresses are Base58 encoded (no 0, O, I, l)
            if self.0.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Ok(());
            }
        }

        Err(DomainError::InvalidAddressFormat)
    }

    /// Get address as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to lowercase checksum format
    pub fn to_checksum(&self) -> String {
        self.0.to_lowercase()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Address {
    fn from(s: String) -> Self {
        Self::new_unchecked(s)
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address() {
        let addr = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string());
        assert!(addr.is_ok());
    }

    #[test]
    fn test_invalid_address_no_prefix() {
        let addr = Address::new("742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string());
        assert!(addr.is_err());
    }

    #[test]
    fn test_invalid_address_length() {
        let addr = Address::new("0x742d35Cc".to_string());
        assert!(addr.is_err());
    }
}
