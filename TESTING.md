# Testing Guide

## Test Structure Overview

```
rustwallet/
├── src/
│   ├── domain/value_objects/
│   │   ├── address.rs         # Unit tests for Address validation
│   │   ├── balance.rs         # Unit tests for Balance conversion
│   │   └── network.rs         # Unit tests for Network types
│   ├── application/handlers/
│   │   └── get_balance_handler.rs  # Handler unit tests with mocks
│   └── infrastructure/blockchain/
│       └── alloy_service.rs   # Infrastructure unit tests
└── tests/
    ├── balance_query_integration_test.rs  # Integration tests
    └── README.md              # Integration test documentation
```

## Test Categories

### 1. Unit Tests (No Network Required)

Located within the source files using `#[cfg(test)]` modules.

**Run all unit tests:**
```bash
cargo test --lib
```

**Example output:**
```
running 9 tests
test domain::value_objects::address::tests::test_valid_address ... ok
test domain::value_objects::address::tests::test_invalid_address_no_prefix ... ok
test domain::value_objects::address::tests::test_invalid_address_length ... ok
test domain::value_objects::balance::tests::test_balance_conversion ... ok
test domain::value_objects::balance::tests::test_zero_balance ... ok
test domain::value_objects::balance::tests::test_balance_display ... ok
test domain::value_objects::network::tests::test_network_chain_ids ... ok
test domain::value_objects::network::tests::test_network_is_testnet ... ok
test application::handlers::get_balance_handler::tests::test_get_balance_handler ... ok

test result: ok. 9 passed; 0 failed; 0 ignored
```

### 2. Integration Tests (Network Required)

Located in `tests/` directory, marked with `#[ignore]` attribute.

**Run all integration tests:**
```bash
cargo test --test balance_query_integration_test -- --ignored --nocapture
```

**Available integration tests:**

| Test Name | Description | Network Required |
|-----------|-------------|------------------|
| `test_get_balance_mainnet_integration` | End-to-end balance query | ✅ Yes |
| `test_blockchain_service_connectivity` | Network connectivity check | ✅ Yes |
| `test_query_multiple_addresses` | Query multiple addresses | ✅ Yes |
| `test_different_networks` | Multi-network support | ✅ Yes |
| `test_custom_rpc_url` | Custom RPC endpoint | ✅ Yes |
| `test_query_performance` | Performance measurement | ✅ Yes |
| `test_invalid_address_error` | Error handling | ❌ No |

**Test Results (Latest Run):**
```
✅ test_invalid_address_error ................. PASSED (0.00s)
✅ test_get_balance_mainnet_integration ....... PASSED (1.37s)
   Balance: 3.76229394015046 ETH
✅ test_query_performance ..................... PASSED (1.64s)
   Query completed in 277ms
```

## Performance Benchmarks

### Query Latency Results

Based on `test_query_performance`:

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Cold query | ~1400ms | <2000ms | ✅ PASS |
| Warm query | ~277ms | <500ms | ✅ PASS |
| Maximum allowed | - | <5000ms | ✅ PASS |

**Network:** Ethereum Mainnet
**RPC:** https://eth.llamarpc.com
**Test Date:** 2025-11-19

## Running Tests in CI/CD

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run integration tests
        run: cargo test --test balance_query_integration_test -- --ignored
```

## Test Coverage

### Domain Layer
- ✅ Address validation (invalid format, length, characters)
- ✅ Balance conversion (Wei ↔ Ether)
- ✅ Network type enumeration
- ✅ Query model serialization

### Application Layer
- ✅ GetBalanceHandler with mock blockchain service
- ✅ Error propagation
- ✅ Logging functionality

### Infrastructure Layer
- ✅ Alloy service integration
- ✅ Network connectivity
- ✅ Balance retrieval
- ✅ Block number queries

### Interface Layer
- ✅ CLI argument parsing (via manual testing)
- ✅ Output formatting (via manual testing)

## Test Data

### Well-Known Test Addresses

| Address | Name | Purpose |
|---------|------|---------|
| `0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045` | Vitalik Buterin | Has balance, stable |
| `0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2` | WETH Contract | Well-known contract |

### Test Networks

- ✅ Mainnet (Chain ID: 1) - Primary test target
- ⚠️ Sepolia (Chain ID: 11155111) - RPC issues
- ⚠️ Goerli (Chain ID: 5) - Deprecated
- ⚠️ Holesky (Chain ID: 17000) - Limited RPC availability

## Debugging Tests

### Enable detailed logging

```bash
RUST_LOG=debug cargo test --test balance_query_integration_test test_get_balance_mainnet_integration -- --ignored --nocapture
```

### Run specific test with backtrace

```bash
RUST_BACKTRACE=1 cargo test test_name -- --ignored --nocapture
```

### Check test compilation without running

```bash
cargo test --no-run
```

## Adding New Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Integration Test Template

```rust
#[tokio::test]
#[ignore]
async fn test_integration_feature() {
    // Arrange
    let service = create_test_service().await;

    // Act
    let result = service.perform_action().await;

    // Assert
    assert!(result.is_ok());
    println!("✅ Test passed: {:?}", result);
}
```

## Known Issues

1. **Sepolia RPC Connectivity**
   - Public Sepolia RPC endpoints are unreliable
   - Solution: Use Infura/Alchemy with API key

2. **Rate Limiting**
   - Public RPC endpoints may rate-limit
   - Solution: Add retry logic or use paid RPC

3. **Network Latency**
   - Tests may fail on slow connections
   - Solution: Increase timeout thresholds

## Test Maintenance

### Weekly Tasks
- [ ] Run full integration test suite
- [ ] Check performance benchmarks
- [ ] Verify test addresses still have balances

### Monthly Tasks
- [ ] Update test dependencies
- [ ] Review and update RPC endpoints
- [ ] Check for deprecated networks

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
