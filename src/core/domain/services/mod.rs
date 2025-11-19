use async_trait::async_trait;
use crate::core::domain::{
    errors::DomainError,
    queries::{BalanceQueryResult, GetBalanceQuery},
    value_objects::{Address, Balance, TransactionHash},
};

/// Query handler trait - processes read operations
#[async_trait]
pub trait QueryHandler<Q>: Send + Sync {
    type Output;

    async fn handle(&self, query: Q) -> Result<Self::Output, DomainError>;
}

/// Blockchain service interface for Ethereum/BSC operations
#[async_trait]
pub trait BlockchainService: Send + Sync {
    /// Get balance of an address
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError>;

    /// Transfer funds from one address to another
    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u128,
        private_key: &str,
    ) -> Result<TransactionHash, DomainError>;

    /// Check if connected to the network
    async fn is_connected(&self) -> bool;

    /// Get current block number
    async fn get_block_number(&self) -> Result<u64, DomainError>;
}

/// Get balance query handler
#[async_trait]
pub trait GetBalanceQueryHandler: QueryHandler<GetBalanceQuery, Output = BalanceQueryResult> {}
