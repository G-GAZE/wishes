//! # 逻辑定义 (静态蓝图) 相关类型

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::domain::{ids::{LogicId, WishRuleLocalId}, tag::Tagged};


/// 规则类型枚举.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RuleType {
    /// 内置规则, 使用 Rust 代码实现, 拥有更好的性能.
    Builtin {
        /// 内置规则名称.
        builtin_name: String,
    },
    /// 自定义规则, 由蓝图等其他方式实现 (未来拓展)
    Custom {
        script_path: String,
        properties: JsonValue,
    }
}


/// 静态规则定义蓝本 (单个规则).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    /// 规则在逻辑内部唯一的唯一标识, 用于状态字典的键.
    pub local_id: WishRuleLocalId,
    /// 规则类型.
    pub rule_type: RuleType,
    /// 规则的静态参数.
    pub params: JsonValue,
}


/// 逻辑变体枚举.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "variant", content = "config")]
pub enum LogicVariant {
    /// 硬编码逻辑, 由 Rust 代码直接实现, 无规则链调用, 性能更好.
    Hardcoded {
        /// 内置执行器名称.
        /// 目前已实现的执行器包括:
        /// - `genshin_character_up`: 原神角色 UP 卡池逻辑 (含捕获明光)
        /// - `starrail_character_up`: 崩坏星穹铁道角色 UP 卡池逻辑
        executor_name: String,
    },
    /// 基于规则链的组合式逻辑.
    RuleBased {
        /// 有序规则列表, 按顺序执行内部的规则.
        rules: Vec<RuleDefinition>,
    }
}

/// 全局的静态逻辑定义蓝本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicDefinition {
    /// 唯一标识.
    pub id: LogicId,
    /// 逻辑名称.
    pub name: String,
    /// 逻辑变体.
    pub variant: LogicVariant,
}

/// 带标签的逻辑定义.
/// 即 `Tagged<LogicDefinition>`
pub type TaggedLogicDefinition = Tagged<LogicDefinition>;
