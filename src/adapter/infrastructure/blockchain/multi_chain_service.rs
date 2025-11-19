use async_trait::async_trait;
use std::sync::Arc;
use crate::core::domain::{
    errors::DomainError,
    services::BlockchainService,
    value_objects::{Address, Balance, ChainType, Network, TransactionHash},
};
use super::{AlloyBlockchainService, BitcoinBlockchainService, SolanaBlockchainService};

/// Multi-chain blockchain service that routes requests to the appropriate chain-specific service
///
/// This service acts as a facade that automatically routes blockchain operations to the correct
/// underlying service (Ethereum/Bitcoin/Solana) based on the network's chain type.
///
/// # Architecture
///
/// ```text
/// ┌─────────────────────────────────────┐
/// │  MultiChainBlockchainService        │
/// │  (Facade Pattern)                   │
/// └──────────────┬──────────────────────┘
///                │
///                ├─── route_service(network) ───┐
///                │                              │
///      ┌─────────┼──────────────┬───────────────┤
///      │         │              │               │
///      ▼         ▼              ▼               ▼
/// ┌────────┐ ┌────────┐ ┌──────────┐ ┌─────────────┐
/// │ Alloy  │ │Bitcoin │ │  Solana  │ │   Cache     │
/// │Service │ │Service │ │ Service  │ │  (Future)   │
/// └────────┘ └────────┘ └──────────┘ └─────────────┘
/// ```
pub struct MultiChainBlockchainService {
    /// Ethereum/EVM service (Alloy-based)
    evm_service: Option<Arc<AlloyBlockchainService>>,
    /// Bitcoin service
    bitcoin_service: Option<Arc<BitcoinBlockchainService>>,
    /// Solana service
    solana_service: Option<Arc<SolanaBlockchainService>>,
    /// Current network context (if set)
    current_network: Option<Network>,
}

impl MultiChainBlockchainService {
    /// Create a new multi-chain service with all services initialized
    pub async fn new() -> Result<Self, DomainError> {
        Ok(Self {
            evm_service: None,
            bitcoin_service: None,
            solana_service: None,
            current_network: None,
        })
    }

    /// Create a multi-chain service for a specific network
    ///
    /// This will only initialize the service for the given network's chain type,
    /// saving resources when you know you'll only use one chain.
    pub async fn new_for_network(network: Network) -> Result<Self, DomainError> {
        let mut service = Self::new().await?;
        service.initialize_for_network(&network).await?;
        service.current_network = Some(network);
        Ok(service)
    }

    /// Initialize services for all supported chains
    pub async fn initialize_all(&mut self) -> Result<(), DomainError> {
        // Initialize Ethereum service (default to Mainnet)
        self.evm_service = Some(Arc::new(
            AlloyBlockchainService::new_with_default_rpc(Network::Mainnet).await?
        ));

        // Initialize Bitcoin service
        self.bitcoin_service = Some(Arc::new(
            BitcoinBlockchainService::new(Network::BitcoinMainnet).await?
        ));

        // Initialize Solana service
        self.solana_service = Some(Arc::new(
            SolanaBlockchainService::new(Network::SolanaMainnet).await?
        ));

        Ok(())
    }

    /// Initialize service for a specific network
    pub async fn initialize_for_network(&mut self, network: &Network) -> Result<(), DomainError> {
        match network.chain_type() {
            ChainType::Ethereum => {
                if self.evm_service.is_none() {
                    self.evm_service = Some(Arc::new(
                        AlloyBlockchainService::new_with_default_rpc(network.clone()).await?
                    ));
                }
            }
            ChainType::Bitcoin => {
                if self.bitcoin_service.is_none() {
                    self.bitcoin_service = Some(Arc::new(
                        BitcoinBlockchainService::new(network.clone()).await?
                    ));
                }
            }
            ChainType::Solana => {
                if self.solana_service.is_none() {
                    self.solana_service = Some(Arc::new(
                        SolanaBlockchainService::new(network.clone()).await?
                    ));
                }
            }
        }
        Ok(())
    }

    /// Get the appropriate service for a given network
    fn get_service_for_network(&self, network: &Network) -> Result<Arc<dyn BlockchainService>, DomainError> {
        match network.chain_type() {
            ChainType::Ethereum => {
                self.evm_service
                    .as_ref()
                    .map(|s| s.clone() as Arc<dyn BlockchainService>)
                    .ok_or_else(|| DomainError::ConfigurationError(
                        "Ethereum service not initialized. Call initialize_for_network() first.".to_string()
                    ))
            }
            ChainType::Bitcoin => {
                self.bitcoin_service
                    .as_ref()
                    .map(|s| s.clone() as Arc<dyn BlockchainService>)
                    .ok_or_else(|| DomainError::ConfigurationError(
                        "Bitcoin service not initialized. Call initialize_for_network() first.".to_string()
                    ))
            }
            ChainType::Solana => {
                self.solana_service
                    .as_ref()
                    .map(|s| s.clone() as Arc<dyn BlockchainService>)
                    .ok_or_else(|| DomainError::ConfigurationError(
                        "Solana service not initialized. Call initialize_for_network() first.".to_string()
                    ))
            }
        }
    }

    /// Get balance for an address on a specific network
    ///
    /// This method automatically routes to the correct blockchain service based on the network.
    pub async fn get_balance_for_network(
        &self,
        address: &Address,
        network: &Network,
    ) -> Result<Balance, DomainError> {
        let service = self.get_service_for_network(network)?;
        service.get_balance(address).await
    }

    /// Transfer funds on a specific network
    pub async fn transfer_on_network(
        &self,
        network: &Network,
        from: &Address,
        to: &Address,
        amount: u128,
        private_key: &str,
    ) -> Result<TransactionHash, DomainError> {
        let service = self.get_service_for_network(network)?;
        service.transfer(from, to, amount, private_key).await
    }

    /// Check if a specific network is connected
    pub async fn is_network_connected(&self, network: &Network) -> bool {
        match self.get_service_for_network(network) {
            Ok(service) => service.is_connected().await,
            Err(_) => false,
        }
    }

    /// Get block number for a specific network
    pub async fn get_block_number_for_network(&self, network: &Network) -> Result<u64, DomainError> {
        let service = self.get_service_for_network(network)?;
        service.get_block_number().await
    }
}

// Implement BlockchainService for the current network context
#[async_trait]
impl BlockchainService for MultiChainBlockchainService {
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError> {
        let network = self.current_network.as_ref().ok_or_else(|| {
            DomainError::ConfigurationError(
                "No network context set. Use get_balance_for_network() or create with new_for_network()".to_string()
            )
        })?;
        self.get_balance_for_network(address, network).await
    }

    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u128,
        private_key: &str,
    ) -> Result<TransactionHash, DomainError> {
        let network = self.current_network.as_ref().ok_or_else(|| {
            DomainError::ConfigurationError(
                "No network context set. Use transfer_on_network() or create with new_for_network()".to_string()
            )
        })?;
        self.transfer_on_network(network, from, to, amount, private_key).await
    }

    async fn is_connected(&self) -> bool {
        match &self.current_network {
            Some(network) => self.is_network_connected(network).await,
            None => false,
        }
    }

    async fn get_block_number(&self) -> Result<u64, DomainError> {
        let network = self.current_network.as_ref().ok_or_else(|| {
            DomainError::ConfigurationError(
                "No network context set. Use get_block_number_for_network() or create with new_for_network()".to_string()
            )
        })?;
        self.get_block_number_for_network(network).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multi_chain_service_creation() {
        let service = MultiChainBlockchainService::new().await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_multi_chain_service_for_network() {
        // Test Ethereum network
        let eth_service = MultiChainBlockchainService::new_for_network(Network::Sepolia).await;
        assert!(eth_service.is_ok());

        // Test Bitcoin network
        let btc_service = MultiChainBlockchainService::new_for_network(Network::BitcoinMainnet).await;
        assert!(btc_service.is_ok());

        // Test Solana network
        let sol_service = MultiChainBlockchainService::new_for_network(Network::SolanaDevnet).await;
        assert!(sol_service.is_ok());
    }

    #[tokio::test]
    async fn test_service_routing_by_chain_type() {
        let mut service = MultiChainBlockchainService::new().await.unwrap();

        // Initialize services
        service.initialize_for_network(&Network::Mainnet).await.unwrap();
        service.initialize_for_network(&Network::BitcoinMainnet).await.unwrap();
        service.initialize_for_network(&Network::SolanaMainnet).await.unwrap();

        // Verify Ethereum service is available
        let eth_service = service.get_service_for_network(&Network::Mainnet);
        assert!(eth_service.is_ok());

        // Verify Bitcoin service is available
        let btc_service = service.get_service_for_network(&Network::BitcoinMainnet);
        assert!(btc_service.is_ok());

        // Verify Solana service is available
        let sol_service = service.get_service_for_network(&Network::SolanaMainnet);
        assert!(sol_service.is_ok());
    }

    #[tokio::test]
    async fn test_uninitialized_service_error() {
        let service = MultiChainBlockchainService::new().await.unwrap();

        // Try to get a service without initializing
        let result = service.get_service_for_network(&Network::Mainnet);
        assert!(result.is_err());

        match result {
            Err(DomainError::ConfigurationError(msg)) => {
                assert!(msg.contains("not initialized"));
            }
            _ => panic!("Expected ConfigurationError"),
        }
    }
}
