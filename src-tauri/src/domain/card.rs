//! # 卡片 Card 定义
//! 包含卡片的基本信息.

use serde::{Serialize, Deserialize};
use crate::domain::{ids::GlobalId, localized_string::LocalizedString, origin::Origin, tag::Tagged};


/// 卡片核心数据 (不包含标签 `Tag`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 全局唯一标识 (创建时生成, 永不变).
    pub global_id: GlobalId,

    /// 对象来源.
    /// 
    /// 即使是默认值 `"local"` 也**始终序列化**.
    pub origin: Origin,

    /// 派生自哪个对象的 `global_id`.
    /// 
    /// 非派生对象 (官方或拓展包对象) **不应**出现该字段.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forked_from: Option<GlobalId>,

    /// 卡片主内容 (至少一种语言).
    pub content: LocalizedString,

    /// 称号 / 副标题, 可选字段, 仅部分卡片需要.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<LocalizedString>,

    /// 立绘等资源引用.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<CardAssets>,
}

/// 卡片资产引用.
/// 
/// 所有字段都是**相对于所属来源 `assets/` 目录**的路径,
/// 如 `"cards/0192a3c4-....png"` 对应 `data/official/assets/cards/0192a3c4-....png`.
/// 
/// # 缺失资产
/// 前端渲染时若文件不存在则显示占位图, 不报错.
/// 加载器启动时**不检查**资产文件存在性, 以避免大量文件 I/O.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardAssets {
    /// 立绘 (主资产).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,

    /// 缩略图.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    /// 动画 (预留).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<String>,
}

impl CardAssets {
    /// 是否所有资产引用都为空.
    pub fn is_empty(&self) -> bool {
        self.portrait.is_none() && self.thumbnail.is_none() && self.animation.is_none()
    }
}

/// 带标签的卡片, 实际存储和传递的类型.
/// 即 `Tagged<Card>`.
pub type TaggedCard = Tagged<Card>;
