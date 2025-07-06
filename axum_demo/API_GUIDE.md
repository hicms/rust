# API 使用指南

这是 Axum Demo 项目的详细 API 文档，适合初学者学习使用。

## 📖 基础概念

### 什么是 REST API？
REST (Representational State Transfer) 是一种 Web API 设计风格，使用 HTTP 方法来操作资源：
- **GET**: 获取数据（读取）
- **POST**: 创建新数据
- **PUT**: 更新现有数据
- **DELETE**: 删除数据

### 什么是 WebSocket？
WebSocket 是一种网络通信协议，允许客户端和服务器之间进行实时双向通信，常用于：
- 实时聊天
- 游戏
- 股票价格更新
- 协作编辑

## 🔌 REST API 端点详解

### 基础 URL
```
http://127.0.0.1:3000
```

---

## 1. 健康检查 API

检查服务器是否正常运行。

### 请求
```http
GET /api/health
```

### 响应
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### 使用场景
- 监控系统检查服务器状态
- 负载均衡器健康检查
- 部署后验证服务是否正常

### 示例代码

**JavaScript (fetch)**
```javascript
fetch('http://127.0.0.1:3000/api/health')
  .then(response => response.json())
  .then(data => console.log(data));
```

**curl 命令**
```bash
curl http://127.0.0.1:3000/api/health
```

---

## 2. 用户管理 API

### 2.1 获取用户列表

获取所有用户的列表，支持分页。

#### 请求
```http
GET /api/users
GET /api/users?limit=5&offset=0
```

#### 查询参数
| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| limit | 数字 | 否 | 10 | 返回的用户数量限制 |
| offset | 数字 | 否 | 0 | 跳过的用户数量（分页偏移） |

#### 响应
```json
[
  {
    "id": 0,
    "name": "User 0",
    "email": "user0@example.com"
  },
  {
    "id": 1,
    "name": "User 1",
    "email": "user1@example.com"
  }
]
```

#### 分页示例
```javascript
// 获取第一页（前10个用户）
fetch('http://127.0.0.1:3000/api/users?limit=10&offset=0')

// 获取第二页（第11-20个用户）
fetch('http://127.0.0.1:3000/api/users?limit=10&offset=10')

// 获取前5个用户
fetch('http://127.0.0.1:3000/api/users?limit=5')
```

---

### 2.2 获取特定用户

根据用户 ID 获取单个用户信息。

#### 请求
```http
GET /api/users/{id}
```

#### 路径参数
| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| id | 数字 | 是 | 用户的唯一标识符 |

#### 响应
```json
{
  "id": 1,
  "name": "User 1",
  "email": "user1@example.com"
}
```

#### 示例
```javascript
// 获取 ID 为 1 的用户
fetch('http://127.0.0.1:3000/api/users/1')
  .then(response => response.json())
  .then(user => console.log(user));
```

```bash
# curl 示例
curl http://127.0.0.1:3000/api/users/1
```

---

### 2.3 创建新用户

创建一个新的用户。

#### 请求
```http
POST /api/users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com"
}
```

#### 请求体参数
| 字段 | 类型 | 必需 | 说明 |
|------|------|------|------|
| name | 字符串 | 是 | 用户姓名 |
| email | 字符串 | 是 | 用户邮箱 |

#### 响应
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com"
}
```

#### 示例代码

**JavaScript**
```javascript
const userData = {
  name: "Jane Smith",
  email: "jane@example.com"
};

fetch('http://127.0.0.1:3000/api/users', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(userData)
})
.then(response => response.json())
.then(user => console.log('Created user:', user));
```

**curl 命令**
```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Johnson", "email": "alice@example.com"}'
```

---

## 🔗 WebSocket API

WebSocket 提供实时双向通信功能。

### 连接地址
```
ws://127.0.0.1:3000/ws
```

### 支持的消息格式

所有消息都是文本格式，服务器会根据消息内容返回不同的响应。

#### 1. Ping/Pong 测试
```
发送: "ping"
响应: "pong"
```

#### 2. 时间查询
```
发送: "time"
响应: "Current time: 2024-01-01 12:00:00 UTC"
```

#### 3. 回声测试
```
发送: "echo:Hello World"
响应: "Server echoed:Hello World"
```

#### 4. 通用消息
```
发送: "任何其他消息"
响应: "Server received:任何其他消息"
```

### WebSocket 使用示例

#### JavaScript 客户端
```javascript
// 1. 建立连接
const socket = new WebSocket('ws://127.0.0.1:3000/ws');

// 2. 监听连接打开事件
socket.onopen = function(event) {
    console.log('Connected to WebSocket server');
    
    // 发送测试消息
    socket.send('ping');
};

// 3. 监听消息
socket.onmessage = function(event) {
    console.log('Received:', event.data);
};

// 4. 监听连接关闭
socket.onclose = function(event) {
    console.log('WebSocket connection closed');
};

// 5. 监听错误
socket.onerror = function(error) {
    console.log('WebSocket error:', error);
};

// 6. 发送不同类型的消息
socket.send('time');                    // 获取时间
socket.send('echo:Hello WebSocket');    // 回声测试
socket.send('这是一条测试消息');          // 通用消息
```

#### Node.js 客户端示例
```javascript
const WebSocket = require('ws');

const ws = new WebSocket('ws://127.0.0.1:3000/ws');

ws.on('open', function open() {
    console.log('Connected');
    ws.send('ping');
});

ws.on('message', function message(data) {
    console.log('Received:', data.toString());
});
```

## 🧪 测试工具和方法

### 1. 使用浏览器测试页面
访问 http://127.0.0.1:3000 可以使用内置的测试界面。

### 2. 使用 curl 测试 REST API
```bash
# 健康检查
curl http://127.0.0.1:3000/api/health

# 获取用户列表
curl http://127.0.0.1:3000/api/users

# 获取特定用户
curl http://127.0.0.1:3000/api/users/1

# 创建用户
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Test User", "email": "test@example.com"}'
```

### 3. 使用 Postman 测试
1. 导入以下 API 集合到 Postman
2. 设置 Base URL 为 `http://127.0.0.1:3000`
3. 测试各个端点

### 4. 使用 WebSocket 测试工具
- **在线工具**: websocket.org/echo.html
- **Chrome 扩展**: WebSocket Test Client
- **命令行工具**: wscat

```bash
# 安装 wscat
npm install -g wscat

# 连接到 WebSocket
wscat -c ws://127.0.0.1:3000/ws

# 然后可以直接输入消息进行测试
```

## 📋 常见错误和解决方案

### 1. 连接被拒绝 (Connection Refused)
**错误**: `Failed to connect to 127.0.0.1:3000`
**原因**: 服务器未启动
**解决**: 运行 `cargo run` 启动服务器

### 2. CORS 错误
**错误**: `CORS policy: Cross origin requests are only supported for protocol schemes`
**原因**: 跨域访问被阻止
**解决**: 项目已配置 CORS 支持，确保从正确的域名访问

### 3. WebSocket 连接失败
**错误**: `WebSocket connection failed`
**原因**: 网络问题或服务器问题
**解决**: 
- 检查服务器是否运行
- 确认防火墙设置
- 检查网络连接

### 4. JSON 格式错误
**错误**: `400 Bad Request` 或解析错误
**原因**: 发送的 JSON 格式不正确
**解决**: 验证 JSON 格式，确保所有必需字段都存在

## 🎯 实践练习

### 练习 1: API 调用顺序
1. 先调用健康检查 API
2. 获取用户列表
3. 创建一个新用户
4. 获取刚创建的用户信息

### 练习 2: WebSocket 交互
1. 连接 WebSocket
2. 发送 "ping" 消息
3. 获取当前时间
4. 发送自定义消息
5. 测试回声功能

### 练习 3: 分页测试
1. 获取前5个用户
2. 获取第6-10个用户
3. 比较结果，理解分页原理

### 练习 4: 错误处理
1. 尝试访问不存在的用户 ID
2. 发送格式错误的 JSON
3. 观察错误响应格式

这些练习会帮助你更好地理解 API 的工作原理和最佳实践。