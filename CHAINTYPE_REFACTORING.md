# ChainType 重构文档

## 概述

本次重构为 `GetBalanceQuery` 增加了 `ChainType` 字段，使得同一接口可以查询不同区块链（Ethereum、Bitcoin、Solana）的余额。

## 新增组件

### 1. ChainType 枚举

**文件**: `src/core/domain/value_objects/chain_type.rs`

```rust
pub enum ChainType {
    /// Ethereum and EVM-compatible chains (Ethereum, BSC, Polygon, etc.)
    Ethereum,
    /// Bitcoin and Bitcoin-based chains
    Bitcoin,
    /// Solana
    Solana,
}
```

#### ChainType 提供的方法

| 方法 | 返回值 | 说明 |
|-----|--------|------|
| `name()` | `&str` | 获取链类型名称 ("Ethereum", "Bitcoin", "Solana") |
| `native_currency()` | `&str` | 获取原生代币符号 ("ETH", "BTC", "SOL") |
| `smallest_unit()` | `&str` | 获取最小单位名称 ("Wei", "Satoshi", "Lamport") |
| `decimals()` | `u8` | 获取小数位数 (18, 8, 9) |

#### 使用示例

```rust
use rustwallet::core::domain::value_objects::ChainType;

let chain = ChainType::Ethereum;
println!("Chain: {}", chain.name());              // "Ethereum"
println!("Currency: {}", chain.native_currency()); // "ETH"
println!("Unit: {}", chain.smallest_unit());       // "Wei"
println!("Decimals: {}", chain.decimals());        // 18
```

### 2. Network 新增 chain_type() 方法

**文件**: `src/core/domain/value_objects/network.rs`

```rust
impl Network {
    /// Get the chain type for this network
    pub fn chain_type(&self) -> ChainType {
        if self.is_bitcoin() {
            ChainType::Bitcoin
        } else if self.is_solana() {
            ChainType::Solana
        } else {
            ChainType::Ethereum  // EVM networks
        }
    }
}
```

#### 使用示例

```rust
use rustwallet::core::domain::value_objects::Network;

// Ethereum networks
assert_eq!(Network::Mainnet.chain_type(), ChainType::Ethereum);
assert_eq!(Network::Sepolia.chain_type(), ChainType::Ethereum);
assert_eq!(Network::BscMainnet.chain_type(), ChainType::Ethereum);

// Bitcoin networks
assert_eq!(Network::BitcoinMainnet.chain_type(), ChainType::Bitcoin);

// Solana networks
assert_eq!(Network::SolanaMainnet.chain_type(), ChainType::Solana);
```

### 3. GetBalanceQuery 重构

**文件**: `src/core/domain/queries/mod.rs`

#### 变更前

```rust
pub struct GetBalanceQuery {
    pub address: Address,
    pub network: Network,
}
```

#### 变更后

```rust
pub struct GetBalanceQuery {
    pub address: Address,
    pub network: Network,
    pub chain_type: ChainType,  // ⭐ 新增字段
}
```

#### 构造函数

```rust
impl GetBalanceQuery {
    /// Create a new get balance query
    /// The chain_type is automatically derived from the network
    pub fn new(address: Address, network: Network) -> Self {
        let chain_type = network.chain_type();  // ⭐ 自动推导
        Self {
            address,
            network,
            chain_type,
        }
    }

    /// Create with explicit chain type (for testing/custom validation)
    pub fn new_with_chain_type(
        address: Address,
        network: Network,
        chain_type: ChainType,
    ) -> Self {
        Self { address, network, chain_type }
    }
}
```

### 4. BalanceQueryResult 同步更新

```rust
pub struct BalanceQueryResult {
    pub address: Address,
    pub network: Network,
    pub chain_type: ChainType,  // ⭐ 新增字段
    pub balance: Balance,
}
```

## 使用指南

### 基本用法：查询不同链的余额

#### Ethereum 查询

```rust
use rustwallet::core::domain::{
    queries::GetBalanceQuery,
    value_objects::{Address, Network},
};

// 创建查询 - chain_type 自动设置为 Ethereum
let eth_address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())?;
let query = GetBalanceQuery::new(eth_address, Network::Mainnet);

println!("Chain Type: {}", query.chain_type);  // "Ethereum"
println!("Network: {}", query.network);         // "Ethereum Mainnet (Chain ID: 1)"
```

#### Bitcoin 查询

```rust
// 创建查询 - chain_type 自动设置为 Bitcoin
let btc_address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())?;
let query = GetBalanceQuery::new(btc_address, Network::BitcoinMainnet);

println!("Chain Type: {}", query.chain_type);  // "Bitcoin"
println!("Network: {}", query.network);         // "Bitcoin Mainnet"
```

#### Solana 查询

```rust
// 创建查询 - chain_type 自动设置为 Solana
let sol_address = Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())?;
let query = GetBalanceQuery::new(sol_address, Network::SolanaMainnet);

println!("Chain Type: {}", query.chain_type);  // "Solana"
println!("Network: {}", query.network);         // "Solana Mainnet"
```

### 高级用法：基于 ChainType 路由

```rust
async fn query_balance(query: GetBalanceQuery) -> Result<BalanceQueryResult, DomainError> {
    // 根据 chain_type 路由到不同的服务
    match query.chain_type {
        ChainType::Ethereum => {
            // 使用 EVM 服务
            let service = AlloyBlockchainService::new_with_default_rpc(query.network).await?;
            let handler = GetBalanceHandler::new(Arc::new(service));
            handler.handle(query).await
        }
        ChainType::Bitcoin => {
            // 使用 Bitcoin 服务
            let service = BitcoinBlockchainService::new(query.network).await?;
            let handler = GetBalanceHandler::new(Arc::new(service));
            handler.handle(query).await
        }
        ChainType::Solana => {
            // 使用 Solana 服务
            let service = SolanaBlockchainService::new(query.network).await?;
            let handler = GetBalanceHandler::new(Arc::new(service));
            handler.handle(query).await
        }
    }
}
```

### 统一的多链查询接口示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义多链查询
    let queries = vec![
        ("ETH", GetBalanceQuery::new(
            Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())?,
            Network::Sepolia,
        )),
        ("BTC", GetBalanceQuery::new(
            Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())?,
            Network::BitcoinMainnet,
        )),
        ("SOL", GetBalanceQuery::new(
            Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())?,
            Network::SolanaMainnet,
        )),
    ];

    // 统一接口处理
    for (name, query) in queries {
        println!("Querying {} balance...", name);
        println!("  Chain Type: {}", query.chain_type);
        println!("  Currency: {}", query.chain_type.native_currency());

        let result = query_balance(query).await?;
        println!("  Balance: {} {}",
            result.balance.to_wei(),
            result.chain_type.smallest_unit()
        );
    }

    Ok(())
}
```

## 优势总结

### 1. 统一接口，支持多链

**之前**：需要为每条链创建不同的查询类型
```rust
// ❌ 不同的查询类型
let eth_query = EthBalanceQuery::new(...);
let btc_query = BtcBalanceQuery::new(...);
let sol_query = SolBalanceQuery::new(...);
```

**现在**：同一个 `GetBalanceQuery` 支持所有链
```rust
// ✅ 统一的查询接口
let eth_query = GetBalanceQuery::new(eth_addr, Network::Mainnet);
let btc_query = GetBalanceQuery::new(btc_addr, Network::BitcoinMainnet);
let sol_query = GetBalanceQuery::new(sol_addr, Network::SolanaMainnet);
```

### 2. 自动类型推导

`ChainType` 会自动从 `Network` 推导，无需手动指定：

```rust
let query = GetBalanceQuery::new(address, Network::BscMainnet);
// query.chain_type 自动设置为 ChainType::Ethereum (因为 BSC 是 EVM 链)
```

### 3. 类型安全的路由

可以使用 Rust 的 `match` 表达式进行编译时安全的路由：

```rust
match query.chain_type {
    ChainType::Ethereum => { /* EVM 逻辑 */ }
    ChainType::Bitcoin => { /* Bitcoin 逻辑 */ }
    ChainType::Solana => { /* Solana 逻辑 */ }
}
// 编译器会确保所有情况都被处理
```

### 4. 丰富的元数据

`ChainType` 提供了有用的元数据：

```rust
println!("Currency: {}", query.chain_type.native_currency());  // ETH/BTC/SOL
println!("Unit: {}", query.chain_type.smallest_unit());        // Wei/Satoshi/Lamport
println!("Decimals: {}", query.chain_type.decimals());         // 18/8/9
```

### 5. 向后兼容

所有现有代码无需修改，因为 `chain_type` 自动推导：

```rust
// 现有代码继续工作
let query = GetBalanceQuery::new(address, network);
// chain_type 自动添加
```

## 测试验证

运行以下命令验证 ChainType 功能：

```bash
# 运行 ChainType 相关测试
cargo test chain_type -- --nocapture

# 运行完整集成测试
cargo test --test bitcoin_solana_integration_test -- --nocapture
```

### 测试覆盖

- ✅ ChainType 自动识别（3个链 × 多个网络）
- ✅ GetBalanceQuery 包含 ChainType
- ✅ BalanceQueryResult 包含 ChainType
- ✅ 多链统一查询接口
- ✅ 基于 ChainType 的服务路由

## 架构图

```
┌─────────────────────────────────────────────────────────┐
│                    GetBalanceQuery                      │
│  ┌───────────┬────────────┬─────────────────────┐     │
│  │  Address  │  Network   │  ChainType (自动)    │     │
│  └───────────┴────────────┴─────────────────────┘     │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
          ┌──────────────┴──────────────┐
          │    match chain_type         │
          └──────────────┬──────────────┘
                         │
      ┌──────────────────┼──────────────────┐
      │                  │                  │
      ▼                  ▼                  ▼
┌───────────┐     ┌────────────┐    ┌─────────────┐
│ Ethereum  │     │  Bitcoin   │    │   Solana    │
│  Service  │     │  Service   │    │   Service   │
└───────────┘     └────────────┘    └─────────────┘
      │                  │                  │
      ▼                  ▼                  ▼
┌───────────┐     ┌────────────┐    ┌─────────────┐
│   Alloy   │     │blockchain  │    │  JSON-RPC   │
│           │     │   .info    │    │             │
└───────────┘     └────────────┘    └─────────────┘
```

## 相关文件

- `src/core/domain/value_objects/chain_type.rs` - ChainType 定义
- `src/core/domain/value_objects/network.rs` - Network::chain_type() 方法
- `src/core/domain/queries/mod.rs` - GetBalanceQuery 和 BalanceQueryResult
- `tests/bitcoin_solana_integration_test.rs` - ChainType 集成测试

## 下一步扩展

### 可能的扩展方向

1. **Transfer 命令支持 ChainType**
   ```rust
   pub struct TransferCommand {
       pub chain_type: ChainType,
       // ...
   }
   ```

2. **添加更多链类型**
   ```rust
   pub enum ChainType {
       Ethereum,
       Bitcoin,
       Solana,
       Polygon,      // 新增
       Avalanche,    // 新增
       Cosmos,       // 新增
   }
   ```

3. **基于 ChainType 的地址验证**
   ```rust
   impl Address {
       pub fn validate_for_chain(&self, chain_type: ChainType) -> Result<(), DomainError> {
           // 针对特定链的严格验证
       }
   }
   ```

4. **Chain-specific 元数据**
   ```rust
   impl ChainType {
       pub fn block_time(&self) -> Duration { /* ... */ }
       pub fn finality_blocks(&self) -> u64 { /* ... */ }
       pub fn explorer_url(&self) -> &str { /* ... */ }
   }
   ```

## 总结

本次重构成功地将 `ChainType` 集成到 `GetBalanceQuery` 中，实现了：

- ✅ 同一接口支持多条链（Ethereum、Bitcoin、Solana）
- ✅ 自动类型推导，无需手动指定
- ✅ 类型安全的路由机制
- ✅ 丰富的链元数据（货币符号、单位、小数位数）
- ✅ 向后兼容，现有代码无需修改
- ✅ 完整的测试覆盖

这为未来支持更多区块链打下了坚实的基础！
