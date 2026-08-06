use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};


/// 获取最终使用的 data 目录
/// 若不存在则从资源中复制默认内容
pub fn get_or_create_data_dir(handle: &AppHandle) -> Result<PathBuf> {
    // 开发环境下使用项目数据
    #[cfg(debug_assertions)]
    {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let dev_dir = PathBuf::from(manifest_dir).join("data");
            if dev_dir.exists() && check_data_dir(&dev_dir) {
                println!("DEV - 使用开发环境数据目录: {:?}", &dev_dir);
                return Ok(dev_dir);
            }
        }
    }

    // 生成环境下使用用户数据目录数据
    let user_data_dir = handle
        .path()
        .data_dir()
        .with_context(|| "无法获取用户数据目录")?;
    let target = user_data_dir.join("wishes").join("data");

    if !target.exists() {
        std::fs::create_dir_all(&target)?;
        let resource_dir = handle
            .path()
            .resource_dir()
            .with_context(|| "无法获取资源目录")?;
        let source = resource_dir.join("data");
        if source.exists() && check_data_dir(&source) {
            copy_dir_all(&source, &target)?;
        }
    }

    Ok(target)
}

/// 递归复制整个目录
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


/// 检查数据目录
pub fn check_data_dir(dir: &Path) -> bool {
    if dir.is_dir() {
        // TODO: 检查必要子目录是否存在
        true
    } else {
        false
    }
}