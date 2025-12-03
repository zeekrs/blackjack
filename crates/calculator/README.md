# Blackjack Calculator

Blackjack 游戏计算器核心库，提供概率计算、期望值计算等功能。

## 功能

- 概率计算
- 期望值计算
- 最佳策略计算
- 支持多种游戏规则配置

## 使用示例

```rust
use calculator::{Calculator, GameRules};

let rules = GameRules::default();
let calc = Calculator::new(rules);

// 计算期望值
// let ev = calc.calculate_ev(&player_hand, &dealer_up_card);
```

