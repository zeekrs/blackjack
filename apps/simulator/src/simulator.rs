//! Blackjack 模拟器核心逻辑

use calculator::{Calculator, rules::GameRules};
use anyhow::Result;

/// 模拟器配置
#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    /// 游戏规则
    pub rules: GameRules,
    /// 模拟局数
    pub rounds: u64,
    /// 并发线程数
    pub threads: usize,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            rules: GameRules::default(),
            rounds: 1000000,
            threads: num_cpus::get(),
        }
    }
}

/// 模拟器
pub struct Simulator {
    config: SimulatorConfig,
    calculator: Calculator,
}

impl Simulator {
    /// 创建新的模拟器实例
    pub fn new(config: SimulatorConfig) -> Self {
        let calculator = Calculator::new(config.rules.clone());
        Self {
            config,
            calculator,
        }
    }

    /// 运行模拟
    pub fn run(&self) -> Result<SimulationResult> {
        // TODO: 实现模拟逻辑
        Ok(SimulationResult::default())
    }
}

/// 模拟结果
#[derive(Debug, Clone, Default)]
pub struct SimulationResult {
    /// 总局数
    pub total_rounds: u64,
    /// 玩家获胜局数
    pub player_wins: u64,
    /// 庄家获胜局数
    pub dealer_wins: u64,
    /// 平局局数
    pub pushes: u64,
    /// 总投注金额
    pub total_bet: f64,
    /// 总收益
    pub total_profit: f64,
    /// 期望值
    pub expected_value: f64,
}

