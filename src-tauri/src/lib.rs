//! # Wishes 应用入口
//! 
//! `Tauri` 应用程序主入口.

pub mod app;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod utils;
use tauri::{Manager, State};
use anyhow::{Context, Result};
use crate::{
    app::state::AppState, domain::{ids::{BannerId, CardId}, tag::Tag}, interface::{
        banner_info::{BannerInfo, BannerSummary}, card_response::{CardCreateRequest, CardSummary, CardUpdateRequest}, catalog_stats::CatalogStats, wish_response::WishResponse
    }, utils::path::get_or_create_data_dir
};

/// 执行单次抽卡
/// 
/// # 参数
/// - `banner_id`: 目标卡池的 Id (`u64`).
/// - `state`: 当前应用状态.
/// 
/// # 返回值
/// 返回 `WishResponse` 包含抽到的卡片信息, 若失败则返回错误信息.
#[tauri::command]
fn wish(banner_id: u64, state: State<AppState>) -> Result<WishResponse, String> {
    let banner_id = BannerId(banner_id);
    match state.wish(banner_id) {
        Ok(result) => {
            Ok(WishResponse::new(result))
        },
        Err(e) => Err(e.to_string())
    }
}

/// 获取所有卡池的摘要列表.
/// 
/// 返回 `Vec<BannerSummary>` 供前端展示.
#[tauri::command]
fn get_banners(state: State<AppState>) -> Result<Vec<BannerSummary>, String> {
    Ok(state.banner_service.get_banner_summaries())
}

/// 获取指定卡池的详细信息.
/// 
/// # 参数
/// - `banner_id`: 卡池 Id (`u64`).
/// 
/// 返回 `BannerInfo` 包含卡池名称、标签、关联卡组/逻辑名称及总抽数.
#[tauri::command]
fn get_banner_info(banner_id: u64, state: State<AppState>) -> Result<BannerInfo, String> {
    let banner_id = BannerId(banner_id);
    state.banner_service.get_banner_info(banner_id)
        .map_err(|e| e.to_string())
}

/// 获取当前图鉴的统计信息.
#[tauri::command]
fn get_catalog_stats(state: State<AppState>) -> Result<CatalogStats, String> {
    Ok(state.get_catalog_stats())
}

/// 获取当前使用的数据目录路径.
#[tauri::command]
fn get_data_dir_path(state: State<AppState>) -> Result<String, String> {
    Ok(state.data_dir.to_string_lossy().into_owned())
}

/// 获取所有卡片信息.
#[tauri::command]
fn list_cards(state: State<AppState>) -> Result<Vec<CardSummary>, String> {
    let cards = state.card_manager.list_all();
    Ok(cards.iter().map(|card| CardSummary::from(card.as_ref())).collect())
}

/// 获取根据标签筛选后的卡片的信息.
#[tauri::command]
fn list_cards_by_tags(tags: Vec<Tag>, state: State<AppState>) -> Result<Vec<CardSummary>, String> {
    let cards = state.card_manager.list_by_tags(&tags);
    Ok(cards.iter().map(|card| CardSummary::from(card.as_ref())).collect())
}

/// 创建新的卡片.
#[tauri::command]
fn create_card(req: CardCreateRequest, state: State<AppState>) -> Result<CardSummary, String> {
    let card = state.card_manager.create_card(req.content, req.tags)
        .map_err(|e| e.to_string())?;
    Ok(CardSummary::from(&card))
}

/// 更新卡片信息.
#[tauri::command]
fn update_card(req: CardUpdateRequest, state: State<AppState>) -> Result<CardSummary, String> {
    let id = CardId(req.id);
    let card = state.card_manager.update_card(
        id, 
        req.new_content,
        req.new_tags
    ).map_err(|e| e.to_string())?;
    Ok(CardSummary::from(&card))
}

/// 删除卡片.
#[tauri::command]
fn delete_card(id: u64, state: State<AppState>) -> Result<(), String> {
    let id = CardId(id);
    state.card_manager.delete_card(id).map_err(|e| e.to_string())?;
    Ok(())
}

/// `Tauri` 应用启动入口.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_env_filter("wishes=debug")
        .with_target(false)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_env_filter("wishes=info")
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let data_dir = get_or_create_data_dir(&handle)
                .with_context(|| "获取数据目录失败")?;
            
            let app_state = AppState::load(&data_dir).map_err(|e| {
                tracing::error!("Wishes 启动失败");
                for cause in e.chain() {
                    tracing::error!("- {}", cause);
                }
                e
            })?;
            app.manage(app_state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            wish,
            get_banners,
            get_banner_info,
            get_catalog_stats,
            get_data_dir_path,
            list_cards,
            list_cards_by_tags,
            create_card,
            update_card,
            delete_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Wishes");
}
