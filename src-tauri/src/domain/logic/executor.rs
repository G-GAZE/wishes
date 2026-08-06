use std::collections::HashSet;

use serde_json::Value as JsonValue;
use rand::rngs::ChaCha12Rng;

use crate::domain::{logic::result::LogicResult, tag::{EventTag, Tag}};


pub trait HardcodedExecutor: Send + Sync {
    fn execute(&self,
        state: &mut JsonValue,      // 逻辑实例的状态
        rng: &mut ChaCha12Rng,
    ) -> LogicResult;

    // 所有可能的输出结果, 用于校验 Deck 是否包含所需 Card
    fn possible_output_combinations(&self) -> Vec<(HashSet<Tag>, HashSet<EventTag>)>;
}

pub trait RuleExecutor: Send + Sync {
    fn apply(&self,
        params: &JsonValue,         // 规则定义时的静态参数
        state: &mut JsonValue,      // 运行时规则在逻辑实例中的状态
        res: &mut LogicResult,      // 最终输出
        rng: &mut ChaCha12Rng,
    );
}