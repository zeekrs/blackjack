//! 概率计算核心（优化版：分层计算 + 组合数学）

use crate::types::{
    Card, CardCounts, GameOutcome, Hand, PlayingCard, PointCounts, Suit,
    card_counts_to_point_counts, point_index_to_value,
};
use crate::rules::{DealerRules, GameRules};
use crate::strategy::BasicStrategy;

/// 概率计算器
pub struct ProbabilityCalculator {
    rules: GameRules,
    strategy: BasicStrategy,
    memo: std::collections::HashMap<(u8, bool, usize, u8, u64), GameOutcome>,
}

impl ProbabilityCalculator {
    /// 创建新的概率计算器
    pub fn new(rules: GameRules) -> Self {
        Self {
            rules,
            strategy: BasicStrategy::new(),
            memo: std::collections::HashMap::new(),
        }
    }

    /// 计算上桌 EV（主入口）
    pub fn calculate_table_ev(&mut self, deck: &CardCounts) -> GameOutcome {
        self.memo.clear();
        
        let point_counts = card_counts_to_point_counts(deck);
        let total_cards: u32 = point_counts.iter().sum();
        
        if total_cards < 4 {
            return GameOutcome::zero();
        }

        let mut total_outcome = GameOutcome::zero();
        
        // 优化策略：分层计算
        // 1. 遍历所有可能的玩家初始手牌组合（按点数分组）
        // 2. 对每种玩家手牌，遍历所有可能的庄家明牌
        // 3. 对每种组合，计算所有可能的庄家暗牌
        self.calculate_layered(&point_counts, total_cards, &mut total_outcome);
        
        total_outcome
    }

    /// 分层计算：先玩家手牌，再庄家明牌，最后庄家暗牌
    fn calculate_layered(
        &mut self,
        counts: &PointCounts,
        total_cards: u32,
        outcome: &mut GameOutcome,
    ) {
        // 第一层：玩家第一张牌
        for p1_idx in 0..=10 {
            if counts[p1_idx] == 0 {
                continue;
            }
            let p1_count = counts[p1_idx];
            let p1_value = point_index_to_value(p1_idx);
            let prob1 = p1_count as f64 / total_cards as f64;
            
            let mut counts_after_p1 = *counts;
            counts_after_p1[p1_idx] -= 1;
            let remaining_after_p1 = total_cards - 1;
            
            // 第二层：庄家明牌
            for d_up_idx in 0..=10 {
                if counts_after_p1[d_up_idx] == 0 {
                    continue;
                }
                let d_up_count = counts_after_p1[d_up_idx];
                let d_up_value = point_index_to_value(d_up_idx);
                let dealer_up_card = if d_up_idx == 0 {
                    Card::Ace
                } else if d_up_idx == 10 {
                    Card::Face
                } else {
                    Card::Number(d_up_idx as u8)
                };
                let prob2 = d_up_count as f64 / remaining_after_p1 as f64;
                
                let mut counts_after_d_up = counts_after_p1;
                counts_after_d_up[d_up_idx] -= 1;
                let remaining_after_d_up = remaining_after_p1 - 1;
                
                // 第三层：玩家第二张牌
                for p2_idx in 0..=10 {
                    if counts_after_d_up[p2_idx] == 0 {
                        continue;
                    }
                    let p2_count = counts_after_d_up[p2_idx];
                    let p2_value = point_index_to_value(p2_idx);
                    let prob3 = p2_count as f64 / remaining_after_d_up as f64;
                    
                    let mut counts_after_p2 = counts_after_d_up;
                    counts_after_p2[p2_idx] -= 1;
                    let remaining_after_p2 = remaining_after_d_up - 1;
                    
                    // 第四层：庄家暗牌
                    for d_hidden_idx in 0..=10 {
                        if counts_after_p2[d_hidden_idx] == 0 {
                            continue;
                        }
                        let d_hidden_count = counts_after_p2[d_hidden_idx];
                        let d_hidden_value = point_index_to_value(d_hidden_idx);
                        let prob4 = d_hidden_count as f64 / remaining_after_p2 as f64;
                        
                        let initial_prob = prob1 * prob2 * prob3 * prob4;
                        
                        // 创建剩余牌组
                        let mut new_counts = counts_after_p2;
                        new_counts[d_hidden_idx] -= 1;
                        
                        // 创建实际手牌对象（用于策略查询）
                        let player_hand = self.create_hand_from_values(p1_value, p2_value);
                        let dealer_hand = self.create_hand_from_values(d_up_value, d_hidden_value);
                        
                        // 计算该初始组合的游戏结果
                        let sub_outcome = self.calculate_game_outcome(
                            &player_hand,
                            &dealer_hand,
                            dealer_up_card,
                            &new_counts,
                        );
                        
                        outcome.add(&sub_outcome, initial_prob);
                    }
                }
            }
        }
    }

    /// 从两张牌的值创建手牌对象
    fn create_hand_from_values(&self, card1: u8, card2: u8) -> Hand {
        let mut hand = Hand::new();
        hand.add_card(PlayingCard {
            card: if card1 == 1 {
                Card::Ace
            } else if card1 == 10 {
                Card::Face
            } else {
                Card::Number(card1)
            },
            suit: Suit::Spades,
        });
        hand.add_card(PlayingCard {
            card: if card2 == 1 {
                Card::Ace
            } else if card2 == 10 {
                Card::Face
            } else {
                Card::Number(card2)
            },
            suit: Suit::Spades,
        });
        hand
    }


    /// 递归计算游戏结果（返回条件概率）
    fn calculate_game_outcome(
        &mut self,
        player_hand: &Hand,
        dealer_hand: &Hand,
        dealer_up_card: Card,
        counts: &PointCounts,
    ) -> GameOutcome {
        // 检查玩家黑杰克
        if player_hand.is_blackjack() {
            if dealer_hand.is_blackjack() {
                return GameOutcome {
                    push_prob: 1.0,
                    ..GameOutcome::zero()
                };
            } else {
                return GameOutcome {
                    player_blackjack_prob: 1.0,
                    ..GameOutcome::zero()
                };
            }
        }
        
        // 检查庄家黑杰克
        if dealer_hand.is_blackjack() {
            return GameOutcome {
                dealer_blackjack_prob: 1.0,
                ..GameOutcome::zero()
            };
        }
        
        // 检查玩家是否爆牌
        if player_hand.is_busted() {
            return GameOutcome {
                dealer_win_prob: 1.0,
                ..GameOutcome::zero()
            };
        }
        
        // 根据基础策略决定玩家动作
        // 注意：上桌EV计算不考虑分牌，即使规则允许
        let mut action = self.strategy.get_action(player_hand, dealer_up_card);
        
        // 强制忽略分牌（上桌EV计算不需要考虑分牌）
        if action == crate::types::Action::Split {
            action = self.strategy.get_action_without_split_surrender(player_hand, dealer_up_card);
        }
        
        // 如果策略是投降，但规则不允许，则回退到其他策略
        if action == crate::types::Action::Surrender && (!self.rules.allow_surrender || !player_hand.can_double()) {
            // 不允许投降，查询其他策略（Hit/Stand/Double）
            action = self.strategy.get_action_without_split_surrender(player_hand, dealer_up_card);
        }
        
        match action {
            crate::types::Action::Stand => {
                self.dealer_play_outcome(dealer_hand, player_hand, counts)
            }
            crate::types::Action::Hit => {
                self.player_hit_outcome(player_hand, dealer_hand, dealer_up_card, counts)
            }
            crate::types::Action::Double => {
                // 加倍：投注翻倍，所以结果需要标记为加倍
                let mut double_outcome = self.player_double_outcome(player_hand, dealer_hand, counts);
                // 将普通投注的概率转移到加倍投注
                double_outcome.player_win_prob_double = double_outcome.player_win_prob;
                double_outcome.dealer_win_prob_double = double_outcome.dealer_win_prob;
                double_outcome.push_prob_double = double_outcome.push_prob;
                double_outcome.player_blackjack_prob_double = double_outcome.player_blackjack_prob;
                double_outcome.dealer_blackjack_prob_double = double_outcome.dealer_blackjack_prob;
                // 清空普通投注的概率（因为已经加倍）
                double_outcome.player_win_prob = 0.0;
                double_outcome.dealer_win_prob = 0.0;
                double_outcome.push_prob = 0.0;
                double_outcome.player_blackjack_prob = 0.0;
                double_outcome.dealer_blackjack_prob = 0.0;
                double_outcome
            }
            crate::types::Action::Split => {
                // 分牌：上桌EV计算不考虑分牌，这里不应该到达
                // 如果到达这里，说明逻辑有误，回退到普通策略
                self.dealer_play_outcome(dealer_hand, player_hand, counts)
            }
            crate::types::Action::Surrender => {
                // 投降：损失0.5倍投注
                GameOutcome {
                    surrender_prob: 1.0,
                    ..GameOutcome::zero()
                }
            }
        }
    }

    /// 玩家要牌后的结果
    fn player_hit_outcome(
        &mut self,
        player_hand: &Hand,
        dealer_hand: &Hand,
        dealer_up_card: Card,
        counts: &PointCounts,
    ) -> GameOutcome {
        let mut outcome = GameOutcome::zero();
        let total_cards: u32 = counts.iter().sum();
        
        if total_cards == 0 {
            return outcome;
        }
        
        for point_idx in 0..=10 {
            if counts[point_idx] == 0 {
                continue;
            }
            
            let prob = counts[point_idx] as f64 / total_cards as f64;
            
            let card = if point_idx == 0 {
                Card::Ace
            } else if point_idx == 10 {
                Card::Face
            } else {
                Card::Number(point_idx as u8)
            };
            
            let mut new_player_hand = player_hand.clone();
            new_player_hand.add_card(PlayingCard {
                card,
                suit: Suit::Spades,
            });
            
            // 检查玩家是否爆牌
            if new_player_hand.is_busted() {
                outcome.add(
                    &GameOutcome {
                        dealer_win_prob: 1.0,
                        ..GameOutcome::zero()
                    },
                    prob,
                );
                continue;
            }
            
            let mut new_counts = *counts;
            new_counts[point_idx] -= 1;
            
            // 递归计算
            let sub_outcome = self.calculate_game_outcome(
                &new_player_hand,
                dealer_hand,
                dealer_up_card,
                &new_counts,
            );
            
            outcome.add(&sub_outcome, prob);
        }
        
        outcome
    }

    /// 玩家加倍后的结果
    fn player_double_outcome(
        &mut self,
        player_hand: &Hand,
        dealer_hand: &Hand,
        counts: &PointCounts,
    ) -> GameOutcome {
        let mut outcome = GameOutcome::zero();
        let total_cards: u32 = counts.iter().sum();
        
        if total_cards == 0 {
            return outcome;
        }
        
        for point_idx in 0..=10 {
            if counts[point_idx] == 0 {
                continue;
            }
            
            let prob = counts[point_idx] as f64 / total_cards as f64;
            
            let card = if point_idx == 0 {
                Card::Ace
            } else if point_idx == 10 {
                Card::Face
            } else {
                Card::Number(point_idx as u8)
            };
            
            let mut new_player_hand = player_hand.clone();
            new_player_hand.add_card(PlayingCard {
                card,
                suit: Suit::Spades,
            });
            
            // 检查玩家是否爆牌
            if new_player_hand.is_busted() {
                outcome.add(
                    &GameOutcome {
                        dealer_win_prob: 1.0,
                        ..GameOutcome::zero()
                    },
                    prob,
                );
                continue;
            }
            
            let mut new_counts = *counts;
            new_counts[point_idx] -= 1;
            
            // 玩家停牌，庄家回合
            let dealer_outcome = self.dealer_play_outcome(
                dealer_hand,
                &new_player_hand,
                &new_counts,
            );
            
            outcome.add(&dealer_outcome, prob);
        }
        
        outcome
    }

    /// 庄家回合的结果
    fn dealer_play_outcome(
        &mut self,
        dealer_hand: &Hand,
        player_hand: &Hand,
        counts: &PointCounts,
    ) -> GameOutcome {
        // 检查记忆化
        let player_value = player_hand.value();
        let player_is_soft = player_hand.is_soft();
        let player_card_count = player_hand.card_count();
        let dealer_value = dealer_hand.value();
        let deck_sig = self.calculate_deck_signature(counts);
        
        let memo_key = (
            player_value,
            player_is_soft,
            player_card_count,
            dealer_value,
            deck_sig,
        );
        
        if let Some(cached) = self.memo.get(&memo_key) {
            return cached.clone();
        }
        
        // 判断庄家是否需要要牌
        if !DealerRules::should_hit(dealer_hand, self.rules.dealer_stands_on_soft_17) {
            let result = self.compare_hands(player_hand, dealer_hand);
            self.memo.insert(memo_key, result.clone());
            return result;
        }
        
        // 庄家要牌
        let mut outcome = GameOutcome::zero();
        let total_cards: u32 = counts.iter().sum();
        
        if total_cards == 0 {
            return outcome;
        }
        
        for point_idx in 0..=10 {
            if counts[point_idx] == 0 {
                continue;
            }
            
            let prob = counts[point_idx] as f64 / total_cards as f64;
            
            let card = if point_idx == 0 {
                Card::Ace
            } else if point_idx == 10 {
                Card::Face
            } else {
                Card::Number(point_idx as u8)
            };
            
            let mut new_dealer_hand = dealer_hand.clone();
            new_dealer_hand.add_card(PlayingCard {
                card,
                suit: Suit::Spades,
            });
            
            // 检查庄家是否爆牌
            if new_dealer_hand.is_busted() {
                outcome.add(
                    &GameOutcome {
                        player_win_prob: 1.0,
                        ..GameOutcome::zero()
                    },
                    prob,
                );
                continue;
            }
            
            let mut new_counts = *counts;
            new_counts[point_idx] -= 1;
            
            let sub_outcome = self.dealer_play_outcome(
                &new_dealer_hand,
                player_hand,
                &new_counts,
            );
            
            outcome.add(&sub_outcome, prob);
        }
        
        self.memo.insert(memo_key, outcome.clone());
        outcome
    }

    /// 比较玩家和庄家手牌，返回结果
    fn compare_hands(&self, player_hand: &Hand, dealer_hand: &Hand) -> GameOutcome {
        let player_value = player_hand.value();
        let dealer_value = dealer_hand.value();
        
        if player_value > dealer_value {
            GameOutcome {
                player_win_prob: 1.0,
                ..GameOutcome::zero()
            }
        } else if player_value < dealer_value {
            GameOutcome {
                dealer_win_prob: 1.0,
                ..GameOutcome::zero()
            }
        } else {
            GameOutcome {
                push_prob: 1.0,
                ..GameOutcome::zero()
            }
        }
    }

    /// 计算牌组签名（用于记忆化）
    fn calculate_deck_signature(&self, counts: &PointCounts) -> u64 {
        let mut hash: u64 = 0;
        for &count in counts.iter() {
            hash = hash.wrapping_mul(31).wrapping_add(count as u64);
        }
        hash
    }
}
