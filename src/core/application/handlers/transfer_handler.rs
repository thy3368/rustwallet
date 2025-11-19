use crate::core::domain::{
    commands::{TransferCommand, TransferResult},
    errors::DomainError,
    services::{BlockchainService, CommandHandler},
};
use async_trait::async_trait;
use std::sync::Arc;

/// Transfer command handler - orchestrates the transfer use case
///
/// This handler implements the CQRS Command pattern for transfer operations.
/// It coordinates between the domain model and infrastructure services.
///
/// # Architecture
///
/// ```text
/// TransferCommand -> TransferHandler -> BlockchainService -> Network
///                         ↓
///                  TransferResult
/// ```
pub struct TransferHandler {
    blockchain_service: Arc<dyn BlockchainService>,
}

impl TransferHandler {
    /// Create a new transfer handler
    pub fn new(blockchain_service: Arc<dyn BlockchainService>) -> Self {
        Self {
            blockchain_service,
        }
    }
}

#[async_trait]
impl CommandHandler<TransferCommand> for TransferHandler {
    type Output = TransferResult;

    /// Handle transfer command
    ///
    /// # Workflow
    ///
    /// 1. Extract command parameters
    /// 2. Delegate to blockchain service for execution
    /// 3. Build and return transfer result
    ///
    /// # Errors
    ///
    /// - `InvalidPrivateKey`: Private key format invalid
    /// - `InsufficientBalance`: Not enough balance for transfer
    /// - `TransferFailed`: Transaction submission failed
    /// - `NetworkError`: Network communication issues
    async fn handle(&self, command: TransferCommand) -> Result<Self::Output, DomainError> {
        // Execute transfer via blockchain service
        let tx_hash = self
            .blockchain_service
            .transfer(
                &command.from_address,
                &command.to_address,
                command.amount.to_wei(),
                &command.private_key,
            )
            .await?;

        // Build and return result
        Ok(TransferResult::new(
            tx_hash,
            command.from_address,
            command.to_address,
            command.amount,
            command.network,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::value_objects::{Address, Amount, Balance, Network, TransactionHash};

    struct MockBlockchainService {
        expected_tx_hash: String,
    }

    #[async_trait]
    impl BlockchainService for MockBlockchainService {
        async fn get_balance(&self, _address: &Address) -> Result<Balance, DomainError> {
            Ok(Balance::from_ether(10.0))
        }

        async fn transfer(
            &self,
            _from: &Address,
            _to: &Address,
            _amount: u128,
            _private_key: &str,
        ) -> Result<TransactionHash, DomainError> {
            TransactionHash::new(self.expected_tx_hash.clone())
        }

        async fn is_connected(&self) -> bool {
            true
        }

        async fn get_block_number(&self) -> Result<u64, DomainError> {
            Ok(12345)
        }
    }

    #[tokio::test]
    async fn test_transfer_handler() {
        // Setup
        let expected_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let mock_service = Arc::new(MockBlockchainService {
            expected_tx_hash: expected_hash.to_string(),
        });

        let handler = TransferHandler::new(mock_service);

        // Create command
        let from_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
            .expect("Valid address");
        let to_address = Address::new("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string())
            .expect("Valid address");
        let amount = Amount::from_ether(0.001);

        let command = TransferCommand::new(
            from_address.clone(),
            to_address.clone(),
            amount,
            Network::Sepolia,
            "test_private_key".to_string(),
        );

        // Execute
        let result = handler.handle(command).await;

        // Verify
        assert!(result.is_ok());
        let transfer_result = result.unwrap();
        assert_eq!(transfer_result.tx_hash.as_str(), expected_hash);
        assert_eq!(transfer_result.from_address, from_address);
        assert_eq!(transfer_result.to_address, to_address);
        assert_eq!(transfer_result.amount, amount);
        assert_eq!(transfer_result.network, Network::Sepolia);
    }

    #[tokio::test]
    async fn test_transfer_handler_error_propagation() {
        // Test that errors from blockchain service are properly propagated
        struct FailingBlockchainService;

        #[async_trait]
        impl BlockchainService for FailingBlockchainService {
            async fn get_balance(&self, _address: &Address) -> Result<Balance, DomainError> {
                Ok(Balance::from_ether(10.0))
            }

            async fn transfer(
                &self,
                _from: &Address,
                _to: &Address,
                _amount: u128,
                _private_key: &str,
            ) -> Result<TransactionHash, DomainError> {
                Err(DomainError::InsufficientBalance)
            }

            async fn is_connected(&self) -> bool {
                false
            }

            async fn get_block_number(&self) -> Result<u64, DomainError> {
                Err(DomainError::NetworkError("Test error".to_string()))
            }
        }

        let failing_service = Arc::new(FailingBlockchainService);
        let handler = TransferHandler::new(failing_service);

        let from_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
            .expect("Valid address");
        let to_address = Address::new("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string())
            .expect("Valid address");

        let command = TransferCommand::new(
            from_address,
            to_address,
            Amount::from_ether(0.001),
            Network::Sepolia,
            "test_key".to_string(),
        );

        let result = handler.handle(command).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::InsufficientBalance));
    }
}
