//! Blackjack 类型定义

use serde::{Deserialize, Serialize};

/// 牌面值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Card {
    /// A (Ace)
    Ace,
    /// 2-10
    Number(u8),
    /// J, Q, K
    Face,
}

/// 花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

/// 完整牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayingCard {
    pub card: Card,
    pub suit: Suit,
}

/// 玩家动作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    /// 要牌 (Hit)
    Hit,
    /// 停牌 (Stand)
    Stand,
    /// 加倍 (Double Down)
    Double,
    /// 分牌 (Split)
    Split,
    /// 投降 (Surrender)
    Surrender,
}

/// 游戏结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameResult {
    /// 玩家获胜
    PlayerWin,
    /// 庄家获胜
    DealerWin,
    /// 平局
    Push,
    /// 玩家黑杰克
    PlayerBlackjack,
    /// 庄家黑杰克
    DealerBlackjack,
}

/// 手牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<PlayingCard>,
}

impl Hand {
    /// 创建空手牌
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// 添加牌
    pub fn add_card(&mut self, card: PlayingCard) {
        self.cards.push(card);
    }

    /// 计算手牌点数
    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            match card.card {
                Card::Ace => {
                    aces += 1;
                    total += 11;
                }
                Card::Number(n) => total += n,
                Card::Face => total += 10,
            }
        }

        // 处理 A 的软硬点数
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }

    /// 是否为黑杰克 (21点，两张牌)
    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    /// 是否爆牌 (超过21点)
    pub fn is_busted(&self) -> bool {
        self.value() > 21
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

