//! # 逻辑执行结果.

use std::collections::HashSet;
use crate::domain::tag::{EventTag, Tag};

/// 逻辑执行结果, 包含筛选卡片所需的普通标签和活动标签条件.
pub struct LogicResult {
    /// 普通标签 (用于 `Deck.query_cards` 的 `tags`).
    pub tags: HashSet<Tag>,
    /// 活动标签 (用于 `Deck.query_cards` 的 `event_tags`).
    pub event_tags: HashSet<EventTag>,
    // pub appointed_card: Option<CardId>
}

impl LogicResult {
    /// 创建空结果.
    pub fn new() -> Self {
        Self {
            tags: HashSet::new(),
            event_tags: HashSet::new()
        }
    }

    /// 添加一个普通标签, 可链式调用.
    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// 添加一个活动标签, 可链式调用.
    pub fn with_event_tag(mut self, event_tag: EventTag) -> Self {
        self.event_tags.insert(event_tag);
        self
    }
}