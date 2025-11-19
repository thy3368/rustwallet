use clap::{Parser, Subcommand};
use std::sync::Arc;
use crate::{
    core::application::GetBalanceHandler,
    core::domain::{
        queries::GetBalanceQuery,
        services::QueryHandler,
        value_objects::{Address, Network},
    },
};
use crate::adapter::infrastructure::AlloyBlockchainService;
use crate::core::domain::services::BlockchainService;

#[derive(Parser)]
#[command(name = "rustwallet")]
#[command(about = "Ethereum wallet CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get balance of an Ethereum address
    Balance {
        /// Ethereum address (0x...)
        #[arg(short, long)]
        address: String,

        /// Network (mainnet, sepolia, goerli, holesky)
        #[arg(short, long, default_value = "sepolia")]
        network: String,

        /// Custom RPC URL (optional)
        #[arg(short, long)]
        rpc_url: Option<String>,
    },
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Balance {
                address,
                network,
                rpc_url,
            } => {
                Self::handle_balance_static(address, network, rpc_url).await?;
            }
        }
        Ok(())
    }

    async fn handle_balance_static(
        address_str: String,
        network_str: String,
        rpc_url: Option<String>,
    ) -> anyhow::Result<()> {
        // Parse address
        let address = Address::new(address_str)?;

        // Parse network
        let network = match network_str.to_lowercase().as_str() {
            "mainnet" => Network::Mainnet,
            "sepolia" => Network::Sepolia,
            "goerli" => Network::Goerli,
            "holesky" => Network::Holesky,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown network: {}. Use mainnet, sepolia, goerli, or holesky",
                    network_str
                ));
            }
        };

        println!("🔍 Querying balance...");
        println!("   Address: {}", address);
        println!("   Network: {}", network);

        // Create blockchain service
        let blockchain_service: Arc<dyn BlockchainService> = if let Some(rpc) = rpc_url {
            println!("   RPC URL: {}", rpc);
            Arc::new(AlloyBlockchainService::new(network.clone(), &rpc).await?)
        } else {
            let default_rpc = network.default_rpc_url();
            println!("   RPC URL: {}", default_rpc);
            Arc::new(AlloyBlockchainService::new_with_default_rpc(network.clone()).await?)
        };

        // Test connection
        if !blockchain_service.is_connected().await {
            return Err(anyhow::anyhow!("Failed to connect to network"));
        }

        let block_number = blockchain_service.get_block_number().await?;
        println!("   Current Block: #{}", block_number);
        println!();

        // Create query handler
        let handler = GetBalanceHandler::new(blockchain_service);

        // Execute query
        let query = GetBalanceQuery::new(address, network);
        let result = handler.handle(query).await?;

        // Display result
        println!("✅ Balance Query Result:");
        println!("   Address:  {}", result.address);
        println!("   Network:  {}", result.network);
        println!("   Balance:  {}", result.balance.format_ether(6));
        println!("   Wei:      {} Wei", result.balance.to_wei());

        Ok(())
    }
}
