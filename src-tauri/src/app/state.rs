//! # 应用全局状态
//! 
//! 集成所有注册器、核心服务，并提供统一的初始化和操作入口.
//! 在应用启动时从文件系统加载所有数据, 并建立服务间的依赖关系.


use std::{path::{Path, PathBuf}, sync::Arc};
use anyhow::{Ok, Result};
use crate::{
    app::{
        admin::{card_manager::CardManager, deck_manager::DeckManager}, banner_service::BannerService, logic_engine::LogicEngine
    }, domain::{
        ids::BannerId,
        wish_result::WishResult
    }, infrastructure::{
        registry::{
            BannerRegistry,
            CardRegistry,
            DeckRegistry,
            LogicRegistry
        },
        repository::state_repo::StateRepository
    }, interface::catalog_stats::CatalogStats
};
use super::loader::Loader;

/// 应用状态, 持有所有注册器和服务实例.
/// 
/// 作为顶层容器, 提供对外的接口方法.
pub struct AppState {
    /// 卡片注册器引用
    pub card_registry: Arc<CardRegistry>,
    /// 卡组注册器引用.
    pub deck_registry: Arc<DeckRegistry>,
    /// 逻辑注册器引用.
    pub logic_registry: Arc<LogicRegistry>,
    /// 卡池注册器引用.
    pub banner_registry: Arc<BannerRegistry>,

    /// 卡片管理器实例.
    pub card_manager: CardManager,
    /// 卡组管理器实例.
    pub deck_manager: DeckManager,

    /// 逻辑引擎引用.
    pub logic_engine: Arc<LogicEngine>,

    /// 卡池服务实例.
    pub banner_service: BannerService,

    /// 存储当前使用的数据目录路径.
    pub data_dir: PathBuf,
}

impl AppState {
    /// 从指定数据目录加载所有配置和状态, 构建完整的 `AppState`.
    /// 
    /// # 流程
    /// 1. 使用 `Loader` 加载卡片、卡组、逻辑定义和卡池到注册器。
    /// 2. 将注册器包装为 `Arc`.
    /// 3. 创建逻辑引擎和卡池服务.
    /// 4. 打开状态数据库并加载所有卡池的持久化状态.
    /// 
    /// # 错误
    /// 任何加载或初始化步骤失败都将返回错误.
    pub fn load(data_dir: &Path, db_dir: &Path) -> Result<Self> {
        let loader = Loader::new();

        let card_registry = loader.load_cards_from_dir(&data_dir.join("cards"))?;
        println!("OK - 已加载 {} 个 Card", card_registry.count());

        let deck_registry = loader.load_decks_from_dir(
            &data_dir.join("decks")
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

        // 初始化管理器
        let card_manager = CardManager::new(card_registry.clone(), data_dir.join("cards"));
        let deck_manager = DeckManager::new(
            deck_registry.clone(),
            card_registry.clone(),
            banner_registry.clone(),
            logic_registry.clone(),
            data_dir.join("decks")
        );
        
        // 初始化逻辑执行引擎
        let logic_engine = Arc::new(LogicEngine::new(logic_registry.clone()));

        // 连接逻辑状态数据库
        let state_repository = Arc::new(StateRepository::new(&db_dir.join("states.db"))?);

        let banner_service = BannerService::new(
            card_registry.clone(),
            deck_registry.clone(),
            logic_registry.clone(),
            banner_registry.clone(),
            logic_engine.clone(),
            state_repository.clone(),
        );

        banner_service.load_banner_state()?;

        Ok(Self {
            card_registry,
            deck_registry,
            logic_registry,
            banner_registry,
            card_manager,
            deck_manager,
            logic_engine,
            banner_service,
            data_dir: data_dir.to_path_buf(),
        })
    }

    /// 执行一次抽卡, 委托给 `banner_service`.
    pub fn wish(&self, banner_id: BannerId) -> Result<WishResult> {
        self.banner_service.wish(banner_id)
    }

    /// 获取图鉴统计信息.
    pub fn get_catalog_stats(&self) -> CatalogStats {
        CatalogStats {
            cards: self.card_registry.count(),
            decks: self.deck_registry.count(),
            logics: self.logic_registry.count_definitions(),
        }
    }
}