//! # 卡组 Deck 定义
//! 包含卡片 `Card` 成员和活动标签分组.

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::{
    domain::{
        ids::GlobalId, localized_string::LocalizedString, origin::Origin, tag::{EventTag, Tag, Tagged}
    }, infrastructure::registry::CardRegistry
};


/// 声明卡组成员卡片的条件.
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

    /// 全局拉取: 直接添加这些 `global_id` 的卡片.
    #[serde(rename = "include_ids")]
    IncludeIds{ ids: HashSet<GlobalId> },

    /// 排除: 直接排除这些指定的卡片.
    #[serde(rename = "exclude_ids")]
    ExcludeIds{ ids: HashSet<GlobalId> },

    // TODO[2026-08-19]: 当前的匹配规则主要是正向匹配, 未来加入标签反向排除规则, EventGroupCondition 同理
}

/// 卡组成员规则, 每个条件顺序应用.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    /// 按顺序应用的条件列表.
    #[serde(default)]
    pub conditions: Vec<MembershipCondition>,
}

impl Default for Membership {
    fn default() -> Self {
        Self { conditions: Vec::new() }
    }
}

impl Membership {
    /// 由成员规则计算得到最终成员卡片
    /// 
    /// # 执行顺序
    /// 1. 顺序应用全部包含型与过滤型条件 (顺序会影响结果).
    /// 2. 应用全部排除型条件 (顺序不影响结果).
    pub fn resolve(&self, registry: &CardRegistry) -> HashSet<GlobalId> {
        let mut result = HashSet::new();

        // 1. 应用所有包含型和过滤型条件, 顺序能影响执行效果
        for cond in &self.conditions {
            match cond {
                MembershipCondition::TagAll { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query_all(tags);
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
                        let ids = registry.tag_index.query_all(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                MembershipCondition::FilterTagAny { tags } => {
                    if !tags.is_empty() && !result.is_empty() {
                        let ids = registry.tag_index.query_any(tags);
                        result.retain(|id| ids.contains(id));
                    }
                },
                MembershipCondition::IncludeIds { ids }
                    if !ids.is_empty() => {
                        // 只保留存在的 Id
                        // 直接使用迭代器拓展, 性能更好
                        result.extend(ids.iter().filter(|id| registry.contains_visible(**id)).copied());
                        // TODO[2026-10-03]: 究竟使用 contains_visible (不含遮蔽) 还是 contains_including_shadowed (含遮蔽), 需等待后续决定
                    },
                _ => {},
            }
        }

        // 2. 应用排除型条件, 顺序不影响排除的效果
        for cond in &self.conditions {
            if let MembershipCondition::ExcludeIds { ids } = cond {
                if !ids.is_empty() {
                    result.retain(|id| !ids.contains(id));
                }
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

    /// 显示添加: 直接添加这些 `global_id` 的卡片.
    #[serde(rename = "include_ids")]
    IncludeIds{ ids: HashSet<GlobalId> },

    /// 排除: 直接排除这些 `global_id` 的卡片.
    #[serde(rename = "exclude_ids")]
    ExcludeIds{ ids: HashSet<GlobalId> },

    /// 暂不支持
    #[serde(rename = "include_groups")]
    IncludeGroups{ groups: Vec<EventTag> },

    /// 暂不支持
    #[serde(rename = "exclude_groups")]
    ExcludeGroups{ groups: Vec<EventTag> },
}

/// 活动标签分组.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGroup {
    /// 按顺序应用的条件列表.
    #[serde(default)]
    pub conditions: Vec<EventGroupCondition>
}

impl Default for EventGroup {
    fn default() -> Self {
        Self { conditions: Vec::new() }
    }
}

impl EventGroup {
    /// 根据筛选规则计算实际包含的卡片
    pub fn resolve(&self, members: &HashSet<GlobalId>, registry: &CardRegistry) -> HashSet<GlobalId> {
        let mut result = HashSet::new();

        // 1. 同 members 的处理, 但全局拉取需和 members 取交集
        for cond in &self.conditions {
            match cond {
                EventGroupCondition::All => {
                    result.extend(members.clone());
                },
                EventGroupCondition::TagAll { tags } => {
                    if !tags.is_empty() {
                        let ids = registry.tag_index.query_all(tags);
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
                        let ids = registry.tag_index.query_all(tags);
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
                    // 这里可能存在静默错误, 需等待后续 Banner CRUD 时完善
                    tracing::error!(
                        "IncludeGroups 和 ExcludeGroups 筛选条件当前不支持, 已跳过"
                    )
                },
                _ => {},
            }
        }

        // 2. 应用所有排除型条件
        for cond in &self.conditions {
            if let EventGroupCondition::ExcludeIds { ids } = cond {
                if !ids.is_empty() {
                    result.retain(|id| !ids.contains(id));
                }
            }
        }

        result
    }
}


/// 卡组核心数据 (不包含标签 `Tag`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    /// 全局唯一标识.
    pub global_id: GlobalId,

    /// 对象来源.
    pub origin: Origin,

    /// 派生自哪个对象的 `global_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forked_from: Option<GlobalId>,

    /// 卡组名称.
    pub name: LocalizedString,

    /// 资源引用.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<DeckAssets>,

    /// 成员规则.
    pub members: Membership,

    /// 活动标签分组映射: `EventTag` -> 该分组下的所有卡片 Id 集合.
    /// 这些卡片必须为 `members` 的子集.
    #[serde(default)]
    pub event_groups: HashMap<EventTag, EventGroup>,
}

impl Deck {
    /// 以默认来源 (`local`) 创建一个空卡组.
    pub fn new(name: LocalizedString) -> Self {
        Self {
            global_id: GlobalId::new(),
            origin: Origin::Local,
            forked_from: None,
            name,
            assets: None,
            members: Membership::default(),
            event_groups: HashMap::new(),
        }
    }

    /// 按给定语言解析卡组显示名, 缺失时回退到默认语言或任意可用语言.
    pub fn display_name(&self, locale: &str) -> Option<&str> {
        self.name.get_or_default_locale(locale)
    }

    /// 根据标签条件查询卡组中匹配的卡片 `global_id`.
    /// 
    /// # 参数
    /// - `registry`: 全局卡片注册表 `CardRegistry`, 用于全局标签索引查询.
    /// - `tags`: 普通标签, 要求卡片必须拥有所有提供的标签.
    /// - `event_tags`: 活动标签, 要求卡片必须处于对应的活动分组中.
    /// 
    /// # 返回
    /// 符合条件的卡片 `global` 列表, 顺序不确定.
    pub fn query_cards(&self, registry: &CardRegistry, tags: &[Tag], event_tags: &[EventTag]) -> Vec<GlobalId> {
        let members = self.members.resolve(registry);

        let mut cards = if tags.is_empty() {
            members.clone()
        } else {
            let ids = registry.tag_index.query_all(tags);
            members.intersection(&ids).cloned().collect()
        };

        for event in event_tags {
            let Some(group) = self.event_groups.get(event) else {
                return Vec::new();
            };
            let group_cards = group.resolve(&members, registry);
            cards.retain(|id| group_cards.contains(id));
            
            // NOTE: group.resolve 在 retain 闭包外, 降低了时间复杂度
            // 当前 query_cards 每次还需进行卡组的解析, 是一种为了保留卡组动态性的简化实现
            // 目前实际卡片不超过 1000 张, 性能可以接收
            // 若未来出现性能瓶颈, 再考虑引入 DeckResolver 服务
        }

        cards.into_iter().collect()
    }
}

/// 卡组资产引用.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckAssets {
    /// 卡组封面.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
}

impl DeckAssets {
    /// 是否所有资产引用都为空.
    pub fn is_empty(&self) -> bool {
        self.cover.is_none()
    }
}

/// 带标签的卡组.
/// 即 `Tagged<Deck>`.
pub type TaggedDeck = Tagged<Deck>;
