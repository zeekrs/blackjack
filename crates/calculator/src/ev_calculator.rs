//! 期望值计算
//! 
//! 计算"上桌 EV"（是否上桌的期望值），综合考虑所有可能的游戏情况：
//! - 普通投注（Hit/Stand）
//! - 加倍投注（Double Down）
//! - 投降（Surrender）
//! 
//! 最终返回一个综合的 EV 值，表示玩家按照基础策略上桌后的期望收益。

use crate::types::{GameOutcome, TableEVResult};
use crate::rules::GameRules;

/// 根据游戏结果概率计算期望值
/// 
/// 计算"上桌 EV"，包含所有可能的游戏情况（普通投注、加倍、投降）
pub fn calculate_ev(
    outcome: &GameOutcome,
    rules: &GameRules,
) -> TableEVResult {
    // 普通投注的 EV：
    // EV_normal = P(玩家胜) × 1.0 + P(玩家黑杰克) × 1.5 - P(庄家胜) × 1.0 - P(庄家黑杰克) × 1.0
    // 平局退还本金，所以 EV = 0
    
    let ev_normal: f64 = outcome.player_win_prob * 1.0
        + outcome.player_blackjack_prob * rules.blackjack_payout
        - outcome.dealer_win_prob * 1.0
        - outcome.dealer_blackjack_prob * 1.0;
    
    // 加倍投注的 EV（投注翻倍，所以收益/损失也翻倍）：
    // EV_double = P(玩家胜) × 2.0 + P(玩家黑杰克) × 3.0 - P(庄家胜) × 2.0 - P(庄家黑杰克) × 2.0
    // 注意：加倍时通常不会有黑杰克（因为只能在前两张牌时加倍）
    let ev_double = outcome.player_win_prob_double * 2.0
        + outcome.player_blackjack_prob_double * (rules.blackjack_payout * 2.0)
        - outcome.dealer_win_prob_double * 2.0
        - outcome.dealer_blackjack_prob_double * 2.0;
    
    // 投降的 EV（损失0.5倍投注）
    let ev_surrender = outcome.surrender_prob * (-0.5);
    
    // 总 EV = 普通投注 EV + 加倍投注 EV + 投降 EV
    // 这是"上桌 EV"，综合考虑了所有可能的游戏情况
    let ev = ev_normal + ev_double + ev_surrender;
    
    // 合并概率（用于显示）
    let total_player_win_prob = outcome.player_win_prob + outcome.player_win_prob_double;
    let total_dealer_win_prob = outcome.dealer_win_prob + outcome.dealer_win_prob_double;
    let total_push_prob = outcome.push_prob + outcome.push_prob_double;
    let total_player_blackjack_prob = outcome.player_blackjack_prob + outcome.player_blackjack_prob_double;
    let total_dealer_blackjack_prob = outcome.dealer_blackjack_prob + outcome.dealer_blackjack_prob_double;
    // 注意：投降概率不包含在 TableEVResult 中，因为它不是游戏结果，而是玩家选择

    TableEVResult {
        ev,
        ev_normal,
        ev_double,
        ev_surrender,
        player_win_prob: total_player_win_prob,
        dealer_win_prob: total_dealer_win_prob,
        push_prob: total_push_prob,
        player_blackjack_prob: total_player_blackjack_prob,
        dealer_blackjack_prob: total_dealer_blackjack_prob,
        surrender_prob: outcome.surrender_prob,
    }
}

