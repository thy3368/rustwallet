use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use crate::core::domain::{
    errors::DomainError,
    services::BlockchainService,
    value_objects::{Address, Balance, Network, TransactionHash},
};

/// Bitcoin blockchain service using blockchain.info API
pub struct BitcoinBlockchainService {
    client: Client,
    network: Network,
    api_base_url: String,
}

impl BitcoinBlockchainService {
    /// Create new Bitcoin blockchain service
    pub async fn new(network: Network) -> Result<Self, DomainError> {
        if !network.is_bitcoin() {
            return Err(DomainError::ConfigurationError(
                "Network must be a Bitcoin network".to_string(),
            ));
        }

        let api_base_url = match network {
            Network::BitcoinMainnet => "https://blockchain.info",
            Network::BitcoinTestnet => "https://testnet.blockchain.info",
            _ => unreachable!(),
        };

        Ok(Self {
            client: Client::new(),
            network,
            api_base_url: api_base_url.to_string(),
        })
    }

    /// Get the network this service is connected to
    pub fn network(&self) -> &Network {
        &self.network
    }
}

#[async_trait]
impl BlockchainService for BitcoinBlockchainService {
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError> {
        // Call blockchain.info API: /balance?active=address
        let url = format!("{}/balance?active={}", self.api_base_url, address.as_str());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to query Bitcoin balance: {}", e)))?;

        if !response.status().is_success() {
            return Err(DomainError::NetworkError(format!(
                "Bitcoin API returned error: {}",
                response.status()
            )));
        }

        // Parse response JSON
        let response_text = response
            .text()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to read response: {}", e)))?;

        // The response is like: {"address":{"final_balance":123456}}
        let parsed: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| DomainError::BlockchainError(format!("Failed to parse response: {}", e)))?;

        // Extract balance from nested structure
        let balance_satoshis = parsed
            .get(address.as_str())
            .and_then(|addr_info| addr_info.get("final_balance"))
            .and_then(|bal| bal.as_u64())
            .ok_or_else(|| {
                DomainError::BlockchainError(format!("Failed to extract balance from response: {}", response_text))
            })?;

        // Convert satoshis to Wei for consistency (1 BTC = 10^8 satoshis, 1 ETH = 10^18 Wei)
        // We'll use the same Wei format but interpret it as satoshis for Bitcoin
        Ok(Balance::from_wei(balance_satoshis as u128))
    }

    async fn transfer(
        &self,
        _from: &Address,
        _to: &Address,
        _amount: u128,
        _private_key: &str,
    ) -> Result<TransactionHash, DomainError> {
        Err(DomainError::TransferFailed(
            "Bitcoin transfers not yet implemented".to_string(),
        ))
    }

    async fn is_connected(&self) -> bool {
        // Try to fetch chain info
        let url = format!("{}/latestblock", self.api_base_url);
        self.client.get(&url).send().await.is_ok()
    }

    async fn get_block_number(&self) -> Result<u64, DomainError> {
        let url = format!("{}/latestblock", self.api_base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to get latest block: {}", e)))?;

        #[derive(Deserialize)]
        struct LatestBlock {
            height: u64,
        }

        let block: LatestBlock = response
            .json()
            .await
            .map_err(|e| DomainError::NetworkError(format!("Failed to parse block response: {}", e)))?;

        Ok(block.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_bitcoin_service_creation() {
        let service = BitcoinBlockchainService::new(Network::BitcoinMainnet).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires network connection
    async fn test_bitcoin_get_balance() {
        // Use a well-known Bitcoin address (Satoshi's)
        let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
            .expect("Valid Bitcoin address");

        let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
            .await
            .expect("Service creation failed");

        let balance = service.get_balance(&address).await;
        println!("Balance result: {:?}", balance);
        // Note: This test may fail depending on the address format validation
        // Bitcoin addresses don't have the 0x prefix like Ethereum
    }
}
