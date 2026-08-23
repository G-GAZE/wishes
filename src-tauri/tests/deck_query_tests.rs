use std::collections::{HashMap, HashSet};

use wishes_lib::{
    domain::{
        card::{Card, TaggedCard}, deck::{Deck, EventGroup, EventGroupCondition, Membership, MembershipCondition}, ids::{CardId, DeckId}, tag::{EventTag, Tag}
    }, infrastructure::registry::CardRegistry
};

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

// IncludeIds 包含 Id
#[test]
fn membership_include_ids_only() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));
    registry.insert(create_test_card(2, vec![]));

    let rule = Membership {
        conditions: vec![MembershipCondition::IncludeIds {
            ids: HashSet::from([CardId(1), CardId(2)]),
        }],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// ExcludeIds 排除 Id
#[test]
fn membership_exclude_ids_only() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));

    let rule = Membership {
        conditions: vec![MembershipCondition::ExcludeIds {
            ids: HashSet::from([CardId(1)]),
        }],
    };
    let result = rule.resolve(&registry);
    assert!(result.is_empty());
}

// TagAll 全局拉取
#[test]
fn membership_tag_all_global() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("type", "character")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("type", "weapon")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("type", "character")]));

    let rule = Membership {
        conditions: vec![MembershipCondition::TagAll {
            tags: vec![Tag::new("game", "genshin")],
        }],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// TagAny 全局拉取
#[test]
fn membership_tag_any_global() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));
    registry.insert(create_test_card(2, vec![("type", "character")]));
    registry.insert(create_test_card(3, vec![("game", "zzz")]));

    let rule = Membership {
        conditions: vec![MembershipCondition::TagAny {
            tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
        }],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// FilterTagAll 本地过滤
#[test]
fn membership_filter_tag_all() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("type", "character")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("type", "weapon")]));

    // 先全局拉取 game=genshin, 再过滤 type=character
    let rule = Membership {
        conditions: vec![
            MembershipCondition::TagAll {
                tags: vec![Tag::new("game", "genshin")],
            },
            MembershipCondition::FilterTagAll {
                tags: vec![Tag::new("type", "character")],
            },
        ],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1)]));
}

// FilterTagAny 本地过滤
#[test]
fn membership_filter_tag_any() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));
    registry.insert(create_test_card(2, vec![("type", "character")]));
    registry.insert(create_test_card(3, vec![("game", "zzz")]));

    // 先全局拉取 game=genshin 或 type=character, 再过滤保留 game=genshin 或 type=character (无变化)
    let rule = Membership {
        conditions: vec![
            MembershipCondition::TagAny {
                tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
            },
            MembershipCondition::FilterTagAny {
                tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
            },
        ],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// 组合: IncludeIds + ExcludeIds
#[test]
fn membership_include_and_exclude() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));
    registry.insert(create_test_card(2, vec![]));
    registry.insert(create_test_card(3, vec![]));

    let rule = Membership {
        conditions: vec![
            MembershipCondition::IncludeIds {
                ids: HashSet::from([CardId(1), CardId(2), CardId(3)]),
            },
            MembershipCondition::ExcludeIds {
                ids: HashSet::from([CardId(2)]),
            },
        ],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(3)]));
}

// 顺序敏感: 先 FilterTagAll 再 IncludeIds / 先 IncludeIds 再 FilterTagAll
#[test]
fn membership_order_sensitive() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));
    registry.insert(create_test_card(2, vec![("game", "zzz")]));

    // 先本地过滤 (空), 再添加卡片2, 结果 {2}
    let rule1 = Membership {
        conditions: vec![
            MembershipCondition::FilterTagAll {
                tags: vec![Tag::new("game", "genshin")],
            },
            MembershipCondition::IncludeIds {
                ids: HashSet::from([CardId(2)]),
            },
        ],
    };
    let result1 = rule1.resolve(&registry);
    assert_eq!(result1, HashSet::from([CardId(2)]));

    // 先添加卡片2, 再本地过滤保留 game:genshin, 由于卡片 2 不含该标签, 因此结果为空
    let rule2 = Membership {
        conditions: vec![
            MembershipCondition::IncludeIds {
                ids: HashSet::from([CardId(2)]),
            },
            MembershipCondition::FilterTagAll {
                tags: vec![Tag::new("game", "genshin")],
            },
        ],
    };
    let result2 = rule2.resolve(&registry);
    assert!(result2.is_empty());
}

// 复杂组合: TagAny + FilterTagAll + ExcludeIds
#[test]
fn membership_complex() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("type", "character")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("type", "weapon")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("type", "character")]));
    registry.insert(create_test_card(4, vec![("game", "zzz"), ("type", "weapon")]));

    let rule = Membership {
        conditions: vec![
            MembershipCondition::TagAny {
                tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
            }, // 得到 1,2,3
            MembershipCondition::FilterTagAll {
                tags: vec![Tag::new("type", "character")],
            }, // 过滤保留 1,3
            MembershipCondition::IncludeIds {
                ids: HashSet::from([CardId(4)]),
            }, // 添加 4 => 1,3,4
            MembershipCondition::ExcludeIds {
                ids: HashSet::from([CardId(3)]),
            }, // 移除 3 => 1,4
        ],
    };
    let result = rule.resolve(&registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(4)]));
}

// EventGroup: All 拉取 members 中所有卡片
#[test]
fn event_group_all() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));
    registry.insert(create_test_card(2, vec![]));
    let members = HashSet::from([CardId(1), CardId(2)]);

    let group = EventGroup {
        conditions: vec![EventGroupCondition::All],
    };
    let result = group.resolve(&members, &registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// EventGroup: IncludeIds 包含 Id
#[test]
fn event_group_include_ids_only() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));
    registry.insert(create_test_card(2, vec![]));
    let members = HashSet::from([CardId(1), CardId(2)]);

    let group = EventGroup {
        conditions: vec![EventGroupCondition::IncludeIds {
            ids: HashSet::from([CardId(1)]),
        }],
    };
    let result = group.resolve(&members, &registry);
    assert_eq!(result, HashSet::from([CardId(1)]));
}

// EventGroup: TagAll
#[test]
fn event_group_tag_all() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("rarity", "5")]));
    registry.insert(create_test_card(2, vec![("rarity", "4")]));
    registry.insert(create_test_card(3, vec![("rarity", "5")]));
    let members = HashSet::from([CardId(1), CardId(2)]); // 卡片 3 不在members中

    let group = EventGroup {
        conditions: vec![EventGroupCondition::TagAll {
            tags: vec![Tag::new("rarity", "5")],
        }],
    };
    let result = group.resolve(&members, &registry);
    // 全局查得 1,3, 但只取与 members 的交集 => 1
    assert_eq!(result, HashSet::from([CardId(1)]));
}

// EventGroup: TagAny
#[test]
fn event_group_tag_any() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));
    registry.insert(create_test_card(2, vec![("type", "character")]));
    registry.insert(create_test_card(3, vec![("game", "zzz")]));
    let members = HashSet::from([CardId(1), CardId(3)]); // 卡片 2 不在members中

    let group = EventGroup {
        conditions: vec![EventGroupCondition::TagAny {
            tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
        }],
    };
    let result = group.resolve(&members, &registry);
    // 全局查得 1,2, 与 members 交集 => 1
    assert_eq!(result, HashSet::from([CardId(1)]));
}

// EventGroup: FilterTagAll
#[test]
fn event_group_filter_tag_all() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("type", "character")]));
    registry.insert(create_test_card(2, vec![("type", "weapon")]));
    let members = HashSet::from([CardId(1), CardId(2)]);

    // 先拉取全集, 再用 FilterTagAll 筛选
    let group = EventGroup {
        conditions: vec![
            EventGroupCondition::All,
            EventGroupCondition::FilterTagAll {
                tags: vec![Tag::new("type", "character")],
            },
        ],
    };
    let result = group.resolve(&members, &registry);
    assert_eq!(result, HashSet::from([CardId(1)]));
}

// EventGroup: FilterTagAny
#[test]
fn event_group_filter_tag_any() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));
    registry.insert(create_test_card(2, vec![("type", "character")]));
    registry.insert(create_test_card(3, vec![("game", "zzz")]));
    let members = HashSet::from([CardId(1), CardId(2), CardId(3)]);

    let group = EventGroup {
        conditions: vec![
            EventGroupCondition::IncludeIds {
                ids: HashSet::from([CardId(1), CardId(2), CardId(3)]),
            },
            EventGroupCondition::FilterTagAny {
                tags: vec![Tag::new("game", "genshin"), Tag::new("type", "character")],
            },
        ],
    };
    let result = group.resolve(&members, &registry);
    // 过滤保留卡片 1 或 2 → {1,2}
    assert_eq!(result, HashSet::from([CardId(1), CardId(2)]));
}

// EventGroup: ExcludeIds
#[test]
fn event_group_exclude_ids() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![]));
    registry.insert(create_test_card(2, vec![]));
    let members = HashSet::from([CardId(1), CardId(2)]);

    let group = EventGroup {
        conditions: vec![
            EventGroupCondition::IncludeIds {
                ids: HashSet::from([CardId(1), CardId(2)]),
            },
            EventGroupCondition::ExcludeIds {
                ids: HashSet::from([CardId(1)]),
            },
        ],
    };
    let result = group.resolve(&members, &registry);
    assert_eq!(result, HashSet::from([CardId(2)]));
}

// EventGroup: 复杂组合 (模拟原神 up 组)
#[test]
fn event_group_complex() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("rarity", "5"), ("type", "character")])); // 5 星角色
    registry.insert(create_test_card(2, vec![("rarity", "5"), ("type", "weapon")]));    // 5 星武器
    registry.insert(create_test_card(3, vec![("rarity", "4"), ("type", "character")])); // 4 星角色
    registry.insert(create_test_card(4, vec![("rarity", "4"), ("type", "weapon")]));    // 4 星武器
    registry.insert(create_test_card(5, vec![("rarity", "3"), ("type", "character")])); // 3 星角色
    let members = HashSet::from([CardId(1), CardId(2), CardId(3), CardId(4), CardId(5)]);

    // up 组: 所有 5 星角色 + 指定的 4 星角色, 但排除武器
    let group = EventGroup {
        conditions: vec![
            EventGroupCondition::TagAll {
                tags: vec![Tag::new("rarity", "5"), Tag::new("type", "character")],
            }, // 卡片1
            EventGroupCondition::IncludeIds {
                ids: HashSet::from([CardId(3)]),
            }, // 添加卡片3
            EventGroupCondition::ExcludeIds {
                ids: HashSet::from([CardId(2)]),
            }, // 排除卡片2(不在结果中, 无害)
        ],
    };
    let result = group.resolve(&members, &registry);
    assert_eq!(result, HashSet::from([CardId(1), CardId(3)]));
}

// Deck::query_cards 无活动标签
#[test]
fn deck_query_no_event_tags() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("rarity", "5")]));
    registry.insert(create_test_card(2, vec![("game", "zzz"), ("rarity", "4")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("rarity", "5")]));

    let members = Membership {
        conditions: vec![MembershipCondition::IncludeIds {
            ids: HashSet::from([CardId(1), CardId(2), CardId(3)]),
        }],
    };

    let deck = Deck {
        id: DeckId(1),
        name: "Test-Deck".into(),
        members,
        event_groups: HashMap::new(),
    };

    let tags = vec![Tag::new("game", "genshin")];
    let result = deck.query_cards(&registry, &tags, &[]);
    assert_eq!(result, vec![CardId(1)]);
}

// Deck::query_cards 有活动标签
#[test]
fn deck_query_with_event_tags() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("rarity", "5")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("rarity", "4")]));
    registry.insert(create_test_card(3, vec![("game", "starrail"), ("rarity", "5")]));

    let members = Membership {
        conditions: vec![MembershipCondition::IncludeIds {
            ids: HashSet::from([CardId(1), CardId(2), CardId(3)]),
        }],
    };

    let mut event_groups = HashMap::new();
    let up_group = EventGroup {
        conditions: vec![EventGroupCondition::IncludeIds {
            ids: HashSet::from([CardId(2)]),
        }],
    };
    event_groups.insert(EventTag::up(), up_group);

    let deck = Deck {
        id: DeckId(1),
        name: "Test-Deck".into(),
        members,
        event_groups,
    };

    let tags = vec![Tag::new("game", "genshin")];
    let event_tags = vec![EventTag::up()];
    let result = deck.query_cards(&registry, &tags, &event_tags);
    assert_eq!(result, vec![CardId(2)]);
}

// Deck::query_cards 活动组不存在的情况
#[test]
fn deck_query_event_group_missing() {
    let registry = CardRegistry::new();
    registry.insert(create_test_card(1, vec![("game", "genshin")]));

    let members = Membership {
        conditions: vec![MembershipCondition::IncludeIds {
            ids: HashSet::from([CardId(1)]),
        }],
    };

    let deck = Deck {
        id: DeckId(1),
        name: "Test-Deck".into(),
        members,
        event_groups: HashMap::new(),
    };

    let tags = vec![Tag::new("game", "genshin")];
    let result = deck.query_cards(&registry, &tags, &[EventTag::up()]);
    assert!(result.is_empty());
}

// 完整的原神角色 UP 卡组吗模拟测试
#[test]
fn deck_query_genshin_character_up_simulation() {
    let registry = CardRegistry::new();
    // 插入模拟卡片
    // 4 为常驻, 5 为 UP
    registry.insert(create_test_card(1, vec![("game", "genshin"), ("rarity", "3"), ("type", "weapon")]));
    registry.insert(create_test_card(2, vec![("game", "genshin"), ("rarity", "4"), ("type", "character")]));
    registry.insert(create_test_card(3, vec![("game", "genshin"), ("rarity", "4"), ("type", "weapon")]));
    registry.insert(create_test_card(4, vec![("game", "genshin"), ("rarity", "5"), ("type", "character")]));
    registry.insert(create_test_card(5, vec![("game", "genshin"), ("rarity", "5"), ("type", "character")]));
    registry.insert(create_test_card(6, vec![("game", "starrail"), ("rarity", "5"), ("type", "character")]));

    // members: 所有原神角色(3星、4星、5星)
    let members_rule = Membership {
        conditions: vec![
            MembershipCondition::TagAll {
                tags: vec![Tag::new("game", "genshin"), Tag::new("rarity", "3")],
            },
            MembershipCondition::TagAll {
                tags: vec![Tag::new("game", "genshin"), Tag::new("rarity", "4")],
            },
            // 添加常驻 5 星和 UP 5 星
            MembershipCondition::IncludeIds {
                ids: HashSet::from([CardId(4), CardId(5)]),
            },
        ],
    };

    // up 组: UP 5 星 + 指定 UP  4 星
    let mut event_groups = HashMap::new();
    let up_group = EventGroup {
        conditions: vec![
            EventGroupCondition::IncludeIds {
                ids: HashSet::from([CardId(5), CardId(2)]),
            },
        ],
    };
    event_groups.insert(EventTag::up(), up_group);

    // standard组: 所有成员 排除 up 组 (这里用排除 Id 模拟, 实际可用 ExcludeGroups 但暂未实现)
    let standard_group = EventGroup {
        conditions: vec![
            EventGroupCondition::All,                       // 拉取 members 中所有卡片
            EventGroupCondition::ExcludeIds {
                ids: HashSet::from([CardId(5), CardId(2)]), // 排除UP
            },
        ],
    };
    event_groups.insert(EventTag::standard(), standard_group);

    let deck = Deck {
        id: DeckId(1),
        name: "Genshin Character UP".into(),
        members: members_rule,
        event_groups,
    };

    // 查询 up 组(普通标签仅 game=genshin 和 type=character 应在成员中)
    let tags = vec![Tag::new("game", "genshin"), Tag::new("type", "character")];
    let result_up = deck.query_cards(&registry, &tags, &[EventTag::up()]);

    let result_up_set: HashSet<_> = result_up.into_iter().collect();
    assert_eq!(result_up_set, HashSet::from([CardId(5), CardId(2)]));

    let result_std = deck.query_cards(&registry, &tags, &[EventTag::standard()]);
    let result_std_set: HashSet<_> = result_std.into_iter().collect();

    // 卡片4是常驻 5 星, 且常驻组里只有卡片 4 为角色
    assert_eq!(result_std_set, HashSet::from([CardId(4)]));
}
