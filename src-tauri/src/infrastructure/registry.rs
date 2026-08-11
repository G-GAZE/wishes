//! # 注册器
//! 
//! 实际存储卡片、卡组、卡池和逻辑定义，拥有这些对象的所有权.
//! 每个注册器均内建标签索引以支持快速标签查询.
//! 所有存储均采用并发安全的 `DashMap` 和 `Arc`.

use std::{collections::HashSet, path::PathBuf, sync::Arc};
use dashmap::DashMap;
use parking_lot::Mutex;
use super::{tag_index::TagIndex};
use crate::domain::{
    banner::TaggedBanner, card::TaggedCard, deck::TaggedDeck, ids::*, logic::{
        builtins::hardcoded::{
            genshin::GenshinCharacterUpLogic,
            starrail::StarrailCharacterUpLogic
        }, definition::TaggedLogicDefinition, executor::{HardcodedExecutor, RuleExecutor}
    }, tag::{EventTag, Tag}
};

/// 卡片注册器.
/// 
/// 管理所有 `TaggedCard`.
/// 支持通过 `CardId` 快速查询, 或使用标签查询.
pub struct CardRegistry {
    // id_allocator: IdAllocator<CardId>,
    /// 实际存储结构.
    storage: DashMap<CardId, Arc<TaggedCard>>,
    /// 内建的标签索引.
    pub tag_index: TagIndex<CardId>,
    /// 预留的卡片配置文件路径记录.
    paths: DashMap<CardId, PathBuf>,    // 预留, 后期用于记录 Card 来源文件并写回修改
}

impl CardRegistry {
    /// 创建新卡片注册器.
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
            paths: DashMap::new(),
        }
    }

    /// 获取标签索引的引用.
    /// 
    /// **已弃用**: 可直接访问 `tag_index` 字段.
    pub fn tag_index(&self) -> &TagIndex<CardId> {
        &self.tag_index
    }

    /// 检查指定 Id 的卡片是否存在
    pub fn contains(&self, id: CardId) -> bool {
        self.storage.contains_key(&id)
    }

    /// 插入一张卡片, 同时加入标签索引.
    pub fn insert(&self, card: TaggedCard) {
        let tags = card.tags.clone();
        self.tag_index.insert(card.inner.id, &tags);
        self.storage.insert(card.inner.id, Arc::new(card));
    }

    /// 记录卡片配置文件的路径.
    pub fn insert_path(&self, id: CardId, path: PathBuf) {
        self.paths.insert(id, path);
    }

    /// 通过 Id 获取卡片的一个 `Arc` 引用.
    pub fn get(&self, id: CardId) -> Option<Arc<TaggedCard>> {
        self.storage.get(&id).map(|refs| refs.clone())
    }

    /// 返回卡片总数.
    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

/// 卡组注册器.
/// 
/// 管理所有 `TaggedDeck`.
/// 目前仅提供基础的存储和检索, 未使用标签索引 (但结构已预留).
pub struct DeckRegistry {
    /// 实际存储结构.
    storage: DashMap<DeckId, Arc<TaggedDeck>>,
    /// 标签索引.
    pub tag_index: TagIndex<DeckId>,
}

impl DeckRegistry {
    /// 创建新的卡组注册器.
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
        }
    }

    /// 检查指定 Id 的卡组是否存在.
    pub fn contains(&self, id: DeckId) -> bool {
        self.storage.contains_key(&id)
    }

    /// 插入一个卡组.
    pub fn insert(&self, deck: TaggedDeck) {
        self.storage.insert(deck.inner.id, Arc::new(deck));
    }

    /// 通过 Id 获取卡组的一个 `Arc` 引用.
    pub fn get(&self, id: DeckId) -> Option<Arc<TaggedDeck>> {
        self.storage.get(&id).map(|entry| entry.clone())
    }

    /// 返回卡组总数.
    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

/// 卡池注册器.
/// 
/// 管理所有 `TaggedBanner`.
/// 每个卡池均被包裹在 `Mutex` 中, 以便运行时动态修改状态.
/// 
/// `Mutex` 由 `parking_lot` crate 提供.
pub struct BannerRegistry {
    /// 实际存储结构.
    storage: DashMap<BannerId, Arc<Mutex<TaggedBanner>>>,
    /// 标签索引.
    pub tag_index: TagIndex<BannerId>,
}

impl BannerRegistry {
    /// 创建新的卡池注册器.
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
        }
    }

    /// 检查指定 Id 的卡池是否存在.
    pub fn contains(&self, id: BannerId) -> bool {
        self.storage.contains_key(&id)
    }

    /// 插入一个新卡池.
    pub fn insert(&self, banner: TaggedBanner) {
        self.storage.insert(banner.inner.id, Arc::new(Mutex::new(banner)));
    }

    /// 通过 Id 获取卡池的 `Arc<Mutex<_>>` 引用.
    pub fn get(&self, id: BannerId) -> Option<Arc<Mutex<TaggedBanner>>> {
        self.storage.get(&id).map(|entry| entry.clone())
    }

    /// 返回卡池总数.
    pub fn count(&self) -> usize {
        self.storage.len()
    }

    /// 获取所有卡池 Id 的列表.
    /// 顺序不确定.
    pub fn ids(&self) -> Vec<BannerId> {
        self.storage.iter().map(|entry| entry.key().clone()).collect()
    }
}

/// 逻辑定义与执行器注册器.
/// 
/// 维护逻辑定义 `TaggedLogicDefinition` 以及两类执行器:
/// - 硬编码执行器 `HardcodedExecutor`
/// - 规则执行器 `RuleExecutor`
pub struct LogicRegistry {
    /// 逻辑定义的实际存储结构.
    definitions: DashMap<LogicId, Arc<TaggedLogicDefinition>>,
    /// 硬编码执行器的实际存储结构.
    hardcoded_executors: DashMap<String, Arc<dyn HardcodedExecutor>>,
    /// 规则执行器的实际存储结构.
    rule_executors: DashMap<String, Arc<dyn RuleExecutor>>,
    /// 逻辑定义的标签索引.
    pub tag_index: TagIndex<LogicId>,
}

impl LogicRegistry {
    /// 创建新的逻辑注册器
    /// 同时注册所有内置逻辑执行器.
    pub fn new() -> Self {
        // 注册内置执行器
        let s = Self {
            definitions: DashMap::new(),
            hardcoded_executors: DashMap::new(),
            rule_executors: DashMap::new(),
            tag_index: TagIndex::new(),
        };

        s.register_builtin()
    }

    /// 注册内置逻辑执行器
    fn register_builtin(self) -> Self {
        self.hardcoded_executors.insert("genshin_character_up".into(), Arc::new(GenshinCharacterUpLogic));
        self.hardcoded_executors.insert("starrail_character_up".into(), Arc::new(StarrailCharacterUpLogic));
        self
    }

    /// 检查是否存在指定 Id 的逻辑定义.
    pub fn contains_definition(&self, id: LogicId) -> bool {
        self.definitions.contains_key(&id)
    }

    /// 检查是否存在指定名称的硬编码执行器.
    pub fn contains_hardcoded_executor(&self, name: &str) -> bool {
        self.hardcoded_executors.contains_key(name)
    }

    /// 检查是否存在指定名称的规则执行器.
    pub fn contains_rule_executor(&self, name: &str) -> bool {
        self.rule_executors.contains_key(name)
    }

    /// 插入一个逻辑定义.
    pub fn insert_definition(&self, def: TaggedLogicDefinition) {
        self.definitions.insert(def.inner.id, Arc::new(def));
    }

    /// 通过 Id 获取逻辑定义的 `Arc` 引用.
    pub fn get_definition(&self, logic_id: LogicId) -> Option<Arc<TaggedLogicDefinition>> {
        self.definitions.get(&logic_id).map(|guard| guard.value().clone())
    }

    /// 通过名称获取硬编码执行器的 `Arc` 引用.
    pub fn get_hardcoded_executor(&self, name: &str) -> Option<Arc<dyn HardcodedExecutor>> {
        self.hardcoded_executors.get(name).map(|guard| guard.value().clone())
    }

    /// 通过名称获取规则执行器的 `Arc` 引用.
    pub fn get_rule_executor(&self, name: &str) -> Option<Arc<dyn RuleExecutor>> {
        self.rule_executors.get(name).map(|guard| guard.value().clone())
    }

    /// 返回逻辑定义的总数.
    pub fn count_definitions(&self) -> usize {
        self.definitions.len()
    }

    /// 获取指定硬编码执行器可能输出的所有标签组合 (用于加载器进行标签覆盖性测试).
    pub fn hardcoded_possible_output_combinations(&self, name: &str) -> Option<Vec<(HashSet<Tag>, HashSet<EventTag>)>> {
        self.hardcoded_executors.get(name)
            .map(|guard| guard.value().possible_output_combinations())
    }
}