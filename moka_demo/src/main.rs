use moka::future::Cache;
use std::time::Duration;
use tokio::time::sleep;

/// Moka缓存库演示程序
/// 
/// 本程序展示了Moka缓存库的主要功能，包括：
/// 1. 缓存的创建和配置
/// 2. 基本的增删改查操作
/// 3. 条件计算缓存（get_with）
/// 4. 缓存失效和清理
/// 5. 批量操作
/// 6. 缓存统计信息
#[tokio::main]
async fn main() {
    println!("=== Moka 缓存学习 Demo ===\n");
    
    // 创建一个缓存实例
    // 最大容量为100个条目，TTL为60秒
    // 
    // Cache::builder() 创建一个缓存构建器
    // max_capacity(100) 设置缓存最大容量为100个条目，超过会触发LRU淘汰
    // time_to_live(60秒) 设置缓存条目的生存时间，过期后自动清理
    // build() 构建并返回缓存实例
    let cache: Cache<String, String> = Cache::builder()
        .max_capacity(100)
        .time_to_live(Duration::from_secs(60))
        .build();
    
    println!("1. 创建缓存实例");
    println!("   - 最大容量: 100");
    println!("   - TTL: 60秒\n");
    
    // 基本的缓存操作
    // 展示缓存的基本CRUD操作：插入、读取、更新、删除
    println!("2. 基本缓存操作:");
    
    // 插入数据
    // insert() 方法用于向缓存中插入键值对
    // 该方法是异步的，会立即将数据存入缓存
    cache.insert("user:1".to_string(), "Alice".to_string()).await;
    cache.insert("user:2".to_string(), "Bob".to_string()).await;
    cache.insert("user:3".to_string(), "Charlie".to_string()).await;
    
    println!("   ✓ 插入了3个用户数据");
    
    // 读取数据
    // get() 方法用于根据键获取值
    // 返回Option<V>，如果键不存在则返回None
    if let Some(value) = cache.get(&"user:1".to_string()).await {
        println!("   ✓ 读取 user:1 = {}", value);
    }
    
    if let Some(value) = cache.get(&"user:2".to_string()).await {
        println!("   ✓ 读取 user:2 = {}", value);
    }
    
    // 尝试读取不存在的数据
    // 演示缓存未命中的情况
    if let Some(value) = cache.get(&"user:999".to_string()).await {
        println!("   ✓ 读取 user:999 = {}", value);
    } else {
        println!("   ✗ user:999 不存在于缓存中");
    }
    
    println!("\n3. 缓存统计:");
    // entry_count() 返回当前缓存中的条目数量
    println!("   - 当前缓存大小: {}", cache.entry_count());
    
    // 演示get_with功能 - 如果缓存中不存在，则计算并插入
    // get_with 是一个非常有用的方法，它可以避免缓存击穿的问题
    // 当多个并发请求同时访问同一个不存在的键时，只有一个请求会执行计算逻辑
    println!("\n4. get_with 演示:");
    let expensive_value = cache.get_with("expensive_key".to_string(), async {
        println!("   > 模拟耗时计算...");
        // 模拟一个耗时的计算操作，比如数据库查询、API调用等
        sleep(Duration::from_millis(100)).await;
        "expensive_result".to_string()
    }).await;
    
    println!("   ✓ 第一次获取 expensive_key = {}", expensive_value);
    
    // 再次获取，这次应该直接从缓存返回
    // 由于键已经存在于缓存中，所以不会执行计算逻辑
    let cached_value = cache.get_with("expensive_key".to_string(), async {
        println!("   > 这不应该被执行");
        "should_not_see_this".to_string()
    }).await;
    
    println!("   ✓ 第二次获取 expensive_key = {} (来自缓存)", cached_value);
    
    // 演示缓存失效
    // 缓存失效是主动删除缓存条目的方式
    println!("\n5. 缓存失效演示:");
    // invalidate() 方法用于从缓存中删除指定键的条目
    cache.invalidate(&"user:1".to_string()).await;
    
    if let Some(value) = cache.get(&"user:1".to_string()).await {
        println!("   ✗ user:1 仍然存在: {}", value);
    } else {
        println!("   ✓ user:1 已被删除");
    }
    
    // 最终统计
    println!("\n6. 最终统计:");
    println!("   - 当前缓存大小: {}", cache.entry_count());
    
    // 批量操作演示
    // 在实际应用中，批量操作可以提高缓存的使用效率
    println!("\n7. 批量操作演示:");
    
    // 批量插入
    // 使用循环批量插入多个条目
    for i in 10..20 {
        cache.insert(format!("batch:{}", i), format!("value_{}", i)).await;
    }
    
    println!("   ✓ 批量插入了10个条目");
    
    // 遍历所有键值对
    println!("   当前缓存内容:");
    // run_pending_tasks() 确保所有挂起的后台任务都完成
    // 这包括缓存的维护任务，如过期条目的清理
    cache.run_pending_tasks().await;
    
    println!("   - 总缓存大小: {}", cache.entry_count());
    
    println!("\n=== Demo 完成 ===");
}
