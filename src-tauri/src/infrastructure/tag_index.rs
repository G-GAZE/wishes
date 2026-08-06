use std::collections::HashSet;
use dashmap::DashMap;

use crate::domain::tag::Tag;


// 泛型标签索引
pub struct TagIndex<Id> {
    tag_to_ids: DashMap<Tag, HashSet<Id>>
}

impl<Id> TagIndex<Id>
where
    Id: std::hash::Hash + Eq + Clone + Copy + Send + Sync
{
    pub fn new() -> Self {
        Self { tag_to_ids: DashMap::new() }
    }
    
    pub fn insert(&self, id: Id, tags: &HashSet<Tag>) {
        for tag in tags {
            self.tag_to_ids.entry(tag.clone()).or_insert_with(HashSet::new).insert(id);
        }
    }

    pub fn query(&self, tags: &HashSet<Tag>) -> HashSet<Id> {
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

    pub fn query_all(&self) -> HashSet<Id> {
        let mut res = HashSet::new();
        for entry in self.tag_to_ids.iter() {
            res.extend(entry.value().iter().copied());
        }
        res
    }
}