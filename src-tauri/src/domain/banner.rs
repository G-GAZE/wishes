//! # 卡池 Banner 定义
//! 关联卡组和逻辑, 并持有运行时状态状态实例.

use serde::{Serialize, Deserialize};
use crate::domain::{ids::{BannerId, DeckId}, logic::instance::LogicInstance, tag::Tagged};


/// 卡池核心数据.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    /// 唯一标识.
    pub id: BannerId,
    /// 卡池名称.
    pub name: String,
    /// 关联的卡组 Id.
    pub deck_id: DeckId,
    /// 逻辑状态实例 (包含逻辑定义 Id 和运行时状态).
    pub logic_instance: LogicInstance,
}

/// 带标签的卡池.
/// 即 `Tagged<Banner>`.
pub type TaggedBanner = Tagged<Banner>;
