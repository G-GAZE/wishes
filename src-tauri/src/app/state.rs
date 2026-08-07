use std::{path::{Path, PathBuf}, sync::Arc};

use anyhow::{Ok, Result};

use crate::{app::{banner_service::BannerService, logic_engine::LogicEngine}, domain::{ids::BannerId, wish_result::WishResult}, infrastructure::{registry::{BannerRegistry, CardRegistry, DeckRegistry, LogicRegistry}, repository::state_repo::StateRepository}, interface::catalog_stats::CatalogStats};
use super::loader::Loader;


pub struct AppState {
    pub card_registry: Arc<CardRegistry>,
    pub deck_registry: Arc<DeckRegistry>,
    pub logic_registry: Arc<LogicRegistry>,
    pub banner_registry: Arc<BannerRegistry>,

    pub logic_engine: Arc<LogicEngine>,
    pub banner_service: Arc<BannerService>,

    pub data_dir: PathBuf,
}

impl AppState {
    pub fn load(data_dir: &Path) -> Result<Self> {
        let loader = Loader::new();

        let card_registry = loader.load_cards_from_dir(&data_dir.join("cards"))?;
        println!("OK - 已加载 {} 个 Card", card_registry.count());

        let deck_registry = loader.load_decks_from_dir(
            &data_dir.join("decks"),
            &card_registry
        )?;
        println!("OK - 已加载 {} 个 Deck", deck_registry.count());

        let mut logic_registry = LogicRegistry::new();
        loader.load_logics_from_dir(
            &data_dir.join("logics"),
            &mut logic_registry
        )?;
        println!("OK - 已加载 {} 个 Logic 定义", logic_registry.count_definitions());

        let banner_registry = loader.load_banner_from_dir(
            &data_dir.join("banners"),
            &card_registry,
            &deck_registry,
            &logic_registry
        )?;
        println!("OK - 已加载 {} 个 Banner", banner_registry.count());
        
        // 转为 Arc 指针
        let card_registry = Arc::new(card_registry);
        let deck_registry = Arc::new(deck_registry);
        let logic_registry = Arc::new(logic_registry);
        let banner_registry = Arc::new(banner_registry);
        
        // 初始化逻辑执行引擎
        let logic_engine = Arc::new(LogicEngine::new(logic_registry.clone()));

        // 连接逻辑状态数据库
        let state_repository = Arc::new(StateRepository::new(&data_dir.join("states.db"))?);

        let banner_service = Arc::new(BannerService::new(
            card_registry.clone(),
            deck_registry.clone(),
            logic_registry.clone(),
            banner_registry.clone(),
            logic_engine.clone(),
            state_repository.clone(),
        ));

        banner_service.load_banner_state()?;

        Ok(Self {
            card_registry,
            deck_registry,
            logic_registry,
            banner_registry,
            logic_engine,
            banner_service,
            data_dir: data_dir.to_path_buf(),
        })
    }

    pub fn wish(&self, banner_id: BannerId) -> Result<WishResult> {
        self.banner_service.wish(banner_id)
    }

    pub fn get_catalog_stats(&self) -> CatalogStats {
        CatalogStats {
            cards: self.card_registry.count(),
            decks: self.deck_registry.count(),
            logics: self.logic_registry.count_definitions(),
        }
    }
}