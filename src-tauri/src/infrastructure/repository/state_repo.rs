//! # 状态数据库
//! 
//! 将 用户-卡池 的逻辑实例状态 (`serde_json::Value`) 持久化到 SQLite 数据库.
//! 表 `instance_states` 以 `(user_id, banner_id)` 为复合主键, 状态以 JSON 字符串形式存储.

use std::path::Path;
use parking_lot::Mutex;
use anyhow::{Context, Result};
use rusqlite::{Connection, Error, ToSql, types::{ToSqlOutput, Value as SqlValue}};
use serde_json::Value as JsonValue;
use crate::domain::{ids::{BannerId, UserId}, banner::BannerRuntimeState};

/// 为 `UserId` 实现 `ToSql`, 将其 `u64` 值转为 `i64` 以适配 SQLite 整数.
/// 
/// # 注意
/// 要求 `UserId.0` 不超过 `i64::MAX`, 否则转换将失败并返回 `rusqlite::Error::ToSqlConversionFailure`.
impl ToSql for UserId {                                                         // Id 的 u64 值转为 i64 再转 sql 数据类型
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {    // 需确保 Id 值不会超过 i64::MAX
        let id_i64 = i64::try_from(self.0)
            .map_err(|_| Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("User Id 转换至 sql 失败 (值 {} 超出 i64::MAX)", self.0)
            ))))?;
        Ok(ToSqlOutput::Owned(SqlValue::Integer(id_i64)))
    }
}

/// 为 `BannerId` 实现 `ToSql`, 逻辑同 `UserId`.
impl ToSql for BannerId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let id_i64 = i64::try_from(self.0)
            .map_err(|_| Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Banner Id 转换至 sql 失败 (值 {} 超出 i64::MAX)", self.0)
            ))))?;
        Ok(ToSqlOutput::Owned(SqlValue::Integer(id_i64)))
    }
}

/// 状态数据库.
/// 
/// 包含一个 SQLite 连接, 内部使用 `Mutex` 保证线程安全.
/// 
/// `Mutex` 由 `parking_lot` crate 提供.
pub struct StateRepository {
    conn: Mutex<Connection>,
}

impl StateRepository {
    /// 打开或创建数据库, 并初始化 `instance_states` 表.
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            r#"
                CREATE TABLE IF NOT EXISTS instance_states (
                    user_id         INTEGER NOT NULL,
                    banner_id       INTEGER NOT NULL,
                    total_counter   INTEGER NOT NULL DEFAULT 0,
                    logic_state     Text,
                    PRIMARY KEY     (user_id, banner_id)
                )
            "#,
            []
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// 加载指定用户和卡池的状态, 若不存在则返回 `None`.
    pub fn load(&self,
        user_id: UserId,
        banner_id: BannerId
    ) -> Result<Option<BannerRuntimeState>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT total_counter, logic_state FROM instance_states WHERE user_id = ?1 AND banner_id = ?2"
        )?;
        let mut rows = stmt.query(rusqlite::params![user_id, banner_id])?;     
        if let Some(row) = rows.next()? {
            let total_counter_i64: i64 = row.get(0)?;
            let total_counter = u64::try_from(total_counter_i64)
                .with_context(|| format!(
                    "Banner {} 的 total_counter (value {}) 无法转换为 u64",
                    banner_id.0, total_counter_i64
                ))?;
            
            let logic_state_str: Option<String> = row.get(1)?;
            let logic_state = match logic_state_str {
                Some(s) => serde_json::from_str(&s)
                    .with_context(|| format!(
                        "Banner {} 的 logic_state 反序列化失败",
                        banner_id.0
                    ))?,
                None => JsonValue::Null,
            };

            Ok(Some(BannerRuntimeState{
                total_counter,
                logic_state,
            }))
        } else {
            Ok(None)
        }
    }

    /// 保存或更新指定用户和卡池的状态, 返回插入/更新后的行 Id (`last_insert_rowid`).
    pub fn save(&self,
        user_id: UserId,
        banner_id: BannerId,
        state: &BannerRuntimeState
    ) -> Result<()> {
        let total_counter_i64 = i64::try_from(state.total_counter)
            .with_context(|| format!(
                "Banner {} 的 total_counter (value {}) 无法转换为 i64",
                banner_id.0, state.total_counter
            ))?;
        
        let logic_state_str = if state.logic_state.is_null() {
            None
        } else {
            Some(
                serde_json::to_string(&state.logic_state)
                    .with_context(|| format!(
                        "Banner {} 的 logic_state 序列化为 JSON 文本失败",
                        banner_id.0
                    ))?
            )
        };
        
        let conn = self.conn.lock();
        conn.execute(
            r#"
                INSERT INTO instance_states (user_id, banner_id, total_counter, logic_state)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(user_id, banner_id) DO UPDATE SET
                    total_counter = excluded.total_counter,
                    logic_state = excluded.logic_state;
            "#,
            rusqlite::params![user_id, banner_id, total_counter_i64, logic_state_str]
        )?;
        Ok(())
    }
}