//! `CardManager` 卡片引用查询与删除行为的测试.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use wishes_lib::{
    app::admin::card_manager::CardManager,
    domain::{
        card::{Card, TaggedCard},
        deck::{Deck, Membership, MembershipCondition, TaggedDeck},
        ids::{CardId, DeckId},
    },
    infrastructure::registry::{CardRegistry, DeckRegistry},
};

/// 创建指定 Id 的测试卡片.
fn create_card(id: u64) -> TaggedCard {
    TaggedCard::new(Card {
        id: CardId(id),
        content: format!("Test-Card-{}", id),
    })
}

/// 创建通过 `IncludeIds` 引用指定卡片的测试卡组.
fn create_deck_including(deck_id: u64, card_ids: Vec<CardId>) -> TaggedDeck {
    TaggedDeck::with_tags(
        Deck {
            id: DeckId(deck_id),
            name: format!("Test-Deck-{}", deck_id),
            members: Membership {
                conditions: vec![MembershipCondition::IncludeIds {
                    ids: card_ids.into_iter().collect::<HashSet<_>>(),
                }],
            },
            event_groups: HashMap::new(),
        },
        HashSet::new(),
    )
}

/// 创建通过 `ExcludeIds` 排除指定卡片的测试卡组.
fn create_deck_excluding(deck_id: u64, card_ids: Vec<CardId>) -> TaggedDeck {
    TaggedDeck::with_tags(
        Deck {
            id: DeckId(deck_id),
            name: format!("Test-Deck-{}", deck_id),
            members: Membership {
                conditions: vec![MembershipCondition::ExcludeIds {
                    ids: card_ids.into_iter().collect::<HashSet<_>>(),
                }],
            },
            event_groups: HashMap::new(),
        },
        HashSet::new(),
    )
}

/// 创建使用临时目录的 `CardManager`.
fn create_manager(card_registry: &Arc<CardRegistry>, deck_registry: &Arc<DeckRegistry>) -> CardManager {
    CardManager::new(
        card_registry.clone(),
        deck_registry.clone(),
        std::env::temp_dir(),
    )
}

// 被卡组 IncludeIds 引用的卡片: 删除仍然成功 (仅记录警告)
#[test]
fn delete_card_allowed_when_referenced() {
    let card_registry = Arc::new(CardRegistry::new());
    let deck_registry = Arc::new(DeckRegistry::new());

    card_registry.insert(create_card(1));
    deck_registry.insert(create_deck_including(1, vec![CardId(1)]));

    let manager = create_manager(&card_registry, &deck_registry);
    manager.delete_card(CardId(1)).unwrap();

    assert!(card_registry.get(CardId(1)).is_none(), "被引用的卡片也应允许删除");
}

// 未被任何卡组引用时可以正常删除
#[test]
fn delete_card_allowed_when_not_referenced() {
    let card_registry = Arc::new(CardRegistry::new());
    let deck_registry = Arc::new(DeckRegistry::new());

    card_registry.insert(create_card(1));
    card_registry.insert(create_card(2));
    deck_registry.insert(create_deck_including(1, vec![CardId(1)]));

    let manager = create_manager(&card_registry, &deck_registry);
    manager.delete_card(CardId(2)).unwrap();

    assert!(card_registry.get(CardId(2)).is_none(), "未被引用的卡片应被删除");
    assert!(card_registry.get(CardId(1)).is_some(), "其他卡片不应受影响");
}

// 引用查询: 只返回 IncludeIds 显式引用该卡片的卡组
#[test]
fn referencing_decks_returns_matching_decks() {
    let card_registry = Arc::new(CardRegistry::new());
    let deck_registry = Arc::new(DeckRegistry::new());

    card_registry.insert(create_card(1));
    card_registry.insert(create_card(2));
    deck_registry.insert(create_deck_including(1, vec![CardId(1)]));
    deck_registry.insert(create_deck_including(2, vec![CardId(999)]));

    let manager = create_manager(&card_registry, &deck_registry);

    let references: Vec<u64> = manager
        .referencing_decks(CardId(1))
        .iter()
        .map(|deck| deck.inner.id.0)
        .collect();
    assert_eq!(references, vec![1]);

    assert!(manager.referencing_decks(CardId(2)).is_empty(), "未被引用的卡片应返回空列表");
}

// 仅被 ExcludeIds 排除的卡片不算引用
#[test]
fn referencing_decks_ignores_exclude_ids() {
    let card_registry = Arc::new(CardRegistry::new());
    let deck_registry = Arc::new(DeckRegistry::new());

    card_registry.insert(create_card(1));
    deck_registry.insert(create_deck_excluding(1, vec![CardId(1)]));

    let manager = create_manager(&card_registry, &deck_registry);
    assert!(manager.referencing_decks(CardId(1)).is_empty());
}
