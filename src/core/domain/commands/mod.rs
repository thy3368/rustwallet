use crate::core::domain::value_objects::{Address, Amount, Network, TransactionHash};
use serde::{Deserialize, Serialize};

/// Transfer command - initiate a transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCommand {
    /// Source address (sender)
    pub from_address: Address,
    /// Destination address (recipient)
    pub to_address: Address,
    /// Amount to transfer (in Wei)
    pub amount: Amount,
    /// Network to use
    pub network: Network,
    /// Private key for signing (should be handled securely)
    pub private_key: String,
    /// Optional gas price (in Wei)
    pub gas_price: Option<u128>,
}

impl TransferCommand {
    pub fn new(
        from_address: Address,
        to_address: Address,
        amount: Amount,
        network: Network,
        private_key: String,
    ) -> Self {
        Self {
            from_address,
            to_address,
            amount,
            network,
            private_key,
            gas_price: None,
        }
    }

    pub fn with_gas_price(mut self, gas_price: u128) -> Self {
        self.gas_price = Some(gas_price);
        self
    }
}

/// Transfer result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transaction hash
    pub tx_hash: TransactionHash,
    /// Source address
    pub from_address: Address,
    /// Destination address
    pub to_address: Address,
    /// Amount transferred
    pub amount: Amount,
    /// Network used
    pub network: Network,
}

impl TransferResult {
    pub fn new(
        tx_hash: TransactionHash,
        from_address: Address,
        to_address: Address,
        amount: Amount,
        network: Network,
    ) -> Self {
        Self {
            tx_hash,
            from_address,
            to_address,
            amount,
            network,
        }
    }
}
