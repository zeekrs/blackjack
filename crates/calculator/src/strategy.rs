//! Blackjack 基础策略表

use crate::types::{Action, Hand, Card};
use std::collections::HashMap;

/// 基础策略表
/// Key: (玩家点数, 是否软点数, 玩家牌数, 庄家明牌点数)
/// Value: 最佳动作
type StrategyTable = HashMap<(u8, bool, usize, u8), Action>;

/// 分牌策略表
/// Key: (玩家点数, 庄家明牌点数)
/// Value: 是否分牌
type SplitTable = HashMap<(u8, u8), Action>;

/// 投降策略表
/// Key: (玩家点数, 庄家明牌点数)
/// Value: 是否投降
type SurrenderTable = HashMap<(u8, u8), Action>;

/// 基础策略
pub struct BasicStrategy {
    table: StrategyTable,
    split_table: SplitTable,
    surrender_table: SurrenderTable,
}

impl BasicStrategy {
    /// 创建基础策略实例
    pub fn new() -> Self {
        let mut table = StrategyTable::new();
        let mut split_table = SplitTable::new();
        let mut surrender_table = SurrenderTable::new();
        
        // 初始化基础策略表
        Self::init_strategy_table(&mut table);
        Self::init_split_table(&mut split_table);
        Self::init_surrender_table(&mut surrender_table);
        
        Self { 
            table,
            split_table,
            surrender_table,
        }
    }

    /// 根据当前状态查询最佳动作
    pub fn get_action(
        &self,
        player_hand: &Hand,
        dealer_up_card: Card,
    ) -> Action {
        let player_value = player_hand.value();
        let player_is_soft = player_hand.is_soft();
        let player_card_count = player_hand.card_count();
        let dealer_value = Self::card_to_value(dealer_up_card);

        // 注意：分牌和投降的检查在 probability_calculator 中根据 rules 决定
        // 这里只查询策略表，不做规则检查

        // 检查是否可以分牌（优先级最高）
        if player_hand.can_split() {
            if let Some(action) = self.split_table.get(&(player_value, dealer_value)) {
                if *action == Action::Split {
                    return Action::Split;
                }
            }
        }

        // 检查是否可以投降（只能在初始两张牌时）
        if player_hand.can_double() {
            if let Some(action) = self.surrender_table.get(&(player_value, dealer_value)) {
                if *action == Action::Surrender {
                    return Action::Surrender;
                }
            }
        }

        // 检查是否可以加倍
        if player_hand.can_double() {
            if let Some(action) = self.table.get(&(player_value, player_is_soft, 2, dealer_value)) {
                if *action == Action::Double {
                    return Action::Double;
                }
            }
        }

        // 查询要牌/停牌策略
        self.table
            .get(&(player_value, player_is_soft, player_card_count, dealer_value))
            .copied()
            .unwrap_or(Action::Stand)
    }

    /// 获取动作（跳过分牌和投降，只查询 Hit/Stand/Double）
    /// 用于回退策略，当分牌或投降不允许时
    pub fn get_action_without_split_surrender(
        &self,
        player_hand: &Hand,
        dealer_up_card: Card,
    ) -> Action {
        let player_value = player_hand.value();
        let player_is_soft = player_hand.is_soft();
        let player_card_count = player_hand.card_count();
        let dealer_value = Self::card_to_value(dealer_up_card);

        // 检查是否可以加倍
        if player_hand.can_double() {
            if let Some(action) = self.table.get(&(player_value, player_is_soft, 2, dealer_value)) {
                if *action == Action::Double {
                    return Action::Double;
                }
            }
        }

        // 查询要牌/停牌策略
        self.table
            .get(&(player_value, player_is_soft, player_card_count, dealer_value))
            .copied()
            .unwrap_or(Action::Stand)
    }

    /// 将牌转换为点数（用于策略查询）
    fn card_to_value(card: Card) -> u8 {
        match card {
            Card::Ace => 11, // 庄家明牌A当作11
            Card::Number(n) => n,
            Card::Face => 10,
        }
    }

    /// 初始化基础策略表
    /// 这是简化版本，只包含要牌/停牌/加倍策略
    fn init_strategy_table(table: &mut StrategyTable) {
        // 硬点数策略（非软点数）
        // (玩家点数, false=硬, 牌数, 庄家明牌)
        
        // 硬点数：5-8 总是要牌
        for player_value in 5..=8 {
            for dealer_value in 2..=11 {
                for card_count in 2..=10 {
                    table.insert((player_value, false, card_count, dealer_value), Action::Hit);
                }
            }
        }

        // 硬点数：9
        for dealer_value in 2..=6 {
            table.insert((9, false, 2, dealer_value), Action::Double);
        }
        for dealer_value in 7..=11 {
            table.insert((9, false, 2, dealer_value), Action::Hit);
        }
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((9, false, card_count, dealer_value), Action::Hit);
            }
        }

        // 硬点数：10
        for dealer_value in 2..=9 {
            table.insert((10, false, 2, dealer_value), Action::Double);
        }
        table.insert((10, false, 2, 10), Action::Hit);
        table.insert((10, false, 2, 11), Action::Hit);
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((10, false, card_count, dealer_value), Action::Hit);
            }
        }

        // 硬点数：11
        for dealer_value in 2..=10 {
            table.insert((11, false, 2, dealer_value), Action::Double);
        }
        table.insert((11, false, 2, 11), Action::Hit);
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((11, false, card_count, dealer_value), Action::Hit);
            }
        }

        // 硬点数：12
        for dealer_value in 4..=6 {
            table.insert((12, false, 2, dealer_value), Action::Stand);
        }
        for dealer_value in 2..=3 {
            table.insert((12, false, 2, dealer_value), Action::Hit);
        }
        for dealer_value in 7..=11 {
            table.insert((12, false, 2, dealer_value), Action::Hit);
        }
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((12, false, card_count, dealer_value), Action::Hit);
            }
        }

        // 硬点数：13-16
        for player_value in 13..=16 {
            for dealer_value in 2..=6 {
                table.insert((player_value, false, 2, dealer_value), Action::Stand);
            }
            for dealer_value in 7..=11 {
                table.insert((player_value, false, 2, dealer_value), Action::Hit);
            }
            for dealer_value in 2..=11 {
                for card_count in 3..=10 {
                    table.insert((player_value, false, card_count, dealer_value), Action::Hit);
                }
            }
        }

        // 硬点数：17-21 总是停牌
        for player_value in 17..=21 {
            for dealer_value in 2..=11 {
                for card_count in 2..=10 {
                    table.insert((player_value, false, card_count, dealer_value), Action::Stand);
                }
            }
        }

        // 软点数策略（有A且A当作11）
        // (玩家点数, true=软, 牌数, 庄家明牌)

        // 软13-14 (A,2 或 A,3)
        for player_value in 13..=14 {
            for dealer_value in 5..=6 {
                table.insert((player_value, true, 2, dealer_value), Action::Double);
            }
            for dealer_value in 2..=4 {
                table.insert((player_value, true, 2, dealer_value), Action::Hit);
            }
            for dealer_value in 7..=11 {
                table.insert((player_value, true, 2, dealer_value), Action::Hit);
            }
            for dealer_value in 2..=11 {
                for card_count in 3..=10 {
                    table.insert((player_value, true, card_count, dealer_value), Action::Hit);
                }
            }
        }

        // 软15-16 (A,4 或 A,5)
        for player_value in 15..=16 {
            for dealer_value in 4..=6 {
                table.insert((player_value, true, 2, dealer_value), Action::Double);
            }
            for dealer_value in 2..=3 {
                table.insert((player_value, true, 2, dealer_value), Action::Hit);
            }
            for dealer_value in 7..=11 {
                table.insert((player_value, true, 2, dealer_value), Action::Hit);
            }
            for dealer_value in 2..=11 {
                for card_count in 3..=10 {
                    table.insert((player_value, true, card_count, dealer_value), Action::Hit);
                }
            }
        }

        // 软17 (A,6)
        for dealer_value in 3..=6 {
            table.insert((17, true, 2, dealer_value), Action::Double);
        }
        table.insert((17, true, 2, 2), Action::Hit);
        for dealer_value in 7..=8 {
            table.insert((17, true, 2, dealer_value), Action::Stand);
        }
        for dealer_value in 9..=11 {
            table.insert((17, true, 2, dealer_value), Action::Hit);
        }
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((17, true, card_count, dealer_value), Action::Stand);
            }
        }

        // 软18 (A,7)
        for dealer_value in 2..=6 {
            table.insert((18, true, 2, dealer_value), Action::Double);
        }
        for dealer_value in 7..=8 {
            table.insert((18, true, 2, dealer_value), Action::Stand);
        }
        for dealer_value in 9..=10 {
            table.insert((18, true, 2, dealer_value), Action::Hit);
        }
        table.insert((18, true, 2, 11), Action::Stand);
        for dealer_value in 2..=11 {
            for card_count in 3..=10 {
                table.insert((18, true, card_count, dealer_value), Action::Stand);
            }
        }

        // 软19-21 总是停牌
        for player_value in 19..=21 {
            for dealer_value in 2..=11 {
                for card_count in 2..=10 {
                    table.insert((player_value, true, card_count, dealer_value), Action::Stand);
                }
            }
        }
    }

    /// 初始化分牌策略表
    fn init_split_table(table: &mut SplitTable) {
        // 分牌策略：根据玩家对子和庄家明牌决定
        // (玩家点数, 庄家明牌点数) -> 是否分牌
        
        // A-A: 总是分牌
        for dealer_value in 2..=11 {
            table.insert((11, dealer_value), Action::Split); // A-A = 11点
        }
        
        // 2-2, 3-3: 分牌（2-7），不分（8-11）
        for player_value in 4..=6 { // 2-2=4, 3-3=6
            for dealer_value in 2..=7 {
                table.insert((player_value, dealer_value), Action::Split);
            }
        }
        
        // 4-4: 不分（因为分牌后容易爆牌）
        
        // 5-5: 不分（当作10点处理，可以加倍）
        
        // 6-6: 分牌（2-6），不分（7-11）
        for dealer_value in 2..=6 {
            table.insert((12, dealer_value), Action::Split); // 6-6=12
        }
        
        // 7-7: 分牌（2-7），不分（8-11）
        for dealer_value in 2..=7 {
            table.insert((14, dealer_value), Action::Split); // 7-7=14
        }
        
        // 8-8: 总是分牌
        for dealer_value in 2..=11 {
            table.insert((16, dealer_value), Action::Split); // 8-8=16
        }
        
        // 9-9: 分牌（2-6, 8-9），不分（7, 10, A）
        for dealer_value in 2..=6 {
            table.insert((18, dealer_value), Action::Split); // 9-9=18
        }
        table.insert((18, 8), Action::Split);
        table.insert((18, 9), Action::Split);
        
        // 10-10: 不分（20点很强）
        
        // 注意：这里简化处理，实际策略表更复杂
    }

    /// 初始化投降策略表
    fn init_surrender_table(table: &mut SurrenderTable) {
        // 投降策略：只在不利情况下投降
        // (玩家点数, 庄家明牌点数) -> 是否投降
        
        // 硬15对庄家10: 投降
        table.insert((15, 10), Action::Surrender);
        
        // 硬16对庄家9, 10, A: 投降
        table.insert((16, 9), Action::Surrender);
        table.insert((16, 10), Action::Surrender);
        table.insert((16, 11), Action::Surrender); // A
        
        // 注意：这里只包含最常见的投降情况
        // 实际策略可能更复杂，包括软15、软16等
    }
}

impl Default for BasicStrategy {
    fn default() -> Self {
        Self::new()
    }
}


