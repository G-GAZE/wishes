//! # 标签索引
//! 
//! 提供基于 `Tag` 的泛型倒排索引, 支持插入和按标签集合查询 Id.

use std::collections::HashSet;
use dashmap::DashMap;
use crate::domain::tag::Tag;

/// 泛型标签索引.
/// 
/// 维护从 `Tag` 到一组 Id 的映射.
/// 适用于需要按标签快速检索对象的场景, 主要用于卡片、卡组、卡池、逻辑定义等.
pub struct TagIndex<Id> {
    tag_to_ids: DashMap<Tag, HashSet<Id>>
}

impl<Id> TagIndex<Id>
where
    Id: std::hash::Hash + Eq + Clone + Copy + Send + Sync
{
    /// 创建空的标签索引.
    pub fn new() -> Self {
        Self { tag_to_ids: DashMap::new() }
    }
    
    /// 为指定 Id 插入一组标签.
    /// 
    /// 每个标签都会建立到该 Id 的映射.
    pub fn insert(&self, id: Id, tags: &[Tag]) {
        for tag in tags {
            self.tag_to_ids.entry(tag.clone()).or_insert_with(HashSet::new).insert(id);
        }
    }

    pub fn remove(&self, id: Id, tag: &Tag) {
        if let Some(mut entry) = self.tag_to_ids.get_mut(tag) {
            entry.remove(&id);
            // 集合为空时, 保留
        }
    }

    /// 查询同时拥有给定标签集合的所有 Id (交集查询).
    /// 
    /// 如果传入的标签集合为空, 返回空集.
    pub fn query(&self, tags: &[Tag]) -> HashSet<Id> {
        let mut iter = tags.iter();
        if let Some(first) = iter.next() {
            let mut result = self.tag_to_ids.get(first)
                .map(|id| id.clone())
                .unwrap_or_default();
            for tag in iter {
                if let Some(entry) = self.tag_to_ids.get(tag) {
                    result.retain(|id| entry.contains(id));
                } else {
                    return HashSet::new();
                }
                if result.is_empty() { break; }
            }
            result
        } else {
            HashSet::new()
        }
    }

    /// 返回索引中所有的 Id 集合.
    pub fn query_all(&self) -> HashSet<Id> {
        let mut res = HashSet::new();
        for entry in self.tag_to_ids.iter() {
            res.extend(entry.value().iter().copied());
        }
        res
    }
}