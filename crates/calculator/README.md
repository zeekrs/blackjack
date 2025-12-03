# Blackjack Calculator

Blackjack 游戏计算器核心库，提供概率计算、期望值计算等功能。

## 功能

### 当前实现

- **上桌 EV 计算**：根据当前剩余牌组，计算玩家是否应该上桌下注的期望值
- **基础策略支持**：玩家采用基础策略（Basic Strategy）
- **算牌支持**：支持任意剩余牌组，可进行算牌分析
- **多副牌支持**：支持 1-8 副牌（默认 8 副）

### 计划实现

- 玩家决策 EV 计算（要牌/停牌/加倍等）
- 分牌策略
- 投降策略
- 软 17 规则支持
- WASM 支持

## 游戏规则

### 基本规则

- **目标**：手牌点数尽可能接近 21 点，但不能超过
- **点数计算**：
  - A 可以是 1 或 11（软硬点数）
  - 2-10 按面值
  - J, Q, K 都是 10
- **黑杰克**：A + 10/J/Q/K（两张牌，21 点），赔率 3:2（1.5 倍）
- **平局**：退还本金（EV = 0）

### 庄家规则

- **硬 17 停牌**（默认）：庄家点数达到 17 或以上时停牌
- **软 17 停牌**（计划支持）：庄家软 17 时继续要牌

### 玩家策略

- **基础策略**（Basic Strategy）：
  - 根据玩家手牌和庄家明牌，查询策略表决定动作
  - 支持动作：要牌（Hit）、停牌（Stand）、加倍（Double）
  - 分牌和投降暂不支持

## 使用方法

### 基本用法

```rust
use calculator::{Calculator, GameRules, create_full_8_deck};

// 创建规则配置
let rules = GameRules::default();

// 创建计算器
let calculator = Calculator::new(rules);

// 创建完整 8 副牌
let deck = create_full_8_deck();

// 计算上桌 EV
let result = calculator.calculate_table_ev(&deck);

println!("期望值: {:.4}%", result.ev * 100.0);
println!("玩家胜率: {:.4}%", result.player_win_prob * 100.0);
println!("庄家胜率: {:.4}%", result.dealer_win_prob * 100.0);
println!("平局率: {:.4}%", result.push_prob * 100.0);
println!("黑杰克率: {:.4}%", result.blackjack_prob * 100.0);
```

### 算牌场景

```rust
use calculator::{Calculator, CardCounts};
use std::collections::HashMap;

// 创建自定义剩余牌组（例如：高牌较多）
let mut deck: CardCounts = HashMap::new();
// ... 设置剩余牌组 ...

// 计算当前牌组的 EV
let result = calculator.calculate_table_ev(&deck);

// 如果 EV > 0，说明当前牌组对玩家有利
if result.ev > 0.0 {
    println!("当前牌组对玩家有利，EV: {:.4}%", result.ev * 100.0);
}
```

## 项目结构

```
src/
├── lib.rs                    # 库入口，导出公共接口
├── types.rs                  # 类型定义
│   ├── Card, Hand           # 牌和手牌
│   ├── Action, GameResult   # 动作和游戏结果
│   ├── GameOutcome          # 概率结果结构
│   └── CardCounts           # 牌组表示
├── rules.rs                  # 游戏规则
│   ├── GameRules            # 规则配置
│   └── dealer_play()        # 庄家规则实现
├── strategy.rs               # 基础策略
│   ├── BasicStrategyTable   # 策略表
│   └── get_action()         # 策略查询
├── probability_calculator.rs # 概率计算核心
│   ├── ProbabilityCalculator
│   ├── calculate_table_ev() # 主入口
│   └── calculate_game_outcome() # 递归DFS
├── ev_calculator.rs         # EV计算
│   └── calculate_ev()       # 期望值计算
└── calculator.rs             # 对外接口
    └── Calculator           # 主计算器
```

## 算法说明

### 概率计算方法

使用 **深度优先搜索（DFS）+ 记忆化** 的方法：

1. **遍历所有初始发牌组合**
   - 玩家第一张牌（所有可能）
   - 庄家明牌（所有可能）
   - 玩家第二张牌（所有可能）
   - 庄家暗牌（所有可能）

2. **递归计算游戏结果**
   - 对于每个初始组合，递归计算所有可能的游戏路径
   - 玩家按照基础策略行动
   - 庄家按照固定规则补牌
   - 计算最终胜负概率

3. **记忆化优化**
   - 缓存相同状态的结果
   - 状态定义：`(玩家点数, 是否软点数, 玩家牌数, 庄家明牌, 牌组签名)`

4. **计算期望值**
   - EV = P(玩家胜) × 1.0 + P(玩家黑杰克) × 1.5 - P(庄家胜) × 1.0

### 性能优化

- **记忆化缓存**：避免重复计算相同状态
- **概率剪枝**：忽略概率过小的路径（< 1e-12）
- **早期终止**：玩家/庄家黑杰克或玩家爆牌时立即返回
- **并行计算**：不同初始牌组合可以并行处理（可选）

### 性能目标

- **满 8 副牌**：< 3 秒
- **算牌场景**：根据剩余牌数，通常 < 5 秒

## 测试

### 理论值验证

满 8 副牌，标准规则下的理论 EV 约为 **-0.5% 到 -0.6%**（庄家优势）。

```rust
#[test]
fn test_full_8_decks() {
    let deck = create_full_8_deck();
    let rules = GameRules::default();
    let calculator = Calculator::new(rules);
    
    let result = calculator.calculate_table_ev(&deck);
    
    // 理论值应该在 -0.005 到 -0.006 之间
    assert!((result.ev - (-0.0055)).abs() < 0.001);
}
```

## 开发计划

### 阶段 1：基础框架 ✅
- [x] 项目结构搭建
- [ ] 类型定义完善
- [ ] 手牌点数计算（软硬点数）
- [ ] 庄家规则实现

### 阶段 2：策略和概率计算
- [ ] 基础策略表实现
- [ ] DFS 递归框架
- [ ] 概率计算核心
- [ ] EV 计算

### 阶段 3：优化和测试
- [ ] 记忆化优化
- [ ] 性能测试
- [ ] 理论值验证

### 阶段 4：扩展功能
- [ ] 分牌策略
- [ ] 投降策略
- [ ] 软 17 规则
- [ ] WASM 支持

## 参考资料

- [Blackjack Basic Strategy](https://wizardofodds.com/games/blackjack/strategy/calculator/)
- [Blackjack Theory](https://wizardofodds.com/games/blackjack/)
