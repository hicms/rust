# 雪花算法快速开始指南

🚀 5分钟快速上手雪花算法ID生成器

## 📦 1. 快速安装

### 克隆项目
```bash
git clone https://github.com/your-org/snowflake-rust.git
cd snowflake-rust
```

### 编译项目
```bash
# 开发版本（快速编译）
cargo build

# 生产版本（性能最佳）
cargo build --release
```

## 🏃‍♂️ 2. 立即运行

### 运行演示程序
```bash
cargo run --bin snowflake
```

**输出示例：**
```
=== 🔒 全局锁雪花算法演示 ===
📄 找到配置文件: snowflake.toml
🏢 数据中心+机器ID: DC1 + M5 → Worker ID 69
✅ 使用配置文件映射Worker ID: 69
Worker ID: 69

📝 生成5个示例ID:
ID_1: 10874550226276352
ID_2: 10874550226276353
ID_3: 10874550226276354
ID_4: 10874550226276355
ID_5: 10874550226276356

⚡ 性能测试 (单线程):
生成 100000 个ID，耗时: 23.952011ms
性能: 4175015 IDs/秒
```

### 运行压力测试
```bash
cargo run --release --bin stress_test
```

## 💻 3. 代码集成

### 最简单的使用方式

创建 `main.rs` 文件：

```rust
// main.rs
use snowflake::get_next_id;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 生成一个ID
    let id = get_next_id()?;
    println!("生成的ID: {}", id);
    
    // 批量生成
    for i in 1..=10 {
        let id = get_next_id()?;
        println!("ID_{}: {}", i, id);
    }
    
    Ok(())
}
```

### 添加到 Cargo.toml

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
hostname = "0.3"

[[bin]]
name = "my_app"
path = "main.rs"
```

### 运行你的应用

```bash
cargo run --bin my_app
```

## ⚙️ 4. 基本配置

### 快速配置

编辑 `snowflake.toml` 文件：

```toml
# 数据中心ID (0-3)
datacenter_id = 0

# 机器ID (0-63)  
machine_id = 1

# 时钟回拨容忍度
max_backward_ms = 10
```

### 环境变量配置（推荐）

```bash
# 直接指定Worker ID
export SNOWFLAKE_WORKER_ID=42

# 运行程序
cargo run --bin snowflake
```

## 🐳 5. Docker快速部署

### 创建 Dockerfile

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/snowflake /usr/local/bin/
COPY snowflake.toml /etc/
CMD ["snowflake"]
```

### 构建和运行

```bash
# 构建镜像
docker build -t snowflake .

# 运行容器
docker run -e SNOWFLAKE_WORKER_ID=1 snowflake
```

## 🔧 6. 常见使用场景

### Web API集成

```rust
use snowflake::get_next_id;
use std::collections::HashMap;

// 模拟Web API响应
fn create_order() -> Result<HashMap<String, String>, String> {
    let order_id = get_next_id().map_err(|e| e.to_string())?;
    
    let mut response = HashMap::new();
    response.insert("order_id".to_string(), order_id.to_string());
    response.insert("status".to_string(), "created".to_string());
    
    Ok(response)
}

fn main() {
    match create_order() {
        Ok(order) => println!("订单创建成功: {:?}", order),
        Err(e) => println!("订单创建失败: {}", e),
    }
}
```

### 数据库主键生成

```rust
use snowflake::get_next_id;

struct User {
    id: u64,
    username: String,
    email: String,
}

impl User {
    fn new(username: String, email: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(User {
            id: get_next_id()?,
            username,
            email,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(
        "alice".to_string(),
        "alice@example.com".to_string()
    )?;
    
    println!("新用户: ID={}, 用户名={}", user.id, user.username);
    Ok(())
}
```

### 多线程批量生成

```rust
use snowflake::get_next_id;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    // 启动5个线程，每个生成1000个ID
    for thread_id in 0..5 {
        let results = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let mut thread_ids = Vec::new();
            
            for _ in 0..1000 {
                match get_next_id() {
                    Ok(id) => thread_ids.push(id),
                    Err(e) => eprintln!("线程{} 生成ID失败: {}", thread_id, e),
                }
            }
            
            // 将结果添加到共享向量
            let mut shared_results = results.lock().unwrap();
            shared_results.extend(thread_ids);
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_results = results.lock().unwrap();
    println!("总共生成 {} 个唯一ID", final_results.len());
    
    // 验证唯一性
    let mut sorted_ids = final_results.clone();
    sorted_ids.sort();
    sorted_ids.dedup();
    
    if sorted_ids.len() == final_results.len() {
        println!("✅ 所有ID都是唯一的！");
    } else {
        println!("❌ 发现重复ID！");
    }
    
    Ok(())
}
```

## 📊 7. 性能验证

### 简单性能测试

```rust
use snowflake::get_next_id;
use std::time::Instant;

fn benchmark_single_thread(count: usize) {
    println!("🚀 单线程性能测试 ({} 个ID):", count);
    
    let start = Instant::now();
    let mut success_count = 0;
    
    for _ in 0..count {
        if get_next_id().is_ok() {
            success_count += 1;
        }
    }
    
    let duration = start.elapsed();
    let rate = success_count as f64 / duration.as_secs_f64();
    
    println!("⏱️  耗时: {:?}", duration);
    println!("📈 性能: {:.0} IDs/秒", rate);
    println!("✅ 成功率: {:.2}%", (success_count as f64 / count as f64) * 100.0);
}

fn main() {
    benchmark_single_thread(100_000);
}
```

## 🔍 8. 故障排查

### 检查Worker ID

```rust
use snowflake::{get_next_id, demo};

fn debug_worker_info() {
    println!("🔍 调试信息:");
    
    // 检查环境变量
    if let Ok(worker_id) = std::env::var("SNOWFLAKE_WORKER_ID") {
        println!("📌 环境变量Worker ID: {}", worker_id);
    } else {
        println!("❌ 未设置环境变量SNOWFLAKE_WORKER_ID");
    }
    
    // 生成几个ID看看
    println!("🎯 生成测试ID:");
    for i in 1..=3 {
        match get_next_id() {
            Ok(id) => println!("  ID_{}: {}", i, id),
            Err(e) => println!("  错误_{}: {}", i, e),
        }
    }
    
    // 运行完整演示
    println!("\n🎪 完整演示:");
    demo();
}

fn main() {
    debug_worker_info();
}
```

### 验证配置文件

```bash
# 检查配置文件是否存在
ls -la snowflake.toml

# 查看配置内容  
cat snowflake.toml

# 验证语法
echo "datacenter_id = 1" | cargo run --bin snowflake -- --check-config
```

## 📚 9. 下一步

现在你已经成功运行了雪花算法！继续探索：

### 📖 深入学习
- 阅读 [README.md](README.md) 了解完整功能
- 查看 [API.md](API.md) 学习所有接口
- 研究 [DEPLOYMENT.md](DEPLOYMENT.md) 进行生产部署

### 🛠️ 高级配置
```toml
# 高并发配置
worker_id_bits = 6      # 64个节点
sequence_bits = 14      # 每毫秒16384个ID
max_backward_ms = 100   # 100ms时钟容忍

# 低延迟配置  
worker_id_bits = 10     # 1024个节点
sequence_bits = 10      # 每毫秒1024个ID
max_backward_ms = 5     # 5ms时钟容忍
```

### 🌐 集群部署
```bash
# 机器1
export SNOWFLAKE_WORKER_ID=1
cargo run --release --bin snowflake

# 机器2  
export SNOWFLAKE_WORKER_ID=2
cargo run --release --bin snowflake

# 机器3
export SNOWFLAKE_WORKER_ID=3
cargo run --release --bin snowflake
```

### 📊 监控和告警
```rust
use snowflake::get_next_id;

// 简单的健康检查
fn health_check() -> bool {
    match get_next_id() {
        Ok(_) => true,
        Err(e) => {
            eprintln!("❌ 健康检查失败: {}", e);
            false
        }
    }
}

fn main() {
    loop {
        if health_check() {
            println!("✅ 服务正常");
        } else {
            println!("⚠️ 服务异常");
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
```

## 🆘 获取帮助

如果遇到问题：

1. **查看日志**：程序输出包含详细的调试信息
2. **检查配置**：确保Worker ID没有冲突
3. **验证时钟**：确保系统时钟同步
4. **阅读文档**：查看完整的API和部署文档
5. **提交Issue**：在GitHub上提交问题报告

**快速排查清单：**
- ✅ Rust版本 ≥ 1.70
- ✅ 编译成功，无错误
- ✅ Worker ID唯一（多机部署时）
- ✅ 系统时钟同步
- ✅ 网络连接正常（IP获取）

恭喜！你已经掌握了雪花算法的基本使用 🎉