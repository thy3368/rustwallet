use serde::{Deserialize, Serialize};
use std::fmt;

/// Ethereum network types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    /// Ethereum Mainnet
    Mainnet,
    /// Goerli Testnet (deprecated but still used)
    Goerli,
    /// Sepolia Testnet (recommended)
    Sepolia,
    /// Holesky Testnet
    Holesky,
    /// Custom network
    Custom { name: String, chain_id: u64, rpc_url: String },
}

impl Network {
    /// Get chain ID for the network
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => 1,
            Network::Goerli => 5,
            Network::Sepolia => 11155111,
            Network::Holesky => 17000,
            Network::Custom { chain_id, .. } => *chain_id,
        }
    }

    /// Get default RPC URL for the network
    pub fn default_rpc_url(&self) -> &str {
        match self {
            Network::Mainnet => "https://eth.llamarpc.com",
            Network::Goerli => "https://goerli.infura.io/v3/",
            Network::Sepolia => "https://sepolia.infura.io/v3/",
            Network::Holesky => "https://holesky.infura.io/v3/",
            Network::Custom { rpc_url, .. } => rpc_url,
        }
    }

    /// Get network name
    pub fn name(&self) -> &str {
        match self {
            Network::Mainnet => "Mainnet",
            Network::Goerli => "Goerli",
            Network::Sepolia => "Sepolia",
            Network::Holesky => "Holesky",
            Network::Custom { name, .. } => name,
        }
    }

    /// Check if this is a testnet
    pub fn is_testnet(&self) -> bool {
        !matches!(self, Network::Mainnet)
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Chain ID: {})", self.name(), self.chain_id())
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Sepolia // Default to Sepolia testnet for safety
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_chain_ids() {
        assert_eq!(Network::Mainnet.chain_id(), 1);
        assert_eq!(Network::Sepolia.chain_id(), 11155111);
    }

    #[test]
    fn test_network_is_testnet() {
        assert!(!Network::Mainnet.is_testnet());
        assert!(Network::Sepolia.is_testnet());
    }
}
