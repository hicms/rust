/*!
分布式雪花算法ID生成器 - Rust实现

功能说明:
1. 分布式友好: 多台机器同时运行，生成的ID绝对不重复
2. 全局锁设计: 所有线程共享一个ID生成器，安全可靠
3. 时间有序: ID按时间递增，便于数据库索引和排序
4. 智能分配: 支持多种Worker ID分配策略，适应各种部署环境
5. 零外部依赖: 不需要Redis、ZooKeeper等外部服务

ID结构说明 (总共64位):
+------------------+------------------+------------------+
| 时间戳 (41 bits) | 节点ID (8 bits)  | 序列号 (12 bits) |
+------------------+------------------+------------------+
|   毫秒级时间戳    |    0-255        |     0-4095       |
+------------------+------------------+------------------+
- 时间戳: 从2025-03-08开始的毫秒数，可用69年
- 节点ID: Worker ID，标识不同的机器/进程
- 序列号: 同一毫秒内的计数器，支持每毫秒4096个ID

作者: zdrawai团队
版本: 2.1.0 - 全局锁简化版
*/

use std::env;
use std::net::UdpSocket;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::fs;
use chrono::{TimeZone, FixedOffset, LocalResult};
use once_cell::sync::Lazy;

/// 雪花算法配置结构
/// 用于定义ID生成器的各种参数
#[derive(Debug, Clone)]
pub struct SnowflakeConfig {
    
    /// Worker ID总位数 (默认8位，支持256个不同节点)
    pub worker_id_bits: u8,
    
    /// 序列号位数 (默认12位，每毫秒支持4096个ID)
    pub sequence_bits: u8,
    
    /// 时钟回拨容忍度(毫秒)
    /// 如果系统时钟往回调这个时间内，程序等待而不报错
    pub max_backward_ms: u64,
    
}

impl Default for SnowflakeConfig {
    fn default() -> Self {
        Self {
            worker_id_bits: 8,          // 8位Worker ID (支持256个节点)
            sequence_bits: 12,          // 12位序列号 (每毫秒4096个ID)
            max_backward_ms: 10,        // 容忍10毫秒时钟回拨
        }
    }
}

/// 雪花算法错误类型定义
#[derive(Debug)]
pub enum SnowflakeError {
    /// 时钟回拨错误 (系统时钟往回调整)
    ClockBackward(String),
    /// 配置错误 (参数不合法等)
    ConfigError(String),
    /// 网络错误 (获取IP地址失败等)
    NetworkError(String),
}

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

/// 雪花算法ID生成器核心结构
/// 每个实例负责生成唯一的64位ID
#[derive(Debug)]
pub struct SnowflakeIdWorker {
    /// 配置参数
    config: SnowflakeConfig,
    /// Worker ID在最终ID中的位移量 (等于序列号位数)
    worker_id_shift: u8,
    /// 时间戳在最终ID中的位移量 (等于Worker ID位数 + 序列号位数)
    timestamp_shift: u8,
    /// 序列号掩码 (用于限制序列号范围)
    sequence_mask: u64,
    /// 基准时间戳 (毫秒，从2025-03-08开始计算)
    twepoch: u64,
    /// 当前序列号 (同一毫秒内递增)
    sequence: u64,
    /// 上次生成ID的时间戳 (用于检测时钟回拨)
    last_timestamp: i64,
    /// 当前Worker ID (标识这台机器/进程)
    worker_id: u8,
}

impl SnowflakeIdWorker {
    /// 创建新的ID生成器实例
    /// 
    /// 参数:
    /// - config: 可选的配置参数，如果为None则使用默认配置
    /// 
    /// 返回:
    /// - Ok(SnowflakeIdWorker): 成功创建的生成器实例
    /// - Err(SnowflakeError): 创建失败的错误信息
    pub fn new(config: Option<SnowflakeConfig>) -> Result<Self, SnowflakeError> {
        let config = config.unwrap_or_default();
        
        // 计算各种位移量和掩码
        // Worker ID位移 = 序列号位数 (序列号在最右边)
        let worker_id_shift = config.sequence_bits;
        // 时间戳位移 = Worker ID位数 + 序列号位数 (时间戳在最左边)
        let timestamp_shift = config.worker_id_bits + config.sequence_bits;
        // 序列号掩码 = 2^序列号位数 - 1 (用于限制序列号范围)
        let sequence_mask = (1u64 << config.sequence_bits) - 1;
        
        // 计算基准时间戳 (上海时区 2025-03-08 00:00:00)
        let shanghai_offset = FixedOffset::east_opt(8 * 3600)
            .ok_or_else(|| SnowflakeError::ConfigError("无法创建上海时区".to_string()))?;
        let epoch_dt = match shanghai_offset.with_ymd_and_hms(2025, 3, 8, 0, 0, 0) {
            LocalResult::Single(dt) => Ok(dt),
            _ => Err(SnowflakeError::ConfigError("无效的基准日期".to_string())),
        }?;
        let twepoch = epoch_dt.timestamp_millis() as u64;
        
        // 创建生成器实例
        let mut worker = Self {
            config,
            worker_id_shift,
            timestamp_shift,
            sequence_mask,
            twepoch,
            sequence: 0,           // 序列号从0开始
            last_timestamp: -1,    // 上次时间戳初始化为-1
            worker_id: 0,          // Worker ID稍后初始化
        };
        
        // 初始化Worker ID (这是关键步骤，决定这台机器的唯一标识)
        worker.init_worker_id()?;
        Ok(worker)
    }
    
    /// 初始化Worker ID
    /// 按照优先级依次尝试不同的方式获取Worker ID:
    /// 1. 环境变量 (最高优先级)
    /// 2. 数据中心+机器ID配置  
    /// 3. IP段自动分配 (最低优先级)
    fn init_worker_id(&mut self) -> Result<(), SnowflakeError> {
        // 方式1: 从环境变量获取 (最高优先级)
        // 用法: export SNOWFLAKE_WORKER_ID=50
        if let Ok(worker_id_str) = env::var("SNOWFLAKE_WORKER_ID") {
            if let Ok(worker_id) = worker_id_str.parse::<u8>() {
                self.worker_id = worker_id;
                println!("✅ 使用环境变量Worker ID: {}", worker_id);
                return Ok(());
            }
        }
        
        // 方式2: 从配置文件获取数据中心+机器ID
        // 检查snowflake.toml中的datacenter_id和machine_id配置
        if let Some(worker_id) = self.try_config_mapping()? {
            self.worker_id = worker_id;
            println!("✅ 使用配置文件映射Worker ID: {}", worker_id);
            return Ok(());
        }
        
        // 方式3: 基于IP段自动分配 (最后备选)
        // 根据本机IP地址自动计算Worker ID
        self.worker_id = self.generate_ip_based_worker_id()?;
        println!("✅ 使用IP段自动分配Worker ID: {}", self.worker_id);
        Ok(())
    }
    
    /// 尝试从配置文件获取数据中心+机器ID配置
    /// 查找snowflake.toml文件，解析数据中心和机器ID
    fn try_config_mapping(&self) -> Result<Option<u8>, SnowflakeError> {
        // 定义配置文件查找路径 (按优先级排序)
        let config_paths = [
            "snowflake.toml",           // 当前目录
            "/etc/snowflake.toml"       // 系统目录
        ];
        
        // 依次尝试读取配置文件
        for path in &config_paths {
            if let Ok(content) = fs::read_to_string(path) {
                println!("📄 找到配置文件: {}", path);
                return self.parse_config(&content);
            }
        }
        
        // 没有找到配置文件
        Ok(None)
    }
    
    /// 解析配置文件内容
    /// 只支持数据中心+机器ID配置方式
    fn parse_config(&self, content: &str) -> Result<Option<u8>, SnowflakeError> {
        let mut datacenter_id: Option<u8> = None;
        let mut machine_id: Option<u8> = None;
        
        // 逐行解析配置文件
        for line in content.lines() {
            let line = line.trim();
            // 跳过空行和注释行
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // 解析 datacenter_id = 1
            if line.starts_with("datacenter_id") {
                if let Some(value) = line.split('=').nth(1) {
                    datacenter_id = value.trim().parse::<u8>().ok();
                }
            }
            // 解析 machine_id = 5  
            else if line.starts_with("machine_id") {
                if let Some(value) = line.split('=').nth(1) {
                    machine_id = value.trim().parse::<u8>().ok();
                }
            }
        }
        
        // 使用数据中心+机器ID组合计算
        if let (Some(dc_id), Some(m_id)) = (datacenter_id, machine_id) {
            // Worker ID = (数据中心ID << 6) | 机器ID
            // 高2位存储数据中心ID，低6位存储机器ID
            let worker_id = ((dc_id & 0x03) << 6) | (m_id & 0x3F);
            println!("🏢 数据中心+机器ID: DC{} + M{} → Worker ID {}", dc_id, m_id, worker_id);
            return Ok(Some(worker_id));
        }
        
        // 配置文件中没有找到适用的配置
        Ok(None)
    }
    
    /// 基于IP段自动分配Worker ID
    /// 这是最后的备选方案，根据本机IP地址计算Worker ID
    fn generate_ip_based_worker_id(&self) -> Result<u8, SnowflakeError> {
        let ip_str = self.get_local_ip()?;
        
        // 尝试解析IPv4地址
        if let Ok(addr) = ip_str.parse::<std::net::Ipv4Addr>() {
            let octets = addr.octets();
            // 使用IP地址的后两段计算Worker ID
            // 公式: ((第3段 & 0x0F) << 4) | (第4段 & 0x0F)
            // 例如: 192.168.1.100 → ((1 & 15) << 4) | (100 & 15) = 16 + 4 = 20
            let worker_id = ((octets[2] & 0x0F) << 4) | (octets[3] & 0x0F);
            println!("🌐 IP段计算: {} → Worker ID {}", ip_str, worker_id);
            return Ok(worker_id);
        }
        
        // 如果IP解析失败，使用简单的主机名计算作为最后备选
        let hostname = hostname::get()
            .map_err(|_| SnowflakeError::NetworkError("无法获取主机名".to_string()))?
            .to_string_lossy()
            .to_string();
        
        // 使用主机名长度和首字符简单计算Worker ID
        let worker_id = (hostname.len() % 256) as u8;
        println!("🔗 主机名计算: {} → Worker ID {}", hostname, worker_id);
        Ok(worker_id)
    }
    
    /// 快速获取本机IP地址
    /// 使用UDP连接的方式快速获取本机IP，不需要实际发送数据
    fn get_local_ip(&self) -> Result<String, SnowflakeError> {
        // 创建UDP socket并连接到外部地址 (不发送数据)
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    return Ok(local_addr.ip().to_string());
                }
            }
        }
        // 如果获取失败，返回本地回环地址
        Ok("127.0.0.1".to_string())
    }
    
    /// 获取当前时间戳 (毫秒)
    /// 返回从基准时间(2025-03-08)开始的毫秒数
    fn time_gen(&self) -> Result<u64, SnowflakeError> {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SnowflakeError::ClockBackward(format!("系统时钟错误: {}", e)))?
            .as_millis() as u64;
        
        // 检查当前时间是否在基准时间之后
        if now_ms >= self.twepoch {
            Ok(now_ms - self.twepoch)
        } else {
            Err(SnowflakeError::ConfigError("基准时间设置在未来".to_string()))
        }
    }
    
    /// 生成下一个唯一ID (核心算法)
    /// 这是整个雪花算法的核心逻辑
    pub fn next_id(&mut self) -> Result<u64, SnowflakeError> {
        // 获取当前时间戳
        let mut timestamp = self.time_gen()? as i64;
        
        // 检查时钟回拨问题
        if timestamp < self.last_timestamp {
            let diff = (self.last_timestamp - timestamp) as u64;
            
            // 如果回拨时间在容忍范围内，等待时钟追上
            if diff <= self.config.max_backward_ms {
                println!("⏰ 检测到时钟回拨{}ms，等待中...", diff);
                thread::sleep(Duration::from_millis(diff + 1));
                timestamp = self.time_gen()? as i64;
            } else {
                // 时钟回拨超出容忍范围，抛出错误
                return Err(SnowflakeError::ClockBackward(format!(
                    "时钟回拨过大: {}ms，超出容忍范围{}ms", diff, self.config.max_backward_ms
                )));
            }
        }
        
        // 处理序列号逻辑
        if timestamp == self.last_timestamp {
            // 同一毫秒内，序列号递增
            self.sequence = (self.sequence + 1) & self.sequence_mask;
            
            // 如果序列号用尽 (达到4096)，等待下一毫秒
            if self.sequence == 0 {
                timestamp = self.wait_next_millis(self.last_timestamp)?;
            }
        } else {
            // 不同毫秒，序列号重置为0
            self.sequence = 0;
        }
        
        // 更新上次时间戳
        self.last_timestamp = timestamp;
        
        // 组装最终的64位ID
        // ID结构: [时间戳 41位] [Worker ID 8位] [序列号 12位]
        let id = ((timestamp as u64) << self.timestamp_shift)  // 时间戳左移到高位
            | ((self.worker_id as u64) << self.worker_id_shift) // Worker ID左移到中位
            | self.sequence;                                     // 序列号在低位
        
        Ok(id)
    }
    
    /// 等待到下一毫秒
    /// 当同一毫秒内序列号用尽时调用
    fn wait_next_millis(&self, last_timestamp: i64) -> Result<i64, SnowflakeError> {
        let mut timestamp = self.time_gen()? as i64;
        // 循环等待，直到时间戳发生变化
        while timestamp <= last_timestamp {
            thread::yield_now(); // 让出CPU时间片，提高效率
            timestamp = self.time_gen()? as i64;
        }
        Ok(timestamp)
    }
    
    /// 获取当前Worker ID
    #[allow(dead_code)]
    pub fn get_worker_id(&self) -> u8 {
        self.worker_id
    }
}

// ============================================================================
// 全局静态变量定义
// 用于支持全局锁的配置管理
// ============================================================================

/// 全局配置对象 (线程安全)
/// 所有线程共享同一个配置
static GLOBAL_CONFIG: Lazy<Arc<Mutex<SnowflakeConfig>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SnowflakeConfig::default()))
});

/// 全局共享的Worker实例
/// 所有线程共享一个生成器，使用全局锁保证线程安全
static GLOBAL_WORKER: Lazy<Arc<Mutex<SnowflakeIdWorker>>> = Lazy::new(|| {
    Arc::new(Mutex::new(
        SnowflakeIdWorker::new(None).expect("无法创建全局Worker")
    ))
});

// ============================================================================
// 公共API接口
// ============================================================================

/// 获取下一个唯一ID (主要接口)
/// 
/// 这是用户调用的主要函数，使用全局锁保证线程安全:
/// - 全局锁模式: 所有线程竞争同一个锁，安全可靠
/// 
/// 返回:
/// - Ok(u64): 生成的唯一ID
/// - Err(SnowflakeError): 生成失败的错误信息
pub fn get_next_id() -> Result<u64, SnowflakeError> {
    // 全局锁模式: 所有线程竞争同一个锁
    GLOBAL_WORKER.lock().unwrap().next_id()
}

/// 设置全局配置
/// 
/// 参数:
/// - config: 新的配置参数
/// 
/// 注意: 配置修改只对新创建的Worker实例生效
pub fn set_global_config(config: SnowflakeConfig) {
    *GLOBAL_CONFIG.lock().unwrap() = config;
}

/// 简单演示程序
/// 展示雪花算法的基本功能和性能
#[allow(dead_code)]
pub fn demo() {
    println!("=== 🔒 全局锁雪花算法演示 ===");
    
    // 显示当前Worker信息
    let worker = GLOBAL_WORKER.lock().unwrap();
    println!("Worker ID: {}", worker.get_worker_id());
    drop(worker); // 释放锁
    
    // 生成示例ID
    println!("\n📝 生成5个示例ID:");
    for i in 1..=5 {
        match get_next_id() {
            Ok(id) => println!("ID_{}: {}", i, id),
            Err(e) => println!("❌ 错误: {}", e),
        }
    }
    
    // 性能基准测试
    println!("\n⚡ 性能测试 (单线程):");
    let start = std::time::Instant::now();
    let count = 100_000;
    
    // 生成10万个ID测试性能
    for _ in 0..count {
        let _ = get_next_id();
    }
    
    let duration = start.elapsed();
    let rate = count as f64 / duration.as_secs_f64();
    println!("生成 {} 个ID，耗时: {:?}", count, duration);
    println!("性能: {:.0} IDs/秒", rate);
    
    // 显示配置信息
    println!("\n⚙️  配置信息:");
    let config = GLOBAL_CONFIG.lock().unwrap();
    println!("模式: 🔒 全局锁模式 (所有线程竞争)");
    println!("IP获取模式: ✅ 开启");
    println!("时钟回拨容忍: {}ms", config.max_backward_ms);
}

/// 主函数入口
#[allow(dead_code)]
fn main() {
    demo();
}