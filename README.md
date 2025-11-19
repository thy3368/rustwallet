# Rust Wallet - Multi-Chain Wallet (ETH & BSC)

A Clean Architecture implementation of a multi-chain wallet with balance queries and transfer functionality using Rust and Alloy.

## ✅ Completed Features

### Balance Query
- Query Ethereum and BSC address balances on multiple networks
- Support for Mainnet, Sepolia, Goerli, Holesky, BSC Mainnet, and BSC Testnet
- Custom RPC endpoint support
- Sub-second query latency (~277ms ETH, ~286ms BSC)

### Transfer Functionality ⭐ NEW
- **Full transaction signing and broadcasting**
- Send ETH on Ethereum networks (Mainnet, Sepolia)
- Send BNB on BSC networks (Mainnet, Testnet)
- Private key validation and security checks
- Balance verification before transfer
- Comprehensive error handling
- Production-ready implementation

### Architecture
- Clean Architecture with CQRS pattern
- Type-safe domain modeling with value objects
- Dependency injection with trait-based services
- Low-latency optimizations

## Architecture

This project follows Clean Architecture principles:

```
src/
├── core/
│   ├── domain/              # Core business logic (no external dependencies)
│   │   ├── value_objects/  # Address, Balance, Amount, Network, TransactionHash
│   │   ├── queries/        # GetBalanceQuery
│   │   ├── commands/       # TransferCommand, TransferResult
│   │   ├── services/       # Trait interfaces (BlockchainService, QueryHandler)
│   │   └── errors/         # Domain errors
│   └── application/         # Use case orchestration
│       └── handlers/       # GetBalanceHandler
├── adapter/
│   ├── infrastructure/      # External integrations
│   │   └── blockchain/     # Alloy-based blockchain service (with transfer)
│   └── interfaces/          # User interfaces
│       └── cli/            # Command-line interface
└── tests/                   # Integration tests
    ├── balance_query_integration_test.rs
    ├── bsc_balance_integration_test.rs
    └── transfer_execution_test.rs
```

## Quick Start

### Build
```bash
cargo build --release
```

### Query Balance

Query Vitalik's balance on Ethereum mainnet:
```bash
cargo run -- balance --address "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045" --network mainnet
```

**Example Output:**
```
✅ Balance Query Result:
   Address:  0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
   Network:  Mainnet (Chain ID: 1)
   Balance:  3.762294 ETH
   Wei:      3762293940150460114 Wei
```

### Query BSC Balance

Query a BSC address on BSC Mainnet:
```bash
cargo run -- balance --address "0x8894E0a0c962CB723c1976a4421c95949bE2D4E3" --network bsc
```

## Transfer Funds ⭐ NEW

The wallet now supports full transaction signing and broadcasting on both Ethereum and BSC networks.

### Integration Tests

Run transfer integration tests (requires test funds and environment setup):

```bash
# Setup environment variables
export TEST_PRIVATE_KEY="your_64_char_hex_private_key"
export TEST_FROM_ADDRESS="0x..."
export TEST_TO_ADDRESS="0x..."

# Run all transfer tests
cargo test --test transfer_execution_test -- --ignored --nocapture

# Run specific test
cargo test --test transfer_execution_test test_eth_transfer_sepolia -- --ignored --nocapture
```

### Get Test Funds

- **Sepolia ETH**: https://sepoliafaucet.com/
- **BSC Testnet BNB**: https://testnet.bnbchain.org/faucet-smart

### Programmatic Usage

```rust
use rustwallet::core::domain::{
    services::BlockchainService,
    value_objects::{Address, Amount, Network},
};
use rustwallet::adapter::infrastructure::blockchain::AlloyBlockchainService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = Address::new("0x...".to_string())?;
    let to = Address::new("0x...".to_string())?;
    let amount = Amount::from_ether(0.001);

    let service = AlloyBlockchainService::new_with_default_rpc(
        Network::Sepolia
    ).await?;

    let tx_hash = service.transfer(
        &from,
        &to,
        amount.to_wei(),
        "your_private_key",
    ).await?;

    println!("Transfer successful! TX: {}", tx_hash);
    Ok(())
}
```

### Security Notes

⚠️ **IMPORTANT**:
- Never commit private keys to version control
- Use environment variables or secure key storage
- Test on testnets before mainnet
- Verify addresses before sending transactions
- The implementation includes balance checks and address validation

## Documentation

### Implementation Docs
- [TRANSFER_IMPLEMENTATION_COMPLETE.md](TRANSFER_IMPLEMENTATION_COMPLETE.md) - Full implementation details
- [TRANSFER_FEATURE_DESIGN.md](TRANSFER_FEATURE_DESIGN.md) - Original design specification
- [design/design.md](design/design.md) - Domain model documentation

### Test Coverage
- Balance Query: 18 tests (10 unit + 8 integration)
- Transfer: 7 integration tests
- All tests passing
