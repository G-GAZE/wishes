//! # 逻辑实例 (运行时状态) 定义

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::domain::ids::{LogicId, WishRuleLocalId};

/// 逻辑实例, 存储运行时状态.
/// 
/// 每个卡池均有一个独立的逻辑实例, 其状态随抽卡更新并持久化到数据库.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicInstance {
    /// 关联的逻辑定义 Id.
    pub logic_id: LogicId,
    /// 运行时状态 (`serde_json::Value` 格式).
    /// 
    /// - 对于 `Hardcoded` 逻辑, 直接存储自定义结构.
    /// - 对于 `RuleBased` 逻辑, 存储为 `HashMap<RuleLocalId, serde_json::Value>`.
    /// 
    /// 每个规则状态独立.
    pub state: JsonValue,
}

impl LogicInstance {
    /// 为硬编码逻辑创建初始化状态.
    pub fn new_hardcoded(logic_id: LogicId, initial_state: impl Serialize) -> Self {
        // TODO: 值无效时改用 Default 默认值.
        Self {
            logic_id,
            state: serde_json::to_value(initial_state).unwrap()
        }
    }

    /// 为规则链逻辑创建初始状态, 每个规则对应空对象.
    pub fn new_rule_based(logic_id: LogicId, rule_ids: &[WishRuleLocalId]) -> Self {
        let mut map = serde_json::Map::new();
        for id in rule_ids {
            map.insert(id.0.clone(), JsonValue::default());
        }
        Self {
            logic_id,
            state: JsonValue::Object(map)
        }
    }
}