use serde::{Deserialize, Serialize};
use std::fmt;
use super::ChainType;

/// Blockchain network types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    // EVM Networks
    /// Ethereum Mainnet
    Mainnet,
    /// Goerli Testnet (deprecated but still used)
    Goerli,
    /// Sepolia Testnet (recommended)
    Sepolia,
    /// Holesky Testnet
    Holesky,
    /// BSC Mainnet (Binance Smart Chain)
    BscMainnet,
    /// BSC Testnet
    BscTestnet,

    // Bitcoin Networks
    /// Bitcoin Mainnet
    BitcoinMainnet,
    /// Bitcoin Testnet
    BitcoinTestnet,

    // Solana Networks
    /// Solana Mainnet Beta
    SolanaMainnet,
    /// Solana Devnet
    SolanaDevnet,
    /// Solana Testnet
    SolanaTestnet,

    /// Custom network
    Custom { name: String, chain_id: u64, rpc_url: String },
}

impl Network {
    /// Get chain ID for the network (EVM networks only)
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => 1,
            Network::Goerli => 5,
            Network::Sepolia => 11155111,
            Network::Holesky => 17000,
            Network::BscMainnet => 56,
            Network::BscTestnet => 97,
            Network::BitcoinMainnet => 0, // Bitcoin doesn't use chain IDs
            Network::BitcoinTestnet => 0,
            Network::SolanaMainnet => 0, // Solana doesn't use chain IDs
            Network::SolanaDevnet => 0,
            Network::SolanaTestnet => 0,
            Network::Custom { chain_id, .. } => *chain_id,
        }
    }

    /// Get default RPC URL for the network
    pub fn default_rpc_url(&self) -> &str {
        match self {
            // EVM Networks
            Network::Mainnet => "https://eth.llamarpc.com",
            Network::Goerli => "https://goerli.infura.io/v3/",
            Network::Sepolia => "https://sepolia.infura.io/v3/",
            Network::Holesky => "https://holesky.infura.io/v3/",
            Network::BscMainnet => "https://bsc-dataseed.binance.org",
            Network::BscTestnet => "https://data-seed-prebsc-1-s1.binance.org:8545",

            // Bitcoin Networks (use blockchain.info API)
            Network::BitcoinMainnet => "https://blockchain.info",
            Network::BitcoinTestnet => "https://testnet.blockchain.info",

            // Solana Networks
            Network::SolanaMainnet => "https://api.mainnet-beta.solana.com",
            Network::SolanaDevnet => "https://api.devnet.solana.com",
            Network::SolanaTestnet => "https://api.testnet.solana.com",

            Network::Custom { rpc_url, .. } => rpc_url,
        }
    }

    /// Get network name
    pub fn name(&self) -> &str {
        match self {
            // EVM Networks
            Network::Mainnet => "Ethereum Mainnet",
            Network::Goerli => "Goerli Testnet",
            Network::Sepolia => "Sepolia Testnet",
            Network::Holesky => "Holesky Testnet",
            Network::BscMainnet => "BSC Mainnet",
            Network::BscTestnet => "BSC Testnet",

            // Bitcoin Networks
            Network::BitcoinMainnet => "Bitcoin Mainnet",
            Network::BitcoinTestnet => "Bitcoin Testnet",

            // Solana Networks
            Network::SolanaMainnet => "Solana Mainnet",
            Network::SolanaDevnet => "Solana Devnet",
            Network::SolanaTestnet => "Solana Testnet",

            Network::Custom { name, .. } => name,
        }
    }

    /// Check if this is a testnet
    pub fn is_testnet(&self) -> bool {
        !matches!(
            self,
            Network::Mainnet | Network::BscMainnet | Network::BitcoinMainnet | Network::SolanaMainnet
        )
    }

    /// Check if this is an EVM network
    pub fn is_evm(&self) -> bool {
        matches!(
            self,
            Network::Mainnet
                | Network::Goerli
                | Network::Sepolia
                | Network::Holesky
                | Network::BscMainnet
                | Network::BscTestnet
        )
    }

    /// Check if this is a BSC network
    pub fn is_bsc(&self) -> bool {
        matches!(self, Network::BscMainnet | Network::BscTestnet)
    }

    /// Check if this is a Bitcoin network
    pub fn is_bitcoin(&self) -> bool {
        matches!(self, Network::BitcoinMainnet | Network::BitcoinTestnet)
    }

    /// Check if this is a Solana network
    pub fn is_solana(&self) -> bool {
        matches!(
            self,
            Network::SolanaMainnet | Network::SolanaDevnet | Network::SolanaTestnet
        )
    }

    /// Get the chain type for this network
    pub fn chain_type(&self) -> ChainType {
        if self.is_bitcoin() {
            ChainType::Bitcoin
        } else if self.is_solana() {
            ChainType::Solana
        } else {
            // Default to Ethereum for EVM networks and custom networks
            ChainType::Ethereum
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_evm() {
            write!(f, "{} (Chain ID: {})", self.name(), self.chain_id())
        } else {
            write!(f, "{}", self.name())
        }
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
