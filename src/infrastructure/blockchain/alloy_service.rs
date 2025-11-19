use async_trait::async_trait;
use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::{Client, Http},
};
use crate::domain::{
    errors::DomainError,
    services::BlockchainService,
    value_objects::{Address, Balance, Network},
};

/// Alloy-based Ethereum blockchain service implementation
pub struct AlloyBlockchainService {
    provider: RootProvider<Http<Client>>,
    network: Network,
}

impl AlloyBlockchainService {
    /// Create new Alloy blockchain service
    pub async fn new(network: Network, rpc_url: &str) -> Result<Self, DomainError> {
        let provider = ProviderBuilder::new()
            .on_http(rpc_url.parse().map_err(|e| {
                DomainError::ConfigurationError(format!("Invalid RPC URL: {}", e))
            })?);

        Ok(Self { provider, network })
    }

    /// Create service with default RPC URL for network
    pub async fn new_with_default_rpc(network: Network) -> Result<Self, DomainError> {
        let rpc_url = network.default_rpc_url().to_string();
        Self::new(network, &rpc_url).await
    }

    /// Get the network this service is connected to
    pub fn network(&self) -> &Network {
        &self.network
    }
}

#[async_trait]
impl BlockchainService for AlloyBlockchainService {
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError> {
        // Parse the address string into Alloy's Address type
        let alloy_address: alloy::primitives::Address = address
            .as_str()
            .parse()
            .map_err(|e| DomainError::BlockchainError(format!("Invalid address: {}", e)))?;

        // Get balance from the blockchain
        let balance_wei = self
            .provider
            .get_balance(alloy_address)
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to get balance: {}", e)))?;

        // Convert U256 to u128 (will panic if balance > u128::MAX, which is extremely unlikely)
        let balance_u128 = balance_wei.to::<u128>();

        Ok(Balance::from_wei(balance_u128))
    }

    async fn is_connected(&self) -> bool {
        self.provider.get_block_number().await.is_ok()
    }

    async fn get_block_number(&self) -> Result<u64, DomainError> {
        self.provider
            .get_block_number()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to get block number: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignore by default as it requires network connection
    async fn test_get_balance_real_network() {
        // Vitalik's address for testing
        let address = Address::new("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string())
            .expect("Valid address");

        let service = AlloyBlockchainService::new_with_default_rpc(Network::Mainnet)
            .await
            .expect("Failed to create service");

        let balance = service.get_balance(&address).await;
        assert!(balance.is_ok());
        println!("Balance: {:?}", balance);
    }
}
