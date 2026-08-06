use std::collections::HashSet;

use crate::domain::tag::{EventTag, Tag};


pub struct LogicResult {
    pub tags: HashSet<Tag>,
    pub event_tags: HashSet<EventTag>,
    // pub appointed_card: Option<CardId>
}

impl LogicResult {
    pub fn new() -> Self {
        Self {
            tags: HashSet::new(),
            event_tags: HashSet::new()
        }
    }

    // 方便链式调用的添加方法
    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }

    pub fn with_event_tag(mut self, event_tag: EventTag) -> Self {
        self.event_tags.insert(event_tag);
        self
    }
}