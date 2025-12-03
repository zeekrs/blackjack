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

    /// 判断是否为软点数
    /// 软点数：手牌中有 A 且 A 被当作 11 使用时
    pub fn is_soft(&self) -> bool {
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

        // 如果有 A 且总点数 <= 21，说明是软点数
        aces > 0 && total <= 21
    }

    /// 是否为黑杰克 (21点，两张牌)
    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    /// 是否爆牌 (超过21点)
    pub fn is_busted(&self) -> bool {
        self.value() > 21
    }

    /// 获取牌数
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// 是否可以加倍（通常只能在初始两张牌时）
    pub fn can_double(&self) -> bool {
        self.cards.len() == 2
    }

    /// 是否可以分牌（两张牌且点数相同）
    pub fn can_split(&self) -> bool {
        if self.cards.len() != 2 {
            return false;
        }
        // 检查两张牌的点数是否相同
        let card1_value = self.card_to_point_value(self.cards[0].card);
        let card2_value = self.card_to_point_value(self.cards[1].card);
        card1_value == card2_value
    }

    /// 获取第一张牌（用于分牌）
    pub fn first_card(&self) -> Option<PlayingCard> {
        self.cards.first().copied()
    }

    /// 获取第二张牌（用于分牌）
    pub fn second_card(&self) -> Option<PlayingCard> {
        if self.cards.len() >= 2 {
            Some(self.cards[1])
        } else {
            None
        }
    }

    /// 将牌转换为点数（用于分牌判断）
    fn card_to_point_value(&self, card: Card) -> u8 {
        match card {
            Card::Ace => 1, // 分牌时，A 和 A 可以分，10/J/Q/K 可以分
            Card::Number(n) => n,
            Card::Face => 10,
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

/// 牌组计数（用于算牌）
/// Key: 牌面值（不区分花色，只区分点数）
/// Value: 剩余数量
pub type CardCounts = std::collections::HashMap<Card, u32>;

/// 点数计数数组（用于高效概率计算）
/// 索引：0=A(作为1点), 1=A(作为11点), 2-10=对应点数, 11=10点牌(J/Q/K)
/// 为了简化，我们使用：0=A, 1=2, 2=3, ..., 9=10, 10=10点牌(J/Q/K)
/// 实际上可以合并：0=A, 1=2, ..., 9=10, 10=10点牌
/// 更简单的方式：0=A, 1=2, 2=3, ..., 9=10, 10=10点牌（J/Q/K）
/// 但为了与百家乐一致，我们使用：0=A, 1=2, ..., 9=10
pub type PointCounts = [u32; 11]; // 0=A, 1=2, 2=3, ..., 9=10, 10=10点牌(J/Q/K)

/// 将牌转换为点数索引（用于 PointCounts）
/// 返回 (点数索引, 是否为A)
pub fn card_to_point_index(card: Card) -> (usize, bool) {
    match card {
        Card::Ace => (0, true),
        Card::Number(n) => {
            if n == 10 {
                (9, false) // 10点
            } else {
                (n as usize, false)
            }
        }
        Card::Face => (10, false), // J/Q/K 作为10点牌
    }
}

/// 将点数索引转换为实际点数
pub fn point_index_to_value(index: usize) -> u8 {
    match index {
        0 => 1,  // A
        1..=9 => index as u8, // 2-10
        10 => 10, // J/Q/K
        _ => 0,
    }
}

/// 将牌转换为点数（用于概率计算）
/// A = 1, 2-10 = 2-10, J/Q/K = 10
pub fn card_to_point(card: Card) -> u8 {
    match card {
        Card::Ace => 1,
        Card::Number(n) => n,
        Card::Face => 10,
    }
}

/// 将 CardCounts 转换为 PointCounts
pub fn card_counts_to_point_counts(card_counts: &CardCounts) -> PointCounts {
    let mut point_counts: PointCounts = [0; 11];
    for (&card, &count) in card_counts {
        let (index, _) = card_to_point_index(card);
        point_counts[index] += count;
    }
    point_counts
}

/// 游戏结果概率分布
#[derive(Debug, Clone, Default)]
pub struct GameOutcome {
    /// 玩家获胜概率（普通投注）
    pub player_win_prob: f64,
    /// 庄家获胜概率（普通投注）
    pub dealer_win_prob: f64,
    /// 平局概率（普通投注）
    pub push_prob: f64,
    /// 玩家黑杰克概率（普通投注）
    pub player_blackjack_prob: f64,
    /// 庄家黑杰克概率（普通投注）
    pub dealer_blackjack_prob: f64,
    /// 玩家获胜概率（加倍投注）
    pub player_win_prob_double: f64,
    /// 庄家获胜概率（加倍投注）
    pub dealer_win_prob_double: f64,
    /// 平局概率（加倍投注）
    pub push_prob_double: f64,
    /// 玩家黑杰克概率（加倍投注，加倍时通常不能有黑杰克）
    pub player_blackjack_prob_double: f64,
    /// 庄家黑杰克概率（加倍投注）
    pub dealer_blackjack_prob_double: f64,
    /// 投降概率（损失0.5倍投注）
    pub surrender_prob: f64,
}

impl GameOutcome {
    /// 创建零结果
    pub fn zero() -> Self {
        Self::default()
    }

    /// 累加另一个结果（普通投注）
    pub fn add(&mut self, other: &GameOutcome, weight: f64) {
        self.player_win_prob += other.player_win_prob * weight;
        self.dealer_win_prob += other.dealer_win_prob * weight;
        self.push_prob += other.push_prob * weight;
        self.player_blackjack_prob += other.player_blackjack_prob * weight;
        self.dealer_blackjack_prob += other.dealer_blackjack_prob * weight;
        // 同时累加加倍投注的概率
        self.player_win_prob_double += other.player_win_prob_double * weight;
        self.dealer_win_prob_double += other.dealer_win_prob_double * weight;
        self.push_prob_double += other.push_prob_double * weight;
        self.player_blackjack_prob_double += other.player_blackjack_prob_double * weight;
        self.dealer_blackjack_prob_double += other.dealer_blackjack_prob_double * weight;
        // 累加投降概率
        self.surrender_prob += other.surrender_prob * weight;
    }
    
    /// 累加加倍投注的结果
    pub fn add_double(&mut self, other: &GameOutcome, weight: f64) {
        self.player_win_prob_double += other.player_win_prob * weight;
        self.dealer_win_prob_double += other.dealer_win_prob * weight;
        self.push_prob_double += other.push_prob * weight;
        self.player_blackjack_prob_double += other.player_blackjack_prob * weight;
        self.dealer_blackjack_prob_double += other.dealer_blackjack_prob * weight;
    }

    /// 归一化（确保概率总和为1）
    pub fn normalize(&mut self) {
        let total = self.player_win_prob
            + self.dealer_win_prob
            + self.push_prob
            + self.player_blackjack_prob
            + self.dealer_blackjack_prob
            + self.player_win_prob_double
            + self.dealer_win_prob_double
            + self.push_prob_double
            + self.player_blackjack_prob_double
            + self.dealer_blackjack_prob_double
            + self.surrender_prob;
        if total > 0.0 {
            self.player_win_prob /= total;
            self.dealer_win_prob /= total;
            self.push_prob /= total;
            self.player_blackjack_prob /= total;
            self.dealer_blackjack_prob /= total;
            self.player_win_prob_double /= total;
            self.dealer_win_prob_double /= total;
            self.push_prob_double /= total;
            self.player_blackjack_prob_double /= total;
            self.dealer_blackjack_prob_double /= total;
            self.surrender_prob /= total;
        }
    }
}

/// 上桌 EV 计算结果
#[derive(Debug, Clone)]
pub struct TableEVResult {
    /// 总期望值（EV）
    pub ev: f64,
    /// 普通投注的 EV
    pub ev_normal: f64,
    /// 加倍投注的 EV
    pub ev_double: f64,
    /// 投降的 EV
    pub ev_surrender: f64,
    /// 玩家获胜概率
    pub player_win_prob: f64,
    /// 庄家获胜概率
    pub dealer_win_prob: f64,
    /// 平局概率
    pub push_prob: f64,
    /// 玩家黑杰克概率
    pub player_blackjack_prob: f64,
    /// 庄家黑杰克概率
    pub dealer_blackjack_prob: f64,
    /// 投降概率
    pub surrender_prob: f64,
}

