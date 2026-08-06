use serde::Serialize;

use crate::domain::tag::Tag;



/// 卡池的简略信息
#[derive(Debug, Serialize)]
pub struct BannerSummary {
    pub id: u64,
    pub name: String,
    pub tags: Vec<Tag>,
}


/// 卡池的详细信息
#[derive(Debug, Serialize)]
pub struct BannerInfo {
    pub id: u64,
    pub name: String,
    pub tags: Vec<Tag>,
    pub deck_name: String,
    pub logic_name: String,
    pub total_counter: u32,
}