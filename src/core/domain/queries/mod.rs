use crate::core::domain::value_objects::{Address, Balance, ChainType, Network};
use serde::{Deserialize, Serialize};

/// Query to get balance of a blockchain address
/// Supports multiple chains: Ethereum (EVM), Bitcoin, and Solana
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceQuery {
    /// The blockchain address to query
    pub address: Address,
    /// The network to query on
    pub network: Network,
    /// The blockchain type (Ethereum/Bitcoin/Solana)
    pub chain_type: ChainType,
}

impl GetBalanceQuery {
    /// Create a new get balance query
    /// The chain_type is automatically derived from the network
    pub fn new(address: Address, network: Network) -> Self {
        let chain_type = network.chain_type();
        Self {
            address,
            network,
            chain_type,
        }
    }

    /// Create a get balance query with explicit chain type
    /// Use this for custom validation or testing
    pub fn new_with_chain_type(
        address: Address,
        network: Network,
        chain_type: ChainType,
    ) -> Self {
        Self {
            address,
            network,
            chain_type,
        }
    }
}

/// Result of balance query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceQueryResult {
    /// The queried address
    pub address: Address,
    /// The network queried
    pub network: Network,
    /// The blockchain type
    pub chain_type: ChainType,
    /// The current balance
    pub balance: Balance,
}

impl BalanceQueryResult {
    pub fn new(address: Address, network: Network, balance: Balance) -> Self {
        let chain_type = network.chain_type();
        Self {
            address,
            network,
            chain_type,
            balance,
        }
    }

    /// Create a new result with explicit chain type
    pub fn new_with_chain_type(
        address: Address,
        network: Network,
        chain_type: ChainType,
        balance: Balance,
    ) -> Self {
        Self {
            address,
            network,
            chain_type,
            balance,
        }
    }
}
