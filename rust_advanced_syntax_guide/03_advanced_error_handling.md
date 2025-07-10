# Rust 高级错误处理模式

> 针对有 Python/Java/Node.js 经验的开发者定制，深度对比和详细讲解

## 🔄 错误处理范式的根本差异

### 概念映射表

| 你熟悉的语言 | 错误处理方式 | Rust 对应概念 | 核心差异 |
|-------------|-------------|--------------|----------|
| **Python** | `try/except` + 异常 | `Result<T, E>` | 显式 vs 隐式 |
| **Java** | `try/catch` + 检查异常 | `Result<T, E>` | 编译时强制 vs 运行时 |
| **Node.js** | `Error-first callbacks` + `Promise.catch()` | `Result<T, E>` + `?` 操作符 | 统一语法 vs 分散处理 |

### 思维模式转换图

```mermaid
flowchart TB
    subgraph traditional ["传统异常模式 (Python/Java)"]
        A1[正常代码执行] --> B1{发生错误?}
        B1 -->|"是"| C1[抛出异常]
        B1 -->|"否"| D1[继续执行]
        C1 --> E1[异常冒泡到上层]
        E1 --> F1["try/catch 捕获"]
        F1 --> G1[错误处理逻辑]
    end
    
    subgraph rust ["Rust Result 模式"]
        A2[函数调用] --> B2["返回 Result&lt;T, E&gt;"]
        B2 --> C2{匹配 Result}
        C2 -->|"Ok(value)"| D2[处理成功值]
        C2 -->|"Err(error)"| E2[处理错误值]
        E2 --> F2[决定错误处理策略]
        F2 --> G2[恢复/传播/转换]
    end
    
    subgraph comparison ["关键差异对比"]
        H[关键差异] --> I["Rust: 错误是值的一部分"]
        H --> J["传统: 错误是控制流的中断"]
    end
    
    style A1 fill:#ffcccb
    style A2 fill:#90EE90
    style I fill:#FFD700
    style J fill:#FFA07A
```

### 错误处理模式对比图

```mermaid
flowchart LR
    subgraph python ["Python 异常模型"]
        P1[函数调用] --> P2{执行}
        P2 -->|成功| P3[返回值]
        P2 -->|失败| P4[抛出异常]
        P4 --> P5[异常传播]
    end
    
    subgraph rust2 ["Rust Result 模型"]
        R1[函数调用] --> R2["Result&lt;T, E&gt;"]
        R2 --> R3{模式匹配}
        R3 -->|"Ok(T)"| R4[成功值]
        R3 -->|"Err(E)"| R5[错误值]
        R5 --> R6[显式处理]
    end
    
    style P4 fill:#ffcccb
    style R5 fill:#90EE90
```

## 🎯 为什么 Rust 选择这种方式？

### 核心问题对比

```rust
// Python: 隐式错误
def divide(a, b):
    return a / b  # 可能 ZeroDivisionError，但看不出来

// Java: 部分显式
public int divide(int a, int b) throws ArithmeticException {
    return a / b;  // 显式但冗长
}

// Rust: 完全显式且简洁
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除零错误".to_string())
    } else {
        Ok(a / b)
    }
}
```

## 📊 Result<T, E> 核心概念

### Result 结构和使用

```mermaid
flowchart TB
    subgraph "Result<T, E> 内部结构"
        A[Result枚举] --> B["Ok(T)"]
        A --> C["Err(E)"]
        B --> D[包含成功值]
        C --> E[包含错误信息]
    end
    
    subgraph "使用模式"
        F[match模式匹配] --> G[if let 简化]
        F --> H[? 操作符]
        F --> I[链式调用]
    end
    
    subgraph "错误传播"
        J[? 操作符] --> K[提前返回]
        J --> L[自动转换]
        J --> M[简化代码]
    end
    
    style B fill:#90EE90
    style C fill:#ffcccb
    style H fill:#87CEEB
```

### 基本使用示例

```rust
use std::fs;

// 基础错误处理
fn read_config(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("读取失败: {}", err)),
    }
}

// 使用 ? 操作符简化
fn read_and_parse(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;  // 自动错误传播
    let number = content.trim().parse::<u32>()?;  // 链式错误处理
    Ok(number)
}

// 调用示例
fn main() {
    match read_and_parse("config.txt") {
        Ok(num) => println!("数字: {}", num),
        Err(err) => println!("错误: {}", err),
    }
}
```

## 🏗️ 自定义错误类型

### 错误设计层次图

```mermaid
flowchart TB
    subgraph "错误类型层次"
        A[AppError] --> B[IoError]
        A --> C[ParseError]
        A --> D[BusinessError]
        
        B --> B1[文件不存在]
        B --> B2[权限拒绝]
        
        C --> C1[JSON格式错误]
        C --> C2[数据类型错误]
        
        D --> D1[验证失败]
        D --> D2[业务规则违反]
    end
    
    subgraph "处理策略"
        E[恢复] --> E1[重试]
        E --> E2[使用默认值]
        
        F[传播] --> F1[向上传递]
        F --> F2[转换类型]
        
        G[终止] --> G1[记录日志]
        G --> G2[清理资源]
    end
    
    style A fill:#ff9999
    style E fill:#99ff99
    style F fill:#99ccff
    style G fill:#ffcc99
```

### 实用的错误类型实现

```rust
use std::error::Error;
use std::fmt;

// 自定义错误枚举
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Parse(String),
    Business(String),
}

// 实现 Display trait
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO错误: {}", err),
            AppError::Parse(msg) => write!(f, "解析错误: {}", msg),
            AppError::Business(msg) => write!(f, "业务错误: {}", msg),
        }
    }
}

// 实现 Error trait
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

// 自动转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

// 使用示例
fn process_file(path: &str) -> Result<u32, AppError> {
    let content = std::fs::read_to_string(path)?;  // IO错误自动转换
    
    let number = content.trim().parse::<u32>()
        .map_err(|_| AppError::Parse("无效数字".to_string()))?;
    
    if number == 0 {
        return Err(AppError::Business("数字不能为零".to_string()));
    }
    
    Ok(number)
}
```

## 🔄 错误恢复和重试机制

### 智能重试策略图

```mermaid
flowchart TB
    subgraph "重试决策流程"
        A[发生错误] --> B{错误类型分析}
        
        B -->|网络超时| C[指数退避重试]
        B -->|临时故障| D[固定间隔重试]
        B -->|速率限制| E[遵守限制重试]
        B -->|认证失败| F[刷新令牌后重试]
        B -->|永久错误| G[不重试]
        
        C --> H{重试次数检查}
        D --> H
        E --> H
        F --> H
        
        H -->|未达上限| I[等待后重试]
        H -->|达到上限| J[最终失败]
        
        I --> K[执行操作]
        K --> L{结果}
        L -->|成功| M[返回结果]
        L -->|失败| B
        
        J --> N[错误恢复策略]
        N --> O[降级服务]
        N --> P[缓存回退]
        N --> Q[默认值]
    end
    
    style C fill:#FFE4B5
    style D fill:#E0E0E0
    style E fill:#FFA07A
    style F fill:#98FB98
    style G fill:#FFB6C1
    style O fill:#87CEEB
    style P fill:#DDA0DD
    style Q fill:#F0E68C
```

### 重试模式实现

```rust
use std::time::Duration;

// 重试策略
pub enum RetryStrategy {
    FixedInterval { interval: Duration, max_attempts: usize },
    ExponentialBackoff { initial: Duration, max_attempts: usize },
}

// 可重试错误特征
pub trait RetryableError {
    fn is_retryable(&self) -> bool;
}

// 为自定义错误实现重试逻辑
impl RetryableError for AppError {
    fn is_retryable(&self) -> bool {
        match self {
            AppError::Io(_) => true,       // IO错误可重试
            AppError::Parse(_) => false,   // 解析错误不重试
            AppError::Business(_) => false, // 业务错误不重试
        }
    }
}

// 重试执行器
pub struct RetryExecutor {
    strategy: RetryStrategy,
}

impl RetryExecutor {
    pub fn new(strategy: RetryStrategy) -> Self {
        Self { strategy }
    }
    
    pub fn execute<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: RetryableError,
    {
        let max_attempts = match &self.strategy {
            RetryStrategy::FixedInterval { max_attempts, .. } => *max_attempts,
            RetryStrategy::ExponentialBackoff { max_attempts, .. } => *max_attempts,
        };
        
        for attempt in 0..max_attempts {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if !error.is_retryable() || attempt == max_attempts - 1 {
                        return Err(error);
                    }
                    
                    // 计算等待时间并重试
                    let delay = self.calculate_delay(attempt);
                    std::thread::sleep(delay);
                }
            }
        }
        
        unreachable!()
    }
    
    fn calculate_delay(&self, attempt: usize) -> Duration {
        match &self.strategy {
            RetryStrategy::FixedInterval { interval, .. } => *interval,
            RetryStrategy::ExponentialBackoff { initial, .. } => {
                Duration::from_millis(initial.as_millis() as u64 * 2_u64.pow(attempt as u32))
            }
        }
    }
}

// 使用示例
fn unreliable_network_call() -> Result<String, AppError> {
    // 模拟70%失败率的网络调用
    if rand::random::<f64>() < 0.7 {
        Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "网络超时"
        )))
    } else {
        Ok("成功响应".to_string())
    }
}

fn main() {
    let retry_executor = RetryExecutor::new(RetryStrategy::ExponentialBackoff {
        initial: Duration::from_millis(100),
        max_attempts: 5,
    });
    
    match retry_executor.execute(unreliable_network_call) {
        Ok(response) => println!("✅ 最终成功: {}", response),
        Err(err) => println!("❌ 重试后仍失败: {}", err),
    }
}
```

## 🎯 学习要点总结

### 错误处理学习思维图

```mermaid
mindmap
  root((Rust 错误处理))
    Result类型
      Ok成功值
      Err错误值
      问号操作符
      模式匹配
    自定义错误
      错误枚举
      Display特征
      Error特征
      From转换
    重试机制
      策略模式
      指数退避
      错误分类
      恢复策略
    最佳实践
      显式处理
      错误链追踪
      上下文信息
      类型安全
```

### 核心概念对比表

| 概念 | Python/Java | Node.js | Rust | 关键差异 |
|------|-------------|---------|------|----------|
| **错误表示** | 异常对象 | Error对象 | `Result<T, E>` | 值 vs 控制流 |
| **错误传播** | `throw`/`raise` | `throw`/`reject` | `?` 操作符 | 显式 vs 隐式 |
| **错误处理** | `try/catch` | `try/catch`/`.catch()` | 模式匹配 | 强制 vs 可选 |
| **类型安全** | 运行时检查 | 运行时检查 | 编译时保证 | 静态 vs 动态 |

### 学习路径建议

1. **掌握 Result<T, E>** - 理解 Rust 错误处理的核心
2. **熟练使用 ? 操作符** - 简化错误传播代码  
3. **设计错误类型** - 创建有意义的错误层次
4. **实现重试机制** - 处理临时性错误
5. **培养错误思维** - 将错误视为值而非异常

### 实践建议

- ✅ 优先使用 `Result<T, E>` 而不是 panic
- ✅ 为每个模块定义专门的错误类型
- ✅ 使用 `?` 操作符简化错误传播
- ✅ 提供有用的错误信息和上下文
- ✅ 区分可恢复和不可恢复的错误

---

**掌握了错误处理，下一章我们将学习异步编程的核心概念！** 🚀