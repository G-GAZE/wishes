use std::sync::Arc;

use rand::rngs::ChaCha12Rng;
use serde_json::Value as JsonValue;

use crate::{domain::{ids::LogicId, logic::{definition::{LogicVariant, RuleType}, error::LogicError, instance::LogicInstance, result::LogicResult}}, infrastructure::registry::LogicRegistry};

pub struct LogicEngine {
    pub registry: Arc<LogicRegistry>
}

impl LogicEngine {
    pub fn new(registry: Arc<LogicRegistry>) -> Self {
        Self { registry }
    }

    // 逻辑执行入口
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