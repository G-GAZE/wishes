//! # 抽卡响应
//! 
//! 定义抽卡后返回给前端的完整结果, 包含卡片内容、标签及活动标签.

use serde::Serialize;
use crate::domain::{tag::{EventTag, Tag}, wish_result::WishResult};

/// 每一抽最终返回前端的响应数据, 包含该抽得到的卡片的完整信息.
#[derive(Debug, Clone, Serialize)]
pub struct WishResponse {
    /// 卡片 Id (原始 `u64` 值).
    pub id: u64,
    /// 卡片内容.
    pub content: String,
    /// 卡片拥有的普通标签列表 (按命名空间排序).
    pub tags: Vec<Tag>,
    /// 卡片在当前卡池中附带的活动标签列表.
    pub event_tags: Vec<EventTag>
}

impl WishResponse {
    /// 从 `domain` 的 `WishResult` 转换为前端响应.
    /// 
    /// 会提取卡片内容、克隆标签, 并对普通标签按命名空间顺序进行字母序排序.
    pub fn new(res: WishResult) -> Self {
        let mut tags: Vec<_> = res.card.tags.iter().cloned().collect();
        tags.sort_by(|a, b| a.namespace.cmp(&b.namespace));     // 按命名空间字母顺序排序

        Self {
            id: res.card.inner.id.0,
            content: res.card.inner.content.clone(),
            tags,
            event_tags: res.event_tags,
        }
    }
}
