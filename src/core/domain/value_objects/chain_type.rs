use std::fmt;
use serde::{Deserialize, Serialize};

/// Blockchain type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    /// Ethereum and EVM-compatible chains (Ethereum, BSC, Polygon, etc.)
    Ethereum,
    /// Bitcoin and Bitcoin-based chains
    Bitcoin,
    /// Solana
    Solana,
}

impl ChainType {
    /// Get the name of the chain type
    pub fn name(&self) -> &'static str {
        match self {
            ChainType::Ethereum => "Ethereum",
            ChainType::Bitcoin => "Bitcoin",
            ChainType::Solana => "Solana",
        }
    }

    /// Get the native currency symbol
    pub fn native_currency(&self) -> &'static str {
        match self {
            ChainType::Ethereum => "ETH",
            ChainType::Bitcoin => "BTC",
            ChainType::Solana => "SOL",
        }
    }

    /// Get the smallest unit name
    pub fn smallest_unit(&self) -> &'static str {
        match self {
            ChainType::Ethereum => "Wei",
            ChainType::Bitcoin => "Satoshi",
            ChainType::Solana => "Lamport",
        }
    }

    /// Get the decimals for the native currency
    pub fn decimals(&self) -> u8 {
        match self {
            ChainType::Ethereum => 18, // 1 ETH = 10^18 Wei
            ChainType::Bitcoin => 8,   // 1 BTC = 10^8 Satoshi
            ChainType::Solana => 9,    // 1 SOL = 10^9 Lamport
        }
    }
}

impl fmt::Display for ChainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_type_properties() {
        assert_eq!(ChainType::Ethereum.name(), "Ethereum");
        assert_eq!(ChainType::Ethereum.native_currency(), "ETH");
        assert_eq!(ChainType::Ethereum.smallest_unit(), "Wei");
        assert_eq!(ChainType::Ethereum.decimals(), 18);

        assert_eq!(ChainType::Bitcoin.name(), "Bitcoin");
        assert_eq!(ChainType::Bitcoin.native_currency(), "BTC");
        assert_eq!(ChainType::Bitcoin.smallest_unit(), "Satoshi");
        assert_eq!(ChainType::Bitcoin.decimals(), 8);

        assert_eq!(ChainType::Solana.name(), "Solana");
        assert_eq!(ChainType::Solana.native_currency(), "SOL");
        assert_eq!(ChainType::Solana.smallest_unit(), "Lamport");
        assert_eq!(ChainType::Solana.decimals(), 9);
    }

    #[test]
    fn test_chain_type_display() {
        assert_eq!(format!("{}", ChainType::Ethereum), "Ethereum");
        assert_eq!(format!("{}", ChainType::Bitcoin), "Bitcoin");
        assert_eq!(format!("{}", ChainType::Solana), "Solana");
    }
}
