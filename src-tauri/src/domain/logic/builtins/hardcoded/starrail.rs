//! # 崩坏星穹铁道 硬编码执行器实现

use rand::RngExt;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::rngs::ChaCha12Rng;
use serde::{Serialize, Deserialize};
use serde_json::{Value as JsonValue};
use super::super::super::executor::HardcodedExecutor;
use crate::domain::{tag::{Tag, EventTag}, logic::result::LogicResult};


// 逻辑常量
const RARITY_5: &str = "5";
const RARITY_4: &str = "4";
const RARITY_3: &str = "3";

const TYPE_CHARACTER: &str = "character";
const TYPE_LIGHT_CONE: &str = "light_cone";


/// 崩坏星穹铁道角色 UP 卡池逻辑状态.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct StarrailCharacterUpState {
    counter_5: u16,
    counter_4: u16,
    is_pity_5: bool,
    is_pity_4: bool,
    next_char_4: bool,
}

impl Default for StarrailCharacterUpState {
    fn default() -> Self {
        Self {
            counter_5: 0,
            counter_4: 0,
            is_pity_5: false,
            is_pity_4: false,
            next_char_4: false,
        }
    }
}

/// 崩坏星穹铁道角色 UP 卡池抽卡逻辑.
pub struct StarrailCharacterUpLogic;

impl StarrailCharacterUpLogic {
    const BASE_RATE_5   : f64 = 0.006;
    const BASE_RATE_4   : f64 = 0.051;
    const STEP_RATE_5   : f64 = 0.06;
    const STEP_START_5  : u16 = 74;
    const STEP_START_4  : u16 = 9;
    const STEP_RATE_4   : f64 = 0.51;
    const UP_RATE       : f64 = 0.5;
    const CHAR_RATE_4   : f64 = 0.5;

    fn draw(state: StarrailCharacterUpState, rng: &mut ChaCha12Rng) -> (StarrailCharacterUpState, LogicResult) {
        let mut new_state = state;
        new_state.counter_5 += 1;
        new_state.counter_4 += 1;

        let rate_5 = if new_state.counter_5 >= Self::STEP_START_5 {
            (Self::BASE_RATE_5 + Self::STEP_RATE_5 * (new_state.counter_5 - Self::STEP_START_5 + 1) as f64)
            .min(1.0)
        } else {
            Self::BASE_RATE_5
        };
        let rate_4 = if new_state.counter_4 >= Self::STEP_START_4 {
            (Self::BASE_RATE_4 + Self::STEP_RATE_4 * (new_state.counter_4 - Self::STEP_START_4 + 1) as f64)
            .min(1.0 - rate_5)
        } else {
            Self::BASE_RATE_4.min(1.0 - rate_5)
        };

        let weights = [rate_5, rate_4, 1.0 - rate_5 - rate_4];
        let dist = WeightedIndex::new(weights).expect("StarrailCharacterUpLogic.draw(): 权重错误!");
        match dist.sample(rng) {
            0 => {      // 5 星
                new_state.counter_5 = 0;
                let up = state.is_pity_5 || rng.random_bool(Self::UP_RATE);
                let event_tag = if up { EventTag::up() } else { EventTag::standard() };

                new_state.is_pity_5 = !up;
                
                let result = LogicResult::new()
                    .with_tag(Tag::new(Tag::NAMESPACE_RARITY, RARITY_5))
                    .with_tag(Tag::new(Tag::NAMESPACE_TYPE, TYPE_CHARACTER))
                    .with_event_tag(event_tag);
                (new_state, result)
            },
            1 => {      // 4 星
                new_state.counter_4 = 0;
                let up = state.is_pity_4 || rng.random_bool(Self::UP_RATE);
                let type_ = if state.next_char_4 || up || rng.random_bool(Self::CHAR_RATE_4) {
                    TYPE_CHARACTER
                } else {
                    TYPE_LIGHT_CONE
                };
                let event_tag = if up { EventTag::up() } else { EventTag::standard() };

                new_state.is_pity_4 = !up;
                new_state.next_char_4 = type_ == TYPE_LIGHT_CONE;

                let result = LogicResult::new()
                    .with_tag(Tag::new(Tag::NAMESPACE_RARITY, RARITY_4))
                    .with_tag(Tag::new(Tag::NAMESPACE_TYPE, type_))
                    .with_event_tag(event_tag);
                (new_state, result)
            },
            _ => {      // 3 星
                let result = LogicResult::new()
                    .with_tag(Tag::new(Tag::NAMESPACE_RARITY, RARITY_3))
                    .with_tag(Tag::new(Tag::NAMESPACE_TYPE, TYPE_LIGHT_CONE))
                    .with_event_tag(EventTag::standard());
                (new_state, result)
            }
        }
    }
}

impl HardcodedExecutor for StarrailCharacterUpLogic {
    fn execute(&self,
        state: &mut JsonValue,      // 逻辑实例的状态
        rng: &mut ChaCha12Rng,
    ) -> LogicResult
    {
        let current: StarrailCharacterUpState = match serde_json::from_value(state.clone()) {
            Ok(s) => s,
            Err(_) => StarrailCharacterUpState::default()
        };

        let (new_state, result) = Self::draw(current, rng);

        *state = serde_json::to_value(new_state).unwrap();

        result
    }

    fn possible_output_combinations(&self) -> Vec<(Vec<Tag>, Vec<EventTag>)> {
        vec![
            (
                vec![Tag::new(Tag::NAMESPACE_RARITY, RARITY_5), Tag::new(Tag::NAMESPACE_TYPE, TYPE_CHARACTER)],
                vec![EventTag::up()]
            ),
            (
                vec![Tag::new(Tag::NAMESPACE_RARITY, RARITY_4), Tag::new(Tag::NAMESPACE_TYPE, TYPE_CHARACTER)],
                vec![EventTag::up()]
            ),
            (
                vec![Tag::new(Tag::NAMESPACE_RARITY, RARITY_4), Tag::new(Tag::NAMESPACE_TYPE, TYPE_CHARACTER)],
                vec![EventTag::standard()]
            ),
            (
                vec![Tag::new(Tag::NAMESPACE_RARITY, RARITY_4), Tag::new(Tag::NAMESPACE_TYPE, TYPE_LIGHT_CONE)],
                vec![EventTag::standard()]
            ),
            (
                vec![Tag::new(Tag::NAMESPACE_RARITY, RARITY_3), Tag::new(Tag::NAMESPACE_TYPE, TYPE_LIGHT_CONE)],
                vec![EventTag::standard()]
            )
        ]
    }
}


#[cfg(test)]
mod tests {
    // 单元测试, 逻辑类似原神

    use super::*;
    use serde_json::json;
    use rand::{SeedableRng, rngs::ChaCha12Rng};

    // 固定随机种子
    fn fixed_rng() -> ChaCha12Rng {
        ChaCha12Rng::seed_from_u64(42)
    }

    // 第 90 抽必出 5 星
    #[test]
    fn test_starrail_char_up_5_hard_pity() {
        let mut state = json!({
            "counter_5": 89,
            "counter_4": 0,
            "is_pity_5": false,
            "is_pity_4": false,
            "next_char_4": false,
        });
        let mut rng = fixed_rng();
        let logic = StarrailCharacterUpLogic;
        let result = logic.execute(&mut state, &mut rng);

        // 5 星
        assert!(result.tags.contains(&Tag::new(Tag::NAMESPACE_RARITY, RARITY_5)));
        // 5 星计数重置
        assert_eq!(state["counter_5"].as_u64().unwrap(), 0);
        // 4 星计数正常 +1
        assert_eq!(state["counter_4"].as_u64().unwrap(), 1);
    }

    // 歪过后 5 星必为 up
    #[test]
    fn test_starrail_char_up_5_up_pity() {
        let mut state = json!({
            "counter_5": 89,
            "counter_4": 0,
            "is_pity_5": true,
            "is_pity_4": false,
            "next_char_4": false,
        });
        let mut rng = fixed_rng();
        let logic = StarrailCharacterUpLogic;
        let result = logic.execute(&mut state, &mut rng);

        // 5 星
        assert!(result.tags.contains(&Tag::new(Tag::NAMESPACE_RARITY, RARITY_5)));
        // up
        assert!(result.event_tags.contains(&EventTag::up()));
        // 5 星计数重置
        assert_eq!(state["counter_5"].as_u64().unwrap(), 0);
        // up 保底重置
        assert_eq!(state["is_pity_5"].as_bool().unwrap(), false);
        // 4 星计数正常 +1
        assert_eq!(state["counter_4"].as_u64().unwrap(), 1);
    }

    // 当 counter_4 == 9 但出 5 星时, 4 星保底顺延 (counter_4 不重置)
    #[test]
    fn test_starrail_char_up_4_pity_extension_after_5() {
        let mut state = json!({
            "counter_5": 89,
            "counter_4": 9,
            "is_pity_5": false,
            "is_pity_4": false,
            "next_char_4": false,
        });
        let mut rng = fixed_rng();
        let logic = StarrailCharacterUpLogic;

        let result1 = logic.execute(&mut state, &mut rng);
        assert!(result1.tags.contains(&Tag::new(Tag::NAMESPACE_RARITY, RARITY_5)));
        assert_eq!(state["counter_5"].as_u64().unwrap(), 0);
        assert_eq!(state["counter_4"].as_u64().unwrap(), 10);   // 4 星保底顺延

        // 可能为 4 星或 5 星
        let result2 = logic.execute(&mut state, &mut rng);
        assert!(!result2.tags.contains(&Tag::new(Tag::NAMESPACE_RARITY, RARITY_3)));
    }

    // 第 10 抽必出 4 星及以上
    #[test]
    fn test_starrail_char_up_4_hard_pity() {
        let mut state = json!({
            "counter_5": 0,
            "counter_4": 9,
            "is_pity_5": false,
            "is_pity_4": false,
            "next_char_4": false,
        });
        let mut rng = fixed_rng();
        let logic = StarrailCharacterUpLogic;
        let result = logic.execute(&mut state, &mut rng);

        // 反向测试: 出 4/5 星 -> 不是 3 星
        assert!(!result.tags.contains(&Tag::new(Tag::NAMESPACE_RARITY, RARITY_3)));
    }
}