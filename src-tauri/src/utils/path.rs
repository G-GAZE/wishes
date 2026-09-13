//! # 数据目录路径处理
//! 
//! 负责确定应用的数据目录位置, 并在必要时从资源目录复制默认数据.
//! 开发环境下优先使用项目目录下的 `data` 文件夹, 生产环境下使用用户数据目录.

use std::{fs, path::{Path, PathBuf}};
use anyhow::{Context, Result};
use tauri::{AppHandle, Manager, path::BaseDirectory};

/// 获取最终使用的数据目录, 若目标目录不存在则资源中复制默认内容到目标目录.
/// 
/// # 行为
/// - **开发环境**: 检查项目目录下的 `data` 文件夹是否存在且有效,
///   若存在则直接使用.
/// - **生产环境**: 使用用户数据目录中的 `data` 目录, 若不存在,
/// 则从资源目录中的 `data` 目录复制默认数据.
/// 
/// # 返回值
/// 返回最终可用的数据目录路径, 若任何步骤失败则返回错误.
pub fn get_or_create_data_dir(handle: &AppHandle) -> Result<PathBuf> {
    // 开发环境下使用项目数据
    #[cfg(debug_assertions)]
    {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let dev_dir = PathBuf::from(manifest_dir).join("../data");
            if dev_dir.exists() && check_data_dir(&dev_dir) {
                println!("DEV - 使用开发环境数据目录: {:?}", &dev_dir);
                return Ok(dev_dir);
            }
        }
    }

    // 生成环境下使用用户数据目录数据
    let user_data_dir = handle
        .path()
        .app_data_dir()
        .with_context(|| "无法获取用户数据目录")?;
    let target = user_data_dir.join("data");

    if !target.exists() {
        // std::fs::create_dir_all(&target)?;
        // let resource_dir = handle
        //     .path()
        //     .resource_dir()
        //     .with_context(|| "无法获取资源目录")?;
        // let source = resource_dir.join("data");

        let source = handle
            .path()
            .resolve("data", BaseDirectory::Resource)
            .with_context(|| "无法解析资源路径 'data'")?;

        if source.exists() && check_data_dir(&source) {
            copy_dir_all(&source, &target)?;
        }
    }

    Ok(target)
}

/// 获取最终使用的数据库目录, 若不存在则自动创建
/// 
/// # 行为
/// - **开发环境**: 使用项目根目录下的 db 目录, 若不存在则创建.
/// - **生产环境**: 使用用户数据目录下的 db 目录, 若不存在则创建.
/// 
/// # 返回值
/// 返回最终可用的数据库目录路径, 若任何步骤失败则返回错误.
pub fn get_or_create_db_dir(handle: &AppHandle) -> Result<PathBuf> {
    #[cfg(debug_assertions)]
    {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let dev_dir = PathBuf::from(manifest_dir).join("../db");
            if !dev_dir.exists() {
                fs::create_dir_all(&dev_dir).with_context(|| "创建 db 目录失败")?;
            }
            println!("DEV - 使用开发环境数据库目录: {:?}", &dev_dir);
            return Ok(dev_dir);
        }
    }
    
    let user_data_dir = handle
        .path()
        .app_data_dir()
        .with_context(|| "无法获取用户数据目录")?;

    let target = user_data_dir.join("db");

    if !target.exists() {
        fs::create_dir_all(&target).with_context(|| "创建 db 目录失败")?;
    }

    Ok(target)
}

/// 辅助函数, 递归复制整个目录及其内容 (保留原目录结构).
/// 
/// 若目录不存在则自动创建, 保留源目录的目录结构和文件.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.as_ref().join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// 检查数据目录是否有效.
/// 
/// 当前仅验证路径是否为目录.
pub fn check_data_dir(dir: &Path) -> bool {
    if dir.is_dir() {
        // TODO: 检查必要子目录是否存在
        true
    } else {
        false
    }
}