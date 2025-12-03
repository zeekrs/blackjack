//! 统计计算模块

use crate::simulator::SimulationResult;

/// 统计计算器
pub struct Statistics {
    result: SimulationResult,
}

impl Statistics {
    /// 创建统计计算器
    pub fn new(result: SimulationResult) -> Self {
        Self { result }
    }

    /// 计算胜率
    pub fn win_rate(&self) -> f64 {
        if self.result.total_rounds == 0 {
            return 0.0;
        }
        self.result.player_wins as f64 / self.result.total_rounds as f64
    }

    /// 计算收益率
    pub fn return_rate(&self) -> f64 {
        if self.result.total_bet == 0.0 {
            return 0.0;
        }
        self.result.total_profit / self.result.total_bet
    }

    /// 计算标准差
    pub fn standard_deviation(&self) -> f64 {
        // TODO: 实现标准差计算
        0.0
    }
}

