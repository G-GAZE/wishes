use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{deck::{EventGroup, Membership, TaggedDeck}, tag::{EventTag, Tag}};



#[derive(Debug, Serialize)]
pub struct DeckSummary {
    pub id: u64,
    pub name: String,
    pub members: Membership,
    pub event_groups: HashMap<EventTag, EventGroup>,
    pub tags: Vec<Tag>,
    
}

impl From<&TaggedDeck> for DeckSummary {
    fn from(deck: &TaggedDeck) -> Self {
        let mut tags: Vec<_> = deck.tags.iter().cloned().collect();
        tags.sort_by(|a, b| a.namespace.cmp(&b.namespace));
        Self {
            id: deck.id.0,
            name: deck.name.clone(),
            members: deck.members.clone(),
            event_groups: deck.event_groups.clone(),
            tags,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct CreateDeckRequest {
    pub name: String,
    pub members: Membership,
    pub event_groups: HashMap<EventTag, EventGroup>,
    pub tags: Vec<Tag>,
}


#[derive(Debug, Deserialize)]
pub struct UpdateDeckRequest {
    pub id: u64,
    pub name: Option<String>,
    pub members: Option<Membership>,
    pub event_groups: Option<HashMap<EventTag, EventGroup>>,
    pub tags: Option<Vec<Tag>>,
}