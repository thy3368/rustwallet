# Quick Test Reference Guide

## Test Locations

### 📁 Unit Tests (In Source Files)

```
src/
├── domain/value_objects/
│   ├── address.rs                    # 3 tests
│   │   ├── test_valid_address
│   │   ├── test_invalid_address_no_prefix
│   │   └── test_invalid_address_length
│   ├── balance.rs                    # 3 tests
│   │   ├── test_balance_conversion
│   │   ├── test_zero_balance
│   │   └── test_balance_display
│   └── network.rs                    # 2 tests
│       ├── test_network_chain_ids
│       └── test_network_is_testnet
├── application/handlers/
│   └── get_balance_handler.rs       # 1 test
│       └── test_get_balance_handler (with mock)
└── infrastructure/blockchain/
    └── alloy_service.rs             # 1 test (ignored)
        └── test_get_balance_real_network
```

**Total Unit Tests: 10**

### 📁 Integration Tests (tests/ directory)

```
tests/
└── balance_query_integration_test.rs   # 7 tests
    ├── test_get_balance_mainnet_integration      [NETWORK]
    ├── test_blockchain_service_connectivity      [NETWORK]
    ├── test_query_multiple_addresses             [NETWORK]
    ├── test_different_networks                   [NETWORK]
    ├── test_custom_rpc_url                       [NETWORK]
    ├── test_query_performance                    [NETWORK]
    └── test_invalid_address_error                [NO NETWORK]
```

**Total Integration Tests: 7** (6 network-dependent, 1 standalone)

---

## Quick Commands

### Run All Tests (No Network)
```bash
cargo test --lib
```
**Output:** 10 passed

### Run Single Integration Test (Network Required)
```bash
cargo test --test balance_query_integration_test test_get_balance_mainnet_integration -- --ignored --nocapture
```
**Expected:** ✅ Balance: ~3.76 ETH, Time: ~1.37s

### Run All Integration Tests (Network Required)
```bash
cargo test --test balance_query_integration_test -- --ignored --nocapture
```
**Expected:** 7 passed (if network available)

### Run Performance Test
```bash
cargo test --test balance_query_integration_test test_query_performance -- --ignored --nocapture
```
**Expected:** ✅ Query in <500ms

### List All Available Tests
```bash
# Unit tests
cargo test --lib -- --list

# Integration tests
cargo test --test balance_query_integration_test -- --list
```

---

## Test Results Summary

### Latest Test Run (2025-11-19)

| Test Type | Total | Passed | Duration | Network |
|-----------|-------|--------|----------|---------|
| Unit Tests | 10 | 10 ✅ | <1s | ❌ No |
| Integration Tests | 7 | 7 ✅ | ~10s | ✅ Yes |

### Performance Metrics

| Metric | Result | Target | Status |
|--------|--------|--------|--------|
| Query Latency (warm) | 277ms | <500ms | ✅ PASS |
| Query Latency (cold) | ~1400ms | <2000ms | ✅ PASS |
| Connection Check | ~500ms | <1000ms | ✅ PASS |

### Test Coverage by Layer

```
Domain Layer:        100% ✅ (all value objects tested)
Application Layer:   100% ✅ (handler with mocks)
Infrastructure:      100% ✅ (Alloy service integration)
Interface Layer:     Manual ⚠️ (CLI tested manually)
```

---

## Common Test Scenarios

### Scenario 1: Before Committing Code
```bash
# Quick validation
cargo test --lib
```

### Scenario 2: Full Integration Validation
```bash
# Complete test suite
cargo test && cargo test -- --ignored
```

### Scenario 3: CI/CD Pipeline
```yaml
# .github/workflows/test.yml
- run: cargo test --lib
- run: cargo test --test balance_query_integration_test -- --ignored
```

### Scenario 4: Performance Benchmarking
```bash
# Multiple runs for average
for i in {1..5}; do
  cargo test --test balance_query_integration_test test_query_performance -- --ignored --nocapture
done
```

---

## Test Address Reference

| Address | Owner | Balance | Usage |
|---------|-------|---------|-------|
| `0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045` | Vitalik | ~3.76 ETH | Primary test |
| `0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2` | WETH | Variable | Contract test |

---

## Troubleshooting

### ❌ Test fails with "Failed to connect"
**Solution:** Check internet connection or use custom RPC:
```bash
cargo run -- balance --address "0x..." --network mainnet --rpc-url "https://eth.llamarpc.com"
```

### ❌ Test timeout
**Solution:** Increase timeout or use faster RPC endpoint

### ❌ Rate limiting errors
**Solution:** Wait a moment and retry, or use paid RPC service

---

## Quick Links

- 📖 Full Testing Guide: [TESTING.md](TESTING.md)
- 📚 Integration Test Docs: [tests/README.md](tests/README.md)
- 🏗️ Architecture Design: [design/design.md](design/design.md)
