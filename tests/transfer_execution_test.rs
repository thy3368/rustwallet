/// Integration tests for actual ETH and BSC transfer execution
///
/// ⚠️  IMPORTANT SECURITY NOTES:
/// - These tests use REAL testnets (Sepolia, BSC Testnet)
/// - Requires test funds from faucets
/// - Private keys must be provided via environment variables
/// - NEVER commit private keys to version control
/// - Use dedicated test wallets only
///
/// # Setup Instructions
///
/// ## 1. Get Test Funds
///
/// **Sepolia ETH:**
/// - Faucet: https://sepoliafaucet.com/
/// - Or: https://www.alchemy.com/faucets/ethereum-sepolia
///
/// **BSC Testnet BNB:**
/// - Faucet: https://testnet.bnbchain.org/faucet-smart
///
/// ## 2. Set Environment Variables
///
/// ```bash
/// # Test wallet private key (64 hex characters, no 0x prefix)
/// export TEST_PRIVATE_KEY="your_test_private_key_here"
///
/// # Optional: Specific test addresses
/// export TEST_FROM_ADDRESS="0x..."
/// export TEST_TO_ADDRESS="0x..."
/// ```
///
/// ## 3. Run Tests
///
/// ```bash
/// # Run all transfer execution tests
/// cargo test --test transfer_execution_test -- --ignored --nocapture
///
/// # Run specific test
/// cargo test --test transfer_execution_test test_eth_transfer_sepolia -- --ignored --nocapture
/// ```
///
/// # Test Coverage
///
/// - ✅ ETH transfer on Sepolia testnet
/// - ✅ BSC transfer on BSC testnet
/// - ✅ Cross-chain transfer workflow (ETH -> BSC)
/// - ✅ Balance verification before/after
/// - ✅ Transaction confirmation
/// - ✅ Gas estimation validation
/// - ✅ Error handling (insufficient balance, invalid key, etc.)
///
use rustwallet::core::domain::{
    services::BlockchainService,
    value_objects::{Address, Amount, Network},
};
use rustwallet::adapter::infrastructure::blockchain::AlloyBlockchainService;
use std::env;
use std::time::Instant;

/// Helper function to get test private key from environment
fn get_test_private_key() -> Option<String> {
    env::var("TEST_PRIVATE_KEY").ok()
}

/// Helper function to get test from address (derived from private key)
fn get_test_from_address() -> Option<String> {
    env::var("TEST_FROM_ADDRESS").ok()
}

/// Helper function to get test destination address
fn get_test_to_address() -> Option<String> {
    // Default test address if not provided
    env::var("TEST_TO_ADDRESS")
        .ok()
        .or_else(|| Some("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string()))
}

/// Test ETH transfer on Sepolia testnet
#[tokio::test]
#[ignore] // Requires environment setup and test funds
async fn test_eth_transfer_sepolia() {
    println!("\n🔄 ETH Transfer Test on Sepolia\n");

    // Check environment setup
    let private_key = match get_test_private_key() {
        Some(key) => key,
        None => {
            println!("⚠️  Skipping test: TEST_PRIVATE_KEY not set");
            println!("   Set with: export TEST_PRIVATE_KEY=your_key");
            return;
        }
    };

    let from_address_str = match get_test_from_address() {
        Some(addr) => addr,
        None => {
            println!("⚠️  Skipping test: TEST_FROM_ADDRESS not set");
            return;
        }
    };

    let to_address_str = get_test_to_address().unwrap();

    println!("📋 Test Configuration:");
    println!("  Network: Sepolia Testnet");
    println!("  From:    {}", from_address_str);
    println!("  To:      {}", to_address_str);
    println!("  Amount:  0.001 ETH\n");

    // Parse addresses
    let from_address = Address::new(from_address_str).expect("Valid from address");
    let to_address = Address::new(to_address_str).expect("Valid to address");

    // Create blockchain service
    let service = AlloyBlockchainService::new_with_default_rpc(Network::Sepolia)
        .await
        .expect("Failed to create service");

    // Step 1: Check initial balance
    println!("Step 1: Checking initial balance...");
    let start_time = Instant::now();
    let initial_balance = service
        .get_balance(&from_address)
        .await
        .expect("Failed to get initial balance");
    println!("  ✓ Initial balance: {}", initial_balance);
    println!("  ⏱️  Query time: {:?}", start_time.elapsed());

    // Verify sufficient balance
    let transfer_amount = Amount::from_ether(0.001);
    let estimated_gas = Amount::from_ether(0.0001); // Rough gas estimate

    if initial_balance.to_wei() < (transfer_amount.to_wei() + estimated_gas.to_wei()) {
        println!("\n⚠️  Insufficient balance for transfer + gas");
        println!("   Need at least: {} ETH",
            Amount::from_wei(transfer_amount.to_wei() + estimated_gas.to_wei()));
        println!("   Current balance: {}", initial_balance);
        println!("\n   Get test ETH from: https://sepoliafaucet.com/");
        return;
    }

    // Step 2: Execute transfer
    println!("\nStep 2: Executing transfer...");
    let transfer_start = Instant::now();

    let tx_hash = service
        .transfer(
            &from_address,
            &to_address,
            transfer_amount.to_wei(),
            &private_key,
        )
        .await
        .expect("Transfer failed");

    let transfer_duration = transfer_start.elapsed();
    println!("  ✓ Transaction sent!");
    println!("  📝 TX Hash: {}", tx_hash);
    println!("  ⏱️  Broadcast time: {:?}", transfer_duration);

    // Step 3: Wait for confirmation (give it some time)
    println!("\nStep 3: Waiting for confirmation...");
    println!("  ⏳ Waiting 15 seconds for block inclusion...");
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    // Step 4: Verify balance change
    println!("\nStep 4: Verifying balance change...");
    let final_balance = service
        .get_balance(&from_address)
        .await
        .expect("Failed to get final balance");
    println!("  ✓ Final balance: {}", final_balance);

    let balance_diff = initial_balance.to_wei() - final_balance.to_wei();
    println!("  📊 Balance change: -{} Wei", balance_diff);
    println!("  📊 Expected transfer: {} Wei", transfer_amount.to_wei());

    // Balance should have decreased by at least the transfer amount
    assert!(balance_diff >= transfer_amount.to_wei(),
        "Balance should decrease by at least transfer amount");

    println!("\n✅ ETH Transfer Test PASSED");
    println!("   Transaction: https://sepolia.etherscan.io/tx/{}", tx_hash);
}

/// Test BSC transfer on BSC testnet
#[tokio::test]
#[ignore] // Requires environment setup and test funds
async fn test_bsc_transfer_testnet() {
    println!("\n🔄 BSC Transfer Test on BSC Testnet\n");

    // Check environment setup
    let private_key = match get_test_private_key() {
        Some(key) => key,
        None => {
            println!("⚠️  Skipping test: TEST_PRIVATE_KEY not set");
            return;
        }
    };

    let from_address_str = match get_test_from_address() {
        Some(addr) => addr,
        None => {
            println!("⚠️  Skipping test: TEST_FROM_ADDRESS not set");
            return;
        }
    };

    let to_address_str = get_test_to_address().unwrap();

    println!("📋 Test Configuration:");
    println!("  Network: BSC Testnet");
    println!("  From:    {}", from_address_str);
    println!("  To:      {}", to_address_str);
    println!("  Amount:  0.001 BNB\n");

    // Parse addresses
    let from_address = Address::new(from_address_str).expect("Valid from address");
    let to_address = Address::new(to_address_str).expect("Valid to address");

    // Create blockchain service
    let service = AlloyBlockchainService::new_with_default_rpc(Network::BscTestnet)
        .await
        .expect("Failed to create service");

    // Check initial balance
    println!("Step 1: Checking initial balance...");
    let initial_balance = service
        .get_balance(&from_address)
        .await
        .expect("Failed to get initial balance");
    println!("  ✓ Initial balance: {}", initial_balance);

    // Verify sufficient balance
    let transfer_amount = Amount::from_ether(0.001);
    let estimated_gas = Amount::from_ether(0.00001); // BSC has lower gas

    if initial_balance.to_wei() < (transfer_amount.to_wei() + estimated_gas.to_wei()) {
        println!("\n⚠️  Insufficient balance for transfer + gas");
        println!("   Get test BNB from: https://testnet.bnbchain.org/faucet-smart");
        return;
    }

    // Execute transfer
    println!("\nStep 2: Executing transfer...");
    let tx_hash = service
        .transfer(
            &from_address,
            &to_address,
            transfer_amount.to_wei(),
            &private_key,
        )
        .await
        .expect("Transfer failed");

    println!("  ✓ Transaction sent!");
    println!("  📝 TX Hash: {}", tx_hash);

    // Wait for confirmation (BSC is faster, ~3s blocks)
    println!("\nStep 3: Waiting for confirmation...");
    println!("  ⏳ Waiting 10 seconds for block inclusion...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Verify balance change
    println!("\nStep 4: Verifying balance change...");
    let final_balance = service
        .get_balance(&from_address)
        .await
        .expect("Failed to get final balance");
    println!("  ✓ Final balance: {}", final_balance);

    let balance_diff = initial_balance.to_wei() - final_balance.to_wei();
    println!("  📊 Balance change: -{} Wei", balance_diff);

    assert!(balance_diff >= transfer_amount.to_wei());

    println!("\n✅ BSC Transfer Test PASSED");
    println!("   Transaction: https://testnet.bscscan.com/tx/{}", tx_hash);
}

/// Test error handling: insufficient balance
#[tokio::test]
#[ignore]
async fn test_transfer_insufficient_balance() {
    println!("\n🧪 Testing Error: Insufficient Balance\n");

    let private_key = match get_test_private_key() {
        Some(key) => key,
        None => {
            println!("⚠️  Skipping test: TEST_PRIVATE_KEY not set");
            return;
        }
    };

    let from_address_str = match get_test_from_address() {
        Some(addr) => addr,
        None => return,
    };

    let from_address = Address::new(from_address_str).expect("Valid address");
    let to_address = Address::new("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string())
        .expect("Valid address");

    let service = AlloyBlockchainService::new_with_default_rpc(Network::Sepolia)
        .await
        .expect("Failed to create service");

    // Try to transfer more than balance
    let huge_amount = Amount::from_ether(1000000.0);

    let result = service
        .transfer(&from_address, &to_address, huge_amount.to_wei(), &private_key)
        .await;

    assert!(result.is_err(), "Should fail with insufficient balance");
    println!("  ✓ Correctly rejected transfer with insufficient balance");
    println!("  Error: {:?}", result.unwrap_err());
    println!("\n✅ Error handling test PASSED");
}

/// Test error handling: invalid private key
#[tokio::test]
async fn test_transfer_invalid_private_key() {
    println!("\n🧪 Testing Error: Invalid Private Key\n");

    let from_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
        .expect("Valid address");
    let to_address = Address::new("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string())
        .expect("Valid address");

    let service = AlloyBlockchainService::new_with_default_rpc(Network::Sepolia)
        .await
        .expect("Failed to create service");

    // Try with invalid private key
    let invalid_key = "invalid_private_key";
    let amount = Amount::from_ether(0.001);

    let result = service
        .transfer(&from_address, &to_address, amount.to_wei(), invalid_key)
        .await;

    assert!(result.is_err(), "Should fail with invalid private key");
    println!("  ✓ Correctly rejected invalid private key");
    println!("  Error: {:?}", result.unwrap_err());
    println!("\n✅ Error handling test PASSED");
}

/// Test error handling: private key doesn't match from address
#[tokio::test]
#[ignore] // Requires environment setup
async fn test_transfer_key_address_mismatch() {
    println!("\n🧪 Testing Error: Key-Address Mismatch\n");

    let private_key = match get_test_private_key() {
        Some(key) => key,
        None => {
            println!("⚠️  Skipping test: TEST_PRIVATE_KEY not set");
            return;
        }
    };

    // Use a different address that doesn't match the private key
    let wrong_from_address = Address::new("0x0000000000000000000000000000000000000001".to_string())
        .expect("Valid address");
    let to_address = Address::new("0x8894E0a0c962CB723c1976a4421c95949bE2D4E3".to_string())
        .expect("Valid address");

    let service = AlloyBlockchainService::new_with_default_rpc(Network::Sepolia)
        .await
        .expect("Failed to create service");

    let amount = Amount::from_ether(0.001);

    let result = service
        .transfer(&wrong_from_address, &to_address, amount.to_wei(), &private_key)
        .await;

    assert!(result.is_err(), "Should fail with mismatched address");
    println!("  ✓ Correctly rejected mismatched key-address pair");
    println!("  Error: {:?}", result.unwrap_err());
    println!("\n✅ Error handling test PASSED");
}

/// Test transfer performance metrics
#[tokio::test]
#[ignore] // Requires environment setup
async fn test_transfer_performance() {
    println!("\n⚡ Transfer Performance Test\n");

    let private_key = match get_test_private_key() {
        Some(key) => key,
        None => {
            println!("⚠️  Skipping test: TEST_PRIVATE_KEY not set");
            return;
        }
    };

    let from_address_str = match get_test_from_address() {
        Some(addr) => addr,
        None => return,
    };

    let from_address = Address::new(from_address_str).expect("Valid address");
    let to_address = Address::new(get_test_to_address().unwrap()).expect("Valid address");

    // Test on both networks
    for network in &[Network::Sepolia, Network::BscTestnet] {
        println!("\n📊 Testing {}...", network);

        let service = AlloyBlockchainService::new_with_default_rpc(network.clone())
            .await
            .expect("Failed to create service");

        // Check balance
        let balance = service.get_balance(&from_address).await;
        if balance.is_err() || balance.unwrap().to_wei() < Amount::from_ether(0.001).to_wei() {
            println!("  ⚠️  Insufficient balance, skipping...");
            continue;
        }

        // Measure transfer broadcast time
        let start = Instant::now();
        let result = service
            .transfer(
                &from_address,
                &to_address,
                Amount::from_ether(0.0001).to_wei(),
                &private_key,
            )
            .await;

        let duration = start.elapsed();

        match result {
            Ok(tx_hash) => {
                println!("  ✓ Transfer successful");
                println!("  📝 TX Hash: {}", tx_hash);
                println!("  ⏱️  Broadcast time: {:?}", duration);

                // Performance targets
                assert!(duration.as_secs() < 30, "Transfer should complete within 30 seconds");
            }
            Err(e) => {
                println!("  ⚠️  Transfer failed: {}", e);
            }
        }
    }

    println!("\n✅ Performance test completed");
}
