use async_trait::async_trait;
use alloy::{
    network::EthereumWallet,
    primitives::{Address as AlloyAddress, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use crate::core::domain::{
    errors::DomainError,
    services::BlockchainService,
    value_objects::{Address, Balance, Network, TransactionHash},
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

    /// Transfer funds between addresses
    ///
    /// Implements complete transaction workflow:
    /// 1. Parse private key and create wallet
    /// 2. Build transaction with proper parameters
    /// 3. Sign transaction
    /// 4. Broadcast to network
    /// 5. Return transaction hash
    ///
    /// # Security Notes
    /// - Private keys are handled in memory only
    /// - Keys are not logged or persisted
    /// - Use environment variables or secure key storage in production
    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u128,
        private_key: &str,
    ) -> Result<TransactionHash, DomainError> {
        // Step 1: Parse private key and create signer
        let signer: PrivateKeySigner = private_key
            .parse()
            .map_err(|_| DomainError::InvalidPrivateKey)?;

        // Verify that the signer's address matches the from address
        let signer_address = signer.address();
        let from_alloy: AlloyAddress = from
            .as_str()
            .parse()
            .map_err(|e| DomainError::BlockchainError(format!("Invalid from address: {}", e)))?;

        if signer_address != from_alloy {
            return Err(DomainError::TransferFailed(
                "Private key does not match from address".to_string(),
            ));
        }

        // Step 2: Parse destination address
        let to_alloy: AlloyAddress = to
            .as_str()
            .parse()
            .map_err(|e| DomainError::BlockchainError(format!("Invalid to address: {}", e)))?;

        // Step 3: Check sender balance
        let balance = self.get_balance(from).await?;
        if balance.to_wei() < amount {
            return Err(DomainError::InsufficientBalance);
        }

        // Step 4: Create wallet from signer
        let wallet = EthereumWallet::from(signer);

        // Step 5: Create provider with wallet
        let rpc_url = self.network.default_rpc_url();
        let provider_with_wallet = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url.parse().map_err(|e| {
                DomainError::ConfigurationError(format!("Invalid RPC URL: {}", e))
            })?);

        // Step 6: Build transaction request
        let tx = TransactionRequest::default()
            .to(to_alloy)
            .value(U256::from(amount))
            .from(from_alloy);

        // Step 7: Send transaction and get pending transaction
        let pending_tx = provider_with_wallet
            .send_transaction(tx)
            .await
            .map_err(|e| {
                DomainError::TransferFailed(format!("Failed to send transaction: {}", e))
            })?;

        // Step 8: Get transaction hash
        let tx_hash = *pending_tx.tx_hash();

        // Convert to our domain TransactionHash
        let tx_hash_str = format!("{:?}", tx_hash);
        TransactionHash::new(tx_hash_str)
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
