# SQLite Database Operations Skill

## Description
Specialized skill for implementing database operations in LingoMate using SQLite with Rust. Covers schema design, queries, migrations, and performance optimization.

## When to Use
- Creating new database tables
- Writing SQL queries
- Implementing data access layer
- Performing database migrations
- Optimizing query performance
- Handling database errors

## Database Schema Reference

### Current Tables
- `sessions` - Conversation sessions
- `messages` - Chat messages
- `vocabulary` - User's vocabulary list
- `settings` - Application settings

See [Database Design Document](../../doc/LingoMate-数据库详细设计.md) for complete schema.

## Connection Setup

### Initialize Database
```rust
// src-tauri/src/database.rs

use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)?;
        
        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        
        // Set WAL mode for better concurrent performance
        conn.execute_batch("PRAGMA journal_mode = WAL;")?;
        
        // Optimize cache size (2MB)
        conn.execute_batch("PRAGMA cache_size = -2000;")?;
        
        Ok(Database { conn })
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}
```

### Register with Tauri
```rust
// src-tauri/src/main.rs

use database::Database;
use std::path::PathBuf;

fn main() {
    let db_path = PathBuf::from("/path/to/lingomate.db");
    let db = Database::new(db_path).expect("Failed to initialize database");
    
    tauri::Builder::default()
        .manage(db)  // Register as managed state
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## CRUD Operations

### Create (Insert)

#### Single Record
```rust
use rusqlite::params;

pub fn create_session(
    db: &Connection,
    title: &str,
    scenario: &str,
    proficiency_level: &str,
) -> Result<i64, rusqlite::Error> {
    db.execute(
        "INSERT INTO sessions (title, scenario, proficiency_level) VALUES (?1, ?2, ?3)",
        params![title, scenario, proficiency_level],
    )?;
    
    // Return last inserted row ID
    Ok(db.last_insert_rowid())
}
```

#### Batch Insert with Transaction
```rust
pub fn import_vocabulary(
    db: &Connection,
    words: Vec<VocabularyItem>,
) -> Result<usize, rusqlite::Error> {
    let tx = db.unchecked_transaction()?;
    
    let mut count = 0;
    for word in words {
        tx.execute(
            "INSERT OR IGNORE INTO vocabulary (word, phonetic, definition) VALUES (?1, ?2, ?3)",
            params![word.word, word.phonetic, word.definition],
        )?;
        count += 1;
    }
    
    tx.commit()?;
    Ok(count)
}
```

### Read (Query)

#### Single Record
```rust
pub fn get_session(
    db: &Connection,
    session_id: i64,
) -> Result<Option<Session>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT id, title, scenario, created_at, message_count 
         FROM sessions 
         WHERE id = ?1"
    )?;
    
    let session = stmt.query_row(params![session_id], |row| {
        Ok(Session {
            id: row.get(0)?,
            title: row.get(1)?,
            scenario: row.get(2)?,
            created_at: row.get(3)?,
            message_count: row.get(4)?,
        })
    }).optional()?;
    
    Ok(session)
}
```

#### Multiple Records with Pagination
```rust
pub fn get_sessions(
    db: &Connection,
    limit: i64,
    offset: i64,
) -> Result<Vec<Session>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT id, title, scenario, created_at, message_count 
         FROM sessions 
         ORDER BY updated_at DESC 
         LIMIT ?1 OFFSET ?2"
    )?;
    
    let sessions = stmt.query_map(params![limit, offset], |row| {
        Ok(Session {
            id: row.get(0)?,
            title: row.get(1)?,
            scenario: row.get(2)?,
            created_at: row.get(3)?,
            message_count: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(sessions)
}
```

#### Count Records
```rust
pub fn count_messages(db: &Connection, session_id: i64) -> Result<i64, rusqlite::Error> {
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM messages WHERE session_id = ?1",
        params![session_id],
        |row| row.get(0)
    )?;
    
    Ok(count)
}
```

### Update

#### Simple Update
```rust
pub fn update_setting(
    db: &Connection,
    key: &str,
    value: &str,
) -> Result<usize, rusqlite::Error> {
    db.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?3, updated_at = CURRENT_TIMESTAMP",
        params![key, value, value],
    )
}
```

#### Update with Condition
```rust
pub fn update_vocabulary_mastery(
    db: &Connection,
    word_id: i64,
    new_level: i32,
) -> Result<usize, rusqlite::Error> {
    db.execute(
        "UPDATE vocabulary 
         SET mastery_level = ?1, 
             review_count = review_count + 1,
             last_reviewed = CURRENT_TIMESTAMP,
             next_review_date = CASE ?1
                 WHEN 0 THEN datetime('now', '+0 days')
                 WHEN 1 THEN datetime('now', '+1 days')
                 WHEN 2 THEN datetime('now', '+3 days')
                 WHEN 3 THEN datetime('now', '+7 days')
                 WHEN 4 THEN datetime('now', '+14 days')
                 WHEN 5 THEN datetime('now', '+30 days')
             END
         WHERE id = ?2",
        params![new_level, word_id],
    )
}
```

### Delete

#### Soft Delete (Recommended)
```rust
pub fn archive_session(
    db: &Connection,
    session_id: i64,
) -> Result<usize, rusqlite::Error> {
    // Add is_archived column to sessions table first
    db.execute(
        "UPDATE sessions SET is_archived = 1 WHERE id = ?1",
        params![session_id],
    )
}
```

#### Hard Delete with Cascade
```rust
pub fn delete_session(
    db: &Connection,
    session_id: i64,
) -> Result<usize, rusqlite::Error> {
    // Foreign key cascade will delete associated messages
    db.execute(
        "DELETE FROM sessions WHERE id = ?1",
        params![session_id],
    )
}
```

## Advanced Queries

### JOIN Operations
```rust
pub fn get_session_with_messages(
    db: &Connection,
    session_id: i64,
) -> Result<(Session, Vec<Message>), rusqlite::Error> {
    // Get session
    let session = get_session(db, session_id)?
        .ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    
    // Get messages
    let mut stmt = db.prepare(
        "SELECT id, session_id, role, content, created_at 
         FROM messages 
         WHERE session_id = ?1 
         ORDER BY created_at ASC"
    )?;
    
    let messages = stmt.query_map(params![session_id], |row| {
        Ok(Message {
            id: row.get(0)?,
            session_id: row.get(1)?,
            role: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok((session, messages))
}
```

### Aggregation Queries
```rust
pub fn get_learning_stats(
    db: &Connection,
    user_id: &str,
) -> Result<LearningStats, rusqlite::Error> {
    let stats = db.query_row(
        "SELECT 
            COUNT(DISTINCT s.id) as total_sessions,
            COUNT(m.id) as total_messages,
            COUNT(v.id) as total_words,
            AVG(v.mastery_level) as avg_mastery
         FROM sessions s
         LEFT JOIN messages m ON s.id = m.session_id
         LEFT JOIN vocabulary v ON v.source_session_id = s.id
         WHERE s.user_id = ?1",
        params![user_id],
        |row| {
            Ok(LearningStats {
                total_sessions: row.get(0)?,
                total_messages: row.get(1)?,
                total_words: row.get(2)?,
                avg_mastery: row.get(3)?,
            })
        }
    )?;
    
    Ok(stats)
}
```

### Full-Text Search
```rust
// First, create FTS5 virtual table
db.execute_batch("
    CREATE VIRTUAL TABLE vocabulary_fts USING fts5(word, definition);
    
    -- Trigger to keep FTS index updated
    CREATE TRIGGER vocabulary_ai AFTER INSERT ON vocabulary BEGIN
        INSERT INTO vocabulary_fts(rowid, word, definition)
        VALUES (new.id, new.word, new.definition);
    END;
")?;

// Search function
pub fn search_vocabulary(
    db: &Connection,
    query: &str,
) -> Result<Vec<VocabularyItem>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT v.* FROM vocabulary v
         JOIN vocabulary_fts fts ON v.id = fts.rowid
         WHERE vocabulary_fts MATCH ?1
         ORDER BY rank
         LIMIT 20"
    )?;
    
    let results = stmt.query_map(params![query], |row| {
        // Map to VocabularyItem
        Ok(VocabularyItem { /* ... */ })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(results)
}
```

## Database Migrations

### Migration System
```rust
// src-tauri/src/migrations.rs

pub fn migrate_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    let current_version: i32 = conn.pragma_query_value(
        None, 
        "user_version", 
        |row| row.get(0)
    )?;
    
    if current_version < 1 {
        run_migration_v1(conn)?;
        conn.pragma_update(None, "user_version", 1)?;
    }
    
    if current_version < 2 {
        run_migration_v2(conn)?;
        conn.pragma_update(None, "user_version", 2)?;
    }
    
    Ok(())
}

fn run_migration_v1(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("
        -- Create sessions table
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL DEFAULT 'New Conversation',
            scenario TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create messages table
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
            role TEXT NOT NULL CHECK(role IN ('user', 'assistant')),
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create indexes
        CREATE INDEX IF NOT EXISTS idx_sessions_created ON sessions(created_at DESC);
        CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
    ")?;
    
    Ok(())
}

fn run_migration_v2(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("
        -- Add vocabulary table
        CREATE TABLE IF NOT EXISTS vocabulary (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT UNIQUE NOT NULL COLLATE NOCASE,
            phonetic TEXT,
            definition TEXT,
            mastery_level INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX IF NOT EXISTS idx_vocabulary_word ON vocabulary(word);
    ")?;
    
    Ok(())
}
```

### Migration Files Structure
```
migrations/
├── V1__initial_schema.sql
├── V2__add_vocabulary.sql
├── V3__add_settings.sql
└── README.md
```

## Error Handling

### Custom Error Type
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(#[from] rusqlite::Error),
    
    #[error("Record not found: {0}")]
    NotFound(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => {
                DatabaseError::NotFound("Record not found".to_string())
            }
            rusqlite::Error::SqliteFailure(e, msg) => {
                if e.code == rusqlite::ErrorCode::ConstraintViolation {
                    DatabaseError::ConstraintViolation(msg.unwrap_or_default())
                } else {
                    DatabaseError::ConnectionFailed(rusqlite::Error::SqliteFailure(e, msg))
                }
            }
            _ => DatabaseError::ConnectionFailed(err),
        }
    }
}
```

### Usage in Commands
```rust
#[tauri::command]
pub async fn get_session_detail(
    app_handle: AppHandle,
    session_id: i64,
) -> CommandResult<SessionDetail> {
    let db = app_handle.state::<Database>();
    
    let session = get_session(db.get_connection(), session_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound(format!("Session {} not found", session_id)))?;
    
    Ok(SessionDetail::from(session))
}
```

## Performance Optimization

### Indexing Strategy
```sql
-- Essential indexes for common queries
CREATE INDEX idx_sessions_updated ON sessions(updated_at DESC);
CREATE INDEX idx_messages_session_created ON messages(session_id, created_at);
CREATE INDEX idx_vocabulary_mastery ON vocabulary(mastery_level);
CREATE INDEX idx_vocabulary_next_review ON vocabulary(next_review_date);

-- Composite index for complex queries
CREATE INDEX idx_messages_session_role ON messages(session_id, role);
```

### Query Optimization Tips

1. **Use EXPLAIN QUERY PLAN**
```rust
let mut stmt = db.prepare("EXPLAIN QUERY PLAN SELECT * FROM messages WHERE session_id = ?")?;
stmt.query(params![session_id])?;
```

2. **Avoid SELECT ***
```rust
// ❌ Bad
"SELECT * FROM sessions"

// ✅ Good
"SELECT id, title, scenario FROM sessions"
```

3. **Use LIMIT for Large Result Sets**
```rust
"SELECT * FROM messages WHERE session_id = ? ORDER BY created_at LIMIT 100"
```

4. **Batch Operations**
```rust
// Instead of individual inserts
for item in items {
    db.execute("INSERT ...", params![item])?;
}

// Use transaction
let tx = db.unchecked_transaction()?;
for item in items {
    tx.execute("INSERT ...", params![item])?;
}
tx.commit()?;
```

### Connection Pooling (For Future Scaling)
```rust
use r2d2;
use r2d2_sqlite::SqliteConnectionManager;

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

pub fn create_pool(db_path: &str) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(db_path);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
}
```

## Testing

### Test Setup
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        
        // Run migrations
        migrate_database(&conn).unwrap();
        
        conn
    }
    
    #[test]
    fn test_create_session() {
        let db = setup_test_db();
        
        let id = create_session(&db, "Test Session", "coffee_shop", "intermediate").unwrap();
        
        assert!(id > 0);
    }
    
    #[test]
    fn test_get_nonexistent_session() {
        let db = setup_test_db();
        
        let result = get_session(&db, 999).unwrap();
        
        assert!(result.is_none());
    }
}
```

## Security Best Practices

### ✅ DO
- Always use parameterized queries
- Enable foreign key constraints
- Validate input before querying
- Use transactions for atomicity
- Limit query result sizes
- Sanitize user inputs

### ❌ DON'T
- Never concatenate SQL strings
- Don't expose raw SQL errors to users
- Avoid SELECT * in production
- Don't store sensitive data unencrypted
- Never skip input validation
- Don't ignore connection errors

## Backup and Recovery

### Manual Backup
```rust
use std::fs;

pub fn backup_database(db_path: &str, backup_path: &str) -> Result<(), std::io::Error> {
    fs::copy(db_path, backup_path)?;
    Ok(())
}

pub fn restore_database(backup_path: &str, db_path: &str) -> Result<(), std::io::Error> {
    fs::copy(backup_path, db_path)?;
    Ok(())
}
```

### Automatic Backup on Exit
```rust
// In Tauri shutdown handler
fn on_shutdown(app_handle: &AppHandle) {
    let db_path = get_db_path();
    let backup_path = format!("{}.backup", db_path);
    
    if let Err(e) = backup_database(&db_path, &backup_path) {
        log::error!("Backup failed: {}", e);
    }
}
```

## References
- [rusqlite Documentation](https://docs.rs/rusqlite/)
- [SQLite Official Docs](https://www.sqlite.org/docs.html)
- [Project Database Design](../../doc/LingoMate-数据库详细设计.md)
- [SQL Injection Prevention](https://www.sqlite.org/security.html)
