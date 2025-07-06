use moka::future::Cache;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Moka 缓存学习 Demo ===\n");
    
    // 创建一个缓存实例
    // 最大容量为100个条目，TTL为60秒
    let cache: Cache<String, String> = Cache::builder()
        .max_capacity(100)
        .time_to_live(Duration::from_secs(60))
        .build();
    
    println!("1. 创建缓存实例");
    println!("   - 最大容量: 100");
    println!("   - TTL: 60秒\n");
    
    // 基本的缓存操作
    println!("2. 基本缓存操作:");
    
    // 插入数据
    cache.insert("user:1".to_string(), "Alice".to_string()).await;
    cache.insert("user:2".to_string(), "Bob".to_string()).await;
    cache.insert("user:3".to_string(), "Charlie".to_string()).await;
    
    println!("   ✓ 插入了3个用户数据");
    
    // 读取数据
    if let Some(value) = cache.get(&"user:1".to_string()).await {
        println!("   ✓ 读取 user:1 = {}", value);
    }
    
    if let Some(value) = cache.get(&"user:2".to_string()).await {
        println!("   ✓ 读取 user:2 = {}", value);
    }
    
    // 尝试读取不存在的数据
    if let Some(value) = cache.get(&"user:999".to_string()).await {
        println!("   ✓ 读取 user:999 = {}", value);
    } else {
        println!("   ✗ user:999 不存在于缓存中");
    }
    
    println!("\n3. 缓存统计:");
    println!("   - 当前缓存大小: {}", cache.entry_count());
    
    // 演示get_with功能 - 如果缓存中不存在，则计算并插入
    println!("\n4. get_with 演示:");
    let expensive_value = cache.get_with("expensive_key".to_string(), async {
        println!("   > 模拟耗时计算...");
        sleep(Duration::from_millis(100)).await;
        "expensive_result".to_string()
    }).await;
    
    println!("   ✓ 第一次获取 expensive_key = {}", expensive_value);
    
    // 再次获取，这次应该直接从缓存返回
    let cached_value = cache.get_with("expensive_key".to_string(), async {
        println!("   > 这不应该被执行");
        "should_not_see_this".to_string()
    }).await;
    
    println!("   ✓ 第二次获取 expensive_key = {} (来自缓存)", cached_value);
    
    // 演示缓存失效
    println!("\n5. 缓存失效演示:");
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
    println!("\n7. 批量操作演示:");
    
    // 批量插入
    for i in 10..20 {
        cache.insert(format!("batch:{}", i), format!("value_{}", i)).await;
    }
    
    println!("   ✓ 批量插入了10个条目");
    
    // 遍历所有键值对
    println!("   当前缓存内容:");
    cache.run_pending_tasks().await; // 确保所有挂起的任务都完成
    
    println!("   - 总缓存大小: {}", cache.entry_count());
    
    println!("\n=== Demo 完成 ===");
}
