//! # 卡片 Card 定义
//! 包含卡片的基本信息.

use serde::{Serialize, Deserialize};
use super::tag::Tagged;
use super::ids::CardId;

/// 卡片核心数据 (不包含标签 `Tag`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 唯一标识.
    pub id: CardId,
    /// 卡片内容 (不同卡片允许重复).
    pub content: String,
    // NOTE: title 和 asset 待后续补充
}

/// 带标签的卡片, 实际存储和传递的类型.
/// 即 `Tagged<Card>`.
pub type TaggedCard = Tagged<Card>;
