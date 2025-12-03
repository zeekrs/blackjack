//! 对玩家有利的牌组测试
//! 
//! 测试各种对玩家有利的牌组配置，验证 EV 计算是否正确
//! 对玩家有利的牌组特征：
//! - 高牌（10, J, Q, K）较多：玩家更容易拿到黑杰克
//! - A 较多：玩家更容易拿到黑杰克和软手牌
//! - 低牌（2-6）较少：庄家更容易爆牌

use calculator::{Calculator, Card, CardCounts, GameRules};
use std::collections::HashMap;

/// 创建高牌较多的牌组（对玩家有利）
/// 高牌多意味着玩家更容易拿到黑杰克
fn create_high_card_rich_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 大量高牌（10, J, Q, K）
    deck.insert(Card::Face, 80);  // J/Q/K
    deck.insert(Card::Number(10), 40);
    // 适量的 A
    deck.insert(Card::Ace, 30);
    // 少量中牌
    deck.insert(Card::Number(7), 10);
    deck.insert(Card::Number(8), 10);
    deck.insert(Card::Number(9), 10);
    // 很少的低牌（2-6）
    for n in 2..=6 {
        deck.insert(Card::Number(n), 5);
    }
    deck
}

/// 创建 A 较多的牌组（对玩家有利）
/// A 多意味着玩家更容易拿到黑杰克和软手牌
fn create_ace_rich_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 大量 A
    deck.insert(Card::Ace, 50);
    // 大量高牌配合 A 组成黑杰克
    deck.insert(Card::Face, 60);
    deck.insert(Card::Number(10), 30);
    // 适量中牌
    deck.insert(Card::Number(7), 15);
    deck.insert(Card::Number(8), 15);
    deck.insert(Card::Number(9), 15);
    // 少量低牌
    for n in 2..=6 {
        deck.insert(Card::Number(n), 8);
    }
    deck
}

/// 创建低牌较少的牌组（对玩家有利）
/// 低牌少意味着庄家更容易爆牌（因为庄家必须 <17 要牌）
fn create_low_card_poor_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 正常的高牌
    deck.insert(Card::Face, 40);
    deck.insert(Card::Number(10), 20);
    // 正常的 A
    deck.insert(Card::Ace, 20);
    // 正常的中牌
    deck.insert(Card::Number(7), 20);
    deck.insert(Card::Number(8), 20);
    deck.insert(Card::Number(9), 20);
    // 很少的低牌（2-6）- 这是关键
    for n in 2..=6 {
        deck.insert(Card::Number(n), 2);  // 非常少
    }
    deck
}

/// 创建极端有利的牌组（高牌多 + A 多 + 低牌少）
fn create_extremely_favorable_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 大量高牌
    deck.insert(Card::Face, 100);
    deck.insert(Card::Number(10), 50);
    // 大量 A
    deck.insert(Card::Ace, 40);
    // 适量中牌
    deck.insert(Card::Number(7), 20);
    deck.insert(Card::Number(8), 20);
    deck.insert(Card::Number(9), 20);
    // 极少低牌
    for n in 2..=6 {
        deck.insert(Card::Number(n), 1);
    }
    deck
}

/// 创建只有高牌和 A 的牌组（极端情况）
fn create_only_high_and_ace_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 30);
    deck.insert(Card::Face, 50);
    deck.insert(Card::Number(10), 30);
    deck
}

#[test]
fn test_high_card_rich_deck() {
    let deck = create_high_card_rich_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("高牌较多牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("庄家黑杰克率: {:.6}", result.dealer_blackjack_prob);
    
    // 高牌多时，玩家黑杰克概率应该较高
    assert!(result.player_blackjack_prob > 0.03, "高牌多时玩家黑杰克概率应该较高");
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 高牌多时，EV 应该比满8副牌更好（更接近0或为正）
    let full_deck = calculator::create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    println!("满8副牌 EV: {:.6}", full_result.ev);
    assert!(result.ev > full_result.ev, 
        "高牌多时 EV 应该比满8副牌更好。高牌牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_ace_rich_deck() {
    let deck = create_ace_rich_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("A较多牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    
    // A 多时，玩家黑杰克概率应该较高
    assert!(result.player_blackjack_prob > 0.03, "A多时玩家黑杰克概率应该较高");
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // A 多时，EV 应该比满8副牌更好
    let full_deck = calculator::create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev > full_result.ev, 
        "A多时 EV 应该比满8副牌更好。A多牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_low_card_poor_deck() {
    let deck = create_low_card_poor_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("低牌较少牌组 EV: {:.6}", result.ev);
    println!("玩家胜率: {:.6}", result.player_win_prob);
    println!("庄家胜率: {:.6}", result.dealer_win_prob);
    
    // 低牌少时，庄家更容易爆牌，玩家胜率应该较高
    // 注意：由于庄家必须 <17 要牌，低牌少意味着庄家更容易拿到高牌而爆牌
    assert!(result.player_win_prob > result.dealer_win_prob, 
        "低牌少时玩家胜率应该高于庄家胜率");
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 低牌少时，EV 应该比满8副牌更好
    let full_deck = calculator::create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev > full_result.ev, 
        "低牌少时 EV 应该比满8副牌更好。低牌少牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_extremely_favorable_deck() {
    let deck = create_extremely_favorable_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("极端有利牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("玩家胜率: {:.6}", result.player_win_prob);
    println!("庄家胜率: {:.6}", result.dealer_win_prob);
    
    // 极端有利的牌组，EV 应该为正数（对玩家有利）
    // 注意：由于基础策略和庄家规则，即使牌组非常有利，EV 也可能只是接近0
    // 但应该明显比满8副牌好
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 与满8副牌对比
    let full_deck = calculator::create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    println!("满8副牌 EV: {:.6}", full_result.ev);
    assert!(result.ev > full_result.ev, 
        "极端有利牌组 EV 应该明显比满8副牌好。极端有利牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
    
    // 极端有利时，玩家胜率应该明显高于庄家胜率
    assert!(result.player_win_prob > result.dealer_win_prob,
        "极端有利时玩家胜率应该高于庄家胜率");
}

#[test]
fn test_only_high_and_ace_deck() {
    let deck = create_only_high_and_ace_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("只有高牌和A的牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("庄家黑杰克率: {:.6}", result.dealer_blackjack_prob);
    
    // 只有高牌和A时，黑杰克概率应该非常高
    assert!(result.player_blackjack_prob > 0.05, 
        "只有高牌和A时玩家黑杰克概率应该很高，实际: {}", result.player_blackjack_prob);
    assert!(result.dealer_blackjack_prob > 0.05, 
        "只有高牌和A时庄家黑杰克概率应该很高，实际: {}", result.dealer_blackjack_prob);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
}

#[test]
fn test_balanced_favorable_deck() {
    // 创建一个平衡但略微对玩家有利的牌组
    // 高牌略多，低牌略少
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 35);  // 略多
    deck.insert(Card::Face, 50);  // 略多
    deck.insert(Card::Number(10), 35);  // 略多
    deck.insert(Card::Number(9), 25);
    deck.insert(Card::Number(8), 25);
    deck.insert(Card::Number(7), 25);
    // 低牌较少
    for n in 2..=6 {
        deck.insert(Card::Number(n), 15);  // 较少
    }
    
    let calculator = Calculator::with_default_rules();
    let result = calculator.calculate_table_ev(&deck);
    
    println!("平衡有利牌组 EV: {:.6}", result.ev);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 应该比满8副牌好
    let full_deck = calculator::create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev > full_result.ev, 
        "平衡有利牌组 EV 应该比满8副牌好。平衡有利牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

