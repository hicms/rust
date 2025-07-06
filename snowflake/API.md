# 雪花算法 API 文档

## 概述

本文档详细介绍雪花算法库的所有公共API接口、数据结构和使用方法。

## 公共接口

### 核心函数

#### `get_next_id()`

生成下一个唯一的雪花算法ID。

```rust
pub fn get_next_id() -> Result<u64, SnowflakeError>
```

**返回值**：
- `Ok(u64)` - 成功生成的64位唯一ID
- `Err(SnowflakeError)` - 生成失败的错误信息

**示例**：
```rust
use snowflake::get_next_id;

match get_next_id() {
    Ok(id) => println!("生成的ID: {}", id),
    Err(e) => eprintln!("生成失败: {}", e),
}
```

**线程安全性**：
- ✅ 线程安全：多个线程可以并发调用
- ✅ 全局唯一：保证在所有线程中生成的ID不重复

---

#### `set_global_config()`

设置全局配置参数。

```rust
pub fn set_global_config(config: SnowflakeConfig)
```

**参数**：
- `config: SnowflakeConfig` - 新的配置参数

**注意事项**：
- 配置修改只对新创建的Worker实例生效
- 建议在程序启动时调用一次
- 运行时修改配置可能导致不可预期的行为

**示例**：
```rust
use snowflake::{set_global_config, SnowflakeConfig};

let config = SnowflakeConfig {
    worker_id_bits: 8,
    sequence_bits: 12,
    max_backward_ms: 100,
};
set_global_config(config);
```

---

#### `demo()`

演示程序，展示雪花算法的基本功能和性能。

```rust
pub fn demo()
```

**功能**：
- 显示当前Worker ID
- 生成5个示例ID
- 执行单线程性能测试（10万个ID）
- 显示配置信息

**示例**：
```rust
use snowflake::demo;

demo();
```

## 数据结构

### `SnowflakeConfig`

雪花算法配置结构体。

```rust
#[derive(Debug, Clone)]
pub struct SnowflakeConfig {
    pub worker_id_bits: u8,
    pub sequence_bits: u8,
    pub max_backward_ms: u64,
}
```

**字段说明**：

#### `worker_id_bits: u8`
- **说明**：Worker ID的位数
- **范围**：1-22位
- **默认值**：8位
- **影响**：决定支持的最大节点数量 = 2^worker_id_bits

**推荐配置**：
- 小型部署 (≤16台机器)：6位 (64个节点)
- 中型部署 (≤256台机器)：8位 (256个节点) ⭐ 推荐
- 大型部署 (≤1024台机器)：10位 (1024个节点)

#### `sequence_bits: u8`
- **说明**：序列号的位数
- **范围**：1-22位
- **默认值**：12位
- **影响**：决定每毫秒最大ID生成数量 = 2^sequence_bits

**推荐配置**：
- 低并发：10位 (每毫秒1024个ID)
- 中等并发：12位 (每毫秒4096个ID) ⭐ 推荐
- 高并发：14位 (每毫秒16384个ID)

#### `max_backward_ms: u64`
- **说明**：时钟回拨容忍度（毫秒）
- **范围**：0-10000毫秒
- **默认值**：10毫秒
- **影响**：超过此值的时钟回拨将抛出错误

**推荐配置**：
- 生产环境：10-100毫秒
- 开发环境：1000毫秒
- 时钟不稳定环境：5000毫秒

---

### `Default` 实现

```rust
impl Default for SnowflakeConfig {
    fn default() -> Self {
        Self {
            worker_id_bits: 8,          // 支持256个节点
            sequence_bits: 12,          // 每毫秒4096个ID
            max_backward_ms: 10,        // 容忍10毫秒时钟回拨
        }
    }
}
```

## 错误类型

### `SnowflakeError`

雪花算法错误枚举。

```rust
#[derive(Debug)]
pub enum SnowflakeError {
    ClockBackward(String),
    ConfigError(String),
    NetworkError(String),
}
```

#### `ClockBackward(String)`
- **触发条件**：系统时钟回拨超过容忍范围
- **错误信息**：包含回拨时间和容忍范围
- **处理建议**：等待系统时钟恢复或重启服务

**示例**：
```
时钟回拨错误: 时钟回拨过大: 150ms，超出容忍范围10ms
```

#### `ConfigError(String)`
- **触发条件**：配置参数不合法或基准时间设置错误
- **错误信息**：具体的配置错误描述
- **处理建议**：检查配置参数范围和格式

**常见错误**：
```
配置错误: 无法创建上海时区
配置错误: 无效的基准日期
配置错误: 基准时间设置在未来
```

#### `NetworkError(String)`
- **触发条件**：获取本机IP地址或主机名失败
- **错误信息**：网络操作失败的描述
- **处理建议**：检查网络连接和主机名解析

**常见错误**：
```
网络错误: 无法获取主机名
```

---

### `Display` 和 `Error` 实现

```rust
impl std::fmt::Display for SnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowflakeError::ClockBackward(msg) => write!(f, "时钟回拨错误: {}", msg),
            SnowflakeError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            SnowflakeError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
        }
    }
}

impl std::error::Error for SnowflakeError {}
```

## 内部实现（不公开）

### `SnowflakeIdWorker`

核心ID生成器结构体（内部使用）。

```rust
struct SnowflakeIdWorker {
    config: SnowflakeConfig,
    worker_id_shift: u8,
    timestamp_shift: u8,
    sequence_mask: u64,
    twepoch: u64,
    sequence: u64,
    last_timestamp: i64,
    worker_id: u8,
}
```

**关键方法**：
- `new()` - 创建新实例
- `next_id()` - 生成下一个ID
- `init_worker_id()` - 初始化Worker ID
- `time_gen()` - 获取当前时间戳
- `wait_next_millis()` - 等待下一毫秒

## 使用模式

### 基本使用

```rust
use snowflake::get_next_id;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = get_next_id()?;
    println!("ID: {}", id);
    Ok(())
}
```

### 批量生成

```rust
use snowflake::get_next_id;

fn generate_batch(count: usize) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut ids = Vec::with_capacity(count);
    for _ in 0..count {
        ids.push(get_next_id()?);
    }
    Ok(ids)
}
```

### 多线程使用

```rust
use std::thread;
use std::sync::Arc;
use snowflake::get_next_id;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                for j in 0..1000 {
                    match get_next_id() {
                        Ok(id) => println!("线程{} ID{}: {}", i, j, id),
                        Err(e) => eprintln!("线程{} 错误: {}", i, e),
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    
    Ok(())
}
```

### 自定义配置

```rust
use snowflake::{get_next_id, set_global_config, SnowflakeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 高并发配置
    let config = SnowflakeConfig {
        worker_id_bits: 6,      // 64个节点
        sequence_bits: 14,      // 每毫秒16384个ID
        max_backward_ms: 100,   // 容忍100ms回拨
    };
    set_global_config(config);
    
    let id = get_next_id()?;
    println!("高并发ID: {}", id);
    Ok(())
}
```

### 错误处理

```rust
use snowflake::{get_next_id, SnowflakeError};

fn safe_generate() -> Option<u64> {
    match get_next_id() {
        Ok(id) => Some(id),
        Err(SnowflakeError::ClockBackward(msg)) => {
            eprintln!("时钟回拨，等待重试: {}", msg);
            std::thread::sleep(std::time::Duration::from_millis(100));
            // 可以选择重试
            None
        },
        Err(e) => {
            eprintln!("生成ID失败: {}", e);
            None
        }
    }
}
```

## 性能考虑

### 单线程性能
- **Release模式**：400万+ IDs/秒
- **Debug模式**：100万+ IDs/秒

### 多线程性能
- **8线程**：350万+ IDs/秒
- **32线程**：299万+ IDs/秒
- **128线程**：327万+ IDs/秒

### 内存使用
- **静态内存**：约1KB
- **每次调用**：零分配（除错误情况）

### 优化建议

1. **使用Release模式**：
   ```bash
   cargo build --release
   ```

2. **避免频繁配置更改**：
   ```rust
   // 好的做法：启动时设置一次
   set_global_config(config);
   
   // 避免：运行时频繁更改
   // set_global_config(new_config); // 不推荐
   ```

3. **批量生成**：
   ```rust
   // 高效的批量生成
   let ids: Result<Vec<_>, _> = (0..1000)
       .map(|_| get_next_id())
       .collect();
   ```

## 兼容性

### Rust版本
- **最低要求**：Rust 1.70+
- **推荐版本**：Rust 1.75+

### 平台支持
- ✅ Linux (x86_64, ARM64)
- ✅ macOS (Intel, Apple Silicon)
- ✅ Windows (x86_64)
- ✅ FreeBSD
- ⚠️ 其他Unix系统（未测试）

### 依赖项
- `chrono` - 时间处理
- `once_cell` - 静态初始化
- `hostname` - 主机名获取

所有依赖都是稳定的生产级库，无额外的系统要求。