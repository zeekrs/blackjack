//! Blackjack 游戏规则定义

use crate::types::Hand;
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
            deck_count: 8,
            allow_surrender: true,  // 支持投降
            allow_split: false,     // 暂时禁用分牌（性能问题，待优化）
            dealer_stands_on_soft_17: false, // 默认硬17停牌
            allow_resplit: false,
            allow_double_after_split: false,
            blackjack_payout: 1.5, // 3:2
        }
    }
}

/// 庄家规则实现
pub struct DealerRules;

impl DealerRules {
    /// 判断庄家是否需要继续要牌
    /// 
    /// # Arguments
    /// * `dealer_hand` - 庄家手牌
    /// * `stands_on_soft_17` - 是否在软17停牌
    /// 
    /// # Returns
    /// `true` 表示需要要牌，`false` 表示停牌
    pub fn should_hit(dealer_hand: &Hand, stands_on_soft_17: bool) -> bool {
        let value = dealer_hand.value();
        
        // 如果已经爆牌，不需要要牌
        if value > 21 {
            return false;
        }
        
        // 如果点数 < 17，必须要牌
        if value < 17 {
            return true;
        }
        
        // 如果点数 > 17，必须停牌
        if value > 17 {
            return false;
        }
        
        // 如果点数是 17，需要判断软硬
        if value == 17 {
            if stands_on_soft_17 {
                // 软17停牌：无论软硬都停牌
                return false;
            } else {
                // 硬17停牌：如果是软17，继续要牌；如果是硬17，停牌
                return dealer_hand.is_soft();
            }
        }
        
        false
    }
}

