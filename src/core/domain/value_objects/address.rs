use serde::{Deserialize, Serialize};
use std::fmt;
use crate::DomainError;

/// Ethereum address (42 characters, starts with 0x)
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

    /// Validate Ethereum address format
    pub fn validate(&self) -> Result<(), DomainError> {

        if !self.0.starts_with("0x") {
            return Err(DomainError::InvalidAddressFormat);
        }
        if self.0.len() != 42 {
            return Err(DomainError::InvalidAddressLength);
        }
        // Check hex characters
        if !self.0[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidAddressCharacters);
        }
        Ok(())
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
        let addr = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string());
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
