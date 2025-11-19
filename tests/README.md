# Integration Tests

This directory contains integration tests for the Rust Wallet project.

## Test Structure

- `balance_query_integration_test.rs` - Complete integration tests for balance query use case

## Running Tests

### Run all unit tests (no network required)
```bash
cargo test
```

### Run all integration tests (requires network)
```bash
cargo test --test balance_query_integration_test -- --ignored
```

### Run specific integration test
```bash
cargo test --test balance_query_integration_test test_get_balance_mainnet_integration -- --ignored
```

### Run with output
```bash
cargo test --test balance_query_integration_test -- --ignored --nocapture
```

## Test Categories

### 1. End-to-End Integration Tests (Network Required)

#### `test_get_balance_mainnet_integration`
- Tests complete flow from query to result
- Queries Vitalik's address on mainnet
- Verifies balance retrieval and data correctness

#### `test_blockchain_service_connectivity`
- Tests connection to Ethereum network
- Verifies block number retrieval
- Checks network connectivity

#### `test_query_multiple_addresses`
- Tests querying multiple well-known addresses
- Verifies consistent behavior across different accounts

#### `test_different_networks`
- Tests service creation for different networks
- Covers: Mainnet, Sepolia, Goerli, Holesky

#### `test_custom_rpc_url`
- Tests using custom RPC endpoints
- Verifies flexibility in endpoint configuration

#### `test_query_performance`
- Measures query latency
- Ensures queries complete within acceptable time (<5s)

### 2. Unit Tests (No Network Required)

#### `test_invalid_address_error`
- Tests error handling for invalid addresses
- Covers various invalid address formats

## Expected Output

When running integration tests, you should see output like:

```
running 7 tests
test test_invalid_address_error ... ok
test test_blockchain_service_connectivity ... ok
✅ Connected to Ethereum mainnet at block #23832887

test test_get_balance_mainnet_integration ... ok
✅ Integration Test Passed - Balance: 3.762294 ETH (3762293940150460114 Wei)

test test_query_multiple_addresses ... ok
✅ Vitalik (0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045) - Balance: 3.762294 ETH
✅ WETH Contract (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2) - Balance: 0.000000 ETH

test test_different_networks ... ok
✅ Service created for Mainnet
✅ Service created for Sepolia
✅ Service created for Goerli
✅ Service created for Holesky

test test_custom_rpc_url ... ok
✅ Connected to custom RPC: https://eth.llamarpc.com

test test_query_performance ... ok
✅ Query completed in 287ms

test result: ok. 7 passed; 0 failed; 0 ignored
```

## CI/CD Integration

Integration tests are marked with `#[ignore]` to prevent them from running in CI pipelines by default (as they require network access).

To run in CI with network access:
```bash
cargo test -- --ignored
```

## Performance Benchmarks

The `test_query_performance` test measures query latency. Expected performance:
- First query (cold): < 2000ms
- Subsequent queries (warm): < 500ms
- Target: < 5000ms (enforced by assertion)

## Troubleshooting

### Network Connection Issues
If tests fail with network errors:
1. Check your internet connection
2. Try using a custom RPC URL: `--rpc-url "https://eth.llamarpc.com"`
3. Check if RPC endpoint is rate-limiting

### Testnet Issues
Public testnet RPC endpoints may be unreliable. For production use:
- Use Infura, Alchemy, or QuickNode
- Configure API keys in environment variables

## Adding New Tests

When adding new integration tests:

1. Mark network-dependent tests with `#[ignore]`:
```rust
#[tokio::test]
#[ignore]
async fn test_new_feature() {
    // test code
}
```

2. Add descriptive comments explaining what the test verifies

3. Include meaningful assertions and error messages

4. Update this README with test description
