//! 对庄家有利的牌组测试
//! 
//! 测试各种对庄家有利的牌组配置，验证 EV 计算是否正确
//! 对庄家有利的牌组特征：
//! - 低牌（2-6）较多：庄家不容易爆牌，玩家容易爆牌
//! - 高牌（10, J, Q, K）较少：玩家不容易拿到黑杰克
//! - A 较少：玩家不容易拿到黑杰克

use calculator::{Calculator, Card, CardCounts};
use calculator::create_full_8_deck;

/// 创建低牌较多的牌组（对庄家有利）
/// 低牌多意味着庄家不容易爆牌（因为庄家必须 <17 要牌）
fn create_low_card_rich_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 大量低牌（2-6）
    for n in 2..=6 {
        deck.insert(Card::Number(n), 30);
    }
    // 少量高牌
    deck.insert(Card::Face, 20);
    deck.insert(Card::Number(10), 10);
    // 少量 A
    deck.insert(Card::Ace, 15);
    // 适量中牌
    deck.insert(Card::Number(7), 15);
    deck.insert(Card::Number(8), 15);
    deck.insert(Card::Number(9), 15);
    deck
}

/// 创建高牌较少的牌组（对庄家有利）
/// 高牌少意味着玩家不容易拿到黑杰克
fn create_high_card_poor_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 很少的高牌
    deck.insert(Card::Face, 15);
    deck.insert(Card::Number(10), 8);
    // 很少的 A
    deck.insert(Card::Ace, 10);
    // 正常的中牌
    deck.insert(Card::Number(7), 25);
    deck.insert(Card::Number(8), 25);
    deck.insert(Card::Number(9), 25);
    // 大量的低牌
    for n in 2..=6 {
        deck.insert(Card::Number(n), 25);
    }
    deck
}

/// 创建 A 较少的牌组（对庄家有利）
/// A 少意味着玩家不容易拿到黑杰克
fn create_ace_poor_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 很少的 A
    deck.insert(Card::Ace, 8);
    // 正常的高牌
    deck.insert(Card::Face, 30);
    deck.insert(Card::Number(10), 20);
    // 正常的中牌
    deck.insert(Card::Number(7), 25);
    deck.insert(Card::Number(8), 25);
    deck.insert(Card::Number(9), 25);
    // 大量的低牌
    for n in 2..=6 {
        deck.insert(Card::Number(n), 30);
    }
    deck
}

/// 创建极端对庄家有利的牌组（低牌多 + 高牌少 + A 少）
fn create_extremely_unfavorable_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 极少的高牌
    deck.insert(Card::Face, 10);
    deck.insert(Card::Number(10), 5);
    // 极少的 A
    deck.insert(Card::Ace, 5);
    // 正常的中牌
    deck.insert(Card::Number(7), 20);
    deck.insert(Card::Number(8), 20);
    deck.insert(Card::Number(9), 20);
    // 大量的低牌（2-6）
    for n in 2..=6 {
        deck.insert(Card::Number(n), 40);
    }
    deck
}

/// 创建只有低牌和中牌的牌组（极端情况）
fn create_only_low_and_mid_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 只有低牌和中牌，没有高牌和A
    for n in 2..=9 {
        deck.insert(Card::Number(n), 30);
    }
    deck
}

#[test]
fn test_low_card_rich_deck() {
    let deck = create_low_card_rich_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("低牌较多牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("玩家胜率: {:.6}", result.player_win_prob);
    println!("庄家胜率: {:.6}", result.dealer_win_prob);
    
    // 低牌多时，玩家黑杰克概率应该较低
    assert!(result.player_blackjack_prob < 0.03, 
        "低牌多时玩家黑杰克概率应该较低，实际: {}", result.player_blackjack_prob);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 低牌多时，EV 应该比满8副牌更差（更负）
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    println!("满8副牌 EV: {:.6}", full_result.ev);
    assert!(result.ev < full_result.ev, 
        "低牌多时 EV 应该比满8副牌更差。低牌多牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_high_card_poor_deck() {
    let deck = create_high_card_poor_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("高牌较少牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("庄家黑杰克率: {:.6}", result.dealer_blackjack_prob);
    
    // 高牌少时，玩家黑杰克概率应该较低
    assert!(result.player_blackjack_prob < 0.02, 
        "高牌少时玩家黑杰克概率应该较低，实际: {}", result.player_blackjack_prob);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 高牌少时，EV 应该比满8副牌更差
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev < full_result.ev, 
        "高牌少时 EV 应该比满8副牌更差。高牌少牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_ace_poor_deck() {
    let deck = create_ace_poor_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("A较少牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    
    // A 少时，玩家黑杰克概率应该较低
    assert!(result.player_blackjack_prob < 0.02, 
        "A少时玩家黑杰克概率应该较低，实际: {}", result.player_blackjack_prob);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // A 少时，EV 应该比满8副牌更差
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev < full_result.ev, 
        "A少时 EV 应该比满8副牌更差。A少牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_extremely_unfavorable_deck() {
    let deck = create_extremely_unfavorable_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("极端不利牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("玩家胜率: {:.6}", result.player_win_prob);
    println!("庄家胜率: {:.6}", result.dealer_win_prob);
    
    // 极端不利的牌组，EV 应该非常负（对玩家非常不利）
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 与满8副牌对比
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    println!("满8副牌 EV: {:.6}", full_result.ev);
    assert!(result.ev < full_result.ev, 
        "极端不利牌组 EV 应该明显比满8副牌更差。极端不利牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
    
    // 极端不利时，庄家胜率应该明显高于玩家胜率
    assert!(result.dealer_win_prob > result.player_win_prob,
        "极端不利时庄家胜率应该高于玩家胜率");
    
    // EV 应该非常负
    assert!(result.ev < -0.05, 
        "极端不利牌组 EV 应该非常负，实际: {}", result.ev);
}

#[test]
fn test_only_low_and_mid_deck() {
    let deck = create_only_low_and_mid_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    println!("只有低牌和中牌的牌组 EV: {:.6}", result.ev);
    println!("玩家黑杰克率: {:.6}", result.player_blackjack_prob);
    println!("庄家黑杰克率: {:.6}", result.dealer_blackjack_prob);
    
    // 只有低牌和中牌时，无法组成黑杰克
    assert_eq!(result.player_blackjack_prob, 0.0, 
        "只有低牌和中牌时玩家黑杰克概率应该为0");
    assert_eq!(result.dealer_blackjack_prob, 0.0, 
        "只有低牌和中牌时庄家黑杰克概率应该为0");
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 没有高牌和A时，EV 应该比满8副牌更差
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev < full_result.ev, 
        "只有低牌和中牌时 EV 应该比满8副牌更差。只有低牌和中牌牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_balanced_unfavorable_deck() {
    // 创建一个平衡但略微对庄家有利的牌组
    // 低牌略多，高牌略少
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 20);  // 略少
    deck.insert(Card::Face, 30);  // 略少
    deck.insert(Card::Number(10), 20);  // 略少
    deck.insert(Card::Number(9), 30);
    deck.insert(Card::Number(8), 30);
    deck.insert(Card::Number(7), 30);
    // 低牌较多
    for n in 2..=6 {
        deck.insert(Card::Number(n), 35);  // 较多
    }
    
    let calculator = Calculator::with_default_rules();
    let result = calculator.calculate_table_ev(&deck);
    
    println!("平衡不利牌组 EV: {:.6}", result.ev);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 应该比满8副牌更差
    let full_deck = create_full_8_deck();
    let full_result = calculator.calculate_table_ev(&full_deck);
    
    assert!(result.ev < full_result.ev, 
        "平衡不利牌组 EV 应该比满8副牌更差。平衡不利牌组 EV: {}, 满8副牌 EV: {}", 
        result.ev, full_result.ev);
}

#[test]
fn test_comparison_favorable_vs_unfavorable() {
    // 对比测试：对玩家有利 vs 对庄家有利
    let favorable_deck = {
        let mut deck = CardCounts::new();
        deck.insert(Card::Ace, 40);
        deck.insert(Card::Face, 80);
        deck.insert(Card::Number(10), 50);
        for n in 2..=6 {
            deck.insert(Card::Number(n), 5);
        }
        deck.insert(Card::Number(7), 20);
        deck.insert(Card::Number(8), 20);
        deck.insert(Card::Number(9), 20);
        deck
    };
    
    let unfavorable_deck = {
        let mut deck = CardCounts::new();
        deck.insert(Card::Ace, 10);
        deck.insert(Card::Face, 15);
        deck.insert(Card::Number(10), 10);
        for n in 2..=6 {
            deck.insert(Card::Number(n), 50);
        }
        deck.insert(Card::Number(7), 30);
        deck.insert(Card::Number(8), 30);
        deck.insert(Card::Number(9), 30);
        deck
    };
    
    let calculator = Calculator::with_default_rules();
    let favorable_result = calculator.calculate_table_ev(&favorable_deck);
    let unfavorable_result = calculator.calculate_table_ev(&unfavorable_deck);
    
    println!("有利牌组 EV: {:.6}", favorable_result.ev);
    println!("不利牌组 EV: {:.6}", unfavorable_result.ev);
    println!("EV 差异: {:.6}", favorable_result.ev - unfavorable_result.ev);
    println!("有利牌组 - 玩家胜率: {:.6}, 庄家胜率: {:.6}", 
        favorable_result.player_win_prob, favorable_result.dealer_win_prob);
    println!("不利牌组 - 玩家胜率: {:.6}, 庄家胜率: {:.6}", 
        unfavorable_result.player_win_prob, unfavorable_result.dealer_win_prob);
    println!("有利牌组 - 玩家黑杰克率: {:.6}, 庄家黑杰克率: {:.6}", 
        favorable_result.player_blackjack_prob, favorable_result.dealer_blackjack_prob);
    println!("不利牌组 - 玩家黑杰克率: {:.6}, 庄家黑杰克率: {:.6}", 
        unfavorable_result.player_blackjack_prob, unfavorable_result.dealer_blackjack_prob);
    
    // 有利牌组的 EV 应该明显高于不利牌组（这是最重要的指标）
    assert!(favorable_result.ev > unfavorable_result.ev,
        "有利牌组 EV 应该明显高于不利牌组。有利牌组 EV: {}, 不利牌组 EV: {}",
        favorable_result.ev, unfavorable_result.ev);
    
    // 有利牌组的玩家黑杰克概率应该更高
    assert!(favorable_result.player_blackjack_prob > unfavorable_result.player_blackjack_prob,
        "有利牌组玩家黑杰克概率应该更高。有利牌组: {}, 不利牌组: {}",
        favorable_result.player_blackjack_prob, unfavorable_result.player_blackjack_prob);
    
    // 注意：玩家胜率可能不一定更高，因为有利牌组时庄家黑杰克概率也高
    // 但净胜率（玩家胜率 - 庄家胜率）应该更高
    let favorable_net = favorable_result.player_win_prob - favorable_result.dealer_win_prob;
    let unfavorable_net = unfavorable_result.player_win_prob - unfavorable_result.dealer_win_prob;
    
    assert!(favorable_net > unfavorable_net,
        "有利牌组净胜率应该更高。有利牌组净胜率: {}, 不利牌组净胜率: {}",
        favorable_net, unfavorable_net);
}

