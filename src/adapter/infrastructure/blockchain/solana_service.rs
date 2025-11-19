use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::core::domain::{
    errors::DomainError,
    services::BlockchainService,
    value_objects::{Address, Balance, Network, TransactionHash},
};

/// Solana blockchain service using JSON-RPC API
pub struct SolanaBlockchainService {
    client: Client,
    network: Network,
    rpc_url: String,
}

#[derive(Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize, Debug)]
struct JsonRpcError {
    message: String,
}

impl SolanaBlockchainService {
    /// Create new Solana blockchain service
    pub async fn new(network: Network) -> Result<Self, DomainError> {
        if !network.is_solana() {
            return Err(DomainError::ConfigurationError(
                "Network must be a Solana network".to_string(),
            ));
        }

        let rpc_url = network.default_rpc_url().to_string();

        Ok(Self {
            client: Client::new(),
            network,
            rpc_url,
        })
    }

    /// Get the network this service is connected to
    pub fn network(&self) -> &Network {
        &self.network
    }

    /// Make a JSON-RPC call
    async fn rpc_call<T>(&self, method: &str, params: Vec<serde_json::Value>) -> Result<T, DomainError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.to_string(),
            params,
        };

        let response = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to send RPC request: {}", e)))?;

        let rpc_response: JsonRpcResponse<T> = response
            .json()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to parse RPC response: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(DomainError::BlockchainError(format!(
                "RPC error: {}",
                error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| DomainError::BlockchainError("No result in RPC response".to_string()))
    }
}

#[async_trait]
impl BlockchainService for SolanaBlockchainService {
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError> {
        // Call getBalance RPC method
        // params: [address (base58 string), optional config object]
        let params = vec![
            serde_json::json!(address.as_str()),
        ];

        let balance_lamports: u64 = self.rpc_call("getBalance", params).await?;

        // Convert lamports to Wei format for consistency
        // 1 SOL = 10^9 lamports
        Ok(Balance::from_wei(balance_lamports as u128))
    }

    async fn transfer(
        &self,
        _from: &Address,
        _to: &Address,
        _amount: u128,
        _private_key: &str,
    ) -> Result<TransactionHash, DomainError> {
        Err(DomainError::TransferFailed(
            "Solana transfers not yet implemented".to_string(),
        ))
    }

    async fn is_connected(&self) -> bool {
        // Try to get health status
        let result: Result<String, DomainError> = self.rpc_call("getHealth", vec![]).await;
        result.is_ok()
    }

    async fn get_block_number(&self) -> Result<u64, DomainError> {
        // Get current slot
        self.rpc_call("getSlot", vec![]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_solana_service_creation() {
        let service = SolanaBlockchainService::new(Network::SolanaDevnet).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_solana_get_balance() {
        // Use a well-known Solana address
        let address = Address::new("Vote111111111111111111111111111111111111111".to_string())
            .expect("Valid Solana address");

        let service = SolanaBlockchainService::new(Network::SolanaDevnet)
            .await
            .expect("Service creation failed");

        let balance = service.get_balance(&address).await;
        println!("Balance result: {:?}", balance);
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_solana_connectivity() {
        let service = SolanaBlockchainService::new(Network::SolanaDevnet)
            .await
            .expect("Service creation failed");

        let connected = service.is_connected().await;
        println!("Connected: {}", connected);
        assert!(connected);
    }
}
