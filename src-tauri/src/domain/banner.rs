//! # 卡池 Banner 定义
//! 关联卡组和逻辑, 并持有运行时状态状态实例.

use serde::{Serialize, Deserialize};
use crate::domain::{ids::{BannerId, DeckId, LogicId}, tag::Tagged};
use serde_json::Value as JsonValue;

/// 卡池核心数据.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    /// 唯一标识.
    pub id: BannerId,
    /// 卡池名称.
    pub name: String,
    /// 关联的卡组 Id.
    pub deck_id: DeckId,
    /// 逻辑 Id.
    pub logic_id: LogicId,
}

/// 带标签的卡池.
/// 即 `Tagged<Banner>`.
pub type TaggedBanner = Tagged<Banner>;


/// 卡池运行时状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerRuntimeState {
    #[serde(default)]
    pub total_counter: u64,

    #[serde(default)]
    pub logic_state: JsonValue,
}

impl Default for BannerRuntimeState {
    fn default() -> Self {
        Self {
            total_counter: 0,
            logic_state: JsonValue::Null,
        }
    }
}
