use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::database;

/// 会话对象
#[derive(Serialize)]
pub struct Session {
    pub id: i64,
    pub title: String,
    pub created_at: String,
    pub message_count: i64,
}

/// 获取会话列表参数
#[derive(Deserialize)]
pub struct GetSessionsParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 { 50 }

/// 获取会话列表结果
#[derive(Serialize)]
pub struct GetSessionsResult {
    pub sessions: Vec<Session>,
    pub total: i64,
}

/// 获取所有对话会话列表
#[tauri::command]
pub async fn get_sessions(
    params: GetSessionsParams,
    state: State<'_, AppState>,
) -> Result<GetSessionsResult, String> {
    log::info!("Getting sessions: limit={}, offset={}", params.limit, params.offset);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    let sessions_json = database::get_sessions(conn, params.limit, params.offset)
        .map_err(|e| format!("Failed to get sessions: {}", e))?;

    let sessions: Vec<Session> = sessions_json.iter().map(|s| {
        Session {
            id: s["id"].as_i64().unwrap_or(0),
            title: s["title"].as_str().unwrap_or("").to_string(),
            created_at: s["created_at"].as_str().unwrap_or("").to_string(),
            message_count: s["message_count"].as_i64().unwrap_or(0),
        }
    }).collect();

    let total = sessions.len() as i64;

    Ok(GetSessionsResult {
        sessions,
        total,
    })
}

/// 消息对象
#[derive(Serialize)]
pub struct Message {
    pub id: i64,
    pub session_id: i64,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

/// 获取消息参数
#[derive(Deserialize)]
pub struct GetMessagesParams {
    pub session_id: i64,
}

/// 获取消息结果
#[derive(Serialize)]
pub struct GetMessagesResult {
    pub messages: Vec<Message>,
}

/// 获取指定会话的消息历史
#[tauri::command]
pub async fn get_messages(
    params: GetMessagesParams,
    state: State<'_, AppState>,
) -> Result<GetMessagesResult, String> {
    log::info!("Getting messages: session_id={}", params.session_id);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    let messages_json = database::get_messages(conn, params.session_id)
        .map_err(|e| format!("Failed to get messages: {}", e))?;

    let messages: Vec<Message> = messages_json.iter().map(|m| {
        Message {
            id: m["id"].as_i64().unwrap_or(0),
            session_id: m["session_id"].as_i64().unwrap_or(0),
            role: m["role"].as_str().unwrap_or("").to_string(),
            content: m["content"].as_str().unwrap_or("").to_string(),
            created_at: m["created_at"].as_str().unwrap_or("").to_string(),
        }
    }).collect();

    Ok(GetMessagesResult {
        messages,
    })
}

/// 生词项对象
#[derive(Serialize)]
pub struct VocabularyItem {
    pub id: i64,
    pub word: String,
    pub first_learned: String,
    pub last_reviewed: String,
    pub review_count: i64,
    pub mastery_level: i64,
}

/// 获取生词本参数
#[derive(Deserialize)]
pub struct GetVocabularyParams {
    #[serde(default = "default_sort")]
    pub sort_by: String,
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_sort() -> String { "first_learned".to_string() }
fn default_order() -> String { "desc".to_string() }

/// 获取生词本结果
#[derive(Serialize)]
pub struct GetVocabularyResult {
    pub vocabulary: Vec<VocabularyItem>,
}

/// 获取生词本列表
#[tauri::command]
pub async fn get_vocabulary(
    params: GetVocabularyParams,
    state: State<'_, AppState>,
) -> Result<GetVocabularyResult, String> {
    log::info!("Getting vocabulary: sort_by={}, order={}", params.sort_by, params.order);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    let vocab_json = database::get_vocabulary(conn, &params.sort_by, &params.order)
        .map_err(|e| format!("Failed to get vocabulary: {}", e))?;

    let vocabulary: Vec<VocabularyItem> = vocab_json.iter().map(|v| {
        VocabularyItem {
            id: v["id"].as_i64().unwrap_or(0),
            word: v["word"].as_str().unwrap_or("").to_string(),
            first_learned: v["first_learned"].as_str().unwrap_or("").to_string(),
            last_reviewed: v["last_reviewed"].as_str().unwrap_or("").to_string(),
            review_count: v["review_count"].as_i64().unwrap_or(0),
            mastery_level: v["mastery_level"].as_i64().unwrap_or(0),
        }
    }).collect();

    Ok(GetVocabularyResult {
        vocabulary,
    })
}

/// 删除会话参数
#[derive(Deserialize)]
pub struct DeleteSessionParams {
    pub session_id: i64,
}

/// 删除会话结果
#[derive(Serialize)]
pub struct DeleteSessionResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 删除指定会话及其所有消息
#[tauri::command]
pub async fn delete_session(
    params: DeleteSessionParams,
    state: State<'_, AppState>,
) -> Result<DeleteSessionResult, String> {
    log::info!("Deleting session: session_id={}", params.session_id);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    database::delete_session(conn, params.session_id)
        .map_err(|e| format!("Failed to delete session: {}", e))?;

    Ok(DeleteSessionResult {
        success: true,
        error: None,
    })
}

/// 生词详情对象
#[derive(Serialize)]
pub struct VocabularyDetail {
    pub id: i64,
    pub word: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub example_sentence: Option<String>,
    pub first_learned: String,
    pub last_reviewed: Option<String>,
    pub next_review_date: Option<String>,
    pub review_count: i64,
    pub mastery_level: i64,
    pub user_notes: Option<String>,
}

/// 获取生词详情参数
#[derive(Deserialize)]
pub struct GetVocabularyDetailParams {
    pub word: String,
}

/// 获取生词详情结果
#[derive(Serialize)]
pub struct GetVocabularyDetailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary: Option<VocabularyDetail>,
}

/// 获取指定生词的详细信息
#[tauri::command]
pub async fn get_vocabulary_detail(
    params: GetVocabularyDetailParams,
    state: State<'_, AppState>,
) -> Result<GetVocabularyDetailResult, String> {
    log::info!("Getting vocabulary detail: word={}", params.word);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    let detail_json = database::get_vocabulary_detail(conn, &params.word)
        .map_err(|e| format!("Failed to get vocabulary detail: {}", e))?;

    let vocabulary = detail_json.map(|v| VocabularyDetail {
        id: v["id"].as_i64().unwrap_or(0),
        word: v["word"].as_str().unwrap_or("").to_string(),
        phonetic: v["phonetic"].as_str().map(|s| s.to_string()),
        definition: v["definition"].as_str().map(|s| s.to_string()),
        example_sentence: v["example_sentence"].as_str().map(|s| s.to_string()),
        first_learned: v["first_learned"].as_str().unwrap_or("").to_string(),
        last_reviewed: v["last_reviewed"].as_str().map(|s| s.to_string()),
        next_review_date: v["next_review_date"].as_str().map(|s| s.to_string()),
        review_count: v["review_count"].as_i64().unwrap_or(0),
        mastery_level: v["mastery_level"].as_i64().unwrap_or(0),
        user_notes: v["user_notes"].as_str().map(|s| s.to_string()),
    });

    Ok(GetVocabularyDetailResult {
        vocabulary,
    })
}

/// 更新生词复习参数
#[derive(Deserialize)]
pub struct UpdateVocabularyReviewParams {
    pub word_id: i64,
    pub mastery_level: i64,
}

/// 更新生词复习结果
#[derive(Serialize)]
pub struct UpdateVocabularyReviewResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 更新生词的复习状态
#[tauri::command]
pub async fn update_vocabulary_review(
    params: UpdateVocabularyReviewParams,
    state: State<'_, AppState>,
) -> Result<UpdateVocabularyReviewResult, String> {
    log::info!("Updating vocabulary review: word_id={}, mastery_level={}", 
               params.word_id, params.mastery_level);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    database::update_vocabulary_review(conn, params.word_id, params.mastery_level)
        .map_err(|e| format!("Failed to update vocabulary review: {}", e))?;

    Ok(UpdateVocabularyReviewResult {
        success: true,
        error: None,
    })
}
