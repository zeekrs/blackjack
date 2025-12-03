//! Blackjack 计算器核心库
//!
//! 提供 Blackjack 游戏的概率计算、期望值计算等功能

pub mod types;
pub mod rules;
pub mod strategy;
pub mod probability_calculator;
pub mod ev_calculator;
pub mod calculator;

pub use calculator::{Calculator, create_full_8_deck};
pub use types::*;
pub use rules::GameRules;

