//! # 状态数据库
//! 
//! 将 用户-卡池 的逻辑实例状态 (`serde_json::Value`) 持久化到 SQLite 数据库.
//! 表 `instance_states` 以 `(user_id, banner_id)` 为复合主键, 状态以 JSON 字符串形式存储.

use std::path::Path;
use parking_lot::Mutex;
use anyhow::Result;
use rusqlite::{Connection, Error, ToSql, types::{ToSqlOutput, Value as SqlValue}};
use serde_json::Value as JsonValue;
use crate::domain::ids::{BannerId, UserId};

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
                    user_id INTEGER NOT NULL,
                    banner_id INTEGER NOT NULL,
                    state_json Text,
                    PRIMARY KEY (user_id, banner_id)
                )
            "#,
            []
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// 加载指定用户和卡池的状态, 若不存在则返回 `None`.
    pub fn load(&self, user_id: UserId, banner_id: BannerId) -> Result<Option<JsonValue>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT state_json FROM instance_states WHERE user_id = ?1 AND banner_id = ?2")?;
        let mut rows = stmt.query(rusqlite::params![user_id, banner_id])?;
        if let Some(row) = rows.next()? {
            let state_str: String = row.get(0)?;
            let state_json: JsonValue = serde_json::from_str(&state_str)?;
            Ok(Some(state_json))
        } else {
            Ok(None)
        }
    }

    /// 保存或更新指定用户和卡池的状态, 返回插入/更新后的行 Id (`last_insert_rowid`).
    pub fn save(&self, user_id: UserId, banner_id: BannerId, state_json: &JsonValue) -> Result<i64> {
        let conn = self.conn.lock();
        conn.execute(
            r#"
                INSERT INTO instance_states (user_id, banner_id, state_json) VALUES (?1, ?2, ?3)
                ON CONFLICT(user_id, banner_id) DO UPDATE SET state_json = excluded.state_json;
            "#,
            rusqlite::params![user_id, banner_id, state_json.to_string()]
        )?;
        Ok(conn.last_insert_rowid())
    }
}