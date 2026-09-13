//! # 卡组 Deck 定义
//! 包含卡片 `Card` 成员和活动标签分组.

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::{domain::{tag::{EventTag, Tag, Tagged}}, infrastructure::registry::CardRegistry};
use super::ids::{DeckId, CardId};


/// 声明卡组成员卡片的条件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MembershipCondition {
    /// 全局拉取: 拥有所有指定标签的卡片.
    #[serde(rename = "tag_all")]
    TagAll{ tags: Vec<Tag> },

    /// 全局拉取: 拥有任意指定标签的卡片.
    #[serde(rename = "tag_any")]
    TagAny{ tags: Vec<Tag> },

    /// 本地过滤: 保留当前结果中拥有所有指定标签的卡片.
    #[serde(rename = "filter_tag_all")]
    FilterTagAll{ tags: Vec<Tag> },

    /// 本地过滤: 保留当前结果中拥有任意指定标签的卡片.
    #[serde(rename = "filter_tag_any")]
    FilterTagAny{tags: Vec<Tag>},

    /// 全局拉取: 直接添加这些 Id 的卡片.
    #[serde(rename = "include_ids")]
    IncludeIds{ ids: HashSet<CardId> },

    /// 排除: 直接排除这些指定的卡片.
    #[serde(rename = "exclude_ids")]
    ExcludeIds{ ids: HashSet<CardId> },

    // TODO[2026-08-19]: 当前的匹配规则主要是正向匹配, 未来加入标签反向排除规则, EventGroupCondition 同理
}

/// 卡组成员规则, 每个条件顺序应用.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub conditions: Vec<MembershipCondition>,
}

impl Membership {
    /// 由成员规则计算得到最终成员卡片
    pub fn resolve(&self, registry: &CardRegistry) -> HashSet<CardId> {
        let mut result = HashSet::new();

        // 1. 应用所有包含型和过滤型条件, 顺序能影响执行效果
        for cond in &self.conditions {
            match cond {
                MembershipCondition::TagAll { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query(tags);
                        result.extend(ids);
                    }
                },
                MembershipCondition::TagAny { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query_any(tags);
                        result.extend(ids);
                    }
                },
                MembershipCondition::FilterTagAll { tags } => {
                    if !tags.is_empty() && !result.is_empty() {
                        let ids = registry.tag_index.query(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                MembershipCondition::FilterTagAny { tags } => {
                    if !tags.is_empty() && !result.is_empty() {
                        let ids = registry.tag_index.query_any(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                MembershipCondition::IncludeIds { ids } => {
                    if !ids.is_empty() {
                        // 只保留存在的 Id
                        // 直接使用迭代器拓展, 性能更好
                        result.extend(ids.iter().filter(|id| registry.contains(**id)).copied());
                    }
                },
                _ => {},
            }
        }

        // 2. 应用排除型条件, 顺序不影响排除的效果
        for cond in &self.conditions {
            match cond {
                MembershipCondition::ExcludeIds { ids } => {
                    if !ids.is_empty() {
                        result.retain(|id| !ids.contains(id));
                    }
                },
                _ => {},
            }
        }

        result
    }
}


/// 声明 `Deck.event_groups` 中每个活动标签组成员卡片的条件.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventGroupCondition {
    /// 拉取 `members` 中的所有卡片.
    #[serde(rename = "all")]
    All,

    /// 全局拉取 (基于 `members`): 拥有所有指定标签的卡片.
    #[serde(rename = "tag_all")]
    TagAll{ tags: Vec<Tag> },

    /// 全局拉取 (基于 `members`): 拥有任意指定标签的卡片.
    #[serde(rename = "tag_any")]
    TagAny{ tags: Vec<Tag> },

    /// 本地过滤: 保留当前结果中拥有所有指定标签的卡片.
    #[serde(rename = "filter_tag_all")]
    FilterTagAll{ tags: Vec<Tag> },

    /// 本地过滤: 保留当前结果中拥有任意指定标签的卡片.
    #[serde(rename = "filter_tag_any")]
    FilterTagAny{ tags: Vec<Tag> },

    /// 显示添加: 直接添加这些 Id 的卡片.
    #[serde(rename = "include_ids")]
    IncludeIds{ ids: HashSet<CardId> },

    /// 排除: 直接排除这些 Id 的卡片.
    #[serde(rename = "exclude_ids")]
    ExcludeIds{ ids: HashSet<CardId> },

    /// 暂不支持
    #[serde(rename = "include_groups")]
    IncludeGroups{ groups: Vec<EventTag> },

    /// 暂不支持
    #[serde(rename = "exclude_groups")]
    ExcludeGroups{ groups: Vec<EventTag> },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGroup {
    pub conditions: Vec<EventGroupCondition>
}

impl EventGroup {
    /// 根据筛选规则计算实际包含的卡片
    pub fn resolve(&self, members: &HashSet<CardId>, registry: &CardRegistry) -> HashSet<CardId> {
        let mut result = HashSet::new();

        // 1. 同 members 的处理, 但全局拉取需和 members 取交集
        for cond in &self.conditions {
            match cond {
                EventGroupCondition::All => {
                    result.extend(members.clone());
                },
                EventGroupCondition::TagAll { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query(tags);
                        // result.retain(|id| ids.contains(id));
                        result.extend(ids.intersection(members).cloned());
                    }
                },
                EventGroupCondition::TagAny { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query_any(tags);
                        // result.retain(|id| ids.contains(id));
                        result.extend(ids.intersection(members).cloned());
                    }
                },
                EventGroupCondition::FilterTagAll { tags } => {
                    if !tags.is_empty() && !result.is_empty() {
                        let ids = registry.tag_index.query(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                EventGroupCondition::FilterTagAny { tags } => {
                    if !tags.is_empty() && !result.is_empty() {
                        let ids = registry.tag_index.query_any(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                EventGroupCondition::IncludeIds { ids } => {
                    if !ids.is_empty() {
                        // 由于 members 中的卡片已确定存在, 活动组只需确定是否存在于 members 中
                        result.extend(ids.iter().filter(|id| members.contains(id)).copied());
                    }
                },
                EventGroupCondition::IncludeGroups { .. } | EventGroupCondition::ExcludeGroups { .. } => {
                    // TODO[2026-08-18]: 未来实现包含/排除其他活动标签组, 并处理循环依赖
                    unimplemented!("IncludeGroups 和 ExcludeGroups 筛选条件当前不支持")
                },
                _ => {},
            }
        }

        // 2. 应用所有排除型条件
        for cond in &self.conditions {
            match cond {
                EventGroupCondition::ExcludeIds { ids } => {
                    if !ids.is_empty() {
                        result.retain(|id| !ids.contains(id));
                    }
                },
                _ => {},
            }
        }

        result
    }
}


/// 卡组核心数据 (不包含标签 `Tag`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    /// 唯一标识.
    pub id: DeckId,
    /// 卡组名称.
    pub name: String,
    /// 卡组包含的所有卡片 Id 全集.
    pub members: Membership,
    /// 活动标签分组映射: `EventTag` -> 该分组下的所有卡片 Id 集合.
    /// 这些 Id 必须为 `members` 的子集.
    pub event_groups: HashMap<EventTag, EventGroup>,
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
        let members = self.members.resolve(registry);

        let mut cards = if tags.is_empty() {
            members.clone()
        } else {
            let ids = registry.tag_index.query(tags);
            members.intersection(&ids).cloned().collect()
        };

        for event in event_tags {
            if let Some(group) = self.event_groups.get(event) {
                cards.retain(|id| group.resolve(&members, registry).contains(id));
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
