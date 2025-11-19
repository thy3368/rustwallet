use crate::domain::value_objects::{Address, Balance, Network};
use serde::{Deserialize, Serialize};

/// Query to get balance of an Ethereum address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceQuery {
    /// The Ethereum address to query
    pub address: Address,
    /// The network to query on
    pub network: Network,
}

impl GetBalanceQuery {
    /// Create a new get balance query
    pub fn new(address: Address, network: Network) -> Self {
        Self { address, network }
    }
}

/// Result of balance query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceQueryResult {
    /// The queried address
    pub address: Address,
    /// The network queried
    pub network: Network,
    /// The current balance
    pub balance: Balance,
}

impl BalanceQueryResult {
    pub fn new(address: Address, network: Network, balance: Balance) -> Self {
        Self {
            address,
            network,
            balance,
        }
    }
}
