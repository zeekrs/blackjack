//! 边界情况和特殊手牌测试

use calculator::{Calculator, Card, CardCounts, GameRules};
use std::collections::HashMap;

/// 创建最小牌组（4张牌）
fn create_minimal_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 1);
    deck.insert(Card::Number(10), 1);
    deck.insert(Card::Face, 1);
    deck.insert(Card::Number(9), 1);
    deck
}

/// 创建只有高牌的牌组
fn create_high_card_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 只有 10, J, Q, K
    deck.insert(Card::Face, 20);
    deck.insert(Card::Number(10), 20);
    deck
}

/// 创建只有低牌的牌组
fn create_low_card_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    // 只有 2-6
    for n in 2..=6 {
        deck.insert(Card::Number(n), 10);
    }
    deck
}

/// 创建只有A的牌组
fn create_ace_only_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 20);
    deck
}

#[test]
fn test_minimal_deck() {
    let deck = create_minimal_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    // 验证概率总和接近1
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
}

#[test]
fn test_high_card_deck() {
    let deck = create_high_card_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    // 高牌较多时，虽然没有A无法组成黑杰克，但应该有游戏结果
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
fn test_low_card_deck() {
    let deck = create_low_card_deck();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    // 低牌较多时，玩家黑杰克概率应该较低（因为没有10点牌）
    // 但应该仍然有游戏结果
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
}

#[test]
fn test_empty_deck() {
    let deck = CardCounts::new();
    let calculator = Calculator::with_default_rules();
    
    let result = calculator.calculate_table_ev(&deck);
    
    // 空牌组应该返回零结果
    assert_eq!(result.ev, 0.0);
    assert_eq!(result.player_win_prob, 0.0);
    assert_eq!(result.dealer_win_prob, 0.0);
}

#[test]
fn test_single_deck() {
    let mut deck = CardCounts::new();
    // 1副牌
    deck.insert(Card::Ace, 4);
    for n in 2..=10 {
        deck.insert(Card::Number(n), 4);
    }
    deck.insert(Card::Face, 12); // J, Q, K
    
    let calculator = Calculator::with_default_rules();
    let result = calculator.calculate_table_ev(&deck);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
    
    // 单副牌的EV应该也是负数（庄家优势）
    assert!(result.ev < 0.0);
}

#[test]
fn test_rules_no_surrender() {
    let deck = create_minimal_deck();
    let mut rules = GameRules::default();
    rules.allow_surrender = false;
    
    let calculator = Calculator::new(rules);
    let result = calculator.calculate_table_ev(&deck);
    
    // 不允许投降时，投降概率应该为0
    assert_eq!(result.surrender_prob, 0.0);
    
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
fn test_rules_soft_17() {
    let deck = create_minimal_deck();
    let mut rules = GameRules::default();
    rules.dealer_stands_on_soft_17 = true; // 软17停牌
    
    let calculator = Calculator::new(rules);
    let result = calculator.calculate_table_ev(&deck);
    
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
fn test_rules_hard_17() {
    let deck = create_minimal_deck();
    let mut rules = GameRules::default();
    rules.dealer_stands_on_soft_17 = false; // 硬17停牌（软17继续要牌）
    
    let calculator = Calculator::new(rules);
    let result = calculator.calculate_table_ev(&deck);
    
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
fn test_blackjack_payout_6_5() {
    let deck = create_minimal_deck();
    let mut rules = GameRules::default();
    rules.blackjack_payout = 1.2; // 6:5 = 1.2
    
    let calculator = Calculator::new(rules);
    let result = calculator.calculate_table_ev(&deck);
    
    // 6:5 赔率下，EV应该更差（对玩家不利）
    let result_3_2 = Calculator::with_default_rules().calculate_table_ev(&deck);
    
    // 6:5 的 EV 应该比 3:2 更差
    assert!(result.ev < result_3_2.ev, "6:5 赔率应该比 3:2 赔率对玩家更不利");
    
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
fn test_very_small_deck() {
    // 测试只有6张牌的情况（最小可玩情况）
    let mut deck = CardCounts::new();
    deck.insert(Card::Ace, 2);
    deck.insert(Card::Number(10), 2);
    deck.insert(Card::Number(9), 2);
    
    let calculator = Calculator::with_default_rules();
    let result = calculator.calculate_table_ev(&deck);
    
    // 验证概率总和
    let total_prob = result.player_win_prob
        + result.dealer_win_prob
        + result.push_prob
        + result.player_blackjack_prob
        + result.dealer_blackjack_prob
        + result.surrender_prob;
    
    assert!((total_prob - 1.0).abs() < 0.01, "概率总和应该接近1，实际: {}", total_prob);
}

