//! 模拟器类型定义

use serde::{Deserialize, Serialize};

/// 投注类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetType {
    /// 普通投注
    Normal,
    /// 保险投注
    Insurance,
    /// 完美配对
    PerfectPair,
    /// 21+3
    TwentyOnePlusThree,
}

/// 投注记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetRecord {
    /// 投注类型
    pub bet_type: BetType,
    /// 投注金额
    pub amount: f64,
    /// 结果
    pub result: f64,
}

/// 单局游戏记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundRecord {
    /// 局号
    pub round_number: u64,
    /// 玩家手牌
    pub player_hand: Vec<u8>,
    /// 庄家手牌
    pub dealer_hand: Vec<u8>,
    /// 投注记录
    pub bets: Vec<BetRecord>,
    /// 净收益
    pub net_profit: f64,
}

