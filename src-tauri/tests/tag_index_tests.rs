use std::collections::HashSet;
use wishes_lib::{domain::tag::Tag, infrastructure::tag_index::TagIndex};


// 标签交集查询测试
#[test]
fn test_tag_index_intersection() {
    let index = TagIndex::<u64>::new();

    // 1: character + 5
    let mut tags_card1 = HashSet::new();
    tags_card1.insert(Tag::new(Tag::NAMESPACE_TYPE, "character"));
    tags_card1.insert(Tag::new(Tag::NAMESPACE_RARITY, "5"));
    index.insert(1, &tags_card1);

    // 2: weapon + 4
    let mut tags_card2 = HashSet::new();
    tags_card2.insert(Tag::new(Tag::NAMESPACE_TYPE, "weapon"));
    tags_card2.insert(Tag::new(Tag::NAMESPACE_RARITY, "4"));
    index.insert(2, &tags_card2);

    // 3: character + 4
    let mut tags_card3 = HashSet::new();
    tags_card3.insert(Tag::new(Tag::NAMESPACE_TYPE, "character"));
    tags_card3.insert(Tag::new(Tag::NAMESPACE_RARITY, "4"));
    index.insert(3, &tags_card3);

    // 查询: character + 4
    let mut query_card = HashSet::new();
    query_card.insert(Tag::new(Tag::NAMESPACE_TYPE, "character"));
    query_card.insert(Tag::new(Tag::NAMESPACE_RARITY, "4"));

    // 应该只有 3 号卡片
    let result = index.query(&query_card);
    assert!(result.contains(&3));
    assert!(!result.contains(&2));
    assert!(!result.contains(&1));
    assert_eq!(result.len(), 1, "应该有且仅有 1 张卡片");
}

// 测试在空查询时返回空集
#[test]
fn test_tag_index_empty_query_returns_empty() {
    let index = TagIndex::<u64>::new();

    let mut tags_card1 = HashSet::new();
    tags_card1.insert(Tag::new(Tag::NAMESPACE_TYPE, "character"));
    tags_card1.insert(Tag::new(Tag::NAMESPACE_RARITY, "5"));
    index.insert(1, &tags_card1);

    let query_card = HashSet::new();
    let result = index.query(&query_card);
    assert!(result.is_empty());
}