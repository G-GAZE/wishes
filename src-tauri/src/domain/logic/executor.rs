//! # 执行器 Trait 定义.

use serde_json::Value as JsonValue;
use rand::rngs::ChaCha12Rng;
use crate::domain::{logic::result::LogicResult, tag::{EventTag, Tag}};

/// 硬编码逻辑执行器 Trait (仅用于 `LogicVariant::Hardcoded`).
pub trait HardcodedExecutor: Send + Sync {
    /// 执行硬编码逻辑, 直接修改逻辑状态并返回最终结果.
    fn execute(&self,
        state: &mut JsonValue,      // 逻辑实例的状态
        rng: &mut ChaCha12Rng,
    ) -> LogicResult;

    /// 返回所有可能的输出组合, 用于加载器进行逻辑到卡组的标签覆盖性测试.
    fn possible_output_combinations(&self) -> Vec<(Vec<Tag>, Vec<EventTag>)>;
}

/// 规则执行器 Trait (用于 `LogicVariant::RuleBased` 中的单个规则).
pub trait RuleExecutor: Send + Sync {
    /// 将规则应用到规则链正在处理的结果上, 修改规则的局部状态并输出标签组合
    fn apply(&self,
        params: &JsonValue,         // 规则定义时的静态参数
        state: &mut JsonValue,      // 运行时规则在逻辑实例中的状态
        res: &mut LogicResult,      // 规则链正在处理的结果
        rng: &mut ChaCha12Rng,
    );
}