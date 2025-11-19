# BSC (Binance Smart Chain) Integration Testing

## Overview

This document describes the integration tests for BSC balance queries.

## BSC Network Information

| Network | Chain ID | RPC Endpoint |
|---------|----------|--------------|
| BSC Mainnet | 56 | https://bsc-dataseed.binance.org |
| BSC Testnet | 97 | https://data-seed-prebsc-1-s1.binance.org:8545 |

## Test Suite

### Available Tests

Total: **8 tests** (7 network-dependent, 1 standalone)

| Test Name | Description | Network Required |
|-----------|-------------|------------------|
| `test_get_balance_bsc_mainnet_integration` | End-to-end BSC balance query | ✅ Yes |
| `test_get_balance_bsc_testnet` | BSC testnet query | ✅ Yes |
| `test_bsc_service_connectivity` | BSC network connectivity | ✅ Yes |
| `test_query_multiple_bsc_addresses` | Query multiple BSC addresses | ✅ Yes |
| `test_bsc_address_format` | BSC/ETH address compatibility | ✅ Yes |
| `test_bsc_query_performance` | Performance measurement | ✅ Yes |
| `test_bsc_custom_rpc` | Custom RPC endpoints | ✅ Yes |
| `test_bsc_network_properties` | Network properties validation | ❌ No |

## Running Tests

### Run All BSC Tests (Network Required)
```bash
cargo test --test bsc_balance_integration_test -- --ignored --nocapture
```

### Run Single Test
```bash
cargo test --test bsc_balance_integration_test test_get_balance_bsc_mainnet_integration -- --ignored --nocapture
```

### Run Without Network (Properties Only)
```bash
cargo test --test bsc_balance_integration_test test_bsc_network_properties -- --nocapture
```

## Test Results

### Latest Test Run (2025-11-19)

#### ✅ test_bsc_network_properties
```
BSC Mainnet Chain ID: 56
BSC Testnet Chain ID: 97
✅ BSC network properties verified
Time: 0.00s
```

#### ✅ test_get_balance_bsc_mainnet_integration
```
Address: 0x28C6c06298d514Db089934071355E5743bf21d60 (Binance Hot Wallet)
Balance: 0.173085 BNB (173085191227471280 Wei)
✅ BSC Integration Test Passed
Time: 2.00s
```

#### ✅ test_bsc_service_connectivity
```
Connected to: BSC Mainnet
Current Block: #68755571
Block height validation: PASS (>20M)
✅ Connected to BSC Mainnet
Time: 1.72s
```

#### ✅ test_bsc_query_performance
```
Warm-up query: Success
Performance measurement:
- Query latency: 286.08ms
- Target: <5000ms
- Status: ✅ PASS (94.3% faster than target)
✅ BSC Query completed in 286ms
Time: 1.76s
```

## Performance Metrics

| Metric | Result | Target | Status |
|--------|--------|--------|--------|
| Query Latency (warm) | 286ms | <500ms | ✅ PASS |
| Query Latency (cold) | ~2000ms | <5000ms | ✅ PASS |
| Connection Check | ~1700ms | <3000ms | ✅ PASS |
| Block Number Query | <500ms | <1000ms | ✅ PASS |

### Comparison: BSC vs Ethereum

| Network | Query Latency | Block Time | Avg Block # |
|---------|---------------|------------|-------------|
| BSC Mainnet | ~286ms | ~3s | 68M+ |
| Ethereum Mainnet | ~277ms | ~12s | 23M+ |
| **Difference** | +3% slower | 4x faster | 3x higher |

## Test Addresses

### BSC Mainnet Test Addresses

| Address | Owner | Type | Expected Balance |
|---------|-------|------|------------------|
| `0x28C6c06298d514Db089934071355E5743bf21d60` | Binance Hot Wallet | EOA | Non-zero |
| `0x8894E0a0c962CB723c1976a4421c95949bE2D4E3` | Binance Hot Wallet 2 | EOA | Non-zero |
| `0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c` | WBNB Token | Contract | Variable |

## CLI Usage

### Query BSC Balance

```bash
cargo run -- balance --address "0x28C6c06298d514Db089934071355E5743bf21d60" --network bsc
```

**Output:**
```
🔍 Querying balance...
   Address: 0x28C6c06298d514Db089934071355E5743bf21d60
   Network: BSC Mainnet (Chain ID: 56)
   RPC URL: https://bsc-dataseed.binance.org
   Current Block: #68755603

✅ Balance Query Result:
   Address:  0x28C6c06298d514Db089934071355E5743bf21d60
   Network:  BSC Mainnet (Chain ID: 56)
   Balance:  0.173085 BNB
   Wei:      173085191227471280 Wei
```

### Query BSC Testnet

```bash
cargo run -- balance --address "0xYourAddress" --network bsc-testnet
```

### Use Custom BSC RPC

```bash
cargo run -- balance \
  --address "0xYourAddress" \
  --network bsc \
  --rpc-url "https://bsc-dataseed1.binance.org"
```

## Alternative BSC RPC Endpoints

| RPC URL | Provider | Reliability |
|---------|----------|-------------|
| `https://bsc-dataseed.binance.org` | Binance | ⭐⭐⭐⭐⭐ Default |
| `https://bsc-dataseed1.binance.org` | Binance | ⭐⭐⭐⭐⭐ |
| `https://bsc-dataseed2.binance.org` | Binance | ⭐⭐⭐⭐⭐ |
| `https://bsc-dataseed3.binance.org` | Binance | ⭐⭐⭐⭐ |
| `https://bsc-dataseed4.binance.org` | Binance | ⭐⭐⭐⭐ |

## Key Features

### ✅ Implemented

- [x] BSC Mainnet support
- [x] BSC Testnet support
- [x] Multiple RPC endpoint support
- [x] Address format compatibility (EVM)
- [x] High-performance queries (<500ms)
- [x] Network property validation
- [x] CLI integration
- [x] Comprehensive integration tests

### 🔄 In Progress

- [ ] BSC token balance queries (BEP-20)
- [ ] Transaction history
- [ ] Gas price estimation

### 📋 Planned

- [ ] BSC smart contract interaction
- [ ] Multi-signature wallet support
- [ ] Hardware wallet integration

## Troubleshooting

### Issue: Connection timeout

**Cause:** BSC RPC endpoint may be rate-limited or down

**Solution:**
1. Try alternative RPC endpoint:
   ```bash
   cargo run -- balance --address "0x..." --network bsc --rpc-url "https://bsc-dataseed1.binance.org"
   ```
2. Check https://chainlist.org for more BSC RPC endpoints

### Issue: Slow queries

**Cause:** Network latency or RPC endpoint congestion

**Solution:**
- Use geographically closer RPC endpoint
- Consider premium RPC services (QuickNode, Ankr, etc.)

### Issue: Invalid address

**Cause:** Address format incorrect

**Solution:**
- Ensure address starts with `0x`
- Verify address is 42 characters long
- Use checksum address format

## Network Characteristics

### BSC vs Ethereum Comparison

| Feature | BSC | Ethereum |
|---------|-----|----------|
| **Consensus** | PoSA (Proof of Staked Authority) | PoS (Proof of Stake) |
| **Block Time** | ~3 seconds | ~12 seconds |
| **Avg Gas Fee** | $0.10-0.50 | $5-50 |
| **EVM Compatible** | ✅ Yes | ✅ Native |
| **Native Token** | BNB | ETH |
| **Address Format** | Same as Ethereum | 0x... (42 chars) |

### Why BSC?

1. **Lower Fees**: 10-100x cheaper than Ethereum
2. **Faster Blocks**: 3s vs 12s block time
3. **EVM Compatible**: Same tooling as Ethereum
4. **High Throughput**: ~300 TPS vs ~15 TPS
5. **DeFi Ecosystem**: Large DeFi presence

## Implementation Details

### Code Structure

```
src/core/domain/value_objects/network.rs
  ├── Network::BscMainnet (Chain ID: 56)
  └── Network::BscTestnet (Chain ID: 97)

tests/bsc_balance_integration_test.rs
  ├── test_get_balance_bsc_mainnet_integration
  ├── test_bsc_service_connectivity
  ├── test_bsc_query_performance
  └── ... (5 more tests)
```

### Architecture

BSC integration uses the same Clean Architecture as Ethereum:

```
CLI/HTTP → Application Layer → Domain Layer → Infrastructure Layer (Alloy)
                                    ↓
                            BSC RPC Endpoint
```

## Best Practices

1. **Always specify network explicitly** to avoid confusion
2. **Use checksum addresses** for better error detection
3. **Handle rate limits** with exponential backoff
4. **Cache results** when appropriate
5. **Monitor RPC health** and have fallback endpoints

## Resources

- [BSC Documentation](https://docs.bnbchain.org)
- [BSC Scan Explorer](https://bscscan.com)
- [BSC RPC Endpoints](https://chainlist.org/?search=binance)
- [BSC GitHub](https://github.com/bnb-chain/bsc)

## Contributing

To add more BSC tests:

1. Add test function to `tests/bsc_balance_integration_test.rs`
2. Mark with `#[tokio::test]` and `#[ignore]` if network required
3. Update this documentation
4. Run all tests to ensure compatibility

## License

MIT
