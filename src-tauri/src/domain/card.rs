use serde::{Serialize, Deserialize};

use super::tag::Tagged;
use super::ids::CardId;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub content: String,
    // NOTE: title 和 asset 待后续补充
}

pub type TaggedCard = Tagged<Card>;
