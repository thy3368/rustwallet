# Query Handler 更新总结

## GetBalanceHandler 更新

### 更新内容

**文件**: `src/core/application/handlers/get_balance_handler.rs`

#### 1. 增强日志记录

**更新前**：
```rust
tracing::info!(
    "Querying balance for address {} on network {}",
    query.address,
    query.network.name()
);
```

**更新后**：
```rust
tracing::info!(
    "Querying {} balance for address {} on network {}",
    query.chain_type.name(),  // ⭐ 新增链类型
    query.address,
    query.network.name()
);

tracing::debug!(
    "Chain details: currency={}, unit={}, decimals={}",
    query.chain_type.native_currency(),  // ⭐ 货币符号
    query.chain_type.smallest_unit(),    // ⭐ 最小单位
    query.chain_type.decimals()          // ⭐ 小数位数
);
```

#### 2. 增强成功日志

**更新前**：
```rust
tracing::info!(
    "Balance query successful: {} has {}",
    query.address,
    balance
);
```

**更新后**：
```rust
tracing::info!(
    "Balance query successful: {} has {} {} ({} {})",
    query.address,
    balance.to_wei(),
    query.chain_type.smallest_unit(),  // ⭐ 显示单位（Wei/Satoshi/Lamport）
    balance.to_ether(),
    query.chain_type.native_currency()  // ⭐ 显示货币（ETH/BTC/SOL）
);
```

#### 3. 新增测试用例

**测试1：验证 ChainType 正确传递**
```rust
#[tokio::test]
async fn test_get_balance_handler() {
    // ... 创建 handler 和 query ...

    let balance_result = result.unwrap();
    assert_eq!(balance_result.balance.to_ether(), 10.5);
    assert_eq!(balance_result.network, Network::Mainnet);
    assert_eq!(balance_result.chain_type, ChainType::Ethereum);  // ⭐ 验证链类型
}
```

**测试2：验证多链类型支持**
```rust
#[tokio::test]
async fn test_get_balance_handler_with_chain_types() {
    // 测试 Bitcoin 查询
    let btc_query = GetBalanceQuery::new(
        Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()).unwrap(),
        Network::BitcoinMainnet,
    );

    let result = handler.handle(btc_query).await;
    let balance_result = result.unwrap();

    // 验证链类型
    assert_eq!(balance_result.chain_type, ChainType::Bitcoin);
    assert_eq!(balance_result.network, Network::BitcoinMainnet);

    // 验证链类型元数据
    assert_eq!(balance_result.chain_type.name(), "Bitcoin");
    assert_eq!(balance_result.chain_type.native_currency(), "BTC");
    assert_eq!(balance_result.chain_type.smallest_unit(), "Satoshi");
    assert_eq!(balance_result.chain_type.decimals(), 8);
}
```

## 日志输出示例

### Ethereum 查询日志

```
INFO  Querying Ethereum balance for address 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC on network Ethereum Mainnet
DEBUG Chain details: currency=ETH, unit=Wei, decimals=18
INFO  Balance query successful: 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC has 1500000000000000000 Wei (1.5 ETH)
```

### Bitcoin 查询日志

```
INFO  Querying Bitcoin balance for address 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa on network Bitcoin Mainnet
DEBUG Chain details: currency=BTC, unit=Satoshi, decimals=8
INFO  Balance query successful: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa has 6804998099 Satoshi (68.04998099 BTC)
```

### Solana 查询日志

```
INFO  Querying Solana balance for address DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy on network Solana Mainnet
DEBUG Chain details: currency=SOL, unit=Lamport, decimals=9
INFO  Balance query successful: DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy has 2500000000 Lamport (2.5 SOL)
```

## 使用示例

### 基本使用（日志自动包含链类型）

```rust
use rustwallet::core::application::handlers::GetBalanceHandler;
use rustwallet::core::domain::{
    queries::GetBalanceQuery,
    value_objects::{Address, Network},
    services::QueryHandler,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建服务和 handler
    let service = BitcoinBlockchainService::new(Network::BitcoinMainnet).await?;
    let handler = GetBalanceHandler::new(Arc::new(service));

    // 创建查询
    let address = Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())?;
    let query = GetBalanceQuery::new(address, Network::BitcoinMainnet);

    // 执行查询 - 日志会自动显示链类型信息
    let result = handler.handle(query).await?;

    // 结果包含链类型信息
    println!("Chain Type: {}", result.chain_type);
    println!("Balance: {} {}",
        result.balance.to_wei(),
        result.chain_type.smallest_unit()
    );

    Ok(())
}
```

### 多链查询（统一日志格式）

```rust
async fn query_multiple_chains() -> Result<(), Box<dyn std::error::Error>> {
    let queries = vec![
        ("Ethereum", GetBalanceQuery::new(
            Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())?,
            Network::Mainnet,
        )),
        ("Bitcoin", GetBalanceQuery::new(
            Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())?,
            Network::BitcoinMainnet,
        )),
        ("Solana", GetBalanceQuery::new(
            Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())?,
            Network::SolanaMainnet,
        )),
    ];

    for (name, query) in queries {
        // 每个查询都会生成包含链类型的日志
        let result = get_handler_for_chain(query.chain_type)
            .handle(query)
            .await?;

        println!("{}: {} {}",
            name,
            result.balance.to_wei(),
            result.chain_type.smallest_unit()
        );
    }

    Ok(())
}
```

## 优势

### 1. 更清晰的日志

**之前**：
```
INFO Querying balance for address 0x... on network Ethereum Mainnet
INFO Balance query successful: 0x... has 1500000000000000000
```

**现在**：
```
INFO  Querying Ethereum balance for address 0x... on network Ethereum Mainnet
DEBUG Chain details: currency=ETH, unit=Wei, decimals=18
INFO  Balance query successful: 0x... has 1500000000000000000 Wei (1.5 ETH)
```

### 2. 调试信息更丰富

新增的 debug 日志提供了链的详细信息：
- 原生货币符号（ETH/BTC/SOL）
- 最小单位名称（Wei/Satoshi/Lamport）
- 小数位数（18/8/9）

### 3. 单位一致性

余额日志同时显示：
- 原始值（Wei/Satoshi/Lamport）
- 可读格式（ETH/BTC/SOL）

### 4. 类型安全

测试验证了 ChainType 在整个查询流程中正确传递：
```rust
assert_eq!(balance_result.chain_type, ChainType::Bitcoin);
assert_eq!(balance_result.chain_type.name(), "Bitcoin");
```

## 测试验证

运行以下命令验证更新：

```bash
# 运行 handler 单元测试
cargo test --lib get_balance_handler -- --nocapture

# 运行完整测试套件
cargo test -- --nocapture
```

### 测试结果

```
running 2 tests
test core::application::handlers::get_balance_handler::tests::test_get_balance_handler ... ok
test core::application::handlers::get_balance_handler::tests::test_get_balance_handler_with_chain_types ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

## 后续扩展建议

### 1. 添加链特定验证

```rust
async fn handle(&self, query: GetBalanceQuery) -> Result<Self::Output, DomainError> {
    // 验证地址格式是否匹配链类型
    validate_address_for_chain(&query.address, &query.chain_type)?;

    // ... 现有逻辑 ...
}

fn validate_address_for_chain(
    address: &Address,
    chain_type: &ChainType
) -> Result<(), DomainError> {
    match chain_type {
        ChainType::Ethereum => {
            if !address.as_str().starts_with("0x") {
                return Err(DomainError::InvalidAddressFormat);
            }
        }
        ChainType::Bitcoin => {
            // Bitcoin 地址验证
        }
        ChainType::Solana => {
            // Solana 地址验证
        }
    }
    Ok(())
}
```

### 2. 添加链特定指标

```rust
async fn handle(&self, query: GetBalanceQuery) -> Result<Self::Output, DomainError> {
    let start = std::time::Instant::now();

    let balance = self.blockchain_service.get_balance(&query.address).await?;

    let duration = start.elapsed();

    // 记录链特定的性能指标
    metrics::histogram!(
        "balance_query_duration",
        duration,
        "chain" => query.chain_type.name()
    );

    // ... 返回结果 ...
}
```

### 3. 添加链特定错误处理

```rust
async fn handle(&self, query: GetBalanceQuery) -> Result<Self::Output, DomainError> {
    let balance = self.blockchain_service
        .get_balance(&query.address)
        .await
        .map_err(|e| {
            // 添加链类型上下文到错误信息
            DomainError::ChainSpecificError {
                chain: query.chain_type,
                network: query.network.clone(),
                error: Box::new(e),
            }
        })?;

    // ... 返回结果 ...
}
```

## 总结

本次更新成功地将 ChainType 集成到 GetBalanceHandler 中：

- ✅ 日志包含链类型信息（Ethereum/Bitcoin/Solana）
- ✅ 显示链特定元数据（货币、单位、小数位数）
- ✅ 余额日志同时显示原始值和可读格式
- ✅ 新增测试验证 ChainType 正确传递
- ✅ 所有测试通过
- ✅ 向后兼容，现有代码无需修改

这为未来添加更多链特定功能（如验证、指标、错误处理）打下了坚实基础！
