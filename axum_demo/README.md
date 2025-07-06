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

## 🏗️ 高级 Axum 教程

### 1. 高级路由模式

#### 嵌套路由 (Nested Routes)
```rust
use axum::{Router, routing::get};

// 创建 API 子路由
let api_routes = Router::new()
    .route("/users", get(list_users))
    .route("/users/:id", get(get_user))
    .route("/posts", get(list_posts))
    .route("/posts/:id", get(get_post));

// 创建管理员子路由
let admin_routes = Router::new()
    .route("/dashboard", get(admin_dashboard))
    .route("/users", get(admin_users))
    .layer(RequireAuthLayer);  // 只对管理员路由添加认证

// 主路由器
let app = Router::new()
    .nest("/api", api_routes)      // 所有 /api/* 路由
    .nest("/admin", admin_routes)  // 所有 /admin/* 路由
    .route("/", get(home_page));
```

#### 路由参数和查询参数
```rust
use axum::extract::{Path, Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserParams {
    id: u32,
    category: Option<String>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
    sort: Option<String>,
}

// 多个路径参数
async fn get_user_post(Path(params): Path<UserParams>) -> String {
    format!("User {} in category {:?}", params.id, params.category)
}

// 复杂查询参数
async fn search_posts(Query(query): Query<SearchQuery>) -> String {
    format!("Search: {:?}, Page: {}, Per page: {}, Sort: {:?}", 
        query.q, 
        query.page.unwrap_or(1), 
        query.per_page.unwrap_or(10),
        query.sort
    )
}

// 路由配置
Router::new()
    .route("/users/:id/posts/:category", get(get_user_post))
    .route("/search", get(search_posts));
```

#### 路由组和中间件
```rust
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, limit::RequestBodyLimitLayer};
use std::time::Duration;

// 为不同路由组应用不同中间件
let app = Router::new()
    // 公共 API - 添加速率限制
    .route("/api/public/*path", get(public_handler))
    .layer(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB
    )
    // 私有 API - 添加认证和更高限制
    .route("/api/private/*path", get(private_handler))
    .layer(
        ServiceBuilder::new()
            .layer(AuthLayer)
            .layer(TimeoutLayer::new(Duration::from_secs(60)))
            .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB
    );
```

### 2. 状态管理和依赖注入

#### 应用状态 (Application State)
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;

// 应用状态结构
#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
    cache: Arc<RwLock<HashMap<String, String>>>,
    config: AppConfig,
}

#[derive(Clone)]
struct AppConfig {
    jwt_secret: String,
    upload_dir: String,
}

// 在处理函数中使用状态
async fn get_user_with_state(
    State(state): State<AppState>,
    Path(user_id): Path<u32>
) -> Result<Json<User>, StatusCode> {
    // 从数据库获取用户
    let user = state.db.get_user(user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 更新缓存
    let cache_key = format!("user:{}", user_id);
    state.cache.write().await.insert(cache_key, serde_json::to_string(&user).unwrap());
    
    Ok(Json(user))
}

// 创建应用
let state = AppState {
    db: Arc::new(Database::new().await),
    cache: Arc::new(RwLock::new(HashMap::new())),
    config: AppConfig {
        jwt_secret: "your-secret-key".to_string(),
        upload_dir: "./uploads".to_string(),
    },
};

let app = Router::new()
    .route("/users/:id", get(get_user_with_state))
    .with_state(state);
```

#### 请求扩展 (Request Extensions)
```rust
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::Response;

// 用户信息结构
#[derive(Clone)]
struct CurrentUser {
    id: u32,
    username: String,
    roles: Vec<String>,
}

// 认证中间件
async fn auth_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 从 JWT token 解析用户信息
    let token = extract_token(&request)?;
    let user = validate_token(token)?;
    
    // 将用户信息添加到请求扩展中
    request.extensions_mut().insert(user);
    
    Ok(next.run(request).await)
}

// 在处理函数中使用扩展
async fn protected_route(
    Extension(user): Extension<CurrentUser>
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": format!("Hello, {}!", user.username),
        "user_id": user.id,
        "roles": user.roles
    }))
}
```

### 3. 自定义提取器 (Custom Extractors)

#### 创建自定义提取器
```rust
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::async_trait;

// 自定义认证提取器
struct AuthUser {
    id: u32,
    username: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for AuthUser
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头获取认证信息
        let auth_header = req.headers()
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        let token = auth_header.to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        // 验证 token 并返回用户信息
        validate_jwt_token(token)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

// 使用自定义提取器
async fn protected_handler(user: AuthUser) -> String {
    format!("Hello, {}! Your ID is {}", user.username, user.id)
}
```

#### JSON 验证提取器
```rust
use validator::{Validate, ValidationError};
use serde::Deserialize;

#[derive(Deserialize, Validate)]
struct CreateUserRequest {
    #[validate(length(min = 2, max = 50))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

// 自定义验证提取器
struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await
            .map_err(|err| (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid JSON",
                "details": err.to_string()
            }))))?;
        
        value.validate()
            .map_err(|err| (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Validation failed",
                "details": err
            }))))?;
        
        Ok(ValidatedJson(value))
    }
}

// 使用验证提取器
async fn create_user(ValidatedJson(user): ValidatedJson<CreateUserRequest>) -> String {
    format!("Creating user: {}", user.username)
}
```

### 4. 错误处理和响应

#### 自定义错误类型
```rust
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
enum AppError {
    Database(sqlx::Error),
    NotFound,
    Unauthorized,
    ValidationError(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            },
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

// 在处理函数中使用
async fn get_user_with_error(Path(user_id): Path<u32>) -> Result<Json<User>, AppError> {
    let user = database::get_user(user_id).await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(user))
}
```

#### 全局错误处理
```rust
use axum::middleware;

async fn handle_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, "Request timeout");
    }

    if err.is::<tower::load_shed::error::Overloaded>() {
        return (StatusCode::SERVICE_UNAVAILABLE, "Service overloaded");
    }

    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}

let app = Router::new()
    .route("/users/:id", get(get_user))
    .layer(middleware::from_fn(handle_error));
```

### 5. 文件上传和处理

#### 单文件上传
```rust
use axum::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        
        if name == "file" {
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            // 保存文件
            let file_path = format!("uploads/{}", filename);
            let mut file = File::create(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            file.write_all(&data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            return Ok(format!("File uploaded: {}", file_path));
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}
```

#### 多文件上传
```rust
async fn upload_multiple_files(mut multipart: Multipart) -> Result<Json<Vec<String>>, StatusCode> {
    let mut uploaded_files = Vec::new();
    
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            // 验证文件类型和大小
            if data.len() > 10 * 1024 * 1024 { // 10MB limit
                return Err(StatusCode::PAYLOAD_TOO_LARGE);
            }
            
            let file_path = format!("uploads/{}", filename);
            let mut file = File::create(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            file.write_all(&data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            uploaded_files.push(file_path);
        }
    }
    
    Ok(Json(uploaded_files))
}
```

### 6. 流式响应和 SSE

#### 服务器发送事件 (Server-Sent Events)
```rust
use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::time::Duration;
use tokio::time;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let data = serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "message": "Hello from server",
            "counter": rand::random::<u32>()
        });
        
        Ok(Event::default().data(data.to_string()))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
```

#### 流式文件下载
```rust
use axum::body::StreamBody;
use tokio_util::io::ReaderStream;

async fn download_file(Path(filename): Path<String>) -> Result<Response, StatusCode> {
    let file_path = format!("uploads/{}", filename);
    let file = File::open(&file_path).await.map_err(|_| StatusCode::NOT_FOUND)?;
    
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    
    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream"),
        (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
    ];
    
    Ok((headers, body).into_response())
}
```

### 7. 中间件开发

#### 请求日志中间件
```rust
use axum::middleware::{self, Next};
use axum::http::Request;
use std::time::Instant;

async fn request_logger<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    tracing::info!(
        "Request {} {} - {} - {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    Ok(response)
}

// 使用中间件
let app = Router::new()
    .route("/users", get(list_users))
    .layer(middleware::from_fn(request_logger));
```

#### 速率限制中间件
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    async fn check_rate_limit(&self, key: &str) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        let user_requests = requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        // 移除过期的请求
        user_requests.retain(|&time| now.duration_since(time) < self.window);
        
        if user_requests.len() >= self.max_requests {
            false
        } else {
            user_requests.push(now);
            true
        }
    }
}

async fn rate_limit_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60)); // 100 requests per minute
    
    // 使用 IP 地址作为限制键
    let client_ip = request.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    if !rate_limiter.check_rate_limit(client_ip).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}
```

### 8. 测试

#### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_user() {
        let app = Router::new()
            .route("/users/:id", get(get_user));
        
        let server = TestServer::new(app).unwrap();
        
        let response = server.get("/users/1").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let user: User = response.json();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "User 1");
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = Router::new()
            .route("/users", post(create_user));
        
        let server = TestServer::new(app).unwrap();
        
        let new_user = json!({
            "name": "Test User",
            "email": "test@example.com"
        });
        
        let response = server.post("/users").json(&new_user).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let created_user: User = response.json();
        assert_eq!(created_user.name, "Test User");
    }
}
```

#### 集成测试
```rust
use axum_test::TestServer;
use std::collections::HashMap;

#[tokio::test]
async fn test_user_workflow() {
    let app = create_app().await;
    let server = TestServer::new(app).unwrap();
    
    // 1. 创建用户
    let new_user = json!({
        "name": "Integration Test User",
        "email": "integration@example.com"
    });
    
    let response = server.post("/api/users").json(&new_user).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // 2. 获取用户列表
    let response = server.get("/api/users").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let users: Vec<User> = response.json();
    assert!(!users.is_empty());
    
    // 3. 获取特定用户
    let response = server.get("/api/users/1").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}
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

### Q: 如何处理大文件上传？
A: 使用流式处理和分块上传：
```rust
use axum::extract::BodyStream;
use futures::StreamExt;

async fn upload_large_file(mut stream: BodyStream) -> Result<String, StatusCode> {
    let mut file = File::create("large_file.bin").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| StatusCode::BAD_REQUEST)?;
        file.write_all(&chunk).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok("Large file uploaded successfully".to_string())
}
```

### Q: 如何实现 API 版本控制？
A: 使用路径前缀或请求头：
```rust
// 路径版本控制
let v1_routes = Router::new()
    .route("/users", get(v1_list_users));

let v2_routes = Router::new()
    .route("/users", get(v2_list_users));

let app = Router::new()
    .nest("/api/v1", v1_routes)
    .nest("/api/v2", v2_routes);
```

### 9. 数据库集成

#### SQLx 集成示例
```rust
// Cargo.toml 添加依赖
// sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }

use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

// 数据库连接池状态
#[derive(Clone)]
struct DatabaseState {
    pool: PgPool,
}

// 创建用户
async fn create_user_db(
    State(state): State<DatabaseState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = Uuid::new_v4();
    
    let result = sqlx::query!(
        "INSERT INTO users (id, username, email, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        user_id,
        payload.username,
        payload.email,
        chrono::Utc::now()
    )
    .fetch_one(&state.pool)
    .await;
    
    match result {
        Ok(row) => {
            let user = User {
                id: row.id,
                username: row.username,
                email: row.email,
                created_at: row.created_at,
            };
            Ok(Json(user))
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create user",
                    "details": e.to_string()
                }))
            ))
        }
    }
}

// 获取用户
async fn get_user_db(
    State(state): State<DatabaseState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, email, created_at FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&state.pool)
    .await;
    
    match result {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 初始化数据库
async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/axum_demo".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // 运行迁移
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
```

#### 连接池和事务管理
```rust
use sqlx::{Postgres, Transaction};

// 事务处理示例
async fn transfer_funds(
    State(state): State<DatabaseState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut tx: Transaction<'_, Postgres> = state.pool.begin().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 扣除发送方余额
    let sender_result = sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2 AND balance >= $1",
        payload.amount,
        payload.from_account
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if sender_result.rows_affected() == 0 {
        tx.rollback().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // 增加接收方余额
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        payload.amount,
        payload.to_account
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // 提交事务
    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "message": "Transfer successful",
        "amount": payload.amount
    })))
}
```

### 10. 身份验证和授权

#### JWT 身份验证
```rust
// Cargo.toml 添加依赖
// jsonwebtoken = "9.0"
// bcrypt = "0.15"

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,    // 用户ID
    username: String,
    roles: Vec<String>,
    exp: usize,     // 过期时间
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    user: UserInfo,
}

#[derive(Debug, Serialize)]
struct UserInfo {
    id: String,
    username: String,
    roles: Vec<String>,
}

// 登录处理
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
    // 从数据库获取用户
    let user = sqlx::query!(
        "SELECT id, username, password_hash, roles FROM users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
        "error": "Database error"
    }))))?;
    
    let user = user.ok_or((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
        "error": "Invalid credentials"
    }))))?;
    
    // 验证密码
    let password_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "error": "Password verification failed"
        }))))?;
    
    if !password_valid {
        return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid credentials"
        }))));
    }
    
    // 生成 JWT token
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        roles: user.roles.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
        "error": "Token generation failed"
    }))))?;
    
    Ok(Json(LoginResponse {
        token,
        user: UserInfo {
            id: user.id.to_string(),
            username: user.username,
            roles: user.roles,
        },
    }))
}

// 用户注册
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // 检查用户名是否已存在
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 OR email = $2",
        payload.username,
        payload.email
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
        "error": "Database error"
    }))))?;
    
    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, Json(serde_json::json!({
            "error": "Username or email already exists"
        }))));
    }
    
    // 密码哈希
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "error": "Password hashing failed"
        }))))?;
    
    // 创建用户
    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash, roles, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        user_id,
        payload.username,
        payload.email,
        password_hash,
        &vec!["user".to_string()],
        chrono::Utc::now()
    )
    .execute(&state.db_pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
        "error": "Failed to create user"
    }))))?;
    
    Ok(Json(serde_json::json!({
        "message": "User created successfully",
        "user_id": user_id
    })))
}
```

#### 权限检查中间件
```rust
// 角色权限检查
#[derive(Clone)]
struct RequireRole {
    required_roles: Vec<String>,
}

impl RequireRole {
    fn new(roles: Vec<&str>) -> Self {
        Self {
            required_roles: roles.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

async fn require_role_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 从请求扩展中获取用户信息
    let user = request.extensions().get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // 检查用户角色
    let required_roles: &RequireRole = request.extensions().get()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let has_required_role = required_roles.required_roles.iter()
        .any(|role| user.roles.contains(role));
    
    if !has_required_role {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}

// 使用示例
let admin_routes = Router::new()
    .route("/dashboard", get(admin_dashboard))
    .route("/users", get(admin_list_users))
    .layer(middleware::from_fn(jwt_auth_middleware))
    .layer(middleware::from_fn(require_role_middleware))
    .layer(Extension(RequireRole::new(vec!["admin"])));
```

### 11. 性能优化

#### 连接池配置
```rust
use sqlx::postgres::PgPoolOptions;

async fn create_optimized_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)                    // 最大连接数
        .min_connections(5)                     // 最小连接数
        .acquire_timeout(Duration::from_secs(30)) // 获取连接超时
        .idle_timeout(Duration::from_secs(600))   // 空闲连接超时
        .max_lifetime(Duration::from_secs(1800))  // 连接最大生存时间
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}
```

#### 缓存中间件
```rust
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Clone)]
struct CacheMiddleware {
    cache: Arc<RwLock<HashMap<String, (String, Instant)>>>,
    ttl: Duration,
}

impl CacheMiddleware {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }
    
    async fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        if let Some((value, timestamp)) = cache.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }
    
    async fn set(&self, key: String, value: String) {
        let mut cache = self.cache.write().await;
        cache.insert(key, (value, Instant::now()));
    }
}

async fn cache_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let cache = request.extensions().get::<CacheMiddleware>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let cache_key = format!("{}:{}", request.method(), request.uri());
    
    // 尝试从缓存获取
    if let Some(cached_response) = cache.get(&cache_key).await {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("X-Cache", "HIT")
            .body(cached_response.into())
            .unwrap());
    }
    
    // 执行请求
    let response = next.run(request).await;
    
    // 缓存响应（仅对成功的 GET 请求）
    if response.status().is_success() {
        // 这里需要克隆响应体，实际实现会更复杂
        let response_body = "..."; // 从响应中提取身体
        cache.set(cache_key, response_body.to_string()).await;
    }
    
    Ok(response)
}
```

### 12. 部署和生产配置

#### Docker 配置
```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye-slim

# 安装 SSL 证书
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/axum_demo .
COPY static ./static

EXPOSE 3000
CMD ["./axum_demo"]
```

#### 环境配置
```rust
use std::env;

#[derive(Clone)]
struct Config {
    server_port: u16,
    database_url: String,
    jwt_secret: String,
    redis_url: Option<String>,
    log_level: String,
}

impl Config {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET must be set")?,
            redis_url: env::var("REDIS_URL").ok(),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}
```

#### 健康检查和监控
```rust
use axum::extract::State;
use serde_json::json;

#[derive(Clone)]
struct HealthChecker {
    db_pool: PgPool,
    start_time: Instant,
}

async fn health_check(State(checker): State<HealthChecker>) -> Json<serde_json::Value> {
    let mut status = "healthy";
    let mut checks = serde_json::Map::new();
    
    // 检查数据库连接
    match sqlx::query("SELECT 1").fetch_one(&checker.db_pool).await {
        Ok(_) => {
            checks.insert("database".to_string(), json!("healthy"));
        }
        Err(_) => {
            checks.insert("database".to_string(), json!("unhealthy"));
            status = "unhealthy";
        }
    }
    
    // 检查运行时间
    let uptime = checker.start_time.elapsed();
    checks.insert("uptime_seconds".to_string(), json!(uptime.as_secs()));
    
    Json(json!({
        "status": status,
        "checks": checks,
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// 指标收集
async fn metrics() -> String {
    // 返回 Prometheus 格式的指标
    format!(
        "# HELP http_requests_total Total HTTP requests\n\
         # TYPE http_requests_total counter\n\
         http_requests_total{{method=\"GET\",status=\"200\"}} 1027\n\
         http_requests_total{{method=\"POST\",status=\"200\"}} 3\n"
    )
}
```

## 🎓 学习建议

1. **理解基础概念**：先学习 Rust 基础语法和所有权系统
2. **异步编程**：了解 async/await 和 Future 概念
3. **Web 开发**：学习 HTTP 协议、REST API 设计
4. **数据库操作**：掌握 SQL 查询和事务处理
5. **安全实践**：学习身份验证、授权和数据验证
6. **实践练习**：
   - 尝试添加新的 API 端点
   - 修改 WebSocket 处理逻辑
   - 添加数据持久化（数据库）
   - 实现用户认证系统
   - 添加缓存层
   - 编写单元测试和集成测试

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