use std::sync::Arc;

use crate::domain::{card::TaggedCard, tag::EventTag};


/// 每一抽的最终结果, 包含标签化后的 Card 和该 Card 在当前 Banner 中的 EventTag 标签列表
pub struct WishResult {
    pub card: Arc<TaggedCard>,
    pub event_tags: Vec<EventTag>,
}
