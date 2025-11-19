# 跨链转账方案设计

## 文档概述

本文档描述多链钱包的跨链转账功能设计，支持不同区块链之间的资产转移（ETH ↔ BTC, ETH ↔ SOL, BTC ↔ SOL 等）。

**版本**: v1.0.0
**创建日期**: 2025-11-20
**架构模式**: Clean Architecture + CQRS + Event Sourcing

---

## 目录

1. [跨链转账基础](#1-跨链转账基础)
2. [技术方案选型](#2-技术方案选型)
3. [架构设计](#3-架构设计)
4. [领域模型](#4-领域模型)
5. [跨链桥实现](#5-跨链桥实现)
6. [安全机制](#6-安全机制)
7. [实现路线图](#7-实现路线图)

---

## 1. 跨链转账基础

### 1.1 什么是跨链转账？

跨链转账是指在不同区块链网络之间转移加密资产的过程。由于不同区块链是独立的系统，无法直接通信，需要通过跨链桥（Cross-Chain Bridge）来实现。

### 1.2 核心挑战

| 挑战 | 描述 | 解决方案 |
|------|------|---------|
| **共识机制差异** | ETH (PoS), BTC (PoW), SOL (PoH) | 中继网络同步状态 |
| **地址格式不兼容** | ETH (0x...), BTC (bc1...), SOL (Base58) | 映射表 + 托管账户 |
| **交易确认时间** | BTC 60分钟, ETH 5分钟, SOL 0.4秒 | 多阶段确认机制 |
| **安全性** | 双花攻击、重放攻击 | 多签验证 + 时间锁 |
| **原子性保证** | 跨链交易不可逆 | 哈希时间锁定合约 (HTLC) |

### 1.3 链特性对比

| 特性 | Ethereum | Bitcoin | Solana |
|------|----------|---------|--------|
| **地址格式** | 0x... (20字节) | bc1... / 1... (可变) | Base58 (32字节) |
| **账户模型** | Account-based | UTXO | Account-based |
| **智能合约** | ✅ EVM | ❌ 有限脚本 | ✅ BPF |
| **交易确认** | ~15秒 (1区块) | ~60分钟 (6区块) | ~0.4秒 (1区块) |
| **交易费用** | 动态 Gas | 按字节收费 | 极低固定费用 |
| **脚本能力** | 图灵完备 | 非图灵完备 | 图灵完备 |

---

## 2. 技术方案选型

### 2.1 跨链桥类型

#### 方案 A: 中心化托管桥 (Centralized Custodial Bridge)

**原理**:
```
用户 → 托管方 → 目标链
  ↓       ↓        ↓
锁定资产  铸造代币  接收代币
```

**优点**:
- ✅ 实现简单，开发周期短
- ✅ 交易速度快
- ✅ 支持任意链组合

**缺点**:
- ❌ 中心化风险（单点故障）
- ❌ 需要信任第三方
- ❌ 监管风险

**适用场景**: MVP 版本、快速验证

---

#### 方案 B: 去中心化哈希时间锁 (HTLC - Hash Time Locked Contract)

**原理**:
```
Alice (ETH链)              Bob (BTC链)
    ↓                          ↓
锁定 ETH (哈希H)          锁定 BTC (哈希H)
    ↓                          ↓
提供原像 → 解锁 BTC    提供原像 → 解锁 ETH
```

**流程**:
1. Alice 生成随机数 `S`，计算哈希 `H = hash(S)`
2. Alice 在 ETH 链锁定资产（HTLC 合约，条件: 提供 S 或超时退款）
3. Bob 在 BTC 链锁定资产（脚本，条件: 提供 S 或超时退款）
4. Alice 在 BTC 链提供 `S` 取走 BTC
5. Bob 看到 `S`，在 ETH 链提供 `S` 取走 ETH

**优点**:
- ✅ 完全去中心化
- ✅ 无需信任第三方
- ✅ 原子性保证（要么全成功，要么全失败）

**缺点**:
- ❌ 需要两条链都支持哈希锁（Bitcoin 脚本能力有限）
- ❌ 需要交易对手方在线
- ❌ 资金锁定时间长

**适用场景**: 点对点交换、无需托管的场景

---

#### 方案 C: 中继网络 + 轻客户端验证 (Relay + Light Client)

**原理**:
```
源链 → 中继网络 → 目标链
 ↓       ↓          ↓
事件    验证区块头   铸造/释放资产
```

**流程**:
1. 用户在源链锁定资产，触发事件
2. 中继节点监听事件，提交源链区块头到目标链
3. 目标链使用轻客户端验证区块头和交易证明
4. 验证通过后，在目标链铸造等量资产

**优点**:
- ✅ 去中心化（多个中继节点）
- ✅ 可验证（轻客户端验证）
- ✅ 适合高频交易

**缺点**:
- ❌ 实现复杂度高
- ❌ 需要链支持轻客户端协议
- ❌ 维护成本高

**适用场景**: 生产环境、高安全要求

---

### 2.2 推荐方案

**阶段化实现策略**:

| 阶段 | 方案 | 目标 |
|------|------|------|
| **Phase 1 (MVP)** | 中心化托管桥 | 快速验证需求 |
| **Phase 2 (Beta)** | HTLC 原子交换 | 去中心化尝试 |
| **Phase 3 (生产)** | 中继网络 + 多签 | 安全可靠的生产方案 |

本文档重点描述 **Phase 1 (中心化托管桥)** 和 **Phase 2 (HTLC)** 的实现方案。

---

## 3. 架构设计

### 3.1 Clean Architecture 分层

```
src/
├── core/domain/                           # 领域层（核心）
│   ├── value_objects/
│   │   ├── chain.rs                      # 链标识 (Ethereum, Bitcoin, Solana)
│   │   ├── cross_chain_address.rs        # 跨链地址映射
│   │   ├── bridge_transaction.rs         # 跨链交易
│   │   └── asset_type.rs                 # 资产类型 (Native, Wrapped)
│   ├── entities/
│   │   ├── bridge.rs                     # 跨链桥聚合根
│   │   └── swap_order.rs                 # 交换订单
│   ├── commands/
│   │   ├── initiate_cross_chain_transfer.rs
│   │   ├── claim_transfer.rs
│   │   └── refund_transfer.rs
│   ├── queries/
│   │   ├── get_bridge_status.rs
│   │   └── get_swap_order.rs
│   ├── services/
│   │   ├── bridge_service.rs             # 跨链桥服务接口
│   │   ├── oracle_service.rs             # 预言机服务（汇率查询）
│   │   └── validator_service.rs          # 交易验证服务
│   └── events/
│       ├── transfer_initiated.rs
│       ├── transfer_completed.rs
│       └── transfer_failed.rs
│
├── core/application/                      # 应用层
│   ├── handlers/
│   │   ├── cross_chain_transfer_handler.rs
│   │   └── htlc_swap_handler.rs
│   └── orchestrators/
│       └── bridge_orchestrator.rs        # 跨链流程编排
│
├── adapter/infrastructure/                # 基础设施层
│   ├── blockchain/
│   │   ├── ethereum_service.rs           # ETH 链服务 (Alloy)
│   │   ├── bitcoin_service.rs            # BTC 链服务 (rust-bitcoin)
│   │   └── solana_service.rs             # SOL 链服务 (solana-sdk)
│   ├── bridges/
│   │   ├── custodial_bridge.rs           # 托管桥实现
│   │   ├── htlc_bridge.rs                # HTLC 桥实现
│   │   └── relay_bridge.rs               # 中继桥实现（未来）
│   ├── oracles/
│   │   └── price_oracle.rs               # 价格预言机（汇率）
│   └── persistence/
│       └── bridge_repository.rs          # 跨链交易持久化
│
└── adapter/interfaces/                    # 接口层
    └── cli/
        └── bridge_commands.rs            # CLI 跨链命令
```

---

## 4. 领域模型

### 4.1 跨链转账领域实体

#### 4.1.1 Chain 枚举

```rust
/// 支持的区块链类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Chain {
    /// 以太坊主网
    Ethereum(EthereumNetwork),
    /// 比特币主网/测试网
    Bitcoin(BitcoinNetwork),
    /// Solana 主网/测试网
    Solana(SolanaNetwork),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum EthereumNetwork {
    Mainnet,
    Sepolia,
    BscMainnet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SolanaNetwork {
    Mainnet,
    Devnet,
}

impl Chain {
    /// 获取链 ID
    pub fn chain_id(&self) -> String {
        match self {
            Chain::Ethereum(net) => match net {
                EthereumNetwork::Mainnet => "eth-1".to_string(),
                EthereumNetwork::Sepolia => "eth-11155111".to_string(),
                EthereumNetwork::BscMainnet => "bsc-56".to_string(),
            },
            Chain::Bitcoin(net) => match net {
                BitcoinNetwork::Mainnet => "btc-main".to_string(),
                BitcoinNetwork::Testnet => "btc-test".to_string(),
            },
            Chain::Solana(net) => match net {
                SolanaNetwork::Mainnet => "sol-main".to_string(),
                SolanaNetwork::Devnet => "sol-dev".to_string(),
            },
        }
    }

    /// 是否支持智能合约
    pub fn supports_smart_contracts(&self) -> bool {
        matches!(self, Chain::Ethereum(_) | Chain::Solana(_))
    }

    /// 地址格式验证
    pub fn validate_address(&self, address: &str) -> Result<(), DomainError> {
        match self {
            Chain::Ethereum(_) => {
                if !address.starts_with("0x") || address.len() != 42 {
                    return Err(DomainError::InvalidAddress);
                }
            }
            Chain::Bitcoin(_) => {
                // 简化验证：支持 bc1, 1, 3 开头
                if !address.starts_with("bc1")
                   && !address.starts_with('1')
                   && !address.starts_with('3') {
                    return Err(DomainError::InvalidAddress);
                }
            }
            Chain::Solana(_) => {
                // Base58 编码，长度通常 32-44
                if address.len() < 32 || address.len() > 44 {
                    return Err(DomainError::InvalidAddress);
                }
            }
        }
        Ok(())
    }
}
```

---

#### 4.1.2 CrossChainAddress 值对象

```rust
/// 跨链地址（支持多链地址映射）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossChainAddress {
    /// 所属链
    chain: Chain,
    /// 原生地址
    native_address: String,
}

impl CrossChainAddress {
    pub fn new(chain: Chain, address: String) -> Result<Self, DomainError> {
        // 验证地址格式
        chain.validate_address(&address)?;

        Ok(Self {
            chain,
            native_address: address,
        })
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    pub fn address(&self) -> &str {
        &self.native_address
    }

    /// 转换为统一格式（用于日志和显示）
    pub fn to_unified_format(&self) -> String {
        format!("{}:{}", self.chain.chain_id(), self.native_address)
    }
}
```

---

#### 4.1.3 BridgeTransaction 跨链交易

```rust
use std::time::SystemTime;

/// 跨链交易状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeTransactionStatus {
    /// 已发起（在源链锁定资产）
    Initiated,
    /// 等待确认（等待源链交易确认）
    WaitingConfirmation { confirmations: u32, required: u32 },
    /// 正在处理（托管方处理中）
    Processing,
    /// 已完成（目标链已接收资产）
    Completed { tx_hash: String },
    /// 已退款（超时或失败）
    Refunded { reason: String },
    /// 失败
    Failed { error: String },
}

/// 跨链交易实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    /// 交易 ID
    id: String,
    /// 源链地址
    from_address: CrossChainAddress,
    /// 目标链地址
    to_address: CrossChainAddress,
    /// 转账金额（源链单位）
    amount: u128,
    /// 资产类型
    asset_type: AssetType,
    /// 源链交易哈希
    source_tx_hash: Option<String>,
    /// 目标链交易哈希
    destination_tx_hash: Option<String>,
    /// 交易状态
    status: BridgeTransactionStatus,
    /// 创建时间
    created_at: SystemTime,
    /// 更新时间
    updated_at: SystemTime,
    /// 超时时间（用于退款）
    timeout_at: SystemTime,
}

impl BridgeTransaction {
    /// 创建新的跨链交易
    pub fn new(
        from: CrossChainAddress,
        to: CrossChainAddress,
        amount: u128,
        asset_type: AssetType,
        timeout_seconds: u64,
    ) -> Result<Self, DomainError> {
        if amount == 0 {
            return Err(DomainError::InvalidAmount);
        }

        let now = SystemTime::now();
        let timeout = now + std::time::Duration::from_secs(timeout_seconds);

        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            from_address: from,
            to_address: to,
            amount,
            asset_type,
            source_tx_hash: None,
            destination_tx_hash: None,
            status: BridgeTransactionStatus::Initiated,
            created_at: now,
            updated_at: now,
            timeout_at: timeout,
        })
    }

    /// 更新源链交易哈希
    pub fn set_source_tx_hash(&mut self, hash: String) {
        self.source_tx_hash = Some(hash);
        self.updated_at = SystemTime::now();
    }

    /// 更新状态
    pub fn update_status(&mut self, status: BridgeTransactionStatus) {
        self.status = status;
        self.updated_at = SystemTime::now();
    }

    /// 检查是否超时
    pub fn is_timeout(&self) -> bool {
        SystemTime::now() > self.timeout_at
    }

    /// 完成交易
    pub fn complete(&mut self, dest_tx_hash: String) -> Result<(), DomainError> {
        if !matches!(self.status, BridgeTransactionStatus::Processing) {
            return Err(DomainError::InvalidStateTransition);
        }

        self.destination_tx_hash = Some(dest_tx_hash.clone());
        self.status = BridgeTransactionStatus::Completed { tx_hash: dest_tx_hash };
        self.updated_at = SystemTime::now();
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn status(&self) -> &BridgeTransactionStatus { &self.status }
    pub fn amount(&self) -> u128 { self.amount }
}
```

---

#### 4.1.4 AssetType 资产类型

```rust
/// 资产类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    /// 原生代币（ETH, BTC, SOL）
    Native { symbol: String },
    /// 包装代币（WBTC, WETH, wSOL）
    Wrapped {
        symbol: String,
        /// 原始链
        original_chain: Chain,
        /// 合约地址（如果适用）
        contract_address: Option<String>,
    },
}

impl AssetType {
    pub fn symbol(&self) -> &str {
        match self {
            AssetType::Native { symbol } => symbol,
            AssetType::Wrapped { symbol, .. } => symbol,
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self, AssetType::Native { .. })
    }
}
```

---

### 4.2 HTLC 原子交换模型

#### 4.2.1 HashLock 哈希锁

```rust
use sha2::{Sha256, Digest};

/// 哈希锁（用于 HTLC）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashLock {
    /// 哈希值（SHA256）
    hash: [u8; 32],
    /// 原像（只有创建者知道）
    preimage: Option<Vec<u8>>,
}

impl HashLock {
    /// 生成新的哈希锁（随机原像）
    pub fn generate() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let preimage: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        let hash = Sha256::digest(&preimage);

        Self {
            hash: hash.into(),
            preimage: Some(preimage),
        }
    }

    /// 从已知原像创建哈希锁
    pub fn from_preimage(preimage: Vec<u8>) -> Self {
        let hash = Sha256::digest(&preimage);
        Self {
            hash: hash.into(),
            preimage: Some(preimage),
        }
    }

    /// 只有哈希值（用于对手方）
    pub fn from_hash(hash: [u8; 32]) -> Self {
        Self {
            hash,
            preimage: None,
        }
    }

    /// 获取哈希值（十六进制字符串）
    pub fn hash_hex(&self) -> String {
        hex::encode(self.hash)
    }

    /// 验证原像
    pub fn verify(&self, preimage: &[u8]) -> bool {
        let hash = Sha256::digest(preimage);
        hash.as_slice() == self.hash
    }

    /// 获取原像（如果有）
    pub fn preimage(&self) -> Option<&[u8]> {
        self.preimage.as_deref()
    }
}
```

---

#### 4.2.2 SwapOrder 交换订单

```rust
/// 原子交换订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapOrder {
    /// 订单 ID
    id: String,
    /// 发起方（Alice）
    initiator: SwapParty,
    /// 接收方（Bob）
    counterparty: SwapParty,
    /// 哈希锁
    hash_lock: HashLock,
    /// 时间锁（秒）
    timelock_seconds: u64,
    /// 订单状态
    status: SwapOrderStatus,
    /// 创建时间
    created_at: SystemTime,
}

/// 交换参与方
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapParty {
    /// 地址
    address: CrossChainAddress,
    /// 锁定金额
    amount: u128,
    /// 资产类型
    asset: AssetType,
    /// 交易哈希（锁定资产的交易）
    lock_tx_hash: Option<String>,
}

/// 交换订单状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwapOrderStatus {
    /// 等待双方锁定资产
    WaitingLocks,
    /// 双方已锁定
    Locked,
    /// 交换成功
    Completed,
    /// 已退款
    Refunded,
}
```

---

## 5. 跨链桥实现

### 5.1 托管桥实现（Phase 1）

#### 5.1.1 CustodialBridge 服务接口

```rust
use async_trait::async_trait;

/// 托管桥服务接口
#[async_trait]
pub trait CustodialBridgeService: Send + Sync {
    /// 发起跨链转账
    async fn initiate_transfer(
        &self,
        from: CrossChainAddress,
        to: CrossChainAddress,
        amount: u128,
        asset_type: AssetType,
    ) -> Result<BridgeTransaction, BridgeError>;

    /// 查询跨链交易状态
    async fn get_transaction_status(
        &self,
        tx_id: &str,
    ) -> Result<BridgeTransaction, BridgeError>;

    /// 认领资产（用户在目标链领取）
    async fn claim_transfer(
        &self,
        tx_id: &str,
        recipient_signature: &str,
    ) -> Result<String, BridgeError>;

    /// 退款（超时或失败）
    async fn refund_transfer(
        &self,
        tx_id: &str,
    ) -> Result<String, BridgeError>;
}
```

---

#### 5.1.2 托管桥工作流程

```rust
/// 托管桥实现
pub struct CustodialBridge {
    /// 源链服务
    source_chain_service: Arc<dyn BlockchainService>,
    /// 目标链服务
    destination_chain_service: Arc<dyn BlockchainService>,
    /// 托管钱包
    custodian_wallet: CustodianWallet,
    /// 交易存储
    tx_repository: Arc<dyn BridgeTransactionRepository>,
}

impl CustodialBridge {
    /// 跨链转账完整流程
    pub async fn execute_cross_chain_transfer(
        &self,
        from: CrossChainAddress,
        to: CrossChainAddress,
        amount: u128,
    ) -> Result<BridgeTransaction, BridgeError> {
        // 步骤 1: 验证参数
        self.validate_transfer(&from, &to, amount)?;

        // 步骤 2: 在源链锁定用户资产
        let source_tx_hash = self.lock_source_assets(&from, amount).await?;

        // 步骤 3: 创建跨链交易记录
        let mut bridge_tx = BridgeTransaction::new(
            from.clone(),
            to.clone(),
            amount,
            AssetType::Native { symbol: "ETH".to_string() },
            3600, // 1小时超时
        )?;
        bridge_tx.set_source_tx_hash(source_tx_hash.clone());

        // 步骤 4: 等待源链确认
        self.wait_for_confirmations(&source_tx_hash, 12).await?;
        bridge_tx.update_status(BridgeTransactionStatus::Processing);

        // 步骤 5: 在目标链释放/铸造资产
        let dest_tx_hash = self.release_destination_assets(&to, amount).await?;

        // 步骤 6: 完成交易
        bridge_tx.complete(dest_tx_hash)?;

        // 步骤 7: 持久化
        self.tx_repository.save(&bridge_tx).await?;

        Ok(bridge_tx)
    }

    /// 锁定源链资产
    async fn lock_source_assets(
        &self,
        from: &CrossChainAddress,
        amount: u128,
    ) -> Result<String, BridgeError> {
        // 调用源链服务，将用户资产转入托管地址
        let custodian_address = self.custodian_wallet.get_address(from.chain());

        let tx_hash = self.source_chain_service
            .transfer(
                &Address::new(from.address().to_string())?,
                &custodian_address,
                amount,
                "", // 需要用户私钥或签名授权
            )
            .await?;

        Ok(tx_hash.as_str().to_string())
    }

    /// 释放目标链资产
    async fn release_destination_assets(
        &self,
        to: &CrossChainAddress,
        amount: u128,
    ) -> Result<String, BridgeError> {
        // 从托管钱包在目标链转账给用户
        let custodian_key = self.custodian_wallet.get_private_key(to.chain());

        let tx_hash = self.destination_chain_service
            .transfer(
                &self.custodian_wallet.get_address(to.chain()),
                &Address::new(to.address().to_string())?,
                amount,
                &custodian_key,
            )
            .await?;

        Ok(tx_hash.as_str().to_string())
    }

    /// 等待确认
    async fn wait_for_confirmations(
        &self,
        tx_hash: &str,
        required: u32,
    ) -> Result<(), BridgeError> {
        // 轮询区块链，等待交易确认
        for _ in 0..60 {
            tokio::time::sleep(Duration::from_secs(10)).await;
            // 检查确认数...
        }
        Ok(())
    }
}
```

---

### 5.2 HTLC 原子交换实现（Phase 2）

#### 5.2.1 HTLCBridge 服务接口

```rust
/// HTLC 原子交换桥接口
#[async_trait]
pub trait HTLCBridgeService: Send + Sync {
    /// Alice: 创建交换订单并锁定资产
    async fn initiate_swap(
        &self,
        initiator: SwapParty,
        counterparty: SwapParty,
        hash_lock: HashLock,
        timelock: u64,
    ) -> Result<SwapOrder, BridgeError>;

    /// Bob: 锁定对手方资产
    async fn lock_counterparty_assets(
        &self,
        order_id: &str,
        tx_hash: String,
    ) -> Result<(), BridgeError>;

    /// Alice: 在 Bob 的链上提取资产（公开原像）
    async fn claim_with_preimage(
        &self,
        order_id: &str,
        preimage: Vec<u8>,
    ) -> Result<String, BridgeError>;

    /// Bob: 看到原像后，在 Alice 的链上提取资产
    async fn claim_revealed(
        &self,
        order_id: &str,
    ) -> Result<String, BridgeError>;

    /// 超时退款
    async fn refund_timeout(
        &self,
        order_id: &str,
    ) -> Result<String, BridgeError>;
}
```

---

#### 5.2.2 HTLC 智能合约示例（Solidity）

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// HTLC 合约（以太坊）
contract HTLCEthereum {
    struct HTLC {
        address payable sender;
        address payable receiver;
        uint256 amount;
        bytes32 hashLock;
        uint256 timeLock;
        bool withdrawn;
        bool refunded;
    }

    mapping(bytes32 => HTLC) public contracts;

    event HTLCCreated(
        bytes32 indexed contractId,
        address indexed sender,
        address indexed receiver,
        uint256 amount,
        bytes32 hashLock,
        uint256 timeLock
    );

    event HTLCWithdrawn(bytes32 indexed contractId, bytes32 preimage);
    event HTLCRefunded(bytes32 indexed contractId);

    /// 创建 HTLC
    function createHTLC(
        address payable _receiver,
        bytes32 _hashLock,
        uint256 _timeLock
    ) external payable returns (bytes32 contractId) {
        require(msg.value > 0, "Amount must be > 0");
        require(_timeLock > block.timestamp, "Timelock in the past");

        contractId = keccak256(
            abi.encodePacked(msg.sender, _receiver, msg.value, _hashLock, _timeLock)
        );

        require(contracts[contractId].sender == address(0), "Contract exists");

        contracts[contractId] = HTLC({
            sender: payable(msg.sender),
            receiver: _receiver,
            amount: msg.value,
            hashLock: _hashLock,
            timeLock: _timeLock,
            withdrawn: false,
            refunded: false
        });

        emit HTLCCreated(contractId, msg.sender, _receiver, msg.value, _hashLock, _timeLock);
    }

    /// 提供原像提取资产
    function withdraw(bytes32 _contractId, bytes memory _preimage) external {
        HTLC storage htlc = contracts[_contractId];

        require(htlc.receiver == msg.sender, "Not receiver");
        require(!htlc.withdrawn, "Already withdrawn");
        require(!htlc.refunded, "Already refunded");
        require(sha256(_preimage) == htlc.hashLock, "Invalid preimage");

        htlc.withdrawn = true;
        htlc.receiver.transfer(htlc.amount);

        emit HTLCWithdrawn(_contractId, sha256(_preimage));
    }

    /// 超时退款
    function refund(bytes32 _contractId) external {
        HTLC storage htlc = contracts[_contractId];

        require(htlc.sender == msg.sender, "Not sender");
        require(!htlc.withdrawn, "Already withdrawn");
        require(!htlc.refunded, "Already refunded");
        require(block.timestamp >= htlc.timeLock, "Timelock not expired");

        htlc.refunded = true;
        htlc.sender.transfer(htlc.amount);

        emit HTLCRefunded(_contractId);
    }
}
```

---

#### 5.2.3 Bitcoin HTLC 脚本

```
OP_IF
    OP_SHA256 <hash_lock> OP_EQUALVERIFY
    <receiver_pubkey> OP_CHECKSIG
OP_ELSE
    <timelock> OP_CHECKLOCKTIMEVERIFY OP_DROP
    <sender_pubkey> OP_CHECKSIG
OP_ENDIF
```

**解释**:
- **IF 分支**: 接收方提供原像 + 签名，可提取 BTC
- **ELSE 分支**: 超时后，发送方签名可退款

---

## 6. 安全机制

### 6.1 托管桥安全措施

| 安全威胁 | 防护措施 |
|---------|---------|
| **私钥泄露** | 多签钱包（2/3, 3/5）+ 硬件安全模块（HSM） |
| **内部作恶** | 多方计算（MPC）+ 时间锁 |
| **重放攻击** | Nonce 管理 + 交易 ID 唯一性 |
| **双花攻击** | 等待足够确认数（BTC 6个, ETH 12个） |
| **监管风险** | KYC/AML 合规 + 交易限额 |

---

### 6.2 HTLC 安全考虑

**时间锁设置**:
```
Alice 的时间锁 > Bob 的时间锁 + 安全裕度

例如:
- Alice 在 ETH 锁定: 24小时
- Bob 在 BTC 锁定: 12小时
- 安全裕度: 12小时
```

**原因**: 防止 Bob 在最后一刻不公开原像，导致 Alice 无法在 BTC 链提取资产。

---

### 6.3 审计和监控

```rust
/// 跨链交易审计日志
#[derive(Debug, Serialize)]
pub struct BridgeAuditLog {
    tx_id: String,
    event_type: String,       // "initiated", "locked", "completed", "refunded"
    timestamp: SystemTime,
    source_chain: String,
    destination_chain: String,
    amount: u128,
    initiator: String,
    status: String,
}

/// 异常检测
pub struct AnomalyDetector {
    // 检测异常大额交易
    pub fn detect_large_transfer(&self, amount: u128) -> bool {
        amount > 100_000_000_000_000_000_000 // > 100 ETH
    }

    // 检测异常频率
    pub fn detect_high_frequency(&self, user: &str) -> bool {
        // 检查用户在过去1小时内的交易次数
        false
    }
}
```

---

## 7. 实现路线图

### Phase 1: 基础设施（2-3周）

**目标**: 支持 ETH ↔ BTC 托管桥

**任务**:
- [ ] 集成 `rust-bitcoin` 库
- [ ] 实现 `BitcoinService`（地址生成、余额查询、转账）
- [ ] 实现 `CustodialBridge` 基本功能
- [ ] 搭建托管钱包基础设施（多签）
- [ ] 单元测试 + 集成测试（Testnet）

---

### Phase 2: HTLC 支持（3-4周）

**目标**: ETH ↔ SOL 原子交换

**任务**:
- [ ] 集成 `solana-sdk` 库
- [ ] 实现 `SolanaService`
- [ ] 开发 HTLC 智能合约（Solidity + Rust）
- [ ] 实现 `HTLCBridge` 服务
- [ ] 原子交换流程测试

---

### Phase 3: 生产优化（4-6周）

**目标**: 安全加固 + 性能优化

**任务**:
- [ ] 多签钱包集成
- [ ] 价格预言机集成（汇率计算）
- [ ] 交易监控和告警系统
- [ ] Gas 优化（动态 Gas 策略）
- [ ] 审计日志和合规报告
- [ ] 压力测试和性能基准

---

### Phase 4: 高级功能（未来）

**任务**:
- [ ] 中继网络 + 轻客户端验证
- [ ] 支持 ERC-20/BEP-20 代币
- [ ] Layer 2 支持（Arbitrum, Optimism）
- [ ] NFT 跨链
- [ ] 流动性池和 AMM

---

## 8. CLI 使用示例

### 8.1 托管桥跨链转账

```bash
# ETH → BTC 跨链转账
cargo run -- bridge transfer \
  --from "eth:0xYourEthAddress" \
  --to "btc:bc1YourBtcAddress" \
  --amount "0.1" \
  --bridge-type custodial

# 查询跨链交易状态
cargo run -- bridge status --tx-id "uuid-xxxx-xxxx"

# 超时退款
cargo run -- bridge refund --tx-id "uuid-xxxx-xxxx"
```

---

### 8.2 HTLC 原子交换

```bash
# Alice: 发起交换（ETH → BTC）
cargo run -- swap initiate \
  --send "eth:0.5 ETH" \
  --receive "btc:0.01 BTC" \
  --counterparty "Bob" \
  --timelock 86400

# Bob: 锁定 BTC
cargo run -- swap lock \
  --order-id "swap-xxxx" \
  --tx-hash "btc-tx-hash"

# Alice: 提取 BTC（公开原像）
cargo run -- swap claim \
  --order-id "swap-xxxx" \
  --preimage "0x..."

# Bob: 提取 ETH（使用公开的原像）
cargo run -- swap claim \
  --order-id "swap-xxxx"
```

---

## 9. 技术栈

### 9.1 Rust 依赖

```toml
[dependencies]
# 已有依赖
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
alloy = { version = "0.6", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"

# 跨链桥新增依赖
rust-bitcoin = "0.31"              # Bitcoin 支持
solana-sdk = "1.18"                # Solana 支持
solana-client = "1.18"             # Solana RPC 客户端
sha2 = "0.10"                      # SHA256 哈希
hex = "0.4"                        # 十六进制编码
uuid = { version = "1.0", features = ["v4", "serde"] }
rand = "0.8"                       # 随机数生成
```

---

## 10. 总结

本设计文档提供了多链钱包跨链转账功能的完整架构：

✅ **领域模型**: Chain, CrossChainAddress, BridgeTransaction
✅ **托管桥方案**: 适合 MVP 快速验证
✅ **HTLC 方案**: 去中心化原子交换
✅ **安全机制**: 多签、时间锁、审计日志
✅ **实现路线**: 分阶段开发，先托管后去中心化

**下一步**:
1. 集成 `rust-bitcoin` 和 `solana-sdk`
2. 实现 `CustodialBridge` MVP
3. 在测试网验证完整流程

---

**文档版本**: v1.0.0
**最后更新**: 2025-11-20
**作者**: Claude Code
