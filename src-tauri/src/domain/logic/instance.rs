use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::domain::ids::{LogicId, WishRuleLocalId};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicInstance {
    pub logic_id: LogicId,

    // 逻辑实例的状态
    // - 对于 Hardcoded 逻辑, 为具体状态结构
    // - 对于 RuleBased 逻辑, 为 HashMap<RuleLocalId, JsonValue>
    pub state: JsonValue,
}


impl LogicInstance {
    pub fn new_hardcoded(logic_id: LogicId, initial_state: impl Serialize) -> Self {
        // TODO: match 检查
        Self {
            logic_id,
            state: serde_json::to_value(initial_state).unwrap()
        }
    }

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