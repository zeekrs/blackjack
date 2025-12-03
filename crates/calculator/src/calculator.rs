//! Blackjack 计算器核心逻辑

use crate::rules::GameRules;
use crate::types::{Action, GameResult, Hand};

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

    /// 计算期望值
    pub fn calculate_ev(&self, player_hand: &Hand, dealer_up_card: &Hand) -> f64 {
        // TODO: 实现期望值计算
        0.0
    }

    /// 计算最佳动作
    pub fn optimal_action(&self, player_hand: &Hand, dealer_up_card: &Hand) -> Action {
        // TODO: 实现最佳动作计算
        Action::Stand
    }

    /// 计算游戏结果
    pub fn calculate_result(&self, player_hand: &Hand, dealer_hand: &Hand) -> GameResult {
        // TODO: 实现结果计算
        GameResult::Push
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::with_default_rules()
    }
}

