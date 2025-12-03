//! Blackjack 计算器核心逻辑

use crate::rules::GameRules;
use crate::types::{CardCounts, TableEVResult};
use crate::probability_calculator::ProbabilityCalculator;
use crate::ev_calculator::calculate_ev;

/// Blackjack 计算器
pub struct Calculator {
    rules: GameRules,
}

impl Calculator {
    /// 创建新的计算器实例
    pub fn new(rules: GameRules) -> Self {
        Self { rules }
    }

    /// 使用默认规则创建计算器
    pub fn with_default_rules() -> Self {
        Self {
            rules: GameRules::default(),
        }
    }

    /// 计算上桌 EV（是否上桌的期望值）
    /// 
    /// 综合考虑所有可能的游戏情况：
    /// - 普通投注（Hit/Stand）
    /// - 加倍投注（Double Down）
    /// - 投降（Surrender）
    /// 
    /// 玩家按照基础策略进行游戏，最终返回一个综合的 EV 值。
    /// 
    /// # Arguments
    /// * `deck` - 当前剩余牌组信息
    /// 
    /// # Returns
    /// `TableEVResult` 包含期望值和各种概率
    pub fn calculate_table_ev(&self, deck: &CardCounts) -> TableEVResult {
        let mut calculator = ProbabilityCalculator::new(self.rules.clone());
        let outcome = calculator.calculate_table_ev(deck);
        calculate_ev(&outcome, &self.rules)
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::with_default_rules()
    }
}

/// 创建完整 8 副牌
pub fn create_full_8_deck() -> CardCounts {
    let mut deck = CardCounts::new();
    
    // 8副牌，每副52张
    for _ in 0..8 {
        // A
        *deck.entry(crate::types::Card::Ace).or_insert(0) += 4;
        // 2-10
        for n in 2..=10 {
            *deck.entry(crate::types::Card::Number(n)).or_insert(0) += 4;
        }
        // J, Q, K
        *deck.entry(crate::types::Card::Face).or_insert(0) += 12; // 4张 * 3种
    }
    
    deck
}

