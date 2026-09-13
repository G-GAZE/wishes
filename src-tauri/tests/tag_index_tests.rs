use std::collections::HashSet;

use wishes_lib::{domain::tag::Tag, infrastructure::tag_index::TagIndex};


// TagIndex::query 包含所有标签查询测试
#[test]
fn test_tag_index_query() {
    let index = TagIndex::<u64>::new();

    // 1: character + 5
    let tags_card1 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "5"),
    ];
    index.insert(1, &tags_card1);

    // 2: weapon + 4
    let tags_card2 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "weapon"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(2, &tags_card2);

    // 3: character + 4
    let tags_card3 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(3, &tags_card3);

    // 查询: character + 4
    let query_card = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];

    // 应该只有 3 号卡片
    let result = index.query(&query_card);
    assert_eq!(HashSet::from([3]), result);
}


// TagIndex::query_any 包含任意标签查询测试
#[test]
fn test_tag_index_query_any() {
    let index = TagIndex::<i64>::new();

    // 1: character + 5
    let tags_card1 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "5"),
    ];
    index.insert(1, &tags_card1);

    // 2: weapon + 4
    let tags_card2 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "weapon"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(2, &tags_card2);

    // 3: character + 4
    let tags_card3 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(3, &tags_card3);

    let query_tags = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "4")
    ];
    let result = index.query_any(&query_tags);
    assert_eq!(HashSet::from([1, 2, 3]), result);
}

// 测试在空查询时返回空集
#[test]
fn test_tag_index_empty_query_returns_empty() {
    let index = TagIndex::<u64>::new();

    let tags_card1 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "5"),
    ];
    index.insert(1, &tags_card1);

    let query_card = Vec::new();
    let result = index.query(&query_card);
    assert!(result.is_empty());
}

// 测试全量查询
#[test]
fn test_tag_index_query_all_ids() {
    let index = TagIndex::<i64>::new();

    // 1: character + 5
    let tags_card1 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "5"),
    ];
    index.insert(1, &tags_card1);

    // 2: weapon + 4
    let tags_card2 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "weapon"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(2, &tags_card2);

    // 3: character + 4
    let tags_card3 = vec![
        Tag::new(Tag::NAMESPACE_TYPE, "character"),
        Tag::new(Tag::NAMESPACE_RARITY, "4"),
    ];
    index.insert(3, &tags_card3);

    let result = index.query_all();
    assert_eq!(HashSet::from([1, 2, 3]), result);
}