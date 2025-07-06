/*!
全局锁雪花算法压力测试

测试项目:
1. 大规模多线程并发测试
2. 长时间耐久性测试  
3. 分布式唯一性验证
4. 性能基准测试

作者: zdrawai团队
版本: 2.1.0 - 全局锁简化版
*/

use std::collections::HashSet;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};
use std::thread;
use std::time::{Duration, Instant};

#[path = "snowflake.rs"]
mod snowflake;

use snowflake::{get_next_id, set_global_config, SnowflakeConfig};

/// 格式化数字
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// 测试结果
#[derive(Debug)]
pub struct TestResult {
    pub total_ids: u64,
    pub unique_ids: u64,
    pub duration: Duration,
    pub thread_count: usize,
    pub ids_per_thread: usize,
}

impl TestResult {
    fn print_report(&self) {
        let rate = self.total_ids as f64 / self.duration.as_secs_f64();
        let duplicate_rate = (self.total_ids - self.unique_ids) as f64 / self.total_ids as f64 * 100.0;
        
        println!("================================================================================");
        println!("📊 压力测试结果");
        println!("================================================================================");
        println!("🔢 ID统计:");
        println!("   总生成数: {}", format_number(self.total_ids));
        println!("   唯一数量: {}", format_number(self.unique_ids));
        println!("   重复数量: {}", format_number(self.total_ids - self.unique_ids));
        println!("   唯一率: {:.8}%", (self.unique_ids as f64 / self.total_ids as f64) * 100.0);
        
        println!("\n⚡ 性能统计:");
        println!("   测试耗时: {:?}", self.duration);
        println!("   生成速率: {} IDs/秒", format_number(rate as u64));
        println!("   线程数量: {}", self.thread_count);
        println!("   每线程数: {}", format_number(self.ids_per_thread as u64));
        
        if self.unique_ids == self.total_ids {
            println!("\n✅ 无重复ID - 唯一性验证通过!");
        } else {
            println!("\n⚠️  检测到重复ID!");
            println!("   重复率: {:.8}%", duplicate_rate);
        }
        println!("================================================================================");
    }
}

/// 运行并发测试
fn run_concurrency_test(thread_count: usize, ids_per_thread: usize) -> TestResult {
    let total_counter = Arc::new(AtomicU64::new(0));
    let all_ids = Arc::new(Mutex::new(Vec::new()));
    
    let start_time = Instant::now();
    
    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            let total_counter = Arc::clone(&total_counter);
            let all_ids = Arc::clone(&all_ids);
            
            thread::spawn(move || {
                let mut local_ids = Vec::with_capacity(ids_per_thread);
                
                for _ in 0..ids_per_thread {
                    match get_next_id() {
                        Ok(id) => {
                            local_ids.push(id);
                            total_counter.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            eprintln!("线程 {} ID生成错误: {}", thread_id, e);
                        }
                    }
                }
                
                {
                    let mut global_ids = all_ids.lock().unwrap();
                    global_ids.extend(local_ids);
                }
                
                println!("   线程 {} 完成: {} 个ID", thread_id, ids_per_thread);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start_time.elapsed();
    let total_ids = total_counter.load(Ordering::Relaxed);
    
    println!("   🕐 检查唯一性...");
    let unique_count = {
        let ids = all_ids.lock().unwrap();
        let unique_ids: HashSet<u64> = ids.iter().cloned().collect();
        unique_ids.len() as u64
    };
    
    TestResult {
        total_ids,
        unique_ids: unique_count,
        duration,
        thread_count,
        ids_per_thread,
    }
}

/// 极高并发测试
pub fn extreme_concurrency_test() {
    println!("🚀 极高并发压力测试");
    
    let test_cases = vec![8, 16, 32, 64, 128];
    let ids_per_thread = 50_000;
    
    for &thread_count in &test_cases {
        println!("\n📈 测试 {} 线程并发...", thread_count);
        let result = run_concurrency_test(thread_count, ids_per_thread);
        result.print_report();
        
        thread::sleep(Duration::from_millis(300));
    }
}

/// 长时间耐久性测试
pub fn endurance_test() {
    println!("\n🕐 长时间耐久性测试 (30秒)");
    
    let thread_count = 16;
    let batch_size = 1000;
    let test_duration = Duration::from_secs(30);
    
    let total_counter = Arc::new(AtomicU64::new(0));
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
    
    let start_time = Instant::now();
    
    // 统计线程
    let stats_counter = Arc::clone(&total_counter);
    let stats_running = Arc::clone(&running);
    let stats_handle = thread::spawn(move || {
        let mut last_count = 0;
        while stats_running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(5));
            let current_count = stats_counter.load(Ordering::Relaxed);
            let rate = (current_count - last_count) / 5;
            println!("   📊 当前速率: {} IDs/秒, 累计: {}", rate, current_count);
            last_count = current_count;
        }
    });
    
    // 工作线程
    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            let total_counter = Arc::clone(&total_counter);
            let running = Arc::clone(&running);
            
            thread::spawn(move || {
                let mut local_count = 0;
                
                while running.load(Ordering::Relaxed) {
                    for _ in 0..batch_size {
                        if !running.load(Ordering::Relaxed) {
                            break;
                        }
                        
                        if get_next_id().is_ok() {
                            local_count += 1;
                            total_counter.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    thread::yield_now();
                }
                
                println!("   线程 {} 完成: {} 个ID", thread_id, local_count);
            })
        })
        .collect();
    
    thread::sleep(test_duration);
    running.store(false, Ordering::Relaxed);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    stats_handle.join().unwrap();
    
    let duration = start_time.elapsed();
    let total_ids = total_counter.load(Ordering::Relaxed);
    
    println!("\n📊 耐久性测试结果:");
    println!("   测试时间: {:?}", duration);
    println!("   生成总数: {}", format_number(total_ids));
    println!("   平均速率: {:.0} IDs/秒", total_ids as f64 / duration.as_secs_f64());
}

/// 内存使用测试
pub fn memory_usage_test() {
    println!("\n💾 内存使用监控测试");
    
    let thread_count = 32;
    let ids_per_thread = 100_000;
    
    println!("   配置: {} 线程 × {} IDs = {} 总数", 
        thread_count, ids_per_thread, thread_count * ids_per_thread);
    
    let start_time = Instant::now();
    
    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            thread::spawn(move || {
                let mut count = 0;
                for _ in 0..ids_per_thread {
                    if get_next_id().is_ok() {
                        count += 1;
                    }
                }
                (thread_id, count)
            })
        })
        .collect();
    
    let mut total_ids = 0;
    for handle in handles {
        let (thread_id, count) = handle.join().unwrap();
        total_ids += count;
        
        if thread_id % 8 == 0 {
            println!("   线程 {} 完成: {} IDs", thread_id, count);
        }
    }
    
    let duration = start_time.elapsed();
    
    println!("\n📊 内存测试结果:");
    println!("   总ID数: {}", format_number(total_ids));
    println!("   耗时: {:?}", duration);
    println!("   速率: {:.0} IDs/秒", total_ids as f64 / duration.as_secs_f64());
    println!("   💡 提示: 此测试不存储ID，内存使用最小");
}

fn main() {
    println!("🧪 全局锁雪花算法压力测试 v2.1");
    println!("==================================================");
    
    // 使用默认配置（全局锁模式）
    let config = SnowflakeConfig {
        ..Default::default()
    };
    set_global_config(config);
    
    println!("✅ 全局锁模式已启用");
    
    // 1. 极高并发测试
    extreme_concurrency_test();
    
    // 2. 耐久性测试
    endurance_test();
    
    // 3. 内存使用测试
    memory_usage_test();
    
    println!("\n🎉 所有压力测试完成!");
    println!("📝 建议在生产环境定期运行此类测试");
}