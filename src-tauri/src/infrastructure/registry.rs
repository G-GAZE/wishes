use std::{collections::HashSet, path::PathBuf, sync::Arc};
use dashmap::DashMap;
use parking_lot::Mutex;

use super::{tag_index::TagIndex};
use crate::domain::{
    banner::TaggedBanner, card::TaggedCard, deck::TaggedDeck, ids::*, logic::{
        builtins::hardcoded::{GenshinCharacterUpLogic, StarrailCharacterUpLogic}, definition::TaggedLogicDefinition, executor::{HardcodedExecutor, RuleExecutor}
    }, tag::{EventTag, Tag}
};


pub struct CardRegistry {
    // id_allocator: IdAllocator<CardId>,
    storage: DashMap<CardId, Arc<TaggedCard>>,
    pub tag_index: TagIndex<CardId>,

    paths: DashMap<CardId, PathBuf>,    // 预留, 后期用于记录 Card 来源文件并写回修改
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
            paths: DashMap::new(),
        }
    }

    pub fn tag_index(&self) -> &TagIndex<CardId> {
        &self.tag_index
    }

    pub fn contains(&self, id: CardId) -> bool {
        self.storage.contains_key(&id)
    }

    pub fn insert(&self, card: TaggedCard) {
        let tags = card.tags.clone();
        self.tag_index.insert(card.inner.id, &tags);
        self.storage.insert(card.inner.id, Arc::new(card));
    }

    pub fn insert_path(&self, id: CardId, path: PathBuf) {
        self.paths.insert(id, path);
    }

    pub fn get(&self, id: CardId) -> Option<Arc<TaggedCard>> {
        self.storage.get(&id).map(|refs| refs.clone())
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}


pub struct DeckRegistry {
    storage: DashMap<DeckId, Arc<TaggedDeck>>,
    pub tag_index: TagIndex<DeckId>,
}

impl DeckRegistry {
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
        }
    }

    pub fn contains(&self, id: DeckId) -> bool {
        self.storage.contains_key(&id)
    }

    pub fn insert(&self, deck: TaggedDeck) {
        self.storage.insert(deck.inner.id, Arc::new(deck));
    }

    pub fn get(&self, id: DeckId) -> Option<Arc<TaggedDeck>> {
        self.storage.get(&id).map(|entry| entry.clone())
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}


pub struct BannerRegistry {
    storage: DashMap<BannerId, Arc<Mutex<TaggedBanner>>>,
    pub tag_index: TagIndex<BannerId>,
}

impl BannerRegistry {
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            tag_index: TagIndex::new(),
        }
    }

    pub fn contains(&self, id: BannerId) -> bool {
        self.storage.contains_key(&id)
    }

    pub fn insert(&self, banner: TaggedBanner) {
        self.storage.insert(banner.inner.id, Arc::new(Mutex::new(banner)));
    }

    pub fn get(&self, id: BannerId) -> Option<Arc<Mutex<TaggedBanner>>> {
        self.storage.get(&id).map(|entry| entry.clone())
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }

    pub fn ids(&self) -> Vec<BannerId> {
        self.storage.iter().map(|entry| entry.key().clone()).collect()
    }
}


pub struct LogicRegistry {
    definitions: DashMap<LogicId, Arc<TaggedLogicDefinition>>,
    hardcoded_executors: DashMap<String, Arc<dyn HardcodedExecutor>>,
    rule_executors: DashMap<String, Arc<dyn RuleExecutor>>,

    pub tag_index: TagIndex<LogicId>,
}

impl LogicRegistry {
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

    pub fn contains_definition(&self, id: LogicId) -> bool {
        self.definitions.contains_key(&id)
    }

    pub fn contains_hardcoded_executor(&self, name: &str) -> bool {
        self.hardcoded_executors.contains_key(name)
    }

    pub fn contains_rule_executor(&self, name: &str) -> bool {
        self.rule_executors.contains_key(name)
    }

    pub fn insert_definition(&self, def: TaggedLogicDefinition) {
        self.definitions.insert(def.inner.id, Arc::new(def));
    }

    pub fn get_definition(&self, logic_id: LogicId) -> Option<Arc<TaggedLogicDefinition>> {
        self.definitions.get(&logic_id).map(|guard| guard.value().clone())
    }

    pub fn get_hardcoded_executor(&self, name: &str) -> Option<Arc<dyn HardcodedExecutor>> {
        self.hardcoded_executors.get(name).map(|guard| guard.value().clone())
    }

    pub fn get_rule_executor(&self, name: &str) -> Option<Arc<dyn RuleExecutor>> {
        self.rule_executors.get(name).map(|guard| guard.value().clone())
    }

    pub fn count_definitions(&self) -> usize {
        self.definitions.len()
    }

    pub fn hardcoded_possible_output_combinations(&self, name: &str) -> Option<Vec<(HashSet<Tag>, HashSet<EventTag>)>> {
        self.hardcoded_executors.get(name)
            .map(|guard| guard.value().possible_output_combinations())
    }
}