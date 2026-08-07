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

#[tauri::command]
fn get_banners(state: State<AppState>) -> Result<Vec<BannerSummary>, String> {
    let state: &AppState = state.inner();
    Ok(state.banner_service.get_banner_summaries())
}

#[tauri::command]
fn get_banner_info(banner_id: u64, state: State<AppState>) -> Result<BannerInfo, String> {
    let banner_id = BannerId(banner_id);
    let state: &AppState = state.inner();
    state.banner_service.get_banner_info(banner_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_catalog_stats(state: State<AppState>) -> Result<CatalogStats, String> {
    let state: &AppState = state.inner();
    Ok(state.get_catalog_stats())
}

#[tauri::command]
fn get_data_dir_path(state: State<AppState>) -> Result<String, String> {
    Ok(state.data_dir.to_string_lossy().into_owned())
}

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
