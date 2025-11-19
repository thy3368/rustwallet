use rustwallet::{
    core::application::GetBalanceHandler,
    core::domain::{
        queries::GetBalanceQuery,
        services::{BlockchainService, QueryHandler},
        value_objects::{Address, Network},
    },
    adapter::infrastructure::AlloyBlockchainService,
};
use std::sync::Arc;

/// Integration test for querying balance on BSC Mainnet
///
/// This test requires network connection and queries real BSC blockchain data.
/// Run with: cargo test --test bsc_balance_integration_test -- --ignored
#[tokio::test]
#[ignore] // Ignore by default as it requires network connection
async fn test_get_balance_bsc_mainnet_integration() {
    // Arrange: Use Binance hot wallet address (well-known, always has balance)
    let address = Address::new("0x28C6c06298d514Db089934071355E5743bf21d60".to_string())
        .expect("Valid BSC address");

    let network = Network::BscMainnet;

    // Create blockchain service
    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(network.clone())
            .await
            .expect("Failed to create blockchain service"),
    );

    // Create query handler
    let handler = GetBalanceHandler::new(blockchain_service.clone());

    // Act: Execute the query
    let query = GetBalanceQuery::new(address.clone(), network.clone());
    let result = handler.handle(query).await;

    // Assert: Verify result
    assert!(result.is_ok(), "BSC balance query should succeed");

    let balance_result = result.unwrap();
    assert_eq!(balance_result.address, address);
    assert_eq!(balance_result.network, network);

    // Binance hot wallet should have a significant balance
    assert!(
        !balance_result.balance.is_zero(),
        "Binance hot wallet should have balance"
    );

    println!(
        "✅ BSC Integration Test Passed - Balance: {}",
        balance_result.balance
    );
}

/// Integration test for BSC Testnet
#[tokio::test]
#[ignore]
async fn test_get_balance_bsc_testnet() {
    // Use a testnet faucet address
    let address = Address::new("0x0000000000000000000000000000000000000000".to_string())
        .expect("Valid address");

    let network = Network::BscTestnet;

    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(network.clone())
            .await
            .expect("Failed to create service"),
    );

    let handler = GetBalanceHandler::new(blockchain_service);

    let query = GetBalanceQuery::new(address.clone(), network.clone());
    let result = handler.handle(query).await;

    // Should succeed even if balance is zero
    assert!(result.is_ok(), "BSC testnet query should succeed");

    println!("✅ BSC Testnet Test Passed");
}

/// Test BSC service connectivity
#[tokio::test]
#[ignore]
async fn test_bsc_service_connectivity() {
    // Arrange
    let service = AlloyBlockchainService::new_with_default_rpc(Network::BscMainnet)
        .await
        .expect("Failed to create BSC service");

    // Act & Assert: Check connection
    assert!(
        service.is_connected().await,
        "Should be connected to BSC mainnet"
    );

    // Act & Assert: Get block number
    let block_number = service.get_block_number().await;
    assert!(block_number.is_ok(), "Should retrieve BSC block number");

    let block = block_number.unwrap();
    assert!(block > 20_000_000, "BSC block number should be reasonable (>20M)");

    println!("✅ Connected to BSC Mainnet at block #{}", block);
}

/// Test querying multiple BSC addresses
#[tokio::test]
#[ignore]
async fn test_query_multiple_bsc_addresses() {
    // Arrange: Well-known BSC addresses
    let addresses = vec![
        (
            "0x28C6c06298d514Db089934071355E5743bf21d60",
            "Binance Hot Wallet",
        ),
        (
            "0x8894E0a0c962CB723c1976a4421c95949bE2D4E3",
            "Binance: Hot Wallet 2",
        ),
        (
            "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c",
            "WBNB Token Contract",
        ),
    ];

    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(Network::BscMainnet)
            .await
            .expect("Failed to create service"),
    );

    let handler = GetBalanceHandler::new(blockchain_service);

    // Act & Assert: Query each address
    for (addr_str, name) in addresses {
        let address = Address::new(addr_str.to_string()).expect("Valid address");
        let query = GetBalanceQuery::new(address.clone(), Network::BscMainnet);

        let result = handler.handle(query).await;
        assert!(result.is_ok(), "Query for {} should succeed", name);

        let balance_result = result.unwrap();
        println!(
            "✅ {} ({}) - Balance: {}",
            name, addr_str, balance_result.balance
        );
    }
}

/// Test BSC vs Ethereum address compatibility
#[tokio::test]
#[ignore]
async fn test_bsc_address_format() {
    // BSC uses same address format as Ethereum
    let eth_format_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC";

    // Should work on both networks
    let address = Address::new(eth_format_address.to_string()).expect("Valid address");

    // Test on BSC
    let bsc_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(Network::BscMainnet)
            .await
            .expect("Failed to create BSC service"),
    );

    let handler = GetBalanceHandler::new(bsc_service);
    let query = GetBalanceQuery::new(address.clone(), Network::BscMainnet);
    let result = handler.handle(query).await;

    assert!(result.is_ok(), "Same address format should work on BSC");

    println!("✅ Address format compatibility verified");
}

/// Performance test - BSC query latency
#[tokio::test]
#[ignore]
async fn test_bsc_query_performance() {
    use std::time::Instant;

    // Arrange
    let address = Address::new("0x28C6c06298d514Db089934071355E5743bf21d60".to_string())
        .expect("Valid address");

    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(Network::BscMainnet)
            .await
            .expect("Failed to create service"),
    );

    let handler = GetBalanceHandler::new(blockchain_service);

    // Warm up
    let query = GetBalanceQuery::new(address.clone(), Network::BscMainnet);
    let _ = handler.handle(query).await;

    // Act: Measure performance
    let start = Instant::now();
    let query = GetBalanceQuery::new(address.clone(), Network::BscMainnet);
    let result = handler.handle(query).await;
    let duration = start.elapsed();

    // Assert
    assert!(result.is_ok(), "Query should succeed");
    assert!(
        duration.as_millis() < 5000,
        "BSC query should complete within 5 seconds, took {:?}",
        duration
    );

    println!("✅ BSC Query completed in {:?}", duration);
}

/// Test BSC with custom RPC endpoint
#[tokio::test]
#[ignore]
async fn test_bsc_custom_rpc() {
    // Alternative BSC RPC endpoints
    let custom_rpcs = vec![
        "https://bsc-dataseed1.binance.org",
        "https://bsc-dataseed2.binance.org",
        "https://bsc-dataseed.binance.org",
    ];

    let address = Address::new("0x28C6c06298d514Db089934071355E5743bf21d60".to_string())
        .expect("Valid address");

    for rpc in custom_rpcs {
        let service = AlloyBlockchainService::new(Network::BscMainnet, rpc)
            .await
            .expect(&format!("Failed to create service with RPC: {}", rpc));

        let handler = GetBalanceHandler::new(Arc::new(service));
        let query = GetBalanceQuery::new(address.clone(), Network::BscMainnet);
        let result = handler.handle(query).await;

        assert!(
            result.is_ok(),
            "Query with custom RPC {} should succeed",
            rpc
        );

        println!("✅ Custom RPC {} works", rpc);
    }
}

/// Test BSC network identification
#[test]
fn test_bsc_network_properties() {
    // BSC Mainnet properties
    assert_eq!(Network::BscMainnet.chain_id(), 56);
    assert_eq!(Network::BscMainnet.name(), "BSC Mainnet");
    assert!(!Network::BscMainnet.is_testnet());
    assert!(Network::BscMainnet.is_bsc());

    // BSC Testnet properties
    assert_eq!(Network::BscTestnet.chain_id(), 97);
    assert_eq!(Network::BscTestnet.name(), "BSC Testnet");
    assert!(Network::BscTestnet.is_testnet());
    assert!(Network::BscTestnet.is_bsc());

    println!("✅ BSC network properties verified");
}
