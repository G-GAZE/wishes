use wishes_lib::{domain::tag::Tag, infrastructure::tag_index::TagIndex};


// 标签交集查询测试
#[test]
fn test_tag_index_intersection() {
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
    assert!(result.contains(&3));
    assert!(!result.contains(&2));
    assert!(!result.contains(&1));
    assert_eq!(result.len(), 1, "应该有且仅有 1 张卡片");
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