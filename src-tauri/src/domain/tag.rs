use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};


// 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub namespace: String,
    pub value: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.value == other.value
    }
}
impl Eq for Tag {}
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespace.hash(state);
        self.value.hash(state);
    }
}

impl Tag {
    pub fn new<T: Into<String>>(namespace: T, value: T) -> Self {
        Tag {namespace: namespace.into(), value: value.into()}
    }

    // 内置命名空间
    pub const NAMESPACE_GAME: &'static str = "game";
    pub const NAMESPACE_TYPE: &'static str = "type";
    pub const NAMESPACE_RARITY: &'static str = "rarity";
}

impl<K, V> From<(K, V)> for Tag
where
    K: Into<String>,
    V: Into<String>,
{
    fn from(value: (K, V)) -> Self {
        Tag {
            namespace: value.0.into(),
            value: value.1.into(),
        }
    }
}


// 活动标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTag(pub String);

impl PartialEq for EventTag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for EventTag {}
impl Hash for EventTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl EventTag {
    pub fn new(value: impl Into<String>) -> Self {
        EventTag(value.into())
    }

    pub fn up() -> Self {
        EventTag::new("up")
    }

    pub fn fes() -> Self {
        EventTag::new("fes")
    }

    pub fn appoint() -> Self {
        EventTag::new("appoint")
    }

    pub fn standard() -> Self {
        EventTag::new("standard")
    }
}

impl From<&str> for EventTag {
    fn from(value: &str) -> Self {
        EventTag::new(value)
    }
}


// 标签化包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tagged<T> {
    #[serde(flatten)]
    pub inner: T,
    #[serde(default)]
    pub tags: HashSet<Tag>
}

impl<T> Tagged<T> {
    pub fn new(inner: T) -> Self {
        Self {inner, tags: HashSet::new()}
    }

    pub fn with_tags(inner: T, tags: HashSet<Tag>) -> Self {
        Self {inner, tags}
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn get_tag(&self, namespace: &str) -> Option<Tag> {
        for tag in &self.tags {
            if tag.namespace == namespace {
                return Some(tag.clone());
            }
        }
        None
    }

    pub fn remove_tag(&mut self, tag: &Tag) {
        self.tags.remove(tag);
    }

    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }

    pub fn contains_all(&self, tags: &[Tag]) -> bool {
        tags.iter().all(|tag| self.tags.contains(tag))
    }

    pub fn iter_tags(&self) -> impl Iterator<Item = &Tag> {
        self.tags.iter()
    }
}