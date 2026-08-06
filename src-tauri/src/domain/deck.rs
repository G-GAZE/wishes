use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::{domain::{tag::{EventTag, Tag, Tagged}}, infrastructure::registry::CardRegistry};
use super::ids::{DeckId, CardId};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: DeckId,
    pub name: String,
    pub members: HashSet<CardId>,

    pub event_groups: HashMap<EventTag, HashSet<CardId>>,
}

impl Deck {
    pub fn query_cards(&self, registry: &CardRegistry, tags: &HashSet<Tag>, event_tags: &HashSet<EventTag>) -> Vec<CardId> {
        let mut cards = registry.tag_index().query(&tags);

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

pub type TaggedDeck = Tagged<Deck>;
