use async_trait::async_trait;
use crate::core::domain::{
    errors::DomainError,
    queries::{BalanceQueryResult, GetBalanceQuery},
    value_objects::{Address, Balance, TransactionHash},
};

/// Query handler trait - processes read operations (CQRS Query)
#[async_trait]
pub trait QueryHandler<Q>: Send + Sync {
    type Output;

    async fn handle(&self, query: Q) -> Result<Self::Output, DomainError>;
}

/// Command handler trait - processes write operations (CQRS Command)
#[async_trait]
pub trait CommandHandler<C>: Send + Sync {
    type Output;

    async fn handle(&self, command: C) -> Result<Self::Output, DomainError>;
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

/// Get balance query handler (Query side of CQRS)
#[async_trait]
pub trait GetBalanceQueryHandler: QueryHandler<GetBalanceQuery, Output = BalanceQueryResult> {}

/// Transfer command handler trait (Command side of CQRS)
#[async_trait]
pub trait TransferCommandHandler: CommandHandler<crate::core::domain::commands::TransferCommand, Output = crate::core::domain::commands::TransferResult> {}
