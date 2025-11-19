use rustwallet::{
    core::application::GetBalanceHandler,
    core::domain::{
        queries::GetBalanceQuery,
        services::{BlockchainService, QueryHandler},
        value_objects::{Address, Network},
    },
    infrastructure::AlloyBlockchainService,
};
use std::sync::Arc;

/// Integration test for querying balance on Ethereum mainnet
///
/// This test requires network connection and queries real blockchain data.
/// Run with: cargo test --test balance_query_integration_test -- --ignored
#[tokio::test]
#[ignore] // Ignore by default as it requires network connection
async fn test_get_balance_mainnet_integration() {
    // Arrange: Use Vitalik's address (well-known, always has balance)
    let address = Address::new("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string())
        .expect("Valid Ethereum address");

    let network = Network::Mainnet;

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
    assert!(result.is_ok(), "Balance query should succeed");

    let balance_result = result.unwrap();
    assert_eq!(balance_result.address, address);
    assert_eq!(balance_result.network, network);

    // Vitalik's address should have a non-zero balance
    assert!(
        !balance_result.balance.is_zero(),
        "Vitalik's address should have balance"
    );

    println!(
        "✅ Integration Test Passed - Balance: {}",
        balance_result.balance
    );
}

/// Integration test for checking blockchain service connectivity
#[tokio::test]
#[ignore]
async fn test_blockchain_service_connectivity() {
    // Arrange
    let service = AlloyBlockchainService::new_with_default_rpc(Network::Mainnet)
        .await
        .expect("Failed to create service");

    // Act & Assert: Check connection
    assert!(
        service.is_connected().await,
        "Should be connected to Ethereum mainnet"
    );

    // Act & Assert: Get block number
    let block_number = service.get_block_number().await;
    assert!(block_number.is_ok(), "Should retrieve block number");

    let block = block_number.unwrap();
    assert!(block > 23_000_000, "Block number should be reasonable");

    println!("✅ Connected to Ethereum mainnet at block #{}", block);
}

/// Integration test for querying multiple addresses
#[tokio::test]
#[ignore]
async fn test_query_multiple_addresses() {
    // Arrange: Well-known Ethereum addresses
    let addresses = vec![
        ("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", "Vitalik"),
        (
            "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            "WETH Contract",
        ),
    ];

    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(Network::Mainnet)
            .await
            .expect("Failed to create service"),
    );

    let handler = GetBalanceHandler::new(blockchain_service);

    // Act & Assert: Query each address
    for (addr_str, name) in addresses {
        let address = Address::new(addr_str.to_string()).expect("Valid address");
        let query = GetBalanceQuery::new(address.clone(), Network::Mainnet);

        let result = handler.handle(query).await;
        assert!(result.is_ok(), "Query for {} should succeed", name);

        let balance_result = result.unwrap();
        println!(
            "✅ {} ({}) - Balance: {}",
            name, addr_str, balance_result.balance
        );
    }
}

/// Integration test for error handling - invalid address
#[tokio::test]
async fn test_invalid_address_error() {
    // Arrange: Invalid addresses
    let invalid_addresses = vec![
        "not_an_address",                             // No 0x prefix
        "0x123",                                      // Too short
        "0xZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ", // Invalid hex
    ];

    // Act & Assert
    for invalid in invalid_addresses {
        let result = Address::new(invalid.to_string());
        assert!(
            result.is_err(),
            "Should reject invalid address: {}",
            invalid
        );
    }
}

/// Integration test for network selection
#[tokio::test]
#[ignore]
async fn test_different_networks() {
    // Note: This test only verifies service creation, not actual balance queries
    // as testnet balances are unpredictable

    let networks = vec![
        Network::Mainnet,
        Network::Sepolia,
        Network::Goerli,
        Network::Holesky,
    ];

    for network in networks {
        let service = AlloyBlockchainService::new_with_default_rpc(network.clone()).await;

        // Service creation should succeed (actual connection may fail for testnets)
        assert!(
            service.is_ok(),
            "Should be able to create service for {}",
            network.name()
        );

        println!("✅ Service created for {}", network);
    }
}

/// Integration test with custom RPC URL
#[tokio::test]
#[ignore]
async fn test_custom_rpc_url() {
    // Arrange: Use public Ethereum RPC
    let custom_rpc = "https://eth.llamarpc.com";

    let service = AlloyBlockchainService::new(Network::Mainnet, custom_rpc)
        .await
        .expect("Failed to create service with custom RPC");

    // Act
    let is_connected = service.is_connected().await;

    // Assert
    assert!(is_connected, "Should connect to custom RPC");

    println!("✅ Connected to custom RPC: {}", custom_rpc);
}

/// Performance test - measure query latency
#[tokio::test]
#[ignore]
async fn test_query_performance() {
    use std::time::Instant;

    // Arrange
    let address = Address::new("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string())
        .expect("Valid address");

    let blockchain_service: Arc<dyn BlockchainService> = Arc::new(
        AlloyBlockchainService::new_with_default_rpc(Network::Mainnet)
            .await
            .expect("Failed to create service"),
    );

    let handler = GetBalanceHandler::new(blockchain_service);

    // Warm up
    let query = GetBalanceQuery::new(address.clone(), Network::Mainnet);
    let _ = handler.handle(query).await;

    // Act: Measure performance
    let start = Instant::now();
    let query = GetBalanceQuery::new(address.clone(), Network::Mainnet);
    let result = handler.handle(query).await;
    let duration = start.elapsed();

    // Assert
    assert!(result.is_ok(), "Query should succeed");
    assert!(
        duration.as_millis() < 5000,
        "Query should complete within 5 seconds, took {:?}",
        duration
    );

    println!("✅ Query completed in {:?}", duration);
}
