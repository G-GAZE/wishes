//! # 卡池信息
//! 
//! 提供卡池的摘要信息 (`BannerSummary`) 和 详细信息 (`BannerInfo`), 
//! 分别用于列表展示和详情页展示.

use serde::Serialize;
use crate::domain::tag::Tag;

/// 卡池的摘要信息, 用于列表或概览展示.
#[derive(Debug, Serialize)]
pub struct BannerSummary {
    /// 卡池 Id.
    pub id: u64,
    /// 卡池名称.
    pub name: String,
    /// 卡池拥有的标签.
    pub tags: Vec<Tag>,
}

/// 卡池的详细信息, 用于详情页或管理界面.
#[derive(Debug, Serialize)]
pub struct BannerInfo {
    /// 卡池 Id.
    pub id: u64,
    /// 卡池名称.
    pub name: String,
    /// 卡池拥有的标签.
    pub tags: Vec<Tag>,
    /// 卡池所引用的卡组名称.
    pub deck_name: String,
    /// 卡池所引用的逻辑定义名称.
    pub logic_name: String,
    /// 该卡池的总抽卡次数 (来自卡池状态中自动维护的总计数器).
    pub total_counter: u32,
}