use crate::core::domain::{
    errors::DomainError,
    queries::{BalanceQueryResult, GetBalanceQuery},
    services::{BlockchainService, GetBalanceQueryHandler, QueryHandler},
};
use async_trait::async_trait;
use std::sync::Arc;

/// Implementation of GetBalanceQueryHandler
pub struct GetBalanceHandler {
    blockchain_service: Arc<dyn BlockchainService>,
}

impl GetBalanceHandler {
    /// Create new GetBalanceHandler with a blockchain service
    pub fn new(blockchain_service: Arc<dyn BlockchainService>) -> Self {
        Self { blockchain_service }
    }
}

#[async_trait]
impl QueryHandler<GetBalanceQuery> for GetBalanceHandler {
    type Output = BalanceQueryResult;

    async fn handle(&self, query: GetBalanceQuery) -> Result<Self::Output, DomainError> {
        tracing::info!(
            "Querying {} balance for address {} on network {}",
            query.chain_type.name(),
            query.address,
            query.network.name()
        );

        tracing::debug!(
            "Chain details: currency={}, unit={}, decimals={}",
            query.chain_type.native_currency(),
            query.chain_type.smallest_unit(),
            query.chain_type.decimals()
        );

        // Get balance from blockchain service
        let balance = self.blockchain_service.get_balance(&query.address).await?;

        tracing::info!(
            "Balance query successful: {} has {} {} ({} {})",
            query.address,
            balance.to_wei(),
            query.chain_type.smallest_unit(),
            balance.to_ether(),
            query.chain_type.native_currency()
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
    use crate::core::domain::value_objects::{Address, Balance, ChainType, Network, TransactionHash};

    struct MockBlockchainService {
        balance: Balance,
    }

    #[async_trait]
    impl BlockchainService for MockBlockchainService {
        async fn get_balance(&self, _address: &Address) -> Result<Balance, DomainError> {
            Ok(self.balance)
        }

        async fn transfer(
            &self,
            _from: &Address,
            _to: &Address,
            _amount: u128,
            _private_key: &str,
        ) -> Result<TransactionHash, DomainError> {
            // Mock implementation for testing
            TransactionHash::new("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string())
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
            Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string()).unwrap(),
            Network::Mainnet,
        );

        let result = handler.handle(query).await;
        assert!(result.is_ok());

        let balance_result = result.unwrap();
        assert_eq!(balance_result.balance.to_ether(), 10.5);
        assert_eq!(balance_result.network, Network::Mainnet);
        assert_eq!(balance_result.chain_type, ChainType::Ethereum);
    }

    #[tokio::test]
    async fn test_get_balance_handler_with_chain_types() {
        use crate::core::domain::value_objects::ChainType;

        let mock_service = Arc::new(MockBlockchainService {
            balance: Balance::from_wei(100_000_000), // 1 BTC in satoshis
        });

        let handler = GetBalanceHandler::new(mock_service);

        // Test Bitcoin query
        let btc_query = GetBalanceQuery::new(
            Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()).unwrap(),
            Network::BitcoinMainnet,
        );

        let result = handler.handle(btc_query).await;
        assert!(result.is_ok());

        let balance_result = result.unwrap();
        assert_eq!(balance_result.chain_type, ChainType::Bitcoin);
        assert_eq!(balance_result.network, Network::BitcoinMainnet);
        assert_eq!(balance_result.balance.to_wei(), 100_000_000);

        // Verify chain type metadata
        assert_eq!(balance_result.chain_type.name(), "Bitcoin");
        assert_eq!(balance_result.chain_type.native_currency(), "BTC");
        assert_eq!(balance_result.chain_type.smallest_unit(), "Satoshi");
        assert_eq!(balance_result.chain_type.decimals(), 8);
    }
}
