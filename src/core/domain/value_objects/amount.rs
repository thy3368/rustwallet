use serde::{Deserialize, Serialize};
use std::fmt;

/// Transfer amount (in Wei, smallest unit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(u128);

impl Amount {
    /// Create zero amount
    pub fn zero() -> Self {
        Self(0)
    }

    /// Create amount from Wei
    pub fn from_wei(wei: u128) -> Self {
        Self(wei)
    }

    /// Create amount from Ether (floating point)
    pub fn from_ether(ether: f64) -> Self {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        Self((ether * WEI_PER_ETHER as f64) as u128)
    }

    /// Get amount in Wei
    pub fn to_wei(&self) -> u128 {
        self.0
    }

    /// Get amount in Ether (floating point)
    pub fn to_ether(&self) -> f64 {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        self.0 as f64 / WEI_PER_ETHER as f64
    }

    /// Check if amount is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Format amount as ETH/BNB string
    pub fn format_ether(&self, decimals: usize) -> String {
        format!("{:.prec$}", self.to_ether(), prec = decimals)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} Wei)", self.to_ether(), self.0)
    }
}

impl From<u128> for Amount {
    fn from(wei: u128) -> Self {
        Self::from_wei(wei)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_conversion() {
        let amount = Amount::from_ether(1.0);
        assert_eq!(amount.to_wei(), 1_000_000_000_000_000_000);
        assert_eq!(amount.to_ether(), 1.0);
    }

    #[test]
    fn test_zero_amount() {
        let amount = Amount::zero();
        assert!(amount.is_zero());
        assert_eq!(amount.to_wei(), 0);
    }
}
