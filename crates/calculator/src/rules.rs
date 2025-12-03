//! Blackjack 游戏规则定义

use serde::{Deserialize, Serialize};

/// 游戏规则配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRules {
    /// 牌组数量
    pub deck_count: u8,
    /// 是否允许投降
    pub allow_surrender: bool,
    /// 是否允许分牌
    pub allow_split: bool,
    /// 庄家是否在软17停牌
    pub dealer_stands_on_soft_17: bool,
    /// 分牌后是否可以再次分牌
    pub allow_resplit: bool,
    /// 分牌后是否可以加倍
    pub allow_double_after_split: bool,
    /// 黑杰克赔率 (通常是 3:2 或 6:5)
    pub blackjack_payout: f64,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            deck_count: 6,
            allow_surrender: true,
            allow_split: true,
            dealer_stands_on_soft_17: true,
            allow_resplit: true,
            allow_double_after_split: true,
            blackjack_payout: 1.5, // 3:2
        }
    }
}

