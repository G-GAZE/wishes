//! # 领域对象的强类型 Id 定义
//! 
//! 所有 Id 均为 `NewType`, 包装 `u64` 并提供类型安全区分 (`RuleLocalId` 为 String).
//! 它们均实现了 `Copy`, `Eq`, `Hash`, `Serialize`, `Deserialize` 等 trait.


use serde::{Serialize, Deserialize};

/// 卡片 Id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardId(pub u64);
impl From<u64> for CardId {
    fn from(id: u64) -> Self {
        CardId(id)
    }
}

/// 卡组 Id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckId(pub u64);
impl From<u64> for DeckId {
    fn from(id: u64) -> Self {
        DeckId(id)
    }
}

/// 逻辑定义 Id (对应到 `LogicDefinition`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogicId(pub u64);
impl From<u64> for LogicId {
    fn from(id: u64) -> Self {
        LogicId(id)
    }
}

/// 抽卡规则 Id (逻辑内部的唯一标识, String 形式)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WishRuleLocalId(pub String);
impl From<String> for WishRuleLocalId {
    fn from(value: String) -> Self {
        WishRuleLocalId(value)
    }
}

/// 卡池 Id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BannerId(pub u64);
impl From<u64> for BannerId {
    fn from(id: u64) -> Self {
        BannerId(id)
    }
}

/// 用户 Id (用于多用户隔离, 目前阶段固定为 0)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub u64);
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}