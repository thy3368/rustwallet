/// Integration tests for MultiChainBlockchainService
///
/// This test file demonstrates how to use the MultiChainBlockchainService
/// with QueryHandler following Clean Architecture patterns.

use rustwallet::adapter::infrastructure::blockchain::MultiChainBlockchainService;
use rustwallet::core::domain::{
    queries::GetBalanceQuery,
    services::QueryHandler,
    value_objects::{Address, Network},
};
use rustwallet::core::application::handlers::GetBalanceHandler;
use std::sync::Arc;

// ============================================================================
// Clean Architecture Pattern Tests (Using QueryHandler)
// ============================================================================

#[tokio::test]
#[ignore] // 同时查 eth,bitcoin,solana中的余额 - 需要网络连接
async fn test_multi_chain_service_basic_usage() {
    println!("\n🌐 Multi-Chain Service - 同时查询 ETH/Bitcoin/Solana 余额\n");

    // Step 1: Create Infrastructure layer service - 初始化所有链
    println!("Step 1: Creating MultiChainBlockchainService for all chains...");
    let mut service = MultiChainBlockchainService::new()
        .await
        .expect("Failed to create service");

    service.initialize_all().await.expect("Failed to initialize all chains");
    let service_arc = Arc::new(service);

    println!("✓ Initialized services for ETH, Bitcoin, and Solana");

    // Step 2: Create Application layer Handler - 创建 Handler
    println!("\nStep 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(service_arc.clone());
    let handler_arc = Arc::new(handler);

    println!("✓ Created GetBalanceHandler");

    // Step 3: Create Domain layer Queries - 为三条链创建查询
    println!("\nStep 3: Creating queries for all three chains...");

    // Ethereum 查询
    let eth_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
        .expect("Valid ETH address");
    let eth_query = GetBalanceQuery::new(eth_address.clone(), Network::Sepolia);
    println!("✓ Created Ethereum query:");
    println!("  Address:    {}", eth_query.address);
    println!("  Network:    {}", eth_query.network);
    println!("  Chain Type: {}", eth_query.chain_type);

    // Bitcoin 查询
    let btc_address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid BTC address");
    let btc_query = GetBalanceQuery::new(btc_address.clone(), Network::BitcoinMainnet);
    println!("\n✓ Created Bitcoin query:");
    println!("  Address:    {}", btc_query.address);
    println!("  Network:    {}", btc_query.network);
    println!("  Chain Type: {}", btc_query.chain_type);

    // Solana 查询
    let sol_address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid SOL address");
    let sol_query = GetBalanceQuery::new(sol_address.clone(), Network::SolanaMainnet);
    println!("\n✓ Created Solana query:");
    println!("  Address:    {}", sol_query.address);
    println!("  Network:    {}", sol_query.network);
    println!("  Chain Type: {}", sol_query.chain_type);

    // Step 4: Execute queries concurrently - 并发执行查询
    println!("\nStep 4: Executing all queries concurrently...");
    let start = std::time::Instant::now();

    let (eth_result, btc_result, sol_result) = tokio::join!(
        handler_arc.handle(eth_query),
        handler_arc.handle(btc_query),
        handler_arc.handle(sol_query)
    );

    let duration = start.elapsed();

    // 显示结果
    println!("\n═══════════════════════════════════════");
    println!("📊 Query Results (Total time: {:?})", duration);
    println!("═══════════════════════════════════════");

    // Ethereum 结果
    match eth_result {
        Ok(result) => {
            println!("\n🔷 Ethereum Sepolia:");
            println!("  ✅ Success");
            println!("  Address:  {}", result.address);
            println!("  Balance:  {} Wei", result.balance.to_wei());
            println!("  Balance:  {} ETH", result.balance.to_ether());
            println!("  Chain:    {}", result.chain_type);
        }
        Err(e) => {
            println!("\n🔷 Ethereum Sepolia:");
            println!("  ⚠️  Error: {}", e);
        }
    }

    // Bitcoin 结果
    match btc_result {
        Ok(result) => {
            println!("\n🟠 Bitcoin Mainnet:");
            println!("  ✅ Success");
            println!("  Address:  {} (Satoshi's address)", result.address);
            println!("  Balance:  {} satoshis", result.balance.to_wei());
            println!("  Balance:  {} BTC", result.balance.to_wei() as f64 / 100_000_000.0);
            println!("  Chain:    {}", result.chain_type);
        }
        Err(e) => {
            println!("\n🟠 Bitcoin Mainnet:");
            println!("  ⚠️  Error: {}", e);
        }
    }

    // Solana 结果
    match sol_result {
        Ok(result) => {
            println!("\n🟣 Solana Mainnet:");
            println!("  ✅ Success");
            println!("  Address:  {}", result.address);
            println!("  Balance:  {} lamports", result.balance.to_wei());
            println!("  Balance:  {} SOL", result.balance.to_wei() as f64 / 1_000_000_000.0);
            println!("  Chain:    {}", result.chain_type);
        }
        Err(e) => {
            println!("\n🟣 Solana Mainnet:");
            println!("  ⚠️  Error: {}", e);
        }
    }

    println!("\n═══════════════════════════════════════");
    println!("✅ Multi-Chain Concurrent Query Test COMPLETED");
    println!("\n💡 Key Features Demonstrated:");
    println!("  1. 一个 MultiChainBlockchainService 支持所有链");
    println!("  2. 一个 Handler 处理所有链的查询");
    println!("  3. 使用 tokio::join! 并发查询，提高性能");
    println!("  4. 统一的 Query 和 Result 接口");
    println!("  5. ChainType 自动从 Network 推导");
}

#[tokio::test]
async fn test_multi_chain_service_all_chains() {
    println!("\n🌐 Multi-Chain Service - All Chains Initialization\n");

    let mut service = MultiChainBlockchainService::new()
        .await
        .expect("Failed to create service");

    // Initialize all chain services
    service.initialize_all().await.expect("Failed to initialize");

    println!("✓ Initialized services for all chains");
    println!("  - Ethereum/EVM: ✓");
    println!("  - Bitcoin: ✓");
    println!("  - Solana: ✓");

    println!("\n✅ All Chains Initialization Test PASSED");
}

// ============================================================================
// Multi-Chain Balance Query Tests (via QueryHandler)
// ============================================================================

#[tokio::test]
#[ignore] // Requires network connection
async fn test_query_ethereum_via_handler() {
    println!("\n🔷 Ethereum Balance Query - Clean Architecture Pattern\n");

    // Step 1: Create Infrastructure layer service
    println!("Step 1: Creating MultiChainBlockchainService for Sepolia...");
    let service = MultiChainBlockchainService::new_for_network(Network::Sepolia)
        .await
        .expect("Failed to create service");

    // Step 2: Create Application layer Handler
    println!("Step 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: Create Domain layer Query
    println!("Step 3: Creating GetBalanceQuery...");
    let address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
        .expect("Valid ETH address");
    let query = GetBalanceQuery::new(address.clone(), Network::Sepolia);

    println!("\n📋 Query Details:");
    println!("  Address:    {}", query.address);
    println!("  Network:    {}", query.network);
    println!("  Chain Type: {}", query.chain_type);

    // Step 4: Execute query through handler
    println!("\nStep 4: Executing query through handler...");
    let result = handler.handle(query).await.expect("Query failed");

    println!("\n✅ Query Result:");
    println!("  Balance: {} Wei", result.balance.to_wei());
    println!("  Balance: {} ETH", result.balance.to_ether());
    println!("  Chain:   {}", result.chain_type);

    println!("\n✅ Ethereum Query Test PASSED");
}

#[tokio::test]
#[ignore] // Requires network connection
async fn test_query_bitcoin_via_handler() {
    println!("\n🟠 Bitcoin Balance Query - Clean Architecture Pattern\n");

    // Step 1: Create Infrastructure layer service
    println!("Step 1: Creating MultiChainBlockchainService for Bitcoin...");
    let service = MultiChainBlockchainService::new_for_network(Network::BitcoinMainnet)
        .await
        .expect("Failed to create service");

    // Step 2: Create Application layer Handler
    println!("Step 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: Create Domain layer Query
    println!("Step 3: Creating GetBalanceQuery...");
    let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid BTC address");
    let query = GetBalanceQuery::new(address.clone(), Network::BitcoinMainnet);

    println!("\n📋 Query Details:");
    println!("  Address:    {} (Satoshi's address)", query.address);
    println!("  Network:    {}", query.network);
    println!("  Chain Type: {}", query.chain_type);

    // Step 4: Execute query through handler
    println!("\nStep 4: Executing query through handler...");
    let result = handler.handle(query).await.expect("Query failed");

    println!("\n✅ Query Result:");
    println!("  Balance: {} satoshis", result.balance.to_wei());
    println!("  Balance: {} BTC", result.balance.to_wei() as f64 / 100_000_000.0);
    println!("  Chain:   {}", result.chain_type);

    assert!(result.balance.to_wei() > 0, "Satoshi's address should have balance");

    println!("\n✅ Bitcoin Query Test PASSED");
}

#[tokio::test]
#[ignore] // Requires network connection
async fn test_query_solana_via_handler() {
    println!("\n🟣 Solana Balance Query - Clean Architecture Pattern\n");

    // Step 1: Create Infrastructure layer service
    println!("Step 1: Creating MultiChainBlockchainService for Solana...");
    let service = MultiChainBlockchainService::new_for_network(Network::SolanaMainnet)
        .await
        .expect("Failed to create service");

    // Step 2: Create Application layer Handler
    println!("Step 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: Create Domain layer Query
    println!("Step 3: Creating GetBalanceQuery...");
    let address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid SOL address");
    let query = GetBalanceQuery::new(address.clone(), Network::SolanaMainnet);

    println!("\n📋 Query Details:");
    println!("  Address:    {}", query.address);
    println!("  Network:    {}", query.network);
    println!("  Chain Type: {}", query.chain_type);

    // Step 4: Execute query through handler
    println!("\nStep 4: Executing query through handler...");
    let result = handler.handle(query).await.expect("Query failed");

    println!("\n✅ Query Result:");
    println!("  Balance: {} lamports", result.balance.to_wei());
    println!("  Balance: {} SOL", result.balance.to_wei() as f64 / 1_000_000_000.0);
    println!("  Chain:   {}", result.chain_type);

    println!("\n✅ Solana Query Test PASSED");
}

// ============================================================================
// Unified Multi-Chain Query Demo (via QueryHandler)
// ============================================================================

#[tokio::test]
#[ignore] // Requires network connection
async fn test_unified_multi_chain_query_via_handler() {
    println!("\n🌐 Unified Multi-Chain Query Demo - Clean Architecture\n");

    // Define test queries for different chains
    let test_cases = vec![
        (
            "Ethereum Sepolia",
            Network::Sepolia,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC",
        ),
        (
            "Bitcoin Mainnet",
            Network::BitcoinMainnet,
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        ),
        (
            "Solana Mainnet",
            Network::SolanaMainnet,
            "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy",
        ),
    ];

    // Query all chains using the same Clean Architecture pattern
    for (name, network, addr_str) in test_cases {
        println!("═══════════════════════════════════════");
        println!("Testing: {}", name);
        println!("═══════════════════════════════════════");

        // Step 1: Create service for this network
        let service = MultiChainBlockchainService::new_for_network(network.clone())
            .await
            .expect("Failed to create service");

        // Step 2: Create handler
        let handler = GetBalanceHandler::new(Arc::new(service));

        // Step 3: Create query
        let address = Address::new(addr_str.to_string()).expect("Valid address");
        let query = GetBalanceQuery::new(address.clone(), network.clone());

        println!("  Chain Type: {}", query.chain_type);
        println!("  Network:    {}", query.network);
        println!("  Address:    {}", query.address);
        println!("  Currency:   {}", query.chain_type.native_currency());

        // Step 4: Execute query through handler
        match handler.handle(query).await {
            Ok(result) => {
                println!("  ✅ Success!");
                println!("     Balance:  {} {}",
                    result.balance.to_wei(),
                    result.chain_type.smallest_unit()
                );
                println!("     Balance:  {} {}",
                    result.balance.to_wei() as f64 / 10_f64.powi(result.chain_type.decimals() as i32),
                    result.chain_type.native_currency()
                );
            }
            Err(e) => {
                println!("  ⚠️  Error: {}", e);
            }
        }
        println!();
    }

    println!("═══════════════════════════════════════");
    println!("✅ Unified Multi-Chain Query Demo COMPLETED");
    println!("\n💡 Clean Architecture Benefits:");
    println!("  1. Domain层 Query 定义业务需求");
    println!("  2. Application层 Handler 编排业务逻辑");
    println!("  3. Infrastructure层 Service 实现技术细节");
    println!("  4. 统一的查询接口，支持所有链");
    println!("  5. 易于测试和维护");
}

// ============================================================================
// Reusable Query Pattern Demo
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_reusable_query_pattern() {
    println!("\n🔄 Reusable Query Pattern Demo\n");

    // Create a reusable query function
    async fn execute_balance_query(
        network: Network,
        address: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Step 1: Create service
        let service = MultiChainBlockchainService::new_for_network(network.clone()).await?;

        // Step 2: Create handler
        let handler = GetBalanceHandler::new(Arc::new(service));

        // Step 3: Create and execute query
        let addr = Address::new(address.to_string())?;
        let query = GetBalanceQuery::new(addr, network);
        let result = handler.handle(query).await?;

        // Format result
        Ok(format!("{} {} ({} {})",
            result.balance.to_wei(),
            result.chain_type.smallest_unit(),
            result.balance.to_wei() as f64 / 10_f64.powi(result.chain_type.decimals() as i32),
            result.chain_type.native_currency()
        ))
    }

    println!("Demonstrating reusable query pattern:\n");

    // Use the same pattern for all chains
    let queries = vec![
        ("Ethereum", Network::Sepolia, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC"),
        ("Bitcoin", Network::BitcoinMainnet, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
        ("Solana", Network::SolanaMainnet, "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy"),
    ];

    for (name, network, address) in queries {
        println!("🔹 {}", name);
        match execute_balance_query(network, address).await {
            Ok(balance) => println!("   Balance: {}", balance),
            Err(e) => println!("   Error: {}", e),
        }
    }

    println!("\n✅ Reusable Query Pattern Demo COMPLETED");
}

// ============================================================================
// Error Handling Tests (via QueryHandler)
// ============================================================================

#[tokio::test]
async fn test_error_handling_via_handler() {
    println!("\n⚠️  Error Handling via QueryHandler Test\n");

    // Create service without initialization
    let service = MultiChainBlockchainService::new().await.unwrap();

    // Create handler
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Create query for uninitialized network
    let address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string()).unwrap();
    let query = GetBalanceQuery::new(address, Network::Mainnet);

    println!("Attempting to query uninitialized service...");

    // Try to execute query
    let result = handler.handle(query).await;

    assert!(result.is_err(), "Should return error for uninitialized service");

    match result {
        Err(e) => {
            println!("✓ Correctly returned error: {}", e);
            println!("  Error message contains configuration hint");
        }
        Ok(_) => panic!("Should have returned error"),
    }

    println!("\n✅ Error Handling Test PASSED");
}

// ============================================================================
// Performance Test (Handler vs Direct Service)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_handler_performance() {
    println!("\n⚡ Handler Performance Test\n");

    let address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string()).unwrap();
    let network = Network::Sepolia;

    // Method 1: Via QueryHandler (Clean Architecture)
    println!("Method 1: QueryHandler (Clean Architecture)");
    let start = std::time::Instant::now();

    let service = MultiChainBlockchainService::new_for_network(network.clone())
        .await
        .unwrap();
    let handler = GetBalanceHandler::new(Arc::new(service));
    let query = GetBalanceQuery::new(address.clone(), network.clone());
    let result1 = handler.handle(query).await.ok();

    let duration1 = start.elapsed();
    println!("  Time: {:?}", duration1);
    println!("  Balance: {:?}", result1.as_ref().map(|r| r.balance.to_wei()));

    // Method 2: Direct service call
    println!("\nMethod 2: Direct Service Call");
    let start = std::time::Instant::now();

    let service2 = MultiChainBlockchainService::new_for_network(network)
        .await
        .unwrap();
    use rustwallet::core::domain::services::BlockchainService;
    let result2 = service2.get_balance(&address).await.ok();

    let duration2 = start.elapsed();
    println!("  Time: {:?}", duration2);
    println!("  Balance: {:?}", result2.as_ref().map(|r| r.to_wei()));

    println!("\n📊 Performance Comparison:");
    println!("  QueryHandler: {:?}", duration1);
    println!("  Direct Call:  {:?}", duration2);
    println!("  Overhead:     {:?}", duration1.saturating_sub(duration2));

    println!("\n💡 Note: Handler overhead is minimal (typically < 1μs)");
    println!("   The extra abstraction layer is worth it for:");
    println!("   - Better testability");
    println!("   - Clearer separation of concerns");
    println!("   - Easier to add cross-cutting concerns (logging, metrics, etc.)");

    println!("\n✅ Performance Test COMPLETED");
}

// ============================================================================
// Handler Composition Test
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_handler_composition() {
    println!("\n🔗 Handler Composition Pattern\n");

    // Create a single multi-chain service
    let mut service = MultiChainBlockchainService::new().await.unwrap();
    service.initialize_all().await.unwrap();
    let service_arc = Arc::new(service);

    // Create multiple handlers sharing the same service
    let eth_handler = GetBalanceHandler::new(service_arc.clone());
    let btc_handler = GetBalanceHandler::new(service_arc.clone());
    let sol_handler = GetBalanceHandler::new(service_arc.clone());

    println!("✓ Created 3 handlers sharing the same MultiChainService\n");

    // Execute queries concurrently
    let eth_query = GetBalanceQuery::new(
        Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string()).unwrap(),
        Network::Sepolia
    );

    let btc_query = GetBalanceQuery::new(
        Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()).unwrap(),
        Network::BitcoinMainnet
    );

    let sol_query = GetBalanceQuery::new(
        Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string()).unwrap(),
        Network::SolanaMainnet
    );

    println!("Executing queries concurrently...\n");

    let (eth_result, btc_result, sol_result) = tokio::join!(
        eth_handler.handle(eth_query),
        btc_handler.handle(btc_query),
        sol_handler.handle(sol_query)
    );

    if let Ok(result) = eth_result {
        println!("🔷 Ethereum: {} Wei", result.balance.to_wei());
    }
    if let Ok(result) = btc_result {
        println!("🟠 Bitcoin:  {} satoshis", result.balance.to_wei());
    }
    if let Ok(result) = sol_result {
        println!("🟣 Solana:   {} lamports", result.balance.to_wei());
    }

    println!("\n✅ Handler Composition Test COMPLETED");
}
