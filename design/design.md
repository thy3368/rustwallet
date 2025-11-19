# Rust Wallet Domain Model Design

## Project Overview
Multi-chain Ethereum wallet using Clean Architecture + CQRS + Event Sourcing patterns.

## Architecture Principles
- **Clean Architecture**: Dependencies point inward, domain layer is independent
- **CQRS**: Command Query Responsibility Segregation
- **Event Sourcing**: Rebuild state from event stream
- **Low Latency**: Follows nanosecond-level performance standards

---

## 1. Domain Entities

### 1.1 Wallet Aggregate Root

```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Wallet aggregate root - core domain entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    /// Unique wallet identifier
    id: WalletId,
    /// Ethereum address
    address: Address,
    /// Current balance (in Wei)
    balance: Balance,
    /// Network type
    network: Network,
    /// Creation timestamp
    created_at: SystemTime,
    /// Event version (for Event Sourcing)
    version: u64,
}

impl Wallet {
    /// Create new wallet (factory method)
    pub fn create(id: WalletId, address: Address, network: Network) -> Result<Self, DomainError> {
        // Validate address format
        address.validate()?;

        Ok(Self {
            id,
            address,
            balance: Balance::zero(),
            network,
            created_at: SystemTime::now(),
            version: 0,
        })
    }

    /// Update balance (domain behavior)
    pub fn update_balance(&mut self, new_balance: Balance) -> Result<(), DomainError> {
        if new_balance.is_negative() {
            return Err(DomainError::InvalidBalance);
        }
        self.balance = new_balance;
        self.version += 1;
        Ok(())
    }

    /// Validate transfer (domain rule)
    pub fn can_transfer(&self, amount: Amount) -> Result<(), DomainError> {
        if amount.is_zero() {
            return Err(DomainError::ZeroAmount);
        }
        if amount > self.balance.as_amount() {
            return Err(DomainError::InsufficientBalance);
        }
        Ok(())
    }

    /// Apply domain event (Event Sourcing)
    pub fn apply_event(&mut self, event: &WalletEvent) {
        match event {
            WalletEvent::WalletCreated { id, address, network, .. } => {
                self.id = id.clone();
                self.address = address.clone();
                self.network = network.clone();
                self.version += 1;
            }
            WalletEvent::BalanceUpdated { new_balance, .. } => {
                self.balance = new_balance.clone();
                self.version += 1;
            }
            WalletEvent::TransferCompleted { .. } => {
                // Balance already confirmed on-chain
                self.version += 1;
            }
            _ => {}
        }
    }

    // Getters
    pub fn id(&self) -> &WalletId { &self.id }
    pub fn address(&self) -> &Address { &self.address }
    pub fn balance(&self) -> &Balance { &self.balance }
    pub fn network(&self) -> &Network { &self.network }
    pub fn version(&self) -> u64 { self.version }
}
```

---

## 2. Value Objects

### 2.1 WalletId

```rust
use uuid::Uuid;

/// Wallet unique identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletId(Uuid);

impl WalletId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| DomainError::InvalidWalletId)
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
```

### 2.2 Address

```rust
/// Ethereum address (42 characters, starts with 0x)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    pub fn new(addr: String) -> Result<Self, DomainError> {
        let instance = Self(addr);
        instance.validate()?;
        Ok(instance)
    }

    /// Validate Ethereum address format
    pub fn validate(&self) -> Result<(), DomainError> {
        if !self.0.starts_with("0x") {
            return Err(DomainError::InvalidAddressFormat);
        }
        if self.0.len() != 42 {
            return Err(DomainError::InvalidAddressLength);
        }
        // Check hex characters
        if !self.0[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidAddressCharacters);
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### 2.3 Balance

```rust
use std::fmt;

/// Balance (in Wei, smallest unit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Balance(u128);

impl Balance {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn from_wei(wei: u128) -> Self {
        Self(wei)
    }

    pub fn from_ether(ether: f64) -> Self {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        Self((ether * WEI_PER_ETHER as f64) as u128)
    }

    pub fn to_wei(&self) -> u128 {
        self.0
    }

    pub fn to_ether(&self) -> f64 {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        self.0 as f64 / WEI_PER_ETHER as f64
    }

    pub fn is_negative(&self) -> bool {
        false // u128 is always non-negative
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn as_amount(&self) -> Amount {
        Amount(self.0)
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ETH", self.to_ether())
    }
}
```

### 2.4 Amount

```rust
/// Transfer amount (in Wei)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(u128);

impl Amount {
    pub fn from_wei(wei: u128) -> Self {
        Self(wei)
    }

    pub fn from_ether(ether: f64) -> Self {
        const WEI_PER_ETHER: u128 = 1_000_000_000_000_000_000;
        Self((ether * WEI_PER_ETHER as f64) as u128)
    }

    pub fn to_wei(&self) -> u128 {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
```

### 2.5 Network

```rust
/// Ethereum network types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Goerli,
    Sepolia,
    Custom { name: String, chain_id: u64 },
}

impl Network {
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => 1,
            Network::Goerli => 5,
            Network::Sepolia => 11155111,
            Network::Custom { chain_id, .. } => *chain_id,
        }
    }
}
```

### 2.6 TransactionHash

```rust
/// Ethereum transaction hash
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionHash(String);

impl TransactionHash {
    pub fn new(hash: String) -> Result<Self, DomainError> {
        if !hash.starts_with("0x") || hash.len() != 66 {
            return Err(DomainError::InvalidTransactionHash);
        }
        Ok(Self(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

---

## 3. Domain Events

```rust
use chrono::{DateTime, Utc};

/// Wallet domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletEvent {
    /// Wallet created
    WalletCreated {
        id: WalletId,
        address: Address,
        network: Network,
        timestamp: DateTime<Utc>,
    },

    /// Balance updated
    BalanceUpdated {
        wallet_id: WalletId,
        old_balance: Balance,
        new_balance: Balance,
        timestamp: DateTime<Utc>,
    },

    /// Transfer initiated
    TransferInitiated {
        wallet_id: WalletId,
        to_address: Address,
        amount: Amount,
        tx_hash: TransactionHash,
        timestamp: DateTime<Utc>,
    },

    /// Transfer completed
    TransferCompleted {
        wallet_id: WalletId,
        tx_hash: TransactionHash,
        amount: Amount,
        timestamp: DateTime<Utc>,
    },

    /// Transfer failed
    TransferFailed {
        wallet_id: WalletId,
        tx_hash: Option<TransactionHash>,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}

impl WalletEvent {
    /// Get wallet ID associated with event
    pub fn wallet_id(&self) -> &WalletId {
        match self {
            WalletEvent::WalletCreated { id, .. } => id,
            WalletEvent::BalanceUpdated { wallet_id, .. } => wallet_id,
            WalletEvent::TransferInitiated { wallet_id, .. } => wallet_id,
            WalletEvent::TransferCompleted { wallet_id, .. } => wallet_id,
            WalletEvent::TransferFailed { wallet_id, .. } => wallet_id,
        }
    }

    /// Get event timestamp
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            WalletEvent::WalletCreated { timestamp, .. } => timestamp,
            WalletEvent::BalanceUpdated { timestamp, .. } => timestamp,
            WalletEvent::TransferInitiated { timestamp, .. } => timestamp,
            WalletEvent::TransferCompleted { timestamp, .. } => timestamp,
            WalletEvent::TransferFailed { timestamp, .. } => timestamp,
        }
    }
}
```

---

## 4. CQRS Commands

```rust
/// Create wallet command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletCommand {
    pub wallet_id: WalletId,
    pub network: Network,
    // Private key managed securely by infrastructure layer
}

/// Get balance command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceCommand {
    pub wallet_id: WalletId,
}

/// Transfer command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCommand {
    pub from_wallet_id: WalletId,
    pub to_address: Address,
    pub amount: Amount,
    pub gas_price: Option<u128>,
}
```

---

## 5. CQRS Queries

```rust
/// Get wallet by ID query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletByIdQuery {
    pub wallet_id: WalletId,
}

/// Get wallet by address query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletByAddressQuery {
    pub address: Address,
    pub network: Network,
}

/// Get balance query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceQuery {
    pub wallet_id: WalletId,
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletQueryResult {
    pub id: WalletId,
    pub address: Address,
    pub balance: Balance,
    pub network: Network,
    pub created_at: SystemTime,
}
```

---

## 6. Domain Service Trait Interfaces

### 6.1 Command Handlers

```rust
use async_trait::async_trait;

/// Command handler - processes write operations
#[async_trait]
pub trait CommandHandler<C>: Send + Sync {
    type Output;
    type Error;

    async fn handle(&self, command: C) -> Result<Self::Output, Self::Error>;
}

/// Create wallet command handler
#[async_trait]
pub trait CreateWalletHandler: CommandHandler<CreateWalletCommand, Output = WalletId, Error = DomainError> {}

/// Transfer command handler
#[async_trait]
pub trait TransferHandler: CommandHandler<TransferCommand, Output = TransactionHash, Error = DomainError> {}
```

### 6.2 Query Handlers

```rust
/// Query handler - processes read operations
#[async_trait]
pub trait QueryHandler<Q>: Send + Sync {
    type Output;
    type Error;

    async fn handle(&self, query: Q) -> Result<Self::Output, Self::Error>;
}

/// Get wallet query handler
#[async_trait]
pub trait GetWalletQueryHandler: QueryHandler<GetWalletByIdQuery, Output = WalletQueryResult, Error = DomainError> {}

/// Get balance query handler
#[async_trait]
pub trait GetBalanceQueryHandler: QueryHandler<GetBalanceQuery, Output = Balance, Error = DomainError> {}
```

### 6.3 Blockchain Service Interface

```rust
/// Ethereum blockchain service interface
#[async_trait]
pub trait BlockchainService: Send + Sync {
    /// Generate new address (via private key)
    async fn generate_address(&self) -> Result<(Address, String), BlockchainError>;

    /// Get account balance
    async fn get_balance(&self, address: &Address) -> Result<Balance, BlockchainError>;

    /// Send transaction
    async fn send_transaction(
        &self,
        from: &Address,
        to: &Address,
        amount: Amount,
        gas_price: Option<u128>,
        private_key: &str,
    ) -> Result<TransactionHash, BlockchainError>;

    /// Get transaction status
    async fn get_transaction_status(
        &self,
        tx_hash: &TransactionHash,
    ) -> Result<TransactionStatus, BlockchainError>;
}

/// Transaction status
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed { block_number: u64 },
    Failed { reason: String },
}
```

---

## 7. Repository Interfaces

### 7.1 Wallet Repository

```rust
/// Wallet repository - aggregate root persistence
#[async_trait]
pub trait WalletRepository: Send + Sync {
    /// Save wallet
    async fn save(&self, wallet: &Wallet) -> Result<(), RepositoryError>;

    /// Find wallet by ID
    async fn find_by_id(&self, id: &WalletId) -> Result<Option<Wallet>, RepositoryError>;

    /// Find wallet by address
    async fn find_by_address(&self, address: &Address) -> Result<Option<Wallet>, RepositoryError>;

    /// Delete wallet
    async fn delete(&self, id: &WalletId) -> Result<(), RepositoryError>;
}
```

### 7.2 Event Store Interface

```rust
/// Event store - Event Sourcing core
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Save single event
    async fn save_event(&self, event: &WalletEvent) -> Result<(), EventStoreError>;

    /// Save event batch (atomic)
    async fn save_events(&self, events: &[WalletEvent]) -> Result<(), EventStoreError>;

    /// Get all events for wallet
    async fn get_events(&self, wallet_id: &WalletId) -> Result<Vec<WalletEvent>, EventStoreError>;

    /// Rebuild wallet state from event stream
    async fn rebuild_wallet(&self, wallet_id: &WalletId) -> Result<Wallet, EventStoreError>;
}
```

---

## 8. Domain Error Types

```rust
use thiserror::Error;

/// Domain layer errors
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid wallet ID")]
    InvalidWalletId,

    #[error("Invalid address format")]
    InvalidAddressFormat,

    #[error("Invalid address length")]
    InvalidAddressLength,

    #[error("Invalid address characters")]
    InvalidAddressCharacters,

    #[error("Invalid balance")]
    InvalidBalance,

    #[error("Zero amount")]
    ZeroAmount,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid transaction hash")]
    InvalidTransactionHash,

    #[error("Wallet not found")]
    WalletNotFound,
}

/// Blockchain service errors
#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Gas price too high")]
    GasPriceTooHigh,
}

/// Repository errors
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Entity not found")]
    NotFound,

    #[error("Concurrency conflict")]
    ConcurrencyConflict,
}

/// Event store errors
#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("Event save failed: {0}")]
    SaveFailed(String),

    #[error("Event load failed: {0}")]
    LoadFailed(String),

    #[error("Empty event stream")]
    EmptyEventStream,
}
```

---

## 9. Directory Structure

```
src/
├── domain/                         # Domain layer (core)
│   ├── entities/                  # Entities
│   │   └── wallet.rs             # Wallet aggregate root
│   ├── value_objects/            # Value objects
│   │   ├── wallet_id.rs
│   │   ├── address.rs
│   │   ├── balance.rs
│   │   ├── amount.rs
│   │   ├── network.rs
│   │   └── transaction_hash.rs
│   ├── events/                    # Domain events
│   │   └── wallet_event.rs
│   ├── commands/                  # CQRS commands
│   │   ├── create_wallet.rs
│   │   ├── get_balance.rs
│   │   └── transfer.rs
│   ├── queries/                   # CQRS queries
│   │   ├── get_wallet_by_id.rs
│   │   └── get_balance_query.rs
│   ├── services/                  # Domain service interfaces
│   │   ├── command_handler.rs
│   │   ├── query_handler.rs
│   │   └── blockchain_service.rs
│   ├── repositories/              # Repository interfaces
│   │   ├── wallet_repository.rs
│   │   └── event_store.rs
│   └── errors.rs                  # Domain errors
│
├── application/                    # Application layer (use cases)
│   ├── handlers/                  # Command/query handler implementations
│   │   ├── create_wallet_handler.rs
│   │   ├── transfer_handler.rs
│   │   └── get_balance_handler.rs
│   └── services/                  # Application services
│       └── wallet_service.rs
│
├── infrastructure/                 # Infrastructure layer
│   ├── blockchain/                # Blockchain implementation (Alloy)
│   │   └── alloy_service.rs
│   ├── persistence/               # Persistence implementations
│   │   ├── postgres_wallet_repo.rs
│   │   └── postgres_event_store.rs
│   └── security/                  # Key management
│       └── keystore.rs
│
└── interfaces/                     # Interface layer
    ├── http/                      # HTTP API
    │   └── wallet_controller.rs
    └── cli/                       # Command-line interface
        └── wallet_cli.rs
```

---

## 10. Implementation Roadmap

### Phase 1: Domain Core (Week 1)
1. Define all value objects (Address, Balance, Amount, etc.)
2. Implement Wallet aggregate root
3. Define domain events
4. Define commands and queries
5. Define all trait interfaces

### Phase 2: Infrastructure (Week 2)
1. Integrate Alloy library for BlockchainService
2. Implement PostgreSQL EventStore
3. Implement WalletRepository
4. Implement key management (secure private key storage)

### Phase 3: Application Layer (Week 3)
1. Implement CreateWalletHandler
2. Implement TransferHandler
3. Implement QueryHandlers
4. Integrate Event Sourcing rebuild logic

### Phase 4: Interface Layer (Week 4)
1. Implement HTTP REST API
2. Implement CLI tool
3. Add logging and monitoring

---

## 11. Tech Stack

### Core Dependencies
```toml
[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"

# Ethereum SDK
alloy = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# UUID generation
uuid = { version = "1.0", features = ["v4", "serde"] }

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "chrono", "uuid"] }

# HTTP framework
axum = "0.7"
tower = "0.4"

# Configuration
config = "0.14"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 12. Performance Optimization

Following CLAUDE.md low-latency standards:

### Compiler Optimization
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

### Memory Optimization
- Value objects use Copy trait to reduce cloning
- Use Arc<T> for sharing immutable data
- Avoid dynamic allocation in hot paths

### Concurrency Optimization
- Use lock-free data structures
- Async I/O operations
- Pre-allocate database connection pools

---

## 13. Security Requirements

### Key Management
- Encrypt private keys (AES-256-GCM)
- Zero memory after use
- Support hardware wallet integration (future)

### Transaction Security
- Gas price limit checks
- Amount cap validation
- Transaction signature verification

---

## Summary

This design follows:
- Clean Architecture: Dependencies point inward, domain independence
- CQRS: Command-query separation
- Event Sourcing: Event-driven, auditable
- DDD: Aggregate roots, value objects, domain events
- Rust Best Practices: Type safety, zero-cost abstractions
- Low Latency: Performance optimization, lock-free concurrency

Next step: Start implementing the domain layer code.
