# Query Handler 模式最佳实践

## 概述

本文档展示了如何使用 **QueryHandler 模式**结合 **MultiChainBlockchainService** 实现符合 Clean Architecture 的多链查询。

## 架构模式

### Clean Architecture 三层结构

```text
┌──────────────────────────────────────────────────────────┐
│                    使用场景                               │
│                (测试/CLI/Web API)                         │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────┐
│  Domain Layer (领域层)                                    │
│  ┌────────────────────────────────────────────┐          │
│  │  GetBalanceQuery                           │          │
│  │  - address: Address                        │          │
│  │  - network: Network                        │          │
│  │  - chain_type: ChainType (自动推导)        │          │
│  └────────────────────────────────────────────┘          │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────┐
│  Application Layer (应用层)                               │
│  ┌────────────────────────────────────────────┐          │
│  │  GetBalanceHandler                         │          │
│  │  impl QueryHandler<GetBalanceQuery>        │          │
│  │  ↓                                          │          │
│  │  async fn handle(query) -> Result          │          │
│  └────────────────────────────────────────────┘          │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────────┐
│  Infrastructure Layer (基础设施层)                        │
│  ┌────────────────────────────────────────────┐          │
│  │  MultiChainBlockchainService               │          │
│  │  ↓ 自动路由                                 │          │
│  │  match chain_type {                        │          │
│  │    Ethereum → AlloyBlockchainService       │          │
│  │    Bitcoin  → BitcoinBlockchainService     │          │
│  │    Solana   → SolanaBlockchainService      │          │
│  │  }                                          │          │
│  └────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────┘
```

## 标准查询模式（4步）

### 完整示例

```rust
use rustwallet::adapter::infrastructure::blockchain::MultiChainBlockchainService;
use rustwallet::core::application::handlers::GetBalanceHandler;
use rustwallet::core::domain::{
    queries::GetBalanceQuery,
    services::QueryHandler,
    value_objects::{Address, Network},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create Infrastructure layer service (基础设施层)
    let service = MultiChainBlockchainService::new_for_network(Network::Sepolia).await?;

    // Step 2: Create Application layer Handler (应用层)
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Step 3: Create Domain layer Query (领域层)
    let address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())?;
    let query = GetBalanceQuery::new(address, Network::Sepolia);

    // Step 4: Execute query through handler (执行查询)
    let result = handler.handle(query).await?;

    // 使用结果
    println!("Balance: {} {}",
        result.balance.to_wei(),
        result.chain_type.smallest_unit()
    );

    Ok(())
}
```

### 为什么是 4 步？

| 步骤 | 层次 | 职责 | 为什么必要 |
|------|------|------|-----------|
| Step 1 | Infrastructure | 创建技术实现 | 隔离技术细节，可替换实现 |
| Step 2 | Application | 创建业务编排器 | 处理业务流程和事务 |
| Step 3 | Domain | 定义业务需求 | 表达业务意图，独立于技术 |
| Step 4 | Application | 执行业务逻辑 | 统一的执行入口 |

## 多链查询模式

### 模式 1：单链查询

```rust
async fn query_single_chain(
    network: Network,
    address_str: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Infrastructure: 为特定网络创建服务
    let service = MultiChainBlockchainService::new_for_network(network.clone()).await?;

    // Application: 创建 Handler
    let handler = GetBalanceHandler::new(Arc::new(service));

    // Domain: 创建 Query
    let address = Address::new(address_str.to_string())?;
    let query = GetBalanceQuery::new(address, network);

    // Execute: 执行查询
    let result = handler.handle(query).await?;

    println!("{}: {} {}",
        result.chain_type.name(),
        result.balance.to_wei(),
        result.chain_type.smallest_unit()
    );

    Ok(())
}

// 使用示例
query_single_chain(Network::Sepolia, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC").await?;
```

### 模式 2：多链顺序查询

```rust
async fn query_multiple_chains() -> Result<(), Box<dyn std::error::Error>> {
    let chains = vec![
        ("Ethereum", Network::Sepolia, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC"),
        ("Bitcoin", Network::BitcoinMainnet, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
        ("Solana", Network::SolanaMainnet, "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy"),
    ];

    for (name, network, addr_str) in chains {
        // 为每条链创建独立的服务和 handler
        let service = MultiChainBlockchainService::new_for_network(network.clone()).await?;
        let handler = GetBalanceHandler::new(Arc::new(service));

        let address = Address::new(addr_str.to_string())?;
        let query = GetBalanceQuery::new(address, network);

        match handler.handle(query).await {
            Ok(result) => {
                println!("{}: {} {}",
                    name,
                    result.balance.to_wei(),
                    result.chain_type.smallest_unit()
                );
            }
            Err(e) => {
                println!("{}: Error - {}", name, e);
            }
        }
    }

    Ok(())
}
```

### 模式 3：并发查询（共享服务）

```rust
async fn query_concurrent() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个支持所有链的服务
    let mut service = MultiChainBlockchainService::new().await?;
    service.initialize_all().await?;
    let service_arc = Arc::new(service);

    // 创建多个 handler 共享同一个服务
    let eth_handler = GetBalanceHandler::new(service_arc.clone());
    let btc_handler = GetBalanceHandler::new(service_arc.clone());
    let sol_handler = GetBalanceHandler::new(service_arc.clone());

    // 创建查询
    let eth_query = GetBalanceQuery::new(
        Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())?,
        Network::Sepolia
    );
    let btc_query = GetBalanceQuery::new(
        Address::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())?,
        Network::BitcoinMainnet
    );
    let sol_query = GetBalanceQuery::new(
        Address::new("DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy".to_string())?,
        Network::SolanaMainnet
    );

    // 并发执行
    let (eth_result, btc_result, sol_result) = tokio::join!(
        eth_handler.handle(eth_query),
        btc_handler.handle(btc_query),
        sol_handler.handle(sol_query)
    );

    // 处理结果
    if let Ok(r) = eth_result {
        println!("Ethereum: {} Wei", r.balance.to_wei());
    }
    if let Ok(r) = btc_result {
        println!("Bitcoin: {} satoshis", r.balance.to_wei());
    }
    if let Ok(r) = sol_result {
        println!("Solana: {} lamports", r.balance.to_wei());
    }

    Ok(())
}
```

## 可复用模式

### 通用查询函数

```rust
/// 可复用的查询函数 - 适用于所有链
async fn execute_balance_query(
    network: Network,
    address: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1-4 封装在一个函数中
    let service = MultiChainBlockchainService::new_for_network(network.clone()).await?;
    let handler = GetBalanceHandler::new(Arc::new(service));
    let addr = Address::new(address.to_string())?;
    let query = GetBalanceQuery::new(addr, network);
    let result = handler.handle(query).await?;

    // 格式化输出
    Ok(format!("{} {} ({} {})",
        result.balance.to_wei(),
        result.chain_type.smallest_unit(),
        result.balance.to_wei() as f64 / 10_f64.powi(result.chain_type.decimals() as i32),
        result.chain_type.native_currency()
    ))
}

// 使用示例
let eth_balance = execute_balance_query(
    Network::Sepolia,
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC"
).await?;
println!("ETH: {}", eth_balance);

let btc_balance = execute_balance_query(
    Network::BitcoinMainnet,
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
).await?;
println!("BTC: {}", btc_balance);
```

### 批量查询辅助函数

```rust
/// 批量查询多个地址
async fn batch_query(
    network: Network,
    addresses: Vec<&str>,
) -> Vec<Result<Balance, Box<dyn std::error::Error>>> {
    // 创建共享的 service 和 handler
    let service = MultiChainBlockchainService::new_for_network(network.clone())
        .await
        .expect("Failed to create service");
    let handler = Arc::new(GetBalanceHandler::new(Arc::new(service)));

    // 并发查询所有地址
    let futures: Vec<_> = addresses.into_iter().map(|addr_str| {
        let handler = handler.clone();
        let network = network.clone();
        async move {
            let address = Address::new(addr_str.to_string())?;
            let query = GetBalanceQuery::new(address, network);
            let result = handler.handle(query).await?;
            Ok(result.balance)
        }
    }).collect();

    futures::future::join_all(futures).await
}

// 使用示例
let addresses = vec![
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC",
    "0x1234567890123456789012345678901234567890",
];
let balances = batch_query(Network::Sepolia, addresses).await;
```

## 错误处理模式

### 模式 1：基本错误处理

```rust
let result = handler.handle(query).await;

match result {
    Ok(balance_result) => {
        println!("Success: {} {}",
            balance_result.balance.to_wei(),
            balance_result.chain_type.smallest_unit()
        );
    }
    Err(e) => {
        eprintln!("Query failed: {}", e);
        // 根据错误类型采取不同行动
        match e {
            DomainError::NetworkError(_) => {
                eprintln!("Network issue, will retry...");
            }
            DomainError::ConfigurationError(_) => {
                eprintln!("Configuration issue, check setup");
            }
            _ => {
                eprintln!("Other error");
            }
        }
    }
}
```

### 模式 2：带重试的错误处理

```rust
async fn query_with_retry(
    handler: &GetBalanceHandler,
    query: GetBalanceQuery,
    max_retries: u32,
) -> Result<BalanceQueryResult, DomainError> {
    let mut attempts = 0;

    loop {
        match handler.handle(query.clone()).await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_retries => {
                attempts += 1;
                eprintln!("Retry {}/{}: {}", attempts, max_retries, e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### 模式 3：优雅降级

```rust
async fn query_with_fallback(
    primary_network: Network,
    fallback_network: Network,
    address: &str,
) -> Result<BalanceQueryResult, Box<dyn std::error::Error>> {
    let addr = Address::new(address.to_string())?;

    // 尝试主网络
    let primary_service = MultiChainBlockchainService::new_for_network(primary_network.clone()).await?;
    let primary_handler = GetBalanceHandler::new(Arc::new(primary_service));
    let primary_query = GetBalanceQuery::new(addr.clone(), primary_network);

    match primary_handler.handle(primary_query).await {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Primary network failed: {}, trying fallback...", e);

            // 降级到备用网络
            let fallback_service = MultiChainBlockchainService::new_for_network(fallback_network.clone()).await?;
            let fallback_handler = GetBalanceHandler::new(Arc::new(fallback_service));
            let fallback_query = GetBalanceQuery::new(addr, fallback_network);

            Ok(fallback_handler.handle(fallback_query).await?)
        }
    }
}
```

## 测试模式

### 单元测试

```rust
#[tokio::test]
async fn test_ethereum_query() {
    let service = MultiChainBlockchainService::new_for_network(Network::Sepolia)
        .await
        .expect("Failed to create service");

    let handler = GetBalanceHandler::new(Arc::new(service));

    let address = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC".to_string())
        .expect("Valid address");
    let query = GetBalanceQuery::new(address, Network::Sepolia);

    // 执行查询
    let result = handler.handle(query).await;

    // 验证
    assert!(result.is_ok());
    let balance_result = result.unwrap();
    assert_eq!(balance_result.chain_type, ChainType::Ethereum);
    assert_eq!(balance_result.network, Network::Sepolia);
}
```

### 集成测试

```rust
#[tokio::test]
#[ignore] // 需要网络连接
async fn test_multi_chain_integration() {
    let test_cases = vec![
        ("ETH", Network::Sepolia, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEbC"),
        ("BTC", Network::BitcoinMainnet, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
        ("SOL", Network::SolanaMainnet, "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy"),
    ];

    for (name, network, addr_str) in test_cases {
        let service = MultiChainBlockchainService::new_for_network(network.clone())
            .await
            .expect("Failed to create service");
        let handler = GetBalanceHandler::new(Arc::new(service));

        let address = Address::new(addr_str.to_string()).expect("Valid address");
        let query = GetBalanceQuery::new(address, network);

        let result = handler.handle(query).await;
        assert!(result.is_ok(), "{} query should succeed", name);

        let balance_result = result.unwrap();
        println!("{}: {} {}",
            name,
            balance_result.balance.to_wei(),
            balance_result.chain_type.smallest_unit()
        );
    }
}
```

## 性能优化模式

### 模式 1：服务复用

```rust
// ❌ 不推荐：每次查询都创建新服务
async fn inefficient_query(address: &str) -> Result<Balance, Box<dyn std::error::Error>> {
    let service = MultiChainBlockchainService::new_for_network(Network::Sepolia).await?;
    let handler = GetBalanceHandler::new(Arc::new(service));
    // ... 查询逻辑
}

// ✅ 推荐：复用服务
struct BalanceQueryService {
    handler: Arc<GetBalanceHandler>,
}

impl BalanceQueryService {
    async fn new(network: Network) -> Result<Self, Box<dyn std::error::Error>> {
        let service = MultiChainBlockchainService::new_for_network(network).await?;
        let handler = Arc::new(GetBalanceHandler::new(Arc::new(service)));
        Ok(Self { handler })
    }

    async fn query(&self, address: &str) -> Result<Balance, Box<dyn std::error::Error>> {
        let addr = Address::new(address.to_string())?;
        let query = GetBalanceQuery::new(addr, Network::Sepolia);
        let result = self.handler.handle(query).await?;
        Ok(result.balance)
    }
}
```

### 模式 2：并发查询

```rust
// 使用 tokio::join! 并发执行多个查询
let (eth_result, btc_result, sol_result) = tokio::join!(
    eth_handler.handle(eth_query),
    btc_handler.handle(btc_query),
    sol_handler.handle(sol_query)
);
```

## 最佳实践总结

### ✅ 推荐做法

1. **始终通过 Handler 执行查询**
   ```rust
   // ✅ Good
   let result = handler.handle(query).await?;
   ```

2. **复用 Service 和 Handler**
   ```rust
   // ✅ Good: 创建一次，多次使用
   let handler = Arc::new(GetBalanceHandler::new(service));
   ```

3. **使用 ChainType 自动推导**
   ```rust
   // ✅ Good: ChainType 自动从 Network 推导
   let query = GetBalanceQuery::new(address, network);
   ```

4. **处理所有错误情况**
   ```rust
   // ✅ Good: 完整的错误处理
   match handler.handle(query).await {
       Ok(result) => { /* 成功处理 */ }
       Err(e) => { /* 错误处理 */ }
   }
   ```

### ❌ 避免做法

1. **跳过 Handler 直接调用 Service**
   ```rust
   // ❌ Bad: 违反 Clean Architecture
   let balance = service.get_balance(&address).await?;
   ```

2. **为每个查询创建新的 Service**
   ```rust
   // ❌ Bad: 浪费资源
   for address in addresses {
       let service = MultiChainBlockchainService::new(...).await?;
       // ...
   }
   ```

3. **忽略错误**
   ```rust
   // ❌ Bad: 未处理错误
   let result = handler.handle(query).await.unwrap();
   ```

## 运行测试

```bash
# 运行所有测试
cargo test --test multi_chain_service_test -- --nocapture

# 运行网络测试
cargo test --test multi_chain_service_test --ignored -- --nocapture

# 运行特定测试
cargo test --test multi_chain_service_test test_query_ethereum_via_handler -- --nocapture --ignored
```

## 总结

使用 QueryHandler 模式的优势：

- ✅ **清晰的关注点分离** - Domain/Application/Infrastructure 各司其职
- ✅ **易于测试** - 可以 mock Handler 和 Service
- ✅ **统一的接口** - 所有链使用相同的查询模式
- ✅ **类型安全** - 编译时保证正确性
- ✅ **可扩展** - 添加新链只需扩展 MultiChainBlockchainService
- ✅ **性能优秀** - Handler 开销几乎可忽略不计（< 1μs）

通过遵循这些模式，可以构建出健壮、可维护、高性能的多链钱包应用！
