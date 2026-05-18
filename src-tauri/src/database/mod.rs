// 数据库模块

use rusqlite::{Connection, Result};
use tauri::{AppHandle, Manager};
use std::path::PathBuf;

/// 初始化数据库并返回连接
pub fn init_database_with_connection(app_handle: &AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    let db_path = get_db_path(app_handle)?;

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 连接或创建数据库
    let conn = Connection::open(&db_path)?;

    // 启用外键约束
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    // 运行迁移
    run_migrations(&conn)?;

    log::info!("Database initialized at: {:?}", db_path);
    Ok(conn)
}

/// 初始化数据库 (旧接口,保留兼容)
pub fn init_database(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    init_database_with_connection(app_handle)?;
    Ok(())
}

/// 获取数据库文件路径
fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("./data"));

    Ok(app_data_dir.join("lingomate.db"))
}

/// 运行数据库迁移
fn run_migrations(conn: &Connection) -> Result<()> {
    // 检查 sessions 表是否存在以及是否需要迁移
    let needs_migration = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sessions'",
        [],
        |row| row.get::<_, i64>(0)
    ).unwrap_or(0) > 0;
    
    if needs_migration {
        // 检查旧表的 CHECK 约束是否存在（需要迁移的标志）
        let sql = conn.query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name='sessions'",
            [],
            |row| row.get::<_, String>(0)
        ).unwrap_or_default();
        
        // 如果旧表有 CHECK 约束，需要迁移到无约束版本
        if sql.contains("CHECK(scenario IN") {
            log::info!("Migrating sessions table to remove scenario constraint...");
            
            // 1. 备份数据
            conn.execute_batch(
                r#"
                CREATE TABLE sessions_backup AS SELECT * FROM sessions;
                DROP TABLE sessions;
                "#,
            )?;
            
            // 2. 创建新表（移除场景 CHECK 约束，支持自定义场景）
            conn.execute_batch(
                r#"
                CREATE TABLE sessions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL DEFAULT 'New Conversation',
                    scenario TEXT NOT NULL,
                    proficiency_level TEXT NOT NULL DEFAULT 'intermediate'
                        CHECK(proficiency_level IN ('beginner', 'intermediate', 'advanced')),
                    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    message_count INTEGER NOT NULL DEFAULT 0
                );
                "#,
            )?;
            
            // 3. 恢复所有数据
            conn.execute_batch(
                r#"
                INSERT INTO sessions (id, title, scenario, proficiency_level, created_at, updated_at, message_count)
                SELECT id, title, scenario, proficiency_level, created_at, updated_at, message_count
                FROM sessions_backup;
                DROP TABLE sessions_backup;
                "#,
            )?;
            
            log::info!("Sessions table migrated successfully - scenario constraint removed");
        }
    }
    
    // 创建或确保 sessions 表存在（不再限制场景）
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL DEFAULT 'New Conversation',
            scenario TEXT NOT NULL,
            proficiency_level TEXT NOT NULL DEFAULT 'intermediate'
                CHECK(proficiency_level IN ('beginner', 'intermediate', 'advanced')),
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            message_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_sessions_created_at ON sessions(created_at DESC);
        CREATE INDEX IF NOT EXISTS idx_sessions_scenario ON sessions(scenario);
        CREATE INDEX IF NOT EXISTS idx_sessions_updated_at ON sessions(updated_at DESC);
        "#,
    )?;

    // 创建 messages 表
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
            role TEXT NOT NULL CHECK(role IN ('user', 'assistant', 'system')),
            content TEXT NOT NULL,
            metadata TEXT DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_messages_session_id ON messages(session_id);
        CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);
        CREATE INDEX IF NOT EXISTS idx_messages_role ON messages(role);
        "#,
    )?;

    // 创建 vocabulary 表
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS vocabulary (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL UNIQUE COLLATE NOCASE,
            phonetic TEXT,
            definition TEXT,
            example_sentence TEXT,
            first_learned DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_reviewed DATETIME,
            next_review_date DATETIME,
            review_count INTEGER NOT NULL DEFAULT 0,
            mastery_level INTEGER NOT NULL DEFAULT 0 CHECK(mastery_level BETWEEN 0 AND 5),
            source_session_id INTEGER REFERENCES sessions(id) ON DELETE SET NULL,
            source_message_id INTEGER REFERENCES messages(id) ON DELETE SET NULL,
            user_notes TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_vocabulary_word ON vocabulary(word);
        CREATE INDEX IF NOT EXISTS idx_vocabulary_mastery ON vocabulary(mastery_level);
        CREATE INDEX IF NOT EXISTS idx_vocabulary_next_review ON vocabulary(next_review_date);
        CREATE INDEX IF NOT EXISTS idx_vocabulary_first_learned ON vocabulary(first_learned DESC);
        "#,
    )?;

    // 创建 settings 表
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            type TEXT NOT NULL DEFAULT 'string'
                CHECK(type IN ('string', 'number', 'boolean', 'json')),
            description TEXT,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )?;

    // 插入默认设置
    conn.execute_batch(
        r#"
        INSERT OR IGNORE INTO settings (key, value, type, description) VALUES
            ('ai_model', 'qwen2.5:3b', 'string', '当前使用的 AI 模型'),
            ('performance_mode', 'fluent', 'string', '性能模式: fluent 或 performance'),
            ('tts_mode', 'auto', 'string', 'TTS 模式: auto, edge_only, system_only'),
            ('current_voice', 'default', 'string', '当前音色: default, male, female'),
            ('speech_speed', '1.0', 'number', '语速: 0.5-2.0'),
            ('speech_volume', '1.0', 'number', '音量: 0.0-1.0'),
            ('show_grammar_hints', 'true', 'boolean', '是否显示语法提示'),
            ('user_nickname', '', 'string', '用户昵称'),
            ('user_level', 'intermediate', 'string', '用户英语水平'),
            ('theme', 'light', 'string', '主题: light 或 dark');
        "#,
    )?;

    log::info!("Database migrations completed");
    Ok(())
}

// ============ CRUD 操作 ============

/// 创建新会话
pub fn create_session(
    conn: &Connection,
    scenario: &str,
    proficiency_level: &str,
) -> Result<i64, rusqlite::Error> {
    let title = format!("{} Practice", scenario.replace('_', " ").to_lowercase());

    conn.execute(
        "INSERT INTO sessions (title, scenario, proficiency_level) VALUES (?1, ?2, ?3)",
        rusqlite::params![title, scenario, proficiency_level],
    )?;

    Ok(conn.last_insert_rowid())
}

/// 获取会话列表
pub fn get_sessions(
    conn: &Connection,
    limit: i64,
    offset: i64,
) -> Result<Vec<serde_json::Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, title, scenario, created_at, message_count
         FROM sessions
         ORDER BY updated_at DESC
         LIMIT ?1 OFFSET ?2"
    )?;

    let rows = stmt.query_map(rusqlite::params![limit, offset], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "title": row.get::<_, String>(1)?,
            "scenario": row.get::<_, String>(2)?,
            "created_at": row.get::<_, String>(3)?,
            "message_count": row.get::<_, i64>(4)?,
        }))
    })?;

    let mut sessions = Vec::new();
    for row in rows {
        sessions.push(row?);
    }

    Ok(sessions)
}

/// 插入消息
pub fn insert_message(
    conn: &Connection,
    session_id: i64,
    role: &str,
    content: &str,
) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO messages (session_id, role, content) VALUES (?1, ?2, ?3)",
        rusqlite::params![session_id, role, content],
    )?;

    let message_id = conn.last_insert_rowid();

    // 更新会话的消息计数和更新时间
    conn.execute(
        "UPDATE sessions SET message_count = message_count + 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
        rusqlite::params![session_id],
    )?;

    Ok(message_id)
}

/// 获取会话的最后一条用户消息ID
pub fn get_last_user_message_id(
    conn: &Connection,
    session_id: i64,
) -> Result<Option<i64>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id FROM messages
         WHERE session_id = ?1 AND role = 'user'
         ORDER BY created_at DESC
         LIMIT 1"
    )?;

    let result = stmt.query_row(rusqlite::params![session_id], |row| row.get(0));

    match result {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// 获取会话的消息列表
pub fn get_messages(
    conn: &Connection,
    session_id: i64,
) -> Result<Vec<serde_json::Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, role, content, created_at
         FROM messages
         WHERE session_id = ?1
         ORDER BY created_at ASC"
    )?;

    let rows = stmt.query_map(rusqlite::params![session_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "session_id": row.get::<_, i64>(1)?,
            "role": row.get::<_, String>(2)?,
            "content": row.get::<_, String>(3)?,
            "created_at": row.get::<_, String>(4)?,
        }))
    })?;

    let mut messages = Vec::new();
    for row in rows {
        messages.push(row?);
    }

    Ok(messages)
}

/// 添加或更新生词
pub fn upsert_vocabulary(
    conn: &Connection,
    word: &str,
    source_session_id: Option<i64>,
    source_message_id: Option<i64>,
) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO vocabulary (word, first_learned, source_session_id, source_message_id)
         VALUES (?1, CURRENT_TIMESTAMP, ?2, ?3)
         ON CONFLICT(word) DO NOTHING",
        rusqlite::params![word, source_session_id, source_message_id],
    )?;

    // 获取 ID
    let mut stmt = conn.prepare("SELECT id FROM vocabulary WHERE word = ?1")?;
    let id: i64 = stmt.query_row(rusqlite::params![word], |row| row.get(0))?;

    Ok(id)
}

/// 获取生词本列表
pub fn get_vocabulary(
    conn: &Connection,
    sort_by: &str,
    order: &str,
) -> Result<Vec<serde_json::Value>, rusqlite::Error> {
    let order_clause = if order == "desc" { "DESC" } else { "ASC" };

    let query = match sort_by {
        "mastery_level" => format!(
            "SELECT id, word, phonetic, definition, example_sentence,
                    first_learned, last_reviewed, review_count, mastery_level
             FROM vocabulary
             ORDER BY mastery_level {}, first_learned DESC",
            order_clause
        ),
        _ => format!(
            "SELECT id, word, phonetic, definition, example_sentence,
                    first_learned, last_reviewed, review_count, mastery_level
             FROM vocabulary
             ORDER BY first_learned {}",
            order_clause
        ),
    };

    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "word": row.get::<_, String>(1)?,
            "phonetic": row.get::<_, Option<String>>(2)?,
            "definition": row.get::<_, Option<String>>(3)?,
            "example_sentence": row.get::<_, Option<String>>(4)?,
            "first_learned": row.get::<_, String>(5)?,
            "last_reviewed": row.get::<_, Option<String>>(6)?,
            "review_count": row.get::<_, i64>(7)?,
            "mastery_level": row.get::<_, i64>(8)?,
        }))
    })?;

    let mut vocabulary = Vec::new();
    for row in rows {
        vocabulary.push(row?);
    }

    Ok(vocabulary)
}

/// 删除会话及其所有消息
pub fn delete_session(conn: &Connection, session_id: i64) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM sessions WHERE id = ?1", rusqlite::params![session_id])
}

/// 获取设置值
pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let result = stmt.query_row(rusqlite::params![key], |row| row.get(0));

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// 更新设置值
pub fn update_setting(
    conn: &Connection,
    key: &str,
    value: &str,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO settings (key, value, type) VALUES (?1, ?2, 'string')
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
        rusqlite::params![key, value],
    )?;

    Ok(())
}

/// 获取会话详情
pub fn get_session_by_id(
    conn: &Connection,
    session_id: i64,
) -> Result<Option<serde_json::Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, title, scenario, proficiency_level, created_at, updated_at, message_count
         FROM sessions
         WHERE id = ?1"
    )?;

    let result = stmt.query_row(rusqlite::params![session_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "title": row.get::<_, String>(1)?,
            "scenario": row.get::<_, String>(2)?,
            "proficiency_level": row.get::<_, String>(3)?,
            "created_at": row.get::<_, String>(4)?,
            "updated_at": row.get::<_, String>(5)?,
            "message_count": row.get::<_, i64>(6)?,
        }))
    });

    match result {
        Ok(session) => Ok(Some(session)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// 获取生词详情（包含完整信息）
pub fn get_vocabulary_detail(
    conn: &Connection,
    word: &str,
) -> Result<Option<serde_json::Value>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, word, phonetic, definition, example_sentence,
                first_learned, last_reviewed, next_review_date,
                review_count, mastery_level, source_session_id,
                source_message_id, user_notes
         FROM vocabulary
         WHERE word = ?1"
    )?;

    let result = stmt.query_row(rusqlite::params![word], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "word": row.get::<_, String>(1)?,
            "phonetic": row.get::<_, Option<String>>(2)?,
            "definition": row.get::<_, Option<String>>(3)?,
            "example_sentence": row.get::<_, Option<String>>(4)?,
            "first_learned": row.get::<_, String>(5)?,
            "last_reviewed": row.get::<_, Option<String>>(6)?,
            "next_review_date": row.get::<_, Option<String>>(7)?,
            "review_count": row.get::<_, i64>(8)?,
            "mastery_level": row.get::<_, i64>(9)?,
            "source_session_id": row.get::<_, Option<i64>>(10)?,
            "source_message_id": row.get::<_, Option<i64>>(11)?,
            "user_notes": row.get::<_, Option<String>>(12)?,
        }))
    });

    match result {
        Ok(detail) => Ok(Some(detail)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// 更新生词的复习信息
pub fn update_vocabulary_review(
    conn: &Connection,
    word_id: i64,
    mastery_level: i64,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE vocabulary
         SET last_reviewed = CURRENT_TIMESTAMP,
             next_review_date = datetime('now', '+' || (?2 * 24) || ' hours'),
             review_count = review_count + 1,
             mastery_level = ?2,
             updated_at = CURRENT_TIMESTAMP
         WHERE id = ?1",
        rusqlite::params![word_id, mastery_level],
    )?;

    Ok(())
}

/// 获取所有设置
pub fn get_all_settings(conn: &Connection) -> Result<serde_json::Value, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT key, value, type FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut settings = serde_json::Map::new();
    for row in rows {
        let (key, value, typ) = row?;
        let parsed_value = match typ.as_str() {
            "number" => {
                if let Ok(int_val) = value.parse::<i64>() {
                    serde_json::Value::Number(serde_json::Number::from(int_val))
                } else if let Ok(float_val) = value.parse::<f64>() {
                    serde_json::Value::Number(serde_json::Number::from_f64(float_val).unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::String(value)
                }
            },
            "boolean" => serde_json::Value::Bool(value == "true"),
            _ => serde_json::Value::String(value),
        };
        settings.insert(key, parsed_value);
    }

    Ok(serde_json::Value::Object(settings))
}
