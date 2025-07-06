// Axum WebSocket 和 API 演示项目
// 这是一个使用 Rust Axum 框架创建的 Web 服务器，支持 REST API 和 WebSocket

// 导入需要的库和模块
use axum::{
    // extract 模块用于从 HTTP 请求中提取数据
    extract::{Path, Query, WebSocketUpgrade, ws::{WebSocket, Message}},
    http::StatusCode,  // HTTP 状态码
    response::{Json, Response},  // 响应类型
    routing::get,  // 路由方法
    Router,  // 路由器
};
use serde::{Deserialize, Serialize};  // 序列化和反序列化库，用于 JSON 处理
use std::collections::HashMap;  // 哈希映射数据结构
use tower::ServiceBuilder;  // 中间件构建器
use tower_http::{cors::CorsLayer, trace::TraceLayer, services::ServeDir};  // HTTP 中间件
use tracing_subscriber;  // 日志系统

// ===== 数据结构定义 =====

// 用户数据结构
// Serialize: 可以转换为 JSON
// Deserialize: 可以从 JSON 转换回来
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,        // 用户 ID（无符号32位整数）
    name: String,   // 用户名
    email: String,  // 邮箱
}

// 创建用户时的输入数据结构
// 只需要 name 和 email，id 会自动生成
#[derive(Deserialize)]
struct CreateUser {
    name: String,   // 用户名
    email: String,  // 邮箱
}

// 查询用户列表时的参数结构
#[derive(Deserialize)]
struct UserQuery {
    limit: Option<u32>,   // 限制返回数量（可选）
    offset: Option<u32>,  // 偏移量，用于分页（可选）
}

// ===== API 处理函数 =====

// 获取单个用户的处理函数
// Path(user_id): 从 URL 路径中提取用户 ID
// -> Result<Json<User>, StatusCode>: 返回用户 JSON 或错误状态码
async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
    // 创建一个模拟的用户对象
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),  // 格式化字符串，生成 "User 1", "User 2" 等
        email: format!("user{}@example.com", user_id),  // 生成模拟邮箱
    };
    Ok(Json(user))  // 返回 JSON 格式的用户数据
}

// 获取用户列表的处理函数
// Query(params): 从 URL 查询参数中提取分页信息
async fn list_users(Query(params): Query<UserQuery>) -> Json<Vec<User>> {
    // 获取分页参数，如果没有提供则使用默认值
    let limit = params.limit.unwrap_or(10);   // 默认返回 10 个用户
    let offset = params.offset.unwrap_or(0);  // 默认从第 0 个开始

    // 生成指定范围的用户列表
    let users: Vec<User> = (offset..offset + limit)
        .map(|i| User {  // map: 将每个数字转换为 User 对象
            id: i,
            name: format!("User {}", i),
            email: format!("user{}@example.com", i),
        })
        .collect();  // collect: 将迭代器收集为 Vec

    Json(users)  // 返回用户列表的 JSON
}

// 创建用户的处理函数
// Json(payload): 从请求体中提取 JSON 数据
async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    // 创建新用户对象
    let user = User {
        id: 1,  // 简化演示，固定使用 ID 1
        name: payload.name,   // 使用输入的用户名
        email: payload.email, // 使用输入的邮箱
    };
    Ok(Json(user))  // 返回创建的用户
}

// 健康检查处理函数
// 返回服务器状态信息
async fn health_check() -> Json<HashMap<&'static str, &'static str>> {
    let mut response = HashMap::new();
    response.insert("status", "healthy");   // 服务器状态
    response.insert("version", "0.1.0");    // 版本信息
    Json(response)
}

// ===== WebSocket 处理函数 =====

// WebSocket 升级处理函数
// 当客户端请求升级到 WebSocket 时调用
async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    // 升级连接并指定处理函数
    ws.on_upgrade(handle_socket)
}

// WebSocket 连接处理函数
// 处理 WebSocket 消息的主要逻辑
async fn handle_socket(mut socket: WebSocket) {
    println!("WebSocket connection established");  // 打印连接建立信息

    // 持续监听来自客户端的消息
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {  // 如果消息接收成功
            match msg {  // 根据消息类型进行处理
                Message::Text(text) => {  // 文本消息
                    println!("Received: {}", text);  // 打印接收到的消息

                    // 根据消息内容生成不同的响应
                    let response = if text.starts_with("echo:") {
                        // 如果消息以 "echo:" 开头，返回回声
                        text.replacen("echo:", "Server echoed:", 1)
                    } else if text == "ping" {
                        // 如果是 "ping"，返回 "pong"
                        "pong".to_string()
                    } else if text == "time" {
                        // 如果是 "time"，返回当前时间
                        format!("Current time: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
                    } else {
                        // 其他消息，返回通用响应
                        format!("Server received: {}", text)
                    };

                    // 发送响应给客户端
                    if let Err(e) = socket.send(Message::Text(response)).await {
                        println!("Error sending message: {}", e);
                        break;  // 如果发送失败，退出循环
                    }
                }
                Message::Close(_) => {  // 关闭消息
                    println!("WebSocket connection closed");
                    break;  // 退出循环，结束连接处理
                }
                _ => {}  // 忽略其他类型的消息
            }
        } else {
            break;  // 如果接收消息失败，退出循环
        }
    }
}

// ===== 主函数 =====

// 使用 tokio 异步运行时
// tokio::main: 告诉 Rust 这是一个异步主函数
#[tokio::main]
async fn main() {
    // 初始化日志系统，用于调试和监控
    tracing_subscriber::fmt::init();

    // 创建路由器并配置所有路由
    let app = Router::new()
        // WebSocket 路由：GET /ws -> websocket_handler
        .route("/ws", get(websocket_handler))
        
        // API 路由：
        .route("/api/health", get(health_check))                    // 健康检查
        .route("/api/users", get(list_users).post(create_user))     // 用户列表（GET）和创建用户（POST）
        .route("/api/users/:id", get(get_user))                     // 获取特定用户（GET）
        
        // 静态文件服务：为根路径 "/" 提供 "static" 目录中的文件
        .nest_service("/", ServeDir::new("static"))
        
        // 添加中间件层
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())  // HTTP 请求追踪中间件
                .layer(CorsLayer::permissive()),    // CORS 跨域支持中间件
        );

    // 创建 TCP 监听器，绑定到所有网络接口的 3000 端口
    // 0.0.0.0:3000 意味着可以从任何 IP 地址访问
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    println!("Visit http://127.0.0.1:3000 to access the web interface");

    // 启动服务器，开始处理请求
    axum::serve(listener, app).await.unwrap();
}

/* 
=== 使用说明 ===

1. 启动服务器：cargo run
2. 访问 http://127.0.0.1:3000 查看测试页面
3. 可用的 API 端点：
   - GET /api/health - 检查服务器状态
   - GET /api/users - 获取用户列表
   - GET /api/users/:id - 获取特定用户
   - POST /api/users - 创建新用户
   - GET /ws - WebSocket 连接

4. WebSocket 命令：
   - "ping" -> 返回 "pong"
   - "time" -> 返回当前时间
   - "echo:消息" -> 返回 "Server echoed:消息"
   - 其他消息 -> 返回 "Server received:消息"
*/