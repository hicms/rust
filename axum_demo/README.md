# Axum WebSocket 和 API 演示项目

这是一个使用 Rust 语言和 Axum 框架创建的 Web 服务器演示项目，展示了如何同时支持 REST API 和 WebSocket 连接。

## 📋 项目概述

本项目适合 Rust 初学者学习，包含以下功能：
- REST API 接口（用户管理）
- WebSocket 实时通信
- 静态文件服务（HTML 测试页面）
- 完整的测试界面

## 🛠 技术栈

- **Rust** - 系统编程语言
- **Axum** - 现代 Web 框架
- **Tokio** - 异步运行时
- **Serde** - JSON 序列化/反序列化
- **Tower** - 中间件支持
- **WebSocket** - 实时双向通信

## 📦 依赖说明

```toml
[dependencies]
axum = { version = "0.7", features = ["ws"] }  # Web 框架，启用 WebSocket 支持
tokio = { version = "1.0", features = ["full"] }  # 异步运行时
tower = "0.4"  # 中间件框架
tower-http = { version = "0.5", features = ["cors", "trace", "fs"] }  # HTTP 中间件
tracing = "0.1"  # 日志跟踪
tracing-subscriber = "0.3"  # 日志订阅器
serde = { version = "1.0", features = ["derive"] }  # 序列化
serde_json = "1.0"  # JSON 支持
tokio-tungstenite = "0.21"  # WebSocket 实现
futures-util = "0.3"  # 异步工具
chrono = { version = "0.4", features = ["serde"] }  # 时间处理
```

## 🚀 快速开始

### 1. 安装 Rust

如果还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 进行安装。

### 2. 克隆并运行项目

```bash
# 进入项目目录
cd axum_demo

# 编译并运行
cargo run
```

### 3. 访问测试页面

打开浏览器访问：http://127.0.0.1:3000

## 📚 项目结构

```
axum_demo/
├── Cargo.toml          # 项目配置和依赖
├── README.md           # 项目说明文档
├── src/
│   └── main.rs         # 主要代码文件
└── static/
    └── index.html      # 测试页面
```

## 🔌 API 接口

### 健康检查
- **GET** `/api/health`
- 返回服务器状态信息

**示例响应：**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### 用户管理

#### 获取用户列表
- **GET** `/api/users`
- 查询参数：`limit`（数量限制），`offset`（偏移量）

**示例：**
```
GET /api/users?limit=5&offset=0
```

**响应：**
```json
[
  {
    "id": 0,
    "name": "User 0",
    "email": "user0@example.com"
  },
  ...
]
```

#### 获取特定用户
- **GET** `/api/users/:id`

**示例：**
```
GET /api/users/1
```

**响应：**
```json
{
  "id": 1,
  "name": "User 1",
  "email": "user1@example.com"
}
```

#### 创建用户
- **POST** `/api/users`
- 请求体：JSON 格式

**示例请求：**
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**响应：**
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com"
}
```

## 🔗 WebSocket 功能

### 连接地址
```
ws://127.0.0.1:3000/ws
```

### 支持的命令

| 发送消息 | 服务器响应 | 说明 |
|---------|------------|------|
| `ping` | `pong` | 连接测试 |
| `time` | 当前时间 | 获取服务器时间 |
| `echo:消息` | `Server echoed:消息` | 回声测试 |
| 其他消息 | `Server received:消息` | 通用响应 |

### WebSocket 使用示例

```javascript
// 连接 WebSocket
const socket = new WebSocket('ws://127.0.0.1:3000/ws');

// 监听消息
socket.onmessage = function(event) {
    console.log('收到:', event.data);
};

// 发送消息
socket.send('ping');           // 返回: pong
socket.send('time');           // 返回: Current time: 2024-01-01 12:00:00 UTC
socket.send('echo:Hello');     // 返回: Server echoed:Hello
socket.send('任意消息');        // 返回: Server received:任意消息
```

## 🎯 测试页面使用指南

访问 http://127.0.0.1:3000 可以看到完整的测试界面，分为两个部分：

### WebSocket 测试区域
1. 点击 "Connect" 按钮连接 WebSocket
2. 在消息输入框中输入文本
3. 点击 "Send" 发送消息
4. 使用预设按钮快速测试：
   - "Send Ping" - 测试连接
   - "Get Time" - 获取时间
   - "Echo Test" - 回声测试

### API 测试区域
1. 选择要测试的 API 端点
2. 如果是 POST 请求，会显示数据输入框
3. 点击 "Call API" 发送请求
4. 查看响应结果

## 📖 代码学习指南

### 关键概念解释

#### 1. 异步编程 (async/await)
```rust
async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
    // async 函数可以在等待操作时不阻塞线程
    // await 用于等待异步操作完成
}
```

#### 2. 路由系统
```rust
Router::new()
    .route("/api/users", get(list_users).post(create_user))  // 同一路径支持多种方法
    .route("/api/users/:id", get(get_user))                  // 路径参数 :id
```

#### 3. 数据提取 (Extractors)
```rust
async fn get_user(Path(user_id): Path<u32>) -> ... {        // 从路径提取参数
async fn list_users(Query(params): Query<UserQuery>) -> ... {  // 从查询字符串提取参数
async fn create_user(Json(payload): Json<CreateUser>) -> ... {  // 从请求体提取 JSON
```

#### 4. JSON 序列化
```rust
#[derive(Serialize, Deserialize)]  // 自动生成序列化代码
struct User {
    id: u32,
    name: String,
    email: String,
}
```

#### 5. 中间件 (Middleware)
```rust
.layer(
    ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())  // 请求日志
        .layer(CorsLayer::permissive()),    // 跨域支持
)
```

## 🔧 常见问题

### Q: 编译时出现 "feature 'ws' not enabled" 错误
A: 确保 Cargo.toml 中 axum 依赖包含 WebSocket 功能：
```toml
axum = { version = "0.7", features = ["ws"] }
```

### Q: 服务器启动时显示 "Address already in use"
A: 端口 3000 被占用，可以：
1. 停止占用端口的进程：`pkill -f axum_demo`
2. 或修改代码中的端口号

### Q: 网页无法连接到 WebSocket
A: 检查防火墙设置，确保端口 3000 可访问

### Q: API 请求返回 CORS 错误
A: 项目已配置 CORS 支持，如果仍有问题，检查请求的 Origin 头

## 🎓 学习建议

1. **理解基础概念**：先学习 Rust 基础语法和所有权系统
2. **异步编程**：了解 async/await 和 Future 概念
3. **Web 开发**：学习 HTTP 协议、REST API 设计
4. **实践练习**：
   - 尝试添加新的 API 端点
   - 修改 WebSocket 处理逻辑
   - 添加数据持久化（数据库）

## 📝 扩展建议

可以在此基础上添加的功能：
- 数据库集成（SQLx + PostgreSQL）
- 用户认证和授权
- 输入验证
- 错误处理改进
- 单元测试
- API 文档生成
- 部署配置

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个演示项目！

## 📄 许可证

MIT License