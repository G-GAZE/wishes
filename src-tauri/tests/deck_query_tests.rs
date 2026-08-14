use std::collections::{HashMap, HashSet};

use wishes_lib::{domain::{card::{Card, TaggedCard}, deck::Deck, ids::{CardId, DeckId}, tag::{EventTag, Tag}}, infrastructure::registry::CardRegistry};

// 辅助函数, 创建带标签的卡片
fn create_test_card(id: u64, tags: Vec<(&str, &str)>) -> TaggedCard {
    let mut card = TaggedCard::new(Card {
        id: CardId(id),
        content: "Test-Card".into(),
    });
    for (namespace, value) in tags {
        card.add_tag(Tag::new(namespace, value));
    }
    card
}

#[test]
fn test_deck_query_without_event_tags() {
    let registry = CardRegistry::new();

    registry.insert(create_test_card(1, vec![("game", "genshin"), ("rarity", "5")]));
    registry.insert(create_test_card(2, vec![("game", "zzz"), ("rarity", "4")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("rarity", "5")]));

    let mut members = HashSet::new();
    members.insert(CardId(1));
    members.insert(CardId(2));
    members.insert(CardId(3));

    let deck = Deck {
        id: DeckId(1),
        name: "Test-Deck".into(),
        members,
        event_groups: HashMap::new(),
    };

    let tags = vec![
        Tag::new("game", "genshin")
    ];
    let event_tags = Vec::new();

    let result = deck.query_cards(&registry, &tags, &event_tags);
    assert_eq!(result, vec![CardId(1)]);
}


#[test]
fn test_deck_query_with_event_tags() {
    let registry = CardRegistry::new();

    registry.insert(create_test_card(1, vec![("game", "genshin"), ("rarity", "5")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("rarity", "4")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("rarity", "5")]));

    let mut members = HashSet::new();
    members.insert(CardId(1));
    members.insert(CardId(2));
    members.insert(CardId(3));

    let mut event_groups = HashMap::new();
    let up_group = HashSet::from([CardId(2)]);
    event_groups.insert(EventTag::up(), up_group);

    let deck = Deck {
        id: DeckId(1),
        name: "Test-Deck".into(),
        members,
        event_groups,
    };

    let tags = vec![
        Tag::new("game", "genshin")
    ];
    let event_tags = vec![EventTag::up()];

    let result = deck.query_cards(&registry, &tags, &event_tags);
    assert_eq!(result, vec![CardId(2)]);
}