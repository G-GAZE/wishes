//! # 卡组 Deck 定义
//! 包含卡片 `Card` 成员和活动标签分组.

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::{domain::{tag::{EventTag, Tag, Tagged}}, infrastructure::registry::CardRegistry};
use super::ids::{DeckId, CardId};

/// 卡组核心数据 (不包含标签 `Tag`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    /// 唯一标识.
    pub id: DeckId,
    /// 卡组名称.
    pub name: String,
    /// 卡组包含的所有卡片 Id 全集.
    pub members: HashSet<CardId>,
    /// 活动标签分组映射: `EventTag` -> 该分组下的所有卡片 Id 集合.
    /// 这些 Id 必须为 `members` 的子集.
    pub event_groups: HashMap<EventTag, HashSet<CardId>>,
}

impl Deck {
    /// 根据标签条件查询卡组中匹配的卡片 Id.
    /// 
    /// # 参数
    /// - `registry`: 全局卡片注册表 `CardRegistry`, 用于全局标签索引查询.
    /// - `tags`: 普通标签, 要求卡片必须拥有所有提供的标签.
    /// - `event_tags`: 活动标签, 要求卡片必须处于对应的活动分组中.
    /// 
    /// # 返回
    /// 符合条件的卡片 Id 列表, 顺序不确定.
    pub fn query_cards(&self, registry: &CardRegistry, tags: &[Tag], event_tags: &[EventTag]) -> Vec<CardId> {
        let mut cards = registry.tag_index().query(tags);

        cards.retain(|id| self.members.contains(id));

        for event in event_tags {
            if let Some(group) = self.event_groups.get(event) {
                cards.retain(|id| group.contains(id));
            } else {
                return Vec::new();
            }
        }

        cards.into_iter().collect()
    }
}

/// 带标签的卡组.
/// 即 `Tagged<Deck>`.
pub type TaggedDeck = Tagged<Deck>;
