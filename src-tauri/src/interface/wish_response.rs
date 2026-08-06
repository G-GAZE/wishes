use serde::Serialize;

use crate::domain::{tag::{EventTag, Tag}, wish_result::WishResult};


/// 每一抽最终返回前端的响应数据, 包含该抽得到的卡片的信息
#[derive(Debug, Clone, Serialize)]
pub struct WishResponse {
    pub id: u64,
    pub content: String,
    pub tags: Vec<Tag>,
    pub event_tags: Vec<EventTag>
}

impl WishResponse {
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
