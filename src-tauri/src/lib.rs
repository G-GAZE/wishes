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
    app::state::AppState, domain::ids::BannerId, interface::{
        banner_info::{BannerInfo, BannerSummary}, catalog_stats::CatalogStats, wish_response::WishResponse
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
    let state: &AppState = state.inner();       // rust-analyzer 在此无法给出自动补全提示, 手动解引用以帮助补全
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
    let state: &AppState = state.inner();
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
    let state: &AppState = state.inner();
    state.banner_service.get_banner_info(banner_id)
        .map_err(|e| e.to_string())
}

/// 获取当前图鉴的统计信息.
#[tauri::command]
fn get_catalog_stats(state: State<AppState>) -> Result<CatalogStats, String> {
    let state: &AppState = state.inner();
    Ok(state.get_catalog_stats())
}

/// 获取当前使用的数据目录路径.
#[tauri::command]
fn get_data_dir_path(state: State<AppState>) -> Result<String, String> {
    Ok(state.data_dir.to_string_lossy().into_owned())
}

/// `Tauri` 应用启动入口.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let data_dir = get_or_create_data_dir(&handle)
                .with_context(|| "获取数据目录失败")?;
            
            let app_state = AppState::load(&data_dir).with_context(|| "Wishes 启动失败")?;
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running Wishes");
}
