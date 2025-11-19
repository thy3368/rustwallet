# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

##
用中文

## Project Overview

Multi-chain cryptocurrency wallet built with Rust, following **Clean Architecture**, **CQRS**, and **Event Sourcing** patterns. Currently supports Ethereum (ETH) and Binance Smart Chain (BSC) networks using the Alloy SDK.

**Key Features:**
- Query wallet balances across multiple networks (Mainnet, Sepolia, Goerli, Holesky, BSC)
- Transfer funds between addresses
- Type-safe domain modeling with value objects
- Low-latency optimizations following performance standards

## Build and Development Commands

### Build
```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release
```

### Run CLI
```bash
# Query balance (default: Sepolia testnet)
cargo run -- balance --address "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045" --network mainnet

# Query balance on BSC
cargo run -- balance --address "0xYourAddress" --network bsc

# Use custom RPC endpoint
cargo run -- balance --address "0xYourAddress" --network mainnet --rpc-url "https://eth.llamarpc.com"
```

### Testing

**Run all unit tests (no network required):**
```bash
cargo test
```

**Run specific integration test suite:**
```bash
# Balance query integration tests
cargo test --test balance_query_integration_test -- --ignored --nocapture

# BSC integration tests
cargo test --test bsc_balance_integration_test -- --ignored --nocapture

# Transfer integration tests
cargo test --test transfer_integration_test -- --ignored --nocapture
```

**Run single integration test:**
```bash
cargo test --test balance_query_integration_test test_get_balance_mainnet_integration -- --ignored --nocapture
```

**Notes:**
- Integration tests are marked with `#[ignore]` as they require network access
- Use `--nocapture` to see println! output during tests
- Use `--ignored` flag to run network-dependent tests

## Architecture

This project strictly follows **Clean Architecture** with dependencies pointing inward:

```
src/
├── core/                          # Inner layers (no external dependencies)
│   ├── domain/                    # Domain layer - pure business logic
│   │   ├── value_objects/         # Address, Balance, Network, Amount, TransactionHash
│   │   ├── queries/               # CQRS Query objects (GetBalanceQuery)
│   │   ├── commands/              # CQRS Command objects (TransferCommand)
│   │   ├── services/              # Trait interfaces (BlockchainService, QueryHandler)
│   │   └── errors/                # Domain errors
│   └── application/               # Application layer - use case orchestration
│       └── handlers/              # Query/Command handler implementations
│
├── adapter/                       # Outer layers
│   ├── infrastructure/            # Infrastructure layer - external integrations
│   │   └── blockchain/            # Alloy-based blockchain service
│   └── interfaces/                # Interface layer - user interfaces
│       └── cli/                   # CLI implementation
│
└── main.rs                        # Application entry point
```

### Clean Architecture Rules

1. **Dependency Direction**: Outer layers depend on inner layers, never the reverse
2. **Domain Purity**: `core/domain/` has ZERO external dependencies (no Alloy, no tokio traits)
3. **Trait Boundaries**: Domain defines trait interfaces (`BlockchainService`), infrastructure provides implementations
4. **Value Objects**: All domain primitives are wrapped in type-safe value objects (e.g., `Address`, `Balance`, `Network`)

### Key Architectural Patterns

**CQRS (Command Query Responsibility Segregation):**
- Queries: `GetBalanceQuery` → `GetBalanceHandler` → `BalanceQueryResult`
- Commands: `TransferCommand` → `TransferHandler` → `TransferResult`

**Dependency Injection:**
- Services are injected as `Arc<dyn Trait>` for runtime polymorphism
- Example: `GetBalanceHandler::new(blockchain_service: Arc<dyn BlockchainService>)`

**Value Objects:**
- `Address`: Validates Ethereum address format (0x + 40 hex chars)
- `Balance`: Wei-based balance with ETH conversion utilities
- `Network`: Enum with chain IDs and default RPC URLs
- `Amount`: Type-safe transfer amounts
- `TransactionHash`: Type-safe transaction hash

## Domain Layer Details

### Supported Networks

The `Network` enum in `src/core/domain/value_objects/network.rs` supports:
- **Ethereum**: Mainnet (chain 1), Sepolia (11155111), Goerli (5), Holesky (17000)
- **BSC**: BscMainnet (56), BscTestnet (97)
- **Custom**: Define custom networks with chain ID and RPC URL

Default RPC URLs are embedded in the `Network` type.

### BlockchainService Interface

Defined in `src/core/domain/services/mod.rs`:

```rust
#[async_trait]
pub trait BlockchainService: Send + Sync {
    async fn get_balance(&self, address: &Address) -> Result<Balance, DomainError>;
    async fn transfer(&self, from: &Address, to: &Address, amount: u128, private_key: &str) -> Result<TransactionHash, DomainError>;
    async fn is_connected(&self) -> bool;
    async fn get_block_number(&self) -> Result<u64, DomainError>;
}
```

Implementation: `AlloyBlockchainService` in `src/adapter/infrastructure/blockchain/alloy_service.rs`

### Error Handling

Domain errors in `src/core/domain/errors/mod.rs`:
- `InvalidAddress`: Malformed address format
- `NetworkError`: RPC connection failures
- `InsufficientBalance`: Not enough funds for transfer
- `TransferFailed`: Transaction execution failed
- `BlockchainError`: Generic blockchain errors

## Implementation Status

**Completed Features:**
- ✅ Balance query on ETH and BSC networks
- ✅ Clean Architecture foundation
- ✅ CQRS pattern implementation
- ✅ Value object validation
- ✅ Integration test framework
- ✅ Transfer command structure (design complete)

**In Progress:**
- 🔨 Transfer functionality (domain layer complete, Alloy integration pending)
- 🔨 Private key security and encryption
- 🔨 Gas estimation and EIP-1559 support

**Planned:**
- 📋 Wallet creation
- 📋 Event Sourcing implementation
- 📋 Transaction history
- 📋 ERC-20/BEP-20 token support

## Important Design Decisions

### Why Clean Architecture?

This project prioritizes **testability** and **maintainability** over quick prototyping:
- Domain logic can be tested without blockchain connections
- Blockchain provider (Alloy) can be swapped without touching domain code
- Business rules are isolated from infrastructure concerns

### Type Safety Philosophy

Every primitive is wrapped in a value object:
- Prevents mixing up addresses and transaction hashes
- Validates data at construction time
- Self-documenting function signatures

Example:
```rust
// ❌ Bad: Primitive obsession
async fn transfer(from: String, to: String, amount: u128) -> Result<String, Error>

// ✅ Good: Type-safe domain modeling
async fn transfer(from: &Address, to: &Address, amount: u128) -> Result<TransactionHash, DomainError>
```

### Performance Considerations

This project follows low-latency standards defined in the global CLAUDE.md:

**Rust Release Profile** (Cargo.toml):
```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = "fat"             # Link-time optimization
codegen-units = 1       # Single codegen unit for better optimization
panic = "abort"         # Smaller binaries, faster panics
```

**Hot Path Optimizations:**
- Value objects use `Copy` trait where possible to avoid allocations
- `Arc<T>` for shared immutable data
- Async I/O for blockchain operations (Tokio runtime)

## Testing Philosophy

**Unit Tests**: In-module tests for value objects and domain logic
```rust
#[cfg(test)]
mod tests {
    // Test domain rules without network access
}
```

**Integration Tests**: Network-dependent tests in `tests/` directory
- Marked with `#[ignore]` to prevent CI failures
- Use real blockchain RPC endpoints
- Query well-known addresses (e.g., Vitalik's address for balance verification)

**Test Coverage:**
- Value object validation (address format, amount validation)
- Network connectivity and block number retrieval
- Balance query workflow (end-to-end)
- Multi-network support (ETH, BSC)
- Custom RPC URL support

## Common Development Scenarios

### Adding a New Network

1. Add variant to `Network` enum in `src/core/domain/value_objects/network.rs`
2. Implement `chain_id()` and `default_rpc_url()` methods
3. Update CLI network parsing in `src/adapter/interfaces/cli/mod.rs`
4. Add integration test in `tests/`

### Adding a New Query

1. Define query struct in `src/core/domain/queries/mod.rs`
2. Create handler in `src/core/application/handlers/`
3. Implement `QueryHandler<YourQuery>` trait
4. Add CLI command in `src/adapter/interfaces/cli/mod.rs`

### Working with Alloy

The `AlloyBlockchainService` wraps Alloy SDK types:
- `alloy::primitives::Address` ↔ domain `Address` (conversion at boundary)
- `alloy::primitives::U256` ↔ domain `Balance` (u128 conversion)
- Provider is created with `ProviderBuilder::new().on_http(rpc_url)`

**Key Pattern**: Domain types at API boundaries, Alloy types internal to infrastructure layer.

## Security Notes

**Private Key Handling (CRITICAL):**
- Private keys are currently passed as strings for testing
- Production implementation requires encrypted keystore
- Never commit private keys to version control
- Use environment variables for testing: `PRIVATE_KEY=0x...`

**Transaction Validation:**
- Balance checks before transfers (implemented)
- Address validation at construction time
- Private key verification matches sender address

## Documentation References

- **Domain Model**: See `design/design.md` for comprehensive domain modeling documentation
- **Transfer Feature**: See `TRANSFER_FEATURE_DESIGN.md` for transfer workflow and security considerations
- **Test Documentation**: See `tests/README.md` for integration test details
- **BSC Integration**: See `BSC_INTEGRATION_SUMMARY.md` for BSC-specific implementation notes

## Dependencies

**Core:**
- `tokio`: Async runtime
- `async-trait`: Async trait support
- `alloy`: Ethereum/BSC SDK (version 0.6 with full features)

**Utilities:**
- `serde`, `serde_json`: Serialization
- `thiserror`, `anyhow`: Error handling
- `clap`: CLI argument parsing
- `tracing`: Logging infrastructure

## Code Style Guidelines

1. **Follow Clean Architecture boundaries** - Never import infrastructure types into domain layer
2. **Use value objects** - Wrap all primitives in domain types
3. **Async by default** - All I/O operations use async/await
4. **Error propagation** - Use `?` operator, return `Result` types
5. **Documentation** - Document public APIs and complex business logic
6. **Testing** - Add unit tests for domain logic, integration tests for external interactions
