# 雪花算法部署配置指南

本文档详细介绍雪花算法在各种环境下的部署配置方法、最佳实践和注意事项。

## 🎯 部署架构概览

### 分布式架构原理

```
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│   数据中心 0    │   │   数据中心 1    │   │   数据中心 2    │
│  (东部机房)     │   │  (西部机房)     │   │  (北部机房)     │
├─────────────────┤   ├─────────────────┤   ├─────────────────┤
│ Worker ID: 0-63 │   │ Worker ID:64-127│   │Worker ID:128-191│
│                 │   │                 │   │                 │
│ 🖥️ 机器1 (ID:0) │   │ 🖥️ 机器1(ID:64) │   │ 🖥️ 机器1(ID:128)│
│ 🖥️ 机器2 (ID:1) │   │ 🖥️ 机器2(ID:65) │   │ 🖥️ 机器2(ID:129)│
│ 🖥️ 机器3 (ID:2) │   │ 🖥️ 机器3(ID:66) │   │ 🖥️ 机器3(ID:130)│
│     ...         │   │     ...         │   │     ...         │
│ 🖥️ 机器64(ID:63)│   │ 🖥️ 机器64(ID:127│   │ 🖥️ 机器64(ID:191)│
└─────────────────┘   └─────────────────┘   └─────────────────┘
```

### Worker ID分配策略

| 策略 | 优先级 | 适用场景 | 配置方式 |
|------|--------|----------|----------|
| 环境变量 | 🥇 最高 | 容器化部署、明确指定 | `SNOWFLAKE_WORKER_ID=10` |
| 配置文件 | 🥈 中等 | 传统部署、数据中心管理 | `datacenter_id + machine_id` |
| IP自动分配 | 🥉 最低 | 开发测试、临时部署 | 自动计算 |

## 📋 配置文件详解

### 基础配置模板

```toml
# snowflake.toml - 生产环境配置模板

# =============================================================================
# 数据中心配置 (推荐用于生产环境)
# =============================================================================

# 数据中心ID (0-3)
# 0: 主数据中心/东部
# 1: 备用数据中心/西部  
# 2: 灾备中心/北部
# 3: 测试环境/南部
datacenter_id = 1

# 机器ID (0-63)
# 建议按照机器在数据中心内的编号分配
machine_id = 5

# =============================================================================
# 算法参数配置
# =============================================================================

# Worker ID位数 (建议保持默认8位)
worker_id_bits = 8

# 序列号位数 (根据并发需求调整)
sequence_bits = 12

# 时钟回拨容忍度 (生产环境建议10-100ms)
max_backward_ms = 10
```

### 高并发配置

```toml
# 高并发环境配置 (每毫秒需要>4096个ID)
datacenter_id = 1
machine_id = 1
worker_id_bits = 6        # 减少到6位 (64个节点)
sequence_bits = 14        # 增加到14位 (每毫秒16384个ID)
max_backward_ms = 50
```

### 多数据中心配置

```toml
# 数据中心A (东部) - 配置文件A
datacenter_id = 0
machine_id = 1
worker_id_bits = 8
sequence_bits = 12
max_backward_ms = 10

# 数据中心B (西部) - 配置文件B  
datacenter_id = 1
machine_id = 1
worker_id_bits = 8
sequence_bits = 12
max_backward_ms = 10

# 数据中心C (北部) - 配置文件C
datacenter_id = 2
machine_id = 1
worker_id_bits = 8
sequence_bits = 12
max_backward_ms = 10
```

## 🐳 容器化部署

### Docker部署

#### Dockerfile

```dockerfile
# 多阶段构建Dockerfile
FROM rust:1.75 as builder

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src/

# 编译发布版本
RUN cargo build --release

# 生产镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false snowflake

# 复制编译后的二进制文件
COPY --from=builder /app/target/release/snowflake /usr/local/bin/
COPY --from=builder /app/target/release/stress_test /usr/local/bin/

# 复制配置文件
COPY snowflake.toml /etc/

# 设置权限
RUN chmod +x /usr/local/bin/snowflake /usr/local/bin/stress_test
RUN chown snowflake:snowflake /usr/local/bin/snowflake /usr/local/bin/stress_test

# 切换到非root用户
USER snowflake

# 暴露端口 (如果有HTTP服务)
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD stress_test --health-check || exit 1

# 启动命令
CMD ["snowflake"]
```

#### Docker Compose

```yaml
version: '3.8'

services:
  snowflake-node1:
    build: .
    environment:
      - SNOWFLAKE_WORKER_ID=1
      - RUST_LOG=info
    volumes:
      - ./snowflake.toml:/etc/snowflake.toml:ro
    ports:
      - "8081:8080"
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "stress_test", "--health-check"]
      interval: 30s
      timeout: 10s
      retries: 3

  snowflake-node2:
    build: .
    environment:
      - SNOWFLAKE_WORKER_ID=2
      - RUST_LOG=info
    volumes:
      - ./snowflake.toml:/etc/snowflake.toml:ro
    ports:
      - "8082:8080"
    restart: unless-stopped

  snowflake-node3:
    build: .
    environment:
      - SNOWFLAKE_WORKER_ID=3
      - RUST_LOG=info
    volumes:
      - ./snowflake.toml:/etc/snowflake.toml:ro
    ports:
      - "8083:8080"
    restart: unless-stopped

  # 负载均衡器
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - snowflake-node1
      - snowflake-node2
      - snowflake-node3
```

#### 启动脚本

```bash
#!/bin/bash
# deploy.sh - Docker部署脚本

set -e

echo "🚀 部署雪花算法服务..."

# 构建镜像
echo "📦 构建Docker镜像..."
docker build -t snowflake:latest .

# 创建网络
echo "🌐 创建Docker网络..."
docker network create snowflake-net 2>/dev/null || true

# 启动服务
echo "🔥 启动服务..."
docker-compose up -d

# 检查服务状态
echo "✅ 检查服务状态..."
sleep 5
docker-compose ps

# 健康检查
echo "🩺 执行健康检查..."
for i in {1..3}; do
    if curl -f http://localhost:808${i}/health 2>/dev/null; then
        echo "✅ 节点${i} 健康"
    else
        echo "❌ 节点${i} 异常"
    fi
done

echo "🎉 部署完成！"
```

### Kubernetes部署

#### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: snowflake-config
  namespace: snowflake
data:
  snowflake.toml: |
    datacenter_id = 1
    machine_id = 1
    worker_id_bits = 8
    sequence_bits = 12
    max_backward_ms = 50
```

#### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: snowflake-service
  namespace: snowflake
  labels:
    app: snowflake
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: snowflake
  template:
    metadata:
      labels:
        app: snowflake
    spec:
      # 反亲和性，确保Pod分布在不同节点
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - snowflake
              topologyKey: kubernetes.io/hostname
      
      # 初始化容器，用于生成唯一Worker ID
      initContainers:
      - name: generate-worker-id
        image: busybox:1.35
        command: 
        - sh
        - -c
        - |
          # 基于Pod名称生成唯一Worker ID
          POD_ORDINAL=$(echo $POD_NAME | sed 's/.*-//')
          WORKER_ID=$((POD_ORDINAL + 1))
          echo "SNOWFLAKE_WORKER_ID=$WORKER_ID" > /shared/worker-id
        env:
        - name: POD_NAME
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        volumeMounts:
        - name: shared-data
          mountPath: /shared

      containers:
      - name: snowflake
        image: snowflake:latest
        ports:
        - containerPort: 8080
          name: http
        
        # 环境变量
        env:
        - name: RUST_LOG
          value: "info"
        
        # 从初始化容器获取Worker ID
        command:
        - sh
        - -c
        - |
          source /shared/worker-id
          export $SNOWFLAKE_WORKER_ID
          exec snowflake
        
        # 资源限制
        resources:
          requests:
            memory: "64Mi"
            cpu: "100m"
          limits:
            memory: "128Mi"
            cpu: "500m"
        
        # 健康检查
        livenessProbe:
          exec:
            command:
            - stress_test
            - --health-check
          initialDelaySeconds: 30
          periodSeconds: 30
          timeoutSeconds: 5
          failureThreshold: 3
        
        readinessProbe:
          exec:
            command:
            - stress_test
            - --health-check
          initialDelaySeconds: 5
          periodSeconds: 10
          timeoutSeconds: 3
          failureThreshold: 3
        
        # 挂载卷
        volumeMounts:
        - name: config-volume
          mountPath: /etc/snowflake.toml
          subPath: snowflake.toml
          readOnly: true
        - name: shared-data
          mountPath: /shared
          readOnly: true
      
      volumes:
      - name: config-volume
        configMap:
          name: snowflake-config
      - name: shared-data
        emptyDir: {}

      # 安全上下文
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000
```

#### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: snowflake-service
  namespace: snowflake
  labels:
    app: snowflake
spec:
  type: ClusterIP
  ports:
  - port: 8080
    targetPort: 8080
    protocol: TCP
    name: http
  selector:
    app: snowflake
```

#### Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: snowflake-ingress
  namespace: snowflake
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/load-balance: "round_robin"
spec:
  rules:
  - host: snowflake.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: snowflake-service
            port:
              number: 8080
```

## 🏢 传统虚拟机部署

### 系统服务配置

#### SystemD服务文件

```ini
# /etc/systemd/system/snowflake.service
[Unit]
Description=Snowflake ID Generator
After=network.target
Wants=network.target

[Service]
Type=simple
User=snowflake
Group=snowflake
Environment=SNOWFLAKE_WORKER_ID=1
Environment=RUST_LOG=info
ExecStart=/usr/local/bin/snowflake
ExecReload=/bin/kill -HUP $MAINPID
KillMode=process
Restart=always
RestartSec=5s
TimeoutStopSec=30s

# 安全设置
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/snowflake

# 资源限制
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

#### 安装脚本

```bash
#!/bin/bash
# install.sh - 系统安装脚本

set -e

# 检查权限
if [[ $EUID -ne 0 ]]; then
   echo "❌ 此脚本需要root权限运行" 
   exit 1
fi

echo "🔧 安装雪花算法服务..."

# 创建用户
echo "👤 创建服务用户..."
useradd -r -s /bin/false snowflake || true

# 创建目录
echo "📁 创建目录..."
mkdir -p /etc/snowflake
mkdir -p /var/log/snowflake
mkdir -p /usr/local/bin

# 设置权限
chown snowflake:snowflake /var/log/snowflake

# 复制文件
echo "📋 复制文件..."
cp target/release/snowflake /usr/local/bin/
cp target/release/stress_test /usr/local/bin/
cp snowflake.toml /etc/
cp snowflake.service /etc/systemd/system/

# 设置权限
chmod +x /usr/local/bin/snowflake
chmod +x /usr/local/bin/stress_test
chmod 644 /etc/snowflake.toml
chmod 644 /etc/systemd/system/snowflake.service

# 重载systemd
echo "🔄 重载systemd..."
systemctl daemon-reload

# 启用并启动服务
echo "🚀 启动服务..."
systemctl enable snowflake
systemctl start snowflake

# 检查状态
echo "✅ 检查服务状态..."
systemctl status snowflake

echo "🎉 安装完成！"
echo "📊 查看日志: journalctl -u snowflake -f"
echo "🔧 管理服务: systemctl {start|stop|restart|status} snowflake"
```

### 多机部署脚本

```bash
#!/bin/bash
# deploy-cluster.sh - 集群部署脚本

MACHINES=(
    "192.168.1.10:1"   # 机器1，Worker ID 1
    "192.168.1.11:2"   # 机器2，Worker ID 2
    "192.168.1.12:3"   # 机器3，Worker ID 3
)

echo "🚀 部署雪花算法集群..."

for machine_config in "${MACHINES[@]}"; do
    IFS=':' read -r ip worker_id <<< "$machine_config"
    
    echo "📡 部署到机器 $ip (Worker ID: $worker_id)..."
    
    # 复制文件
    scp -r target/release/* root@$ip:/tmp/
    scp snowflake.toml root@$ip:/tmp/
    scp install.sh root@$ip:/tmp/
    
    # 远程安装
    ssh root@$ip << EOF
        cd /tmp
        chmod +x install.sh
        ./install.sh
        
        # 设置Worker ID
        echo "SNOWFLAKE_WORKER_ID=$worker_id" >> /etc/systemd/system/snowflake.service.d/override.conf
        
        systemctl daemon-reload
        systemctl restart snowflake
        systemctl status snowflake
EOF
    
    echo "✅ 机器 $ip 部署完成"
done

echo "🎉 集群部署完成！"

# 验证集群
echo "🩺 验证集群状态..."
for machine_config in "${MACHINES[@]}"; do
    IFS=':' read -r ip worker_id <<< "$machine_config"
    echo -n "机器 $ip: "
    if ssh root@$ip "systemctl is-active snowflake" >/dev/null 2>&1; then
        echo "✅ 运行中"
    else
        echo "❌ 异常"
    fi
done
```

## ⚠️ 部署注意事项

### Worker ID冲突预防

1. **环境变量方式** (推荐)
   ```bash
   # 为每台机器设置唯一ID
   export SNOWFLAKE_WORKER_ID=1  # 机器1
   export SNOWFLAKE_WORKER_ID=2  # 机器2
   export SNOWFLAKE_WORKER_ID=3  # 机器3
   ```

2. **配置文件方式**
   ```toml
   # 机器1配置
   datacenter_id = 0
   machine_id = 1
   
   # 机器2配置  
   datacenter_id = 0
   machine_id = 2
   ```

3. **自动化分配脚本**
   ```bash
   #!/bin/bash
   # auto-assign-worker-id.sh
   
   # 基于IP地址自动分配
   IP=$(hostname -I | awk '{print $1}')
   LAST_OCTET=$(echo $IP | cut -d. -f4)
   WORKER_ID=$((LAST_OCTET % 256))
   
   export SNOWFLAKE_WORKER_ID=$WORKER_ID
   echo "自动分配Worker ID: $WORKER_ID"
   ```

### 时钟同步

1. **NTP配置**
   ```bash
   # 安装NTP
   sudo apt-get install ntp
   
   # 配置NTP服务器
   echo "server pool.ntp.org" >> /etc/ntp.conf
   
   # 启动NTP服务
   sudo systemctl enable ntp
   sudo systemctl start ntp
   ```

2. **时钟监控脚本**
   ```bash
   #!/bin/bash
   # check-time-sync.sh
   
   # 检查时钟偏移
   OFFSET=$(ntpq -p | awk 'NR==3 {print $9}')
   THRESHOLD=100  # 100ms阈值
   
   if (( $(echo "$OFFSET > $THRESHOLD" | bc -l) )); then
       echo "⚠️ 时钟偏移过大: ${OFFSET}ms"
       # 发送告警或重启服务
   else
       echo "✅ 时钟同步正常: ${OFFSET}ms"
   fi
   ```

### 监控和告警

1. **健康检查端点**
   ```rust
   // 在你的HTTP服务中添加健康检查
   use snowflake::get_next_id;
   
   async fn health_check() -> Result<String, String> {
       match get_next_id() {
           Ok(id) => Ok(format!("OK: {}", id)),
           Err(e) => Err(format!("ERROR: {}", e)),
       }
   }
   ```

2. **监控脚本**
   ```bash
   #!/bin/bash
   # monitor.sh - 服务监控脚本
   
   check_service() {
       local service_name="snowflake"
       
       if systemctl is-active --quiet $service_name; then
           echo "✅ $service_name 服务运行正常"
           return 0
       else
           echo "❌ $service_name 服务异常"
           # 发送告警
           curl -X POST https://hooks.slack.com/... \
               -d "{'text':'Snowflake服务异常'}"
           return 1
       fi
   }
   
   # 定期检查
   while true; do
       check_service
       sleep 60
   done
   ```

### 性能调优

1. **系统参数调优**
   ```bash
   # /etc/sysctl.conf
   
   # 网络优化
   net.core.rmem_max = 16777216
   net.core.wmem_max = 16777216
   
   # 文件描述符限制
   fs.file-max = 65536
   
   # 进程限制
   kernel.pid_max = 32768
   ```

2. **Rust编译优化**
   ```toml
   # Cargo.toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   panic = "abort"
   ```

### 备份和恢复

1. **配置备份**
   ```bash
   # backup.sh
   tar -czf snowflake-config-$(date +%Y%m%d).tar.gz \
       /etc/snowflake.toml \
       /etc/systemd/system/snowflake.service
   ```

2. **数据迁移**
   ```sql
   -- 数据库中ID的迁移
   -- 注意：雪花算法ID无法直接迁移，需要业务层面处理
   
   -- 检查ID范围
   SELECT MIN(id), MAX(id), COUNT(*) FROM orders;
   
   -- 验证ID唯一性
   SELECT COUNT(*), COUNT(DISTINCT id) FROM orders;
   ```

### 故障处理

1. **常见问题排查**
   ```bash
   # 检查服务状态
   systemctl status snowflake
   
   # 查看日志
   journalctl -u snowflake -f
   
   # 检查Worker ID冲突
   ps aux | grep snowflake
   echo $SNOWFLAKE_WORKER_ID
   
   # 检查时钟同步
   ntpq -p
   timedatectl status
   ```

2. **故障恢复脚本**
   ```bash
   #!/bin/bash
   # recovery.sh
   
   echo "🚑 启动故障恢复..."
   
   # 停止服务
   systemctl stop snowflake
   
   # 清理旧的状态文件
   rm -f /tmp/snowflake.pid
   
   # 检查配置
   if ! /usr/local/bin/snowflake --check-config; then
       echo "❌ 配置文件有误"
       exit 1
   fi
   
   # 重启服务
   systemctl start snowflake
   
   # 验证
   sleep 5
   if systemctl is-active --quiet snowflake; then
       echo "✅ 服务恢复成功"
   else
       echo "❌ 服务恢复失败"
       exit 1
   fi
   ```

通过遵循这份部署指南，您可以在各种环境中安全、高效地部署雪花算法服务，确保分布式系统中ID的全局唯一性。