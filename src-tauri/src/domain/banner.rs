use serde::{Serialize, Deserialize};

use crate::domain::{ids::{BannerId, DeckId}, logic::instance::LogicInstance, tag::Tagged};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    pub id: BannerId,
    pub name: String,
    pub deck_id: DeckId,
    pub logic_instance: LogicInstance,
}

pub type TaggedBanner = Tagged<Banner>;
