# 分布式雪花算法ID生成器 - Rust实现

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📖 项目简介

这是一个基于Twitter Snowflake算法的高性能分布式ID生成器，使用Rust语言实现。采用全局锁设计，确保在多线程环境下的线程安全性和ID的全局唯一性。

### 🌟 核心特性

- **分布式友好**：多台机器同时运行，生成的ID绝对不重复
- **全局锁设计**：所有线程共享一个ID生成器，安全可靠
- **时间有序**：ID按时间递增，便于数据库索引和排序
- **智能分配**：支持多种Worker ID分配策略，适应各种部署环境
- **零外部依赖**：不需要Redis、ZooKeeper等外部服务
- **高性能**：Release模式下单线程可达400万+IDs/秒

## 🔢 ID结构说明

雪花算法生成的ID为64位整数，结构如下：

```
+------------------+------------------+------------------+
| 时间戳 (41 bits) | 节点ID (8 bits)  | 序列号 (12 bits) |
+------------------+------------------+------------------+
|   毫秒级时间戳    |    0-255        |     0-4095       |
+------------------+------------------+------------------+
```

- **时间戳 (41位)**：从2025-03-08开始的毫秒数，可用69年
- **节点ID (8位)**：Worker ID，标识不同的机器/进程，支持256个节点
- **序列号 (12位)**：同一毫秒内的计数器，支持每毫秒4096个ID

## 🚀 快速开始

### 安装依赖

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
hostname = "0.3"
```

### 基本使用

```rust
use snowflake::{get_next_id, set_global_config, SnowflakeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用默认配置
    let id = get_next_id()?;
    println!("生成的ID: {}", id);
    
    // 自定义配置
    let config = SnowflakeConfig {
        worker_id_bits: 8,
        sequence_bits: 12,
        max_backward_ms: 10,
    };
    set_global_config(config);
    
    // 批量生成
    for i in 0..10 {
        let id = get_next_id()?;
        println!("ID_{}: {}", i + 1, id);
    }
    
    Ok(())
}
```

### 运行示例

```bash
# 编译项目
cargo build --release

# 运行演示程序
cargo run --release --bin snowflake

# 运行压力测试
cargo run --release --bin stress_test
```

## ⚙️ 配置文件详解

### 配置文件位置

系统会按照以下优先级查找配置文件：
1. `./snowflake.toml` (当前目录)
2. `/etc/snowflake.toml` (系统目录)

### 配置文件格式

```toml
# =============================================================================
# 雪花算法分布式配置文件
# 用途: 确保多台机器生成的ID绝对不重复
# =============================================================================

# === 核心配置 (数据中心+机器ID组合方式) ===
# 数据中心ID (0-3, 占用Worker ID的高2位)
# 例如: 东部=0, 西部=1, 北部=2, 南部=3
datacenter_id = 1

# 机器ID (0-63, 占用Worker ID的低6位)  
# 例如: 第1台机器=1, 第2台机器=2, 依此类推
machine_id = 5

# 最终Worker ID计算公式: (datacenter_id << 6) | machine_id
# 示例: datacenter_id=1, machine_id=5 → Worker ID = 69

# === 算法参数 (一般不需要修改) ===
# Worker ID位数 (8位支持256个不同的Worker ID: 0-255)
worker_id_bits = 8

# 序列号位数 (12位支持每毫秒生成4096个ID)
sequence_bits = 12

# 时钟回拨容忍度 (毫秒)
max_backward_ms = 10
```

### 配置参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `datacenter_id` | u8 | 无 | 数据中心ID (0-3)，用于多数据中心部署 |
| `machine_id` | u8 | 无 | 机器ID (0-63)，用于同一数据中心内区分不同机器 |
| `worker_id_bits` | u8 | 8 | Worker ID的位数，决定支持的节点数量 |
| `sequence_bits` | u8 | 12 | 序列号位数，决定单毫秒内的ID生成数量 |
| `max_backward_ms` | u64 | 10 | 时钟回拨容忍度，超过此值将抛出错误 |

## 🎯 Worker ID分配策略

系统支持三种Worker ID分配方式，按照优先级从高到低：

### 1. 环境变量 (最高优先级)

```bash
# 直接指定Worker ID
export SNOWFLAKE_WORKER_ID=50
./your_app

# Docker部署
docker run -e SNOWFLAKE_WORKER_ID=10 your_image

# Kubernetes部署
env:
  - name: SNOWFLAKE_WORKER_ID
    value: "10"
```

### 2. 配置文件映射 (中等优先级)

通过数据中心ID和机器ID组合计算：

```toml
datacenter_id = 1  # 数据中心1
machine_id = 5     # 机器5
# 最终Worker ID = (1 << 6) | 5 = 69
```

**数据中心分配示例**：
- 东部数据中心 (datacenter_id=0): Worker ID 0-63
- 西部数据中心 (datacenter_id=1): Worker ID 64-127  
- 北部数据中心 (datacenter_id=2): Worker ID 128-191
- 南部数据中心 (datacenter_id=3): Worker ID 192-255

### 3. IP段自动分配 (最低优先级)

系统自动根据本机IP地址计算Worker ID：

```
IP: 192.168.1.100
Worker ID = ((1 & 15) << 4) | (100 & 15) = 20
```

如果IP获取失败，则使用主机名长度取模的方式计算。

## 🔧 部署指南

### 单机部署

```bash
# 使用环境变量
export SNOWFLAKE_WORKER_ID=1
cargo run --release --bin snowflake
```

### 多机部署

每台机器设置不同的Worker ID：

```bash
# 机器1
export SNOWFLAKE_WORKER_ID=1

# 机器2  
export SNOWFLAKE_WORKER_ID=2

# 机器3
export SNOWFLAKE_WORKER_ID=3
```

### Docker部署

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/snowflake /usr/local/bin/
COPY snowflake.toml /etc/
EXPOSE 8080
CMD ["snowflake"]
```

```bash
# 运行容器
docker run -e SNOWFLAKE_WORKER_ID=10 snowflake:latest
```

### Kubernetes部署

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: snowflake-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: snowflake
  template:
    metadata:
      labels:
        app: snowflake
    spec:
      containers:
      - name: snowflake
        image: snowflake:latest
        env:
        - name: SNOWFLAKE_WORKER_ID
          valueFrom:
            fieldRef:
              fieldPath: metadata.name  # 使用Pod名称生成唯一ID
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: config
          mountPath: /etc/snowflake.toml
          subPath: snowflake.toml
      volumes:
      - name: config
        configMap:
          name: snowflake-config
```

## 📊 性能测试

### 测试环境
- **CPU**: Apple M1 Pro
- **内存**: 16GB
- **Rust版本**: 1.70+
- **编译模式**: Release

### 性能指标

| 线程数 | 总ID数 | 耗时 | 性能 (IDs/秒) | 唯一率 |
|--------|---------|------|---------------|--------|
| 1 | 100,000 | 24ms | 4,100,000+ | 100% |
| 8 | 400,000 | 114ms | 3,500,000+ | 100% |
| 16 | 800,000 | 259ms | 3,090,000+ | 100% |
| 32 | 1,600,000 | 535ms | 2,990,000+ | 100% |
| 64 | 3,200,000 | 1.04s | 3,075,000+ | 100% |
| 128 | 6,400,000 | 1.96s | 3,270,000+ | 100% |

## ⚠️ 注意事项

### 时钟回拨处理

系统内置时钟回拨检测和处理机制：

1. **容忍范围内**：自动等待时钟追上，默认容忍10毫秒
2. **超出容忍范围**：抛出 `ClockBackward` 错误

```rust
// 自定义时钟回拨容忍度
let config = SnowflakeConfig {
    max_backward_ms: 100,  // 容忍100毫秒回拨
    ..Default::default()
};
```

### Worker ID冲突避免

- **生产环境**：务必为每台机器设置唯一的Worker ID
- **测试环境**：可以使用IP自动分配
- **容器环境**：建议使用环境变量明确指定

### 序列号耗尽处理

当同一毫秒内序列号用尽(达到4096)时，系统会自动等待到下一毫秒。

### 数据库存储建议

```sql
-- 推荐使用BIGINT存储
CREATE TABLE orders (
    id BIGINT PRIMARY KEY,  -- 雪花算法ID
    user_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_created_at (created_at)
);
```

## 🔍 错误处理

### 常见错误类型

```rust
use snowflake::SnowflakeError;

match get_next_id() {
    Ok(id) => println!("生成ID: {}", id),
    Err(SnowflakeError::ClockBackward(msg)) => {
        eprintln!("时钟回拨错误: {}", msg);
    },
    Err(SnowflakeError::ConfigError(msg)) => {
        eprintln!("配置错误: {}", msg);
    },
    Err(SnowflakeError::NetworkError(msg)) => {
        eprintln!("网络错误: {}", msg);
    },
}
```

### 错误恢复策略

1. **时钟回拨**：等待系统时钟恢复或重启服务
2. **配置错误**：检查配置文件格式和参数范围
3. **网络错误**：检查网络连接和主机名解析

## 🛠️ 开发指南

### 编译和运行

```bash
# 开发模式编译
cargo build

# 发布模式编译（性能最佳）
cargo build --release

# 运行测试
cargo test

# 运行示例
cargo run --bin snowflake

# 运行压力测试
cargo run --bin stress_test
```

### 代码结构

```
├── src/
│   ├── snowflake.rs          # 主要实现文件
│   └── stress_test.rs        # 压力测试程序
├── snowflake.toml            # 配置文件示例
├── Cargo.toml               # 项目配置
└── README.md                # 文档
```

### API参考

```rust
// 核心函数
pub fn get_next_id() -> Result<u64, SnowflakeError>
pub fn set_global_config(config: SnowflakeConfig)

// 配置结构
pub struct SnowflakeConfig {
    pub worker_id_bits: u8,
    pub sequence_bits: u8, 
    pub max_backward_ms: u64,
}

// 错误类型
pub enum SnowflakeError {
    ClockBackward(String),
    ConfigError(String),
    NetworkError(String),
}
```

## 📝 版本历史

- **v2.1.0** - 全局锁简化版
  - 移除复杂的线程本地存储
  - 采用全局锁设计，提高稳定性
  - 优化主机名计算逻辑
  - 清理垃圾代码和废弃注释

- **v2.0.0** - 高性能重构版（已废弃）
  - 线程本地存储实现
  - 混合模式设计

## 🤝 贡献指南

欢迎提交Issue和Pull Request！

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交变更 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 👥 作者

- **zdrawai团队** - *初始工作* - [zdrawai](https://github.com/zdrawai)

## 🙏 致谢

- Twitter Snowflake算法的原始设计
- Rust社区的优秀生态系统
- 所有贡献者和用户的反馈