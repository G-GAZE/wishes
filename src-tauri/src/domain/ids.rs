use serde::{Serialize, Deserialize};

// 所有实体对象的 Id

// 卡片
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardId(pub u64);
impl From<u64> for CardId {
    fn from(id: u64) -> Self {
        CardId(id)
    }
}


// 卡组
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckId(pub u64);
impl From<u64> for DeckId {
    fn from(id: u64) -> Self {
        DeckId(id)
    }
}


// 抽卡逻辑 (定义)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogicId(pub u64);
impl From<u64> for LogicId {
    fn from(id: u64) -> Self {
        LogicId(id)
    }
}


// 抽卡规则
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WishRuleLocalId(pub String);
impl From<String> for WishRuleLocalId {
    fn from(value: String) -> Self {
        WishRuleLocalId(value)
    }
}


// 卡池
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BannerId(pub u64);
impl From<u64> for BannerId {
    fn from(id: u64) -> Self {
        BannerId(id)
    }
}


// 预留
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub u64);
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}