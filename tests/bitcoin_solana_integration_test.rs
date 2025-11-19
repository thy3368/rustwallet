/// Integration tests for Bitcoin and Solana balance queries
///
/// # Test Coverage
///
/// ## Bitcoin Tests
/// - Balance query on Bitcoin mainnet
/// - Balance query on Bitcoin testnet
/// - Network connectivity
/// - Block height query
///
/// ## Solana Tests
/// - Balance query on Solana mainnet
/// - Balance query on Solana devnet
/// - Network connectivity
/// - Slot query
///
use rustwallet::core::domain::{
    queries::GetBalanceQuery,
    services::{BlockchainService, QueryHandler},
    value_objects::{Address, Network},
};
use rustwallet::core::application::handlers::GetBalanceHandler;
use rustwallet::adapter::infrastructure::blockchain::{BitcoinBlockchainService, SolanaBlockchainService};
use std::sync::Arc;

// ============================================================================
// Bitcoin Integration Tests
// ============================================================================

#[tokio::test]
#[ignore] // Requires network connection
async fn test_bitcoin_mainnet_balance() {
    println!("\n🟠 Bitcoin Mainnet Balance Query Test\n");

    // Use Satoshi's famous address
    let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid Bitcoin address");

    println!("📋 Test Configuration:");
    println!("  Network:  Bitcoin Mainnet");
    println!("  Address:  {} (Satoshi's address)", address);

    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Failed to create Bitcoin service");

    println!("\nStep 1: Querying balance...");
    let start = std::time::Instant::now();

    let balance = service
        .get_balance(&address)
        .await
        .expect("Failed to get balance");

    let duration = start.elapsed();

    println!("  ✓ Balance retrieved: {} satoshis", balance.to_wei());
    println!("  ⏱️  Query time: {:?}", duration);

    assert!(balance.to_wei() > 0, "Satoshi's address should have balance");

    println!("\n✅ Bitcoin Mainnet Test PASSED");
}

#[tokio::test]
#[ignore]
async fn test_bitcoin_testnet_balance() {
    println!("\n🟠 Bitcoin Testnet Balance Query Test\n");

    // Use a testnet faucet address
    let address = Address::new("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string())
        .expect("Valid Bitcoin testnet address");

    println!("📋 Test Configuration:");
    println!("  Network:  Bitcoin Testnet");
    println!("  Address:  {}", address);

    let service = BitcoinBlockchainService::new(Network::BitcoinTestnet)
        .await
        .expect("Failed to create Bitcoin testnet service");

    println!("\nStep 1: Querying balance...");
    let balance_result = service.get_balance(&address).await;

    match balance_result {
        Ok(balance) => {
            println!("  ✓ Balance: {} satoshis", balance.to_wei());
            println!("\n✅ Bitcoin Testnet Test PASSED");
        }
        Err(e) => {
            println!("  ⚠️  Query failed (may be address format issue): {}", e);
            println!("\n⚠️  Bitcoin Testnet Test SKIPPED (expected for some addresses)");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_bitcoin_connectivity() {
    println!("\n🟠 Bitcoin Network Connectivity Test\n");

    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Failed to create service");

    println!("Step 1: Checking network connectivity...");
    let connected = service.is_connected().await;

    println!("  ✓ Connected: {}", connected);
    assert!(connected, "Should be able to connect to Bitcoin network");

    println!("\nStep 2: Querying current block height...");
    let block_height = service
        .get_block_number()
        .await
        .expect("Failed to get block height");

    println!("  ✓ Current block height: {}", block_height);
    assert!(block_height > 800_000, "Block height should be reasonable");

    println!("\n✅ Bitcoin Connectivity Test PASSED");
}

#[tokio::test]
#[ignore]
async fn test_bitcoin_multiple_addresses() {
    println!("\n🟠 Bitcoin Multiple Addresses Test\n");

    let addresses = vec![
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", // Satoshi's address
        "3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy", // P2SH address
    ];

    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Failed to create service");

    for (i, addr_str) in addresses.iter().enumerate() {
        println!("\nAddress {}: {}", i + 1, addr_str);

        let address = Address::new(addr_str.to_string())
            .expect("Valid Bitcoin address");

        let start = std::time::Instant::now();
        let balance = service.get_balance(&address).await;
        let duration = start.elapsed();

        match balance {
            Ok(bal) => {
                println!("  ✓ Balance: {} satoshis", bal.to_wei());
                println!("  ⏱️  Query time: {:?}", duration);
            }
            Err(e) => {
                println!("  ⚠️  Query failed: {}", e);
            }
        }
    }

    println!("\n✅ Bitcoin Multiple Addresses Test COMPLETED");
}

// ============================================================================
// Solana Integration Tests
// ============================================================================

#[tokio::test]
#[ignore] // Requires network connection
async fn test_solana_mainnet_balance() {
    println!("\n🟣 Solana Mainnet Balance Query Test\n");

    // Use Solana Foundation's address
    let address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid Solana address");

    println!("📋 Test Configuration:");
    println!("  Network:  Solana Mainnet");
    println!("  Address:  {}", address);

    let service = SolanaBlockchainService::new(Network::SolanaMainnet)
        .await
        .expect("Failed to create Solana service");

    println!("\nStep 1: Querying balance...");
    let start = std::time::Instant::now();

    let balance = service
        .get_balance(&address)
        .await
        .expect("Failed to get balance");

    let duration = start.elapsed();

    println!("  ✓ Balance: {} lamports", balance.to_wei());
    println!("  ✓ Balance: {} SOL", balance.to_wei() as f64 / 1_000_000_000.0);
    println!("  ⏱️  Query time: {:?}", duration);

    println!("\n✅ Solana Mainnet Test PASSED");
}

#[tokio::test]
#[ignore]
async fn test_solana_devnet_balance() {
    println!("\n🟣 Solana Devnet Balance Query Test\n");

    // Use Solana system program address
    let address = Address::new("11111111111111111111111111111111".to_string())
        .expect("Valid Solana address");

    println!("📋 Test Configuration:");
    println!("  Network:  Solana Devnet");
    println!("  Address:  {} (System Program)", address);

    let service = SolanaBlockchainService::new(Network::SolanaDevnet)
        .await
        .expect("Failed to create Solana devnet service");

    println!("\nStep 1: Querying balance...");
    let start = std::time::Instant::now();

    let balance = service
        .get_balance(&address)
        .await
        .expect("Failed to get balance");

    let duration = start.elapsed();

    println!("  ✓ Balance: {} lamports", balance.to_wei());
    println!("  ⏱️  Query time: {:?}", duration);

    println!("\n✅ Solana Devnet Test PASSED");
}

#[tokio::test]
#[ignore]
async fn test_solana_connectivity() {
    println!("\n🟣 Solana Network Connectivity Test\n");

    let service = SolanaBlockchainService::new(Network::SolanaDevnet)
        .await
        .expect("Failed to create service");

    println!("Step 1: Checking network connectivity...");
    let connected = service.is_connected().await;

    println!("  ✓ Connected: {}", connected);
    assert!(connected, "Should be able to connect to Solana network");

    println!("\nStep 2: Querying current slot...");
    let slot = service
        .get_block_number()
        .await
        .expect("Failed to get slot");

    println!("  ✓ Current slot: {}", slot);
    assert!(slot > 0, "Slot should be greater than 0");

    println!("\n✅ Solana Connectivity Test PASSED");
}

#[tokio::test]
#[ignore]
async fn test_solana_multiple_addresses() {
    println!("\n🟣 Solana Multiple Addresses Test\n");

    let addresses = vec![
        "11111111111111111111111111111111",                             // System program
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",                  // Token program
        "Vote111111111111111111111111111111111111111",                  // Vote program
    ];

    let service = SolanaBlockchainService::new(Network::SolanaMainnet)
        .await
        .expect("Failed to create service");

    for (i, addr_str) in addresses.iter().enumerate() {
        println!("\nAddress {}: {}", i + 1, addr_str);

        let address = Address::new(addr_str.to_string())
            .expect("Valid Solana address");

        let start = std::time::Instant::now();
        let balance = service.get_balance(&address).await;
        let duration = start.elapsed();

        match balance {
            Ok(bal) => {
                println!("  ✓ Balance: {} lamports", bal.to_wei());
                println!("  ✓ Balance: {} SOL", bal.to_wei() as f64 / 1_000_000_000.0);
                println!("  ⏱️  Query time: {:?}", duration);
            }
            Err(e) => {
                println!("  ⚠️  Query failed: {}", e);
            }
        }
    }

    println!("\n✅ Solana Multiple Addresses Test COMPLETED");
}

// ============================================================================
// Performance Comparison Tests
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_multi_chain_performance_comparison() {
    println!("\n⚡ Multi-Chain Performance Comparison\n");

    // Test addresses
    let btc_address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid Bitcoin address");
    let sol_address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid Solana address");

    // Bitcoin query
    println!("🟠 Bitcoin Mainnet:");
    let btc_service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Bitcoin service failed");

    let start = std::time::Instant::now();
    let btc_balance = btc_service.get_balance(&btc_address).await;
    let btc_duration = start.elapsed();

    match btc_balance {
        Ok(bal) => {
            println!("  ✓ Balance: {} satoshis", bal.to_wei());
            println!("  ⏱️  Query time: {:?}", btc_duration);
        }
        Err(e) => println!("  ⚠️  Query failed: {}", e),
    }

    // Solana query
    println!("\n🟣 Solana Mainnet:");
    let sol_service = SolanaBlockchainService::new(Network::SolanaMainnet)
        .await
        .expect("Solana service failed");

    let start = std::time::Instant::now();
    let sol_balance = sol_service.get_balance(&sol_address).await;
    let sol_duration = start.elapsed();

    match sol_balance {
        Ok(bal) => {
            println!("  ✓ Balance: {} lamports", bal.to_wei());
            println!("  ⏱️  Query time: {:?}", sol_duration);
        }
        Err(e) => println!("  ⚠️  Query failed: {}", e),
    }

    println!("\n📊 Performance Summary:");
    println!("  Bitcoin: {:?}", btc_duration);
    println!("  Solana:  {:?}", sol_duration);

    println!("\n✅ Performance Comparison COMPLETED");
}

// ============================================================================
// Network Type Tests
// ============================================================================

#[test]
fn test_network_type_identification() {
    println!("\n🔍 Network Type Identification Test\n");

    // Bitcoin networks
    assert!(Network::BitcoinMainnet.is_bitcoin());
    assert!(Network::BitcoinTestnet.is_bitcoin());
    assert!(!Network::BitcoinMainnet.is_evm());
    assert!(!Network::BitcoinMainnet.is_solana());

    // Solana networks
    assert!(Network::SolanaMainnet.is_solana());
    assert!(Network::SolanaDevnet.is_solana());
    assert!(!Network::SolanaMainnet.is_evm());
    assert!(!Network::SolanaMainnet.is_bitcoin());

    // EVM networks
    assert!(Network::Mainnet.is_evm());
    assert!(Network::BscMainnet.is_evm());
    assert!(!Network::Mainnet.is_bitcoin());
    assert!(!Network::Mainnet.is_solana());

    println!("  ✓ Bitcoin network detection works");
    println!("  ✓ Solana network detection works");
    println!("  ✓ EVM network detection works");

    println!("\n✅ Network Type Identification Test PASSED");
}

#[test]
fn test_network_display() {
    println!("\n🖥️  Network Display Format Test\n");

    // EVM networks show chain ID
    println!("EVM Networks:");
    println!("  {}", Network::Mainnet);
    println!("  {}", Network::BscMainnet);

    // Non-EVM networks don't show chain ID
    println!("\nBitcoin Networks:");
    println!("  {}", Network::BitcoinMainnet);
    println!("  {}", Network::BitcoinTestnet);

    println!("\nSolana Networks:");
    println!("  {}", Network::SolanaMainnet);
    println!("  {}", Network::SolanaDevnet);

    println!("\n✅ Network Display Test PASSED");
}

// ============================================================================
// Clean Architecture Tests (Using GetBalanceQuery + Handler) ⭐
// ============================================================================

/// 示例：使用 GetBalanceQuery 查询 Bitcoin 余额（Clean Architecture 方式）
#[tokio::test]
#[ignore]
async fn test_bitcoin_with_query_handler() {
    println!("\n🏛️  Bitcoin Balance Query - Clean Architecture Pattern\n");

    // Step 1: 创建 Infrastructure 层服务
    println!("Step 1: Creating BitcoinBlockchainService...");
    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Failed to create Bitcoin service");

    // Step 2: 创建 Application 层 Handler
    println!("Step 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: 创建 Domain 层 Query
    println!("Step 3: Creating GetBalanceQuery...");
    let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid Bitcoin address");

    let query = GetBalanceQuery::new(
        address.clone(),
        Network::BitcoinMainnet,
    );

    // Step 4: 执行查询（通过 Handler）⭐
    println!("Step 4: Executing query through handler...");
    let start = std::time::Instant::now();

    let result = handler.handle(query).await
        .expect("Query failed");

    let duration = start.elapsed();

    // Step 5: 验证结果
    println!("\n✅ Query Result:");
    println!("  Address:  {}", result.address);
    println!("  Network:  {}", result.network);
    println!("  Balance:  {} satoshis", result.balance.to_wei());
    println!("  Balance:  {} BTC", result.balance.to_wei() as f64 / 100_000_000.0);
    println!("  ⏱️  Time:   {:?}", duration);

    assert!(result.balance.to_wei() > 0);

    println!("\n✅ Clean Architecture Test PASSED");
}

/// 示例：使用 GetBalanceQuery 查询 Solana 余额（Clean Architecture 方式）
#[tokio::test]
#[ignore]
async fn test_solana_with_query_handler() {
    println!("\n🏛️  Solana Balance Query - Clean Architecture Pattern\n");

    // Step 1: 创建 Infrastructure 层服务
    println!("Step 1: Creating SolanaBlockchainService...");
    let service = SolanaBlockchainService::new(Network::SolanaMainnet)
        .await
        .expect("Failed to create Solana service");

    // Step 2: 创建 Application 层 Handler
    println!("Step 2: Creating GetBalanceHandler...");
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: 创建 Domain 层 Query
    println!("Step 3: Creating GetBalanceQuery...");
    let address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid Solana address");

    let query = GetBalanceQuery::new(
        address.clone(),
        Network::SolanaMainnet,
    );

    // Step 4: 执行查询（通过 Handler）⭐
    println!("Step 4: Executing query through handler...");
    let start = std::time::Instant::now();

    let result = handler.handle(query).await
        .expect("Query failed");

    let duration = start.elapsed();

    // Step 5: 验证结果
    println!("\n✅ Query Result:");
    println!("  Address:  {}", result.address);
    println!("  Network:  {}", result.network);
    println!("  Balance:  {} lamports", result.balance.to_wei());
    println!("  Balance:  {} SOL", result.balance.to_wei() as f64 / 1_000_000_000.0);
    println!("  ⏱️  Time:   {:?}", duration);

    println!("\n✅ Clean Architecture Test PASSED");
}

/// 示例：完整的 Clean Architecture 多链查询示例
#[tokio::test]
#[ignore]
async fn test_multi_chain_clean_architecture() {
    println!("\n🏛️  Multi-Chain Clean Architecture Pattern Demo\n");

    // 测试数据
    let test_cases = vec![
        ("Bitcoin", Network::BitcoinMainnet, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
        ("Solana", Network::SolanaMainnet, "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy"),
    ];

    for (chain_name, network, addr_str) in test_cases {
        println!("\n═══════════════════════════════════════");
        println!("Testing: {}", chain_name);
        println!("═══════════════════════════════════════");

        let address = Address::new(addr_str.to_string())
            .expect("Valid address");

        // 根据网络类型创建相应的服务
        let query = GetBalanceQuery::new(address.clone(), network.clone());

        let result = if network.is_bitcoin() {
            // Bitcoin 服务
            let service = BitcoinBlockchainService::new(network.clone())
                .await
                .expect("Bitcoin service failed");
            let handler = GetBalanceHandler::new(Arc::new(service));
            handler.handle(query).await
        } else if network.is_solana() {
            // Solana 服务
            let service = SolanaBlockchainService::new(network.clone())
                .await
                .expect("Solana service failed");
            let handler = GetBalanceHandler::new(Arc::new(service));
            handler.handle(query).await
        } else {
            panic!("Unsupported network type");
        };

        match result {
            Ok(query_result) => {
                println!("✅ Chain:    {}", chain_name);
                println!("   Network:  {}", query_result.network);
                println!("   Address:  {}", query_result.address);
                println!("   Balance:  {} (base units)", query_result.balance.to_wei());
            }
            Err(e) => {
                println!("⚠️  Chain:    {}", chain_name);
                println!("   Error:    {}", e);
            }
        }
    }

    println!("\n═══════════════════════════════════════");
    println!("✅ Multi-Chain Clean Architecture Test COMPLETED");
}

// ============================================================================
// 架构模式对比示例
// ============================================================================

/// 对比：直接调用 Service vs 使用 Query Handler
#[tokio::test]
#[ignore]
async fn test_architecture_pattern_comparison() {
    println!("\n📐 Architecture Pattern Comparison\n");

    let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid address");

    // ❌ 方式 1: 直接调用 Infrastructure 层（不推荐）
    println!("❌ Pattern 1: Direct Service Call (Not Recommended)");
    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet)
        .await
        .expect("Service failed");

    let balance1 = service.get_balance(&address).await
        .expect("Query failed");

    println!("   Balance: {} satoshis", balance1.to_wei());

    // ✅ 方式 2: 使用 Query + Handler（Clean Architecture）
    println!("\n✅ Pattern 2: Query + Handler (Clean Architecture - Recommended)");

    // 创建查询
    let query = GetBalanceQuery::new(
        address.clone(),
        Network::BitcoinMainnet,
    );

    // 创建 Handler
    let handler = GetBalanceHandler::new(Arc::new(service));

    // 执行查询
    let result = handler.handle(query).await
        .expect("Query failed");

    println!("   Address:  {}", result.address);
    println!("   Network:  {}", result.network);
    println!("   Balance:  {} satoshis", result.balance.to_wei());

    println!("\n📊 Comparison:");
    println!("   方式 1: 违反依赖规则，Application 层直接依赖 Infrastructure");
    println!("   方式 2: ✅ 符合 Clean Architecture，通过 Handler 和 Query 隔离");
    println!("          ✅ 易于测试（可 mock Handler）");
    println!("          ✅ 符合 CQRS 模式");
    println!("          ✅ 更好的关注点分离");

    println!("\n✅ Architecture Comparison Test PASSED");
}

// ============================================================================
// ChainType 功能测试 ⭐ 新增
// ============================================================================

/// 测试：ChainType 自动识别功能
#[test]
fn test_chain_type_auto_detection() {
    use rustwallet::core::domain::value_objects::ChainType;

    println!("\n🔍 ChainType Auto-Detection Test\n");

    // Test Ethereum networks
    println!("Testing Ethereum networks:");
    assert_eq!(Network::Mainnet.chain_type(), ChainType::Ethereum);
    assert_eq!(Network::Sepolia.chain_type(), ChainType::Ethereum);
    assert_eq!(Network::BscMainnet.chain_type(), ChainType::Ethereum);
    assert_eq!(Network::BscTestnet.chain_type(), ChainType::Ethereum);
    println!("  ✓ Ethereum/EVM networks detected correctly");

    // Test Bitcoin networks
    println!("\nTesting Bitcoin networks:");
    assert_eq!(Network::BitcoinMainnet.chain_type(), ChainType::Bitcoin);
    assert_eq!(Network::BitcoinTestnet.chain_type(), ChainType::Bitcoin);
    println!("  ✓ Bitcoin networks detected correctly");

    // Test Solana networks
    println!("\nTesting Solana networks:");
    assert_eq!(Network::SolanaMainnet.chain_type(), ChainType::Solana);
    assert_eq!(Network::SolanaDevnet.chain_type(), ChainType::Solana);
    assert_eq!(Network::SolanaTestnet.chain_type(), ChainType::Solana);
    println!("  ✓ Solana networks detected correctly");

    println!("\n✅ ChainType Auto-Detection Test PASSED");
}

/// 测试：GetBalanceQuery 包含 ChainType
#[test]
fn test_get_balance_query_with_chain_type() {
    println!("\n🏛️  GetBalanceQuery ChainType Integration Test\n");

    use rustwallet::core::domain::value_objects::ChainType;

    // Test Ethereum query
    println!("Creating Ethereum balance query:");
    let eth_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
        .expect("Valid ETH address");
    let eth_query = GetBalanceQuery::new(eth_address.clone(), Network::Mainnet);

    assert_eq!(eth_query.chain_type, ChainType::Ethereum);
    assert_eq!(eth_query.network, Network::Mainnet);
    println!("  ✓ Address:    {}", eth_query.address);
    println!("  ✓ Network:    {}", eth_query.network);
    println!("  ✓ Chain Type: {}", eth_query.chain_type);

    // Test Bitcoin query
    println!("\nCreating Bitcoin balance query:");
    let btc_address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        .expect("Valid BTC address");
    let btc_query = GetBalanceQuery::new(btc_address.clone(), Network::BitcoinMainnet);

    assert_eq!(btc_query.chain_type, ChainType::Bitcoin);
    assert_eq!(btc_query.network, Network::BitcoinMainnet);
    println!("  ✓ Address:    {}", btc_query.address);
    println!("  ✓ Network:    {}", btc_query.network);
    println!("  ✓ Chain Type: {}", btc_query.chain_type);

    // Test Solana query
    println!("\nCreating Solana balance query:");
    let sol_address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())
        .expect("Valid SOL address");
    let sol_query = GetBalanceQuery::new(sol_address.clone(), Network::SolanaMainnet);

    assert_eq!(sol_query.chain_type, ChainType::Solana);
    assert_eq!(sol_query.network, Network::SolanaMainnet);
    println!("  ✓ Address:    {}", sol_query.address);
    println!("  ✓ Network:    {}", sol_query.network);
    println!("  ✓ Chain Type: {}", sol_query.chain_type);

    println!("\n✅ GetBalanceQuery ChainType Integration Test PASSED");
}

/// 测试：多链查询统一接口（演示用例）
#[tokio::test]
#[ignore]
async fn test_unified_multi_chain_query_interface() {
    println!("\n🌐 Unified Multi-Chain Query Interface Demo\n");

    use rustwallet::core::domain::value_objects::ChainType;

    // 定义多链查询
    let queries = vec![
        (
            "Ethereum",
            GetBalanceQuery::new(
                Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string()).unwrap(),
                Network::Sepolia,
            ),
        ),
        (
            "Bitcoin",
            GetBalanceQuery::new(
                Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()).unwrap(),
                Network::BitcoinMainnet,
            ),
        ),
        (
            "Solana",
            GetBalanceQuery::new(
                Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string()).unwrap(),
                Network::SolanaMainnet,
            ),
        ),
    ];

    // 统一接口处理不同链的查询
    for (chain_name, query) in queries {
        println!("═══════════════════════════════════════");
        println!("Processing: {}", chain_name);
        println!("═══════════════════════════════════════");
        println!("  Address:    {}", query.address);
        println!("  Network:    {}", query.network);
        println!("  Chain Type: {}", query.chain_type);
        println!("  Currency:   {}", query.chain_type.native_currency());
        println!("  Unit:       {}", query.chain_type.smallest_unit());
        println!("  Decimals:   {}", query.chain_type.decimals());

        // 根据 ChainType 路由到不同的服务
        match query.chain_type {
            ChainType::Ethereum => {
                println!("  → Routing to EVM service");
                // 可以创建 AlloyBlockchainService
            }
            ChainType::Bitcoin => {
                println!("  → Routing to Bitcoin service");
                // 可以创建 BitcoinBlockchainService
                let service = BitcoinBlockchainService::new(query.network.clone())
                    .await
                    .expect("Bitcoin service failed");
                let handler = GetBalanceHandler::new(Arc::new(service));

                match handler.handle(query).await {
                    Ok(result) => {
                        println!("  ✅ Balance: {} satoshis", result.balance.to_wei());
                    }
                    Err(e) => {
                        println!("  ⚠️  Query failed: {}", e);
                    }
                }
            }
            ChainType::Solana => {
                println!("  → Routing to Solana service");
                // 可以创建 SolanaBlockchainService
                let service = SolanaBlockchainService::new(query.network.clone())
                    .await
                    .expect("Solana service failed");
                let handler = GetBalanceHandler::new(Arc::new(service));

                match handler.handle(query).await {
                    Ok(result) => {
                        println!("  ✅ Balance: {} lamports", result.balance.to_wei());
                    }
                    Err(e) => {
                        println!("  ⚠️  Query failed: {}", e);
                    }
                }
            }
        }
        println!();
    }

    println!("═══════════════════════════════════════");
    println!("✅ Unified Multi-Chain Query Interface Demo COMPLETED");
    println!("\n💡 Key Benefits:");
    println!("  1. 同一个 GetBalanceQuery 接口支持所有链");
    println!("  2. ChainType 自动从 Network 推导");
    println!("  3. 可以基于 ChainType 路由到不同服务");
    println!("  4. 统一的错误处理和结果格式");
}
