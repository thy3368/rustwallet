use serde::{Deserialize, Serialize};
use std::fmt;

/// Balance (in Wei, smallest unit)
/// 1 ETH = 1,000,000,000,000,000,000 Wei
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Balance(u128);

impl Balance {
    /// Create zero balance
    pub fn zero() -> Self {
        Self(0)
    }

    /// Create balance from Wei
    pub fn from_wei(wei: u128) -> Self {
        Self(wei)
    }

    /// Create balance from Ether (floating point)
    pub fn from_ether(ether: f64) -> Self {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        Self((ether * WEI_PER_ETHER as f64) as u128)
    }

    /// Get balance in Wei
    pub fn to_wei(&self) -> u128 {
        self.0
    }

    /// Get balance in Ether (floating point)
    pub fn to_ether(&self) -> f64 {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        self.0 as f64 / WEI_PER_ETHER as f64
    }

    /// Check if balance is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Format balance as ETH string with specified decimal places
    pub fn format_ether(&self, decimals: usize) -> String {
        format!("{:.prec$} ETH", self.to_ether(), prec = decimals)
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ETH ({} Wei)", self.to_ether(), self.0)
    }
}

impl From<u128> for Balance {
    fn from(wei: u128) -> Self {
        Self::from_wei(wei)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_conversion() {
        let balance = Balance::from_ether(1.0);
        assert_eq!(balance.to_wei(), 1_000_000_000_000_000_000);
        assert_eq!(balance.to_ether(), 1.0);
    }

    #[test]
    fn test_zero_balance() {
        let balance = Balance::zero();
        assert!(balance.is_zero());
        assert_eq!(balance.to_wei(), 0);
    }

    #[test]
    fn test_balance_display() {
        let balance = Balance::from_ether(2.5);
        let display = format!("{}", balance);
        assert!(display.contains("2.5"));
    }
}
