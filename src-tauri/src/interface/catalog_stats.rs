//! # 图鉴统计信息
//! 
//! 提供当前已加载的各类资源的数量统计

use serde::Serialize;

/// 简略的统计信息, 反映当前内存中各注册器的资源数量.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogStats {
    /// 已加载的卡片 (`Card`) 数量.
    pub cards: usize,
    /// 已加载的卡组 (`Deck`) 数量.
    pub decks: usize,
    /// 已加载的逻辑定义 (`LogicDefinition`) 数量.
    pub logics: usize,
}