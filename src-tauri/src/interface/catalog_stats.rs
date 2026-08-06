use serde::Serialize;



/// 简略的统计信息
#[derive(Debug, Clone, Serialize)]
pub struct CatalogStats {
    pub cards: usize,
    pub decks: usize,
    pub logics: usize,
}