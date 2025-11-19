# Rust Wallet - Ethereum Balance Query

A Clean Architecture implementation of an Ethereum wallet balance query using Rust and Alloy.

## ✅ Completed Features

- Query Ethereum address balance on multiple networks
- Clean Architecture with CQRS pattern
- Support for Mainnet, Sepolia, Goerli, and Holesky networks
- Custom RPC endpoint support
- Type-safe domain modeling with value objects
- Low-latency optimizations

## Architecture

This project follows Clean Architecture principles:

```
src/
├── domain/              # Core business logic (no external dependencies)
│   ├── value_objects/  # Address, Balance, Network
│   ├── queries/        # GetBalanceQuery
│   ├── services/       # Trait interfaces (BlockchainService, QueryHandler)
│   └── errors/         # Domain errors
├── application/         # Use case orchestration
│   └── handlers/       # GetBalanceHandler implementation
├── infrastructure/      # External integrations
│   └── blockchain/     # Alloy-based blockchain service
└── interfaces/          # User interfaces
    └── cli/            # Command-line interface
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

## Design Documentation

See [design/design.md](design/design.md) for detailed domain model documentation.
