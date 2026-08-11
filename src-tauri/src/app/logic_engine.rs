//! # 逻辑执行引擎
//! 
//! 负责根据逻辑定义和当前实例状态执行抽卡逻辑, 生成输出标签集合.
//! 支持硬编码执行器 `HardcodedExecutor` 和规则执行器 `RuleExecutor`.

use std::sync::Arc;
use rand::rngs::ChaCha12Rng;
use serde_json::Value as JsonValue;
use crate::{domain::{ids::LogicId, logic::{definition::{LogicVariant, RuleType}, error::LogicError, instance::LogicInstance, result::LogicResult}}, infrastructure::registry::LogicRegistry};

/// 无状态的逻辑执行引擎.
/// 
/// 持有逻辑注册器的引用.
pub struct LogicEngine {
    pub registry: Arc<LogicRegistry>
}

impl LogicEngine {
    /// 创建新的逻辑引擎实例.
    pub fn new(registry: Arc<LogicRegistry>) -> Self {
        Self { registry }
    }

    /// 执行给定逻辑 Id 对应的逻辑, 修改实例状态并返回逻辑结果.
    /// 
    /// # 参数
    /// - `logic_id`: 要执行的逻辑定义 Id.
    /// - `instance`: 可变的逻辑实例, 其 `state` 会被读取和修改.
    /// - `rng`: 随机数生成器, 用于执行过程中的随机来源.
    /// 
    /// # 返回
    /// 成功时返回 `LogicResult`, 包含输出的标签集合.
    /// 
    /// # 错误
    /// - 逻辑定义不存在.
    /// - 执行器 (硬编码执行器) 未注册.
    /// - 当前无法使用自定义规则.
    pub fn execute(&self,
        logic_id: LogicId,
        instance: &mut LogicInstance,
        rng: &mut ChaCha12Rng
    ) -> Result<LogicResult, LogicError> {
        let tagged_def = self.registry.get_definition(logic_id)
            .ok_or(LogicError::DefinitionNotFound(logic_id))?;

        match &tagged_def.inner.variant {
            LogicVariant::Hardcoded { executor_name} => {
                let executor = self.registry.get_hardcoded_executor(executor_name)
                    .ok_or(LogicError::ExecutorNotFound(executor_name.clone()))?;
                Ok(executor.execute(&mut instance.state, rng))
            },
            LogicVariant::RuleBased { rules } => {
                let mut state_map = match instance.state {
                    JsonValue::Object(ref map) => map.clone(),
                    _=> serde_json::Map::new()
                };

                let mut result = LogicResult::new();

                for rule in rules {
                    let executor = match &rule.rule_type {
                        RuleType::Builtin { builtin_name } => {
                            self.registry.get_rule_executor(builtin_name)
                                .ok_or(LogicError::RuleNotFound(builtin_name.clone()))?
                        },
                        RuleType::Custom { script_path: _, properties: _ } => {
                            return Err(LogicError::CustomRuleUnusable())
                        }
                    };

                    let local_state = state_map.entry(rule.local_id.0.to_string())
                        .or_insert_with(|| JsonValue::Object(Default::default()));

                    executor.apply(&rule.params, local_state, &mut result, rng);
                }

                Ok(result)
            }
        }
    }
}