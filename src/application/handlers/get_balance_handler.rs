use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{
    errors::DomainError,
    queries::{BalanceQueryResult, GetBalanceQuery},
    services::{BlockchainService, GetBalanceQueryHandler, QueryHandler},
};

/// Implementation of GetBalanceQueryHandler
pub struct GetBalanceHandler {
    blockchain_service: Arc<dyn BlockchainService>,
}

impl GetBalanceHandler {
    /// Create new GetBalanceHandler with a blockchain service
    pub fn new(blockchain_service: Arc<dyn BlockchainService>) -> Self {
        Self {
            blockchain_service,
        }
    }
}

#[async_trait]
impl QueryHandler<GetBalanceQuery> for GetBalanceHandler {
    type Output = BalanceQueryResult;

    async fn handle(&self, query: GetBalanceQuery) -> Result<Self::Output, DomainError> {
        tracing::info!(
            "Querying balance for address {} on network {}",
            query.address,
            query.network.name()
        );

        // Get balance from blockchain service
        let balance = self.blockchain_service.get_balance(&query.address).await?;

        tracing::info!(
            "Balance query successful: {} has {}",
            query.address,
            balance
        );

        // Return result
        Ok(BalanceQueryResult::new(
            query.address,
            query.network,
            balance,
        ))
    }
}

#[async_trait]
impl GetBalanceQueryHandler for GetBalanceHandler {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Address, Balance, Network};

    struct MockBlockchainService {
        balance: Balance,
    }

    #[async_trait]
    impl BlockchainService for MockBlockchainService {
        async fn get_balance(&self, _address: &Address) -> Result<Balance, DomainError> {
            Ok(self.balance)
        }

        async fn is_connected(&self) -> bool {
            true
        }

        async fn get_block_number(&self) -> Result<u64, DomainError> {
            Ok(12345)
        }
    }

    #[tokio::test]
    async fn test_get_balance_handler() {
        let mock_service = Arc::new(MockBlockchainService {
            balance: Balance::from_ether(10.5),
        });

        let handler = GetBalanceHandler::new(mock_service);

        let query = GetBalanceQuery::new(
            Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string()).unwrap(),
            Network::Mainnet,
        );

        let result = handler.handle(query).await;
        assert!(result.is_ok());

        let balance_result = result.unwrap();
        assert_eq!(balance_result.balance.to_ether(), 10.5);
    }
}
