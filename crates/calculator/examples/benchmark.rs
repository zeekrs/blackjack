//! Blackjack 计算器性能测试示例
//! 
//! 运行方式：
//! ```bash
//! cargo run --release --example benchmark
//! ```

use calculator::{Calculator, create_full_8_deck};
use std::time::Instant;

fn main() {
    println!("=== Blackjack 计算器性能测试 ===\n");

    // 创建计算器（使用默认规则）
    let calculator = Calculator::with_default_rules();
    
    // 创建完整 8 副牌
    println!("创建完整 8 副牌...");
    let deck = create_full_8_deck();
    let total_cards: u32 = deck.values().sum();
    println!("总牌数: {}\n", total_cards);

    // 显示牌组信息（按顺序）
    println!("牌组分布:");
    // A
    if let Some(count) = deck.get(&calculator::Card::Ace) {
        println!("  A: {} 张", count);
    }
    // 2-10
    for n in 2..=10 {
        if let Some(count) = deck.get(&calculator::Card::Number(n)) {
            println!("  {}: {} 张", n, count);
        }
    }
    // J/Q/K
    if let Some(count) = deck.get(&calculator::Card::Face) {
        println!("  J/Q/K: {} 张", count);
    }
    println!();

    // 开始计时
    println!("开始计算上桌 EV...");
    let start = Instant::now();
    
    // 计算上桌 EV
    let result = calculator.calculate_table_ev(&deck);
    
    // 结束计时
    let duration = start.elapsed();

    // 显示结果
    println!("\n=== 计算结果 ===");
    println!("上桌 EV (期望值): {:.6}", result.ev);
    println!();
    println!("概率分布:");
    println!("  玩家胜率: {:.6}", result.player_win_prob);
    println!("  庄家胜率: {:.6}", result.dealer_win_prob);
    println!("  平局率: {:.6}", result.push_prob);
    println!("  玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("  庄家黑杰克率: {:.6}", result.dealer_blackjack_prob);
    println!("  投降率: {:.6}", result.surrender_prob);
    
    let total_prob = result.player_win_prob 
        + result.dealer_win_prob 
        + result.push_prob 
        + result.player_blackjack_prob 
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    println!("概率总和: {:.6}", total_prob);
    
    println!("\n=== 性能统计 ===");
    println!("计算耗时: {:.3} 秒", duration.as_secs_f64());
    println!("计算耗时: {} 毫秒", duration.as_millis());
    println!("计算耗时: {} 微秒", duration.as_micros());
    
    // 检查是否满足性能要求（3秒以内）
    if duration.as_secs_f64() < 3.0 {
        println!("\n✅ 性能要求满足：耗时 < 3 秒");
    } else {
        println!("\n⚠️  性能要求未满足：耗时 >= 3 秒");
    }
}

