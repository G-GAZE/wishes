//! # 

use serde::{Deserialize, Serialize};
use crate::domain::{card::TaggedCard, tag::Tag};



#[derive(Debug, Clone, Serialize)]
pub struct CardSummary {
    pub id: u64,
    pub content: String,
    pub tags: Vec<Tag>,
}

impl From<&TaggedCard> for CardSummary {
    fn from(value: &TaggedCard) -> Self {
        let mut tags: Vec<_> = value.tags.iter().cloned().collect();
        tags.sort_by(|a, b| a.namespace.cmp(&b.namespace));     // 按命名空间字母排序

        Self {
            id: value.inner.id.0,
            content: value.inner.content.clone(),
            tags: tags,
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct CardCreateRequest {
    pub content: String,
    pub tags: Vec<Tag>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct CardUpdateRequest {
    pub id: u64,
    pub new_content: String,
    pub new_tags: Vec<Tag>,
}