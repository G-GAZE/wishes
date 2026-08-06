use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::domain::{ids::{LogicId, WishRuleLocalId}, tag::Tagged};


// 规则类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RuleType {
    Builtin {
        builtin_name: String,
    },
    Custom {
        script_path: String,
        properties: JsonValue,
    }
}


// 静态规则定义蓝本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub local_id: WishRuleLocalId,
    pub rule_type: RuleType,
    pub params: JsonValue,
}


// 逻辑变体枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "variant", content = "config")]
pub enum LogicVariant {
    Hardcoded {
        executor_name: String,
    },
    RuleBased {
        rules: Vec<RuleDefinition>,
    }
}


// 静态逻辑定义蓝本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicDefinition {
    pub id: LogicId,
    pub name: String,
    // pub tags: HashSet<Tag>,
    pub variant: LogicVariant,
}

pub type TaggedLogicDefinition = Tagged<LogicDefinition>;
