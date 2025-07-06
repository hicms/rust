# Moka Demo - Rust缓存库学习项目

这是一个展示Rust高性能缓存库Moka用法的演示项目。

## 项目概述

Moka是一个基于Rust的高性能并发缓存库，它提供了丰富的缓存功能，包括：

- **异步支持**：完全异步的API设计
- **TTL支持**：自动过期清理
- **LRU淘汰**：基于使用频率的智能淘汰策略
- **并发安全**：内置的并发控制机制
- **防缓存击穿**：get_with方法避免重复计算
- **统计信息**：详细的缓存命中率和性能统计

## 项目结构

```
moka_demo/
├── Cargo.toml          # 项目依赖配置
├── src/
│   └── main.rs         # 主要演示代码
└── README.md           # 项目文档
```

## 依赖项

- `moka` v0.12 - 核心缓存库，启用future特性
- `tokio` v1.0 - 异步运行时，启用全部特性

## 功能演示

### 1. 缓存实例创建

```rust
let cache: Cache<String, String> = Cache::builder()
    .max_capacity(100)                      // 最大容量100个条目
    .time_to_live(Duration::from_secs(60))  // TTL为60秒
    .build();
```

### 2. 基本操作

- **插入**：`cache.insert(key, value).await`
- **获取**：`cache.get(&key).await`
- **删除**：`cache.invalidate(&key).await`
- **统计**：`cache.entry_count()`

### 3. 高级功能

#### get_with - 防缓存击穿

```rust
let value = cache.get_with("key".to_string(), async {
    // 耗时计算逻辑
    expensive_computation().await
}).await;
```

这个方法的优势：
- 如果缓存中存在，直接返回
- 如果不存在，执行计算逻辑并缓存结果
- 多个并发请求同时访问同一个键时，只有一个请求会执行计算

#### 批量操作

支持批量插入和管理大量缓存条目。

#### 后台任务管理

`run_pending_tasks()` 确保所有后台维护任务完成，包括过期条目清理。

## 运行演示

### 前置条件

- Rust 1.60+
- Cargo

### 运行步骤

1. 克隆或下载项目
2. 进入项目目录：
   ```bash
   cd moka_demo
   ```
3. 运行演示：
   ```bash
   cargo run
   ```

## 运行结果

程序会展示以下演示内容：

1. **缓存实例创建** - 展示如何配置缓存
2. **基本缓存操作** - 插入、读取、未命中处理
3. **缓存统计** - 当前缓存大小
4. **get_with演示** - 条件计算和缓存
5. **缓存失效** - 主动删除缓存条目
6. **批量操作** - 批量插入和管理
7. **最终统计** - 完整的缓存状态

## 性能特点

- **高并发**：支持多线程并发访问
- **内存效率**：智能的内存管理和淘汰策略
- **低延迟**：高效的数据结构和算法
- **可配置**：灵活的配置选项满足不同需求

## 适用场景

- Web应用缓存
- API响应缓存
- 计算结果缓存
- 数据库查询缓存
- 会话存储
- 任何需要高性能缓存的场景

## 进一步学习

- [Moka官方文档](https://docs.rs/moka/)
- [Moka GitHub仓库](https://github.com/moka-rs/moka)
- [Rust异步编程](https://tokio.rs/)

## 许可证

本项目遵循MIT许可证。

## 贡献

欢迎提交Issue和Pull Request来改进这个演示项目。