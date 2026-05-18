use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle, Emitter};
use crate::AppState;
use crate::database;
use crate::ollama;

/// 启动对话结果
#[derive(Serialize)]
pub struct StartConversationResult {
    pub session_id: i64,
    pub greeting: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 启动新对话会话
#[tauri::command]
pub async fn start_conversation(
    scenario: String,
    proficiency_level: String,
    state: State<'_, AppState>,
    _app_handle: AppHandle,
) -> Result<StartConversationResult, String> {
    log::info!("=== start_conversation called ===");
    log::info!("scenario={}, level={}", scenario, proficiency_level);

    // 在 blocking task 中创建会话（避免异步问题）
    let scenario_clone = scenario.clone();
    let proficiency_clone = proficiency_level.clone();
    let db_arc = state.db.clone();
    
    let session_id = tokio::task::spawn_blocking(move || {
        let db_guard = db_arc.lock().map_err(|e| {
            log::error!("Failed to lock database: {}", e);
            e.to_string()
        })?;
        
        let conn = db_guard.as_ref().ok_or_else(|| {
            log::error!("Database connection is None - not initialized");
            "Database not initialized".to_string()
        })?;

        // 创建会话
        log::info!("Creating session in database...");
        let id = database::create_session(
            conn,
            &scenario_clone,
            &proficiency_clone
        ).map_err(|e| {
            log::error!("Failed to create session: {}", e);
            format!("Failed to create session: {}", e)
        })?;
        
        log::info!("Session created with id: {}", id);
        Ok::<i64, String>(id)
    }).await.map_err(|e| format!("Task failed: {}", e))??;

    // 生成 AI 开场白（异步操作）
    log::info!("Generating AI greeting for scenario: {}", scenario);
    let greeting = ollama::generate_greeting(&scenario, &proficiency_level)
        .await
        .unwrap_or_else(|e| {
            log::warn!("Failed to generate AI greeting: {}, using default", e);
            get_default_greeting(&scenario)
        });
    
    log::info!("Generated greeting: {}", greeting);

    // 在 blocking task 中保存消息
    let greeting_clone = greeting.clone();
    let db_arc = state.db.clone();
    
    tokio::task::spawn_blocking(move || {
        let db_guard = db_arc.lock().map_err(|e| {
            log::error!("Failed to lock database: {}", e);
            e.to_string()
        })?;
        
        let conn = db_guard.as_ref().ok_or_else(|| {
            log::error!("Database connection is None - not initialized");
            "Database not initialized".to_string()
        })?;

        // 保存系统消息
        log::info!("Saving greeting message...");
        database::insert_message(
            conn,
            session_id,
            "assistant",
            &greeting_clone
        ).map_err(|e| {
            log::error!("Failed to save message: {}", e);
            format!("Failed to save message: {}", e)
        })?;
        
        log::info!("Greeting message saved");
        Ok::<(), String>(())
    }).await.map_err(|e| format!("Task failed: {}", e))??;

    log::info!("=== start_conversation completed successfully ===");
    Ok(StartConversationResult {
        session_id,
        greeting,
        success: true,
        error: None,
    })
}

/// 获取默认开场白（fallback）
fn get_default_greeting(scenario: &str) -> String {
    match scenario {
        "coffee_shop" => "Hi there! Welcome to our coffee shop. What can I get for you today?".to_string(),
        "restaurant" => "Good evening! Welcome to our restaurant. Do you have a reservation?".to_string(),
        "hotel_checkin" => "Hello! Welcome to our hotel. How may I assist you with your check-in?".to_string(),
        "job_interview" => "Good morning! Thank you for coming in today. Please have a seat.".to_string(),
        "airport" => "Hello! Welcome to the airport. Are you checking in for a flight?".to_string(),
        "social_gathering" => "Hi! Nice to meet you! How do you know the host?".to_string(),
        "grocery_shopping" => "Hello! Welcome to the grocery store. Looking for anything specific today?".to_string(),
        "pharmacy" => "Hi! How can I help you? Are you looking for any particular medication?".to_string(),
        "banking" => "Good morning! Welcome to the bank. How may I assist you with your account today?".to_string(),
        "post_office" => "Hello! Welcome to the post office. Do you need to send a package or mail something?".to_string(),
        "taxi_rideshare" => "Hi! Where would you like to go today?".to_string(),
        "train_station" => "Good day! Welcome to the train station. Where are you heading?".to_string(),
        "car_rental" => "Hello! Welcome to the car rental desk. What type of vehicle are you looking for?".to_string(),
        "directions_asking" => "Excuse me! You look like you might need some help. Can I assist you?".to_string(),
        "office_meeting" => "Hi everyone! Thanks for joining. Let's get started with today's agenda.".to_string(),
        "business_call" => "Hello! Thanks for taking my call. Do you have a moment to discuss our project?".to_string(),
        "email_writing" => "Hi! I'm here to help you draft that important email. What's the main point you want to convey?".to_string(),
        "presentation" => "Good afternoon! Ready to practice your presentation? What topic will you be covering?".to_string(),
        "networking_event" => "Hi there! I don't think we've met before. I'm Alex. What brings you to this event?".to_string(),
        "dating" => "Hey! It's great to finally meet you in person. How has your day been?".to_string(),
        "movie_theater" => "Hi! Welcome to the cinema. Which movie are you planning to watch today?".to_string(),
        "gym_fitness" => "Hey! Welcome to the gym. Are you working with a trainer or on your own today?".to_string(),
        "doctor_appointment" => "Good morning! I'm Dr. Smith. What seems to be troubling you today?".to_string(),
        "shopping_clothes" => "Hello! Welcome to our store. Looking for anything specific, or just browsing?".to_string(),
        _ => format!("Hello! Let's practice English in this {} scenario. How can I help you?", scenario.replace('_', " ")),
    }
}

/// 发送消息参数
#[derive(Deserialize)]
pub struct SendMessageParams {
    pub session_id: i64,
    pub text: String,
}

/// 发送消息结果
#[derive(Serialize)]
pub struct SendMessageResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 发送用户消息
#[tauri::command]
pub async fn send_message(
    params: SendMessageParams,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<SendMessageResult, String> {
    log::info!("Sending message: session_id={}, text={}", params.session_id, params.text);

    // 获取数据库连接
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    // 获取会话信息以确定场景和水平
    let session_info = database::get_session_by_id(conn, params.session_id)
        .map_err(|e| format!("Failed to get session info: {}", e))?
        .ok_or_else(|| format!("Session {} not found", params.session_id))?;

    let scenario = session_info["scenario"].as_str()
        .ok_or("Invalid scenario in session")?.to_string();
    let proficiency_level = session_info["proficiency_level"].as_str()
        .ok_or("Invalid proficiency level in session")?.to_string();

    // 保存用户消息
    database::insert_message(
        conn,
        params.session_id,
        "user",
        &params.text
    ).map_err(|e| format!("Failed to save user message: {}", e))?;

    // 在后台线程中处理 AI 响应
    let user_text = params.text.clone();
    let session_id = params.session_id;
    let db_arc = state.db.clone();
    let scenario_clone = scenario.clone();
    let level_clone = proficiency_level.clone();

    tokio::spawn(async move {
        // 获取对话历史
        let history = {
            if let Ok(guard) = db_arc.lock() {
                if let Some(conn) = guard.as_ref() {
                    database::get_messages(conn, session_id)
                        .unwrap_or_default()
                        .iter()
                        .map(|msg| {
                            let role = msg["role"].as_str().unwrap_or("").to_string();
                            let content = msg["content"].as_str().unwrap_or("").to_string();
                            (role, content)
                        })
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        };

        // 调用 Ollama 生成回复
        if let Err(e) = ollama::generate_conversation_response(
            &app_handle,
            session_id,
            &user_text,
            &history,
            &scenario_clone,
            &level_clone,
        ).await {
            log::error!("Ollama generation failed: {}", e);

            // 发送错误事件到前端
            let _ = app_handle.emit("ai_response_error", serde_json::json!({
                "session_id": session_id,
                "error_code": "generation_failed",
                "error_message": e.to_string()
            }));
        }
    });

    Ok(SendMessageResult {
        success: true,
        error: None,
    })
}

/// 教学单词参数
#[derive(Deserialize)]
pub struct TeachWordParams {
    pub session_id: i64,
    pub word: String,
}

/// 教学单词结果
#[derive(Serialize)]
pub struct TeachWordResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 触发"即点即学"功能
#[tauri::command]
pub async fn teach_word(
    params: TeachWordParams,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<TeachWordResult, String> {
    log::info!("Teaching word: session_id={}, word={}", params.session_id, params.word);

    // 获取最后一条用户消息ID并添加到生词本
    {
        let db_guard = state.db.lock().map_err(|e| e.to_string())?;
        let conn = db_guard.as_ref().ok_or("Database not initialized")?;

        // 获取最后一条用户消息ID
        let msg_id = database::get_last_user_message_id(conn, params.session_id)
            .map_err(|e| format!("Failed to get last message: {}", e))?;

        // 添加到生词本
        database::upsert_vocabulary(
            conn,
            &params.word,
            Some(params.session_id),
            msg_id
        ).map_err(|e| format!("Failed to add vocabulary: {}", e))?;
    };

    let word = params.word.clone();
    let session_id = params.session_id;

    tokio::spawn(async move {
        if let Err(e) = ollama::teach_word_with_events(&app_handle, session_id, &word).await {
            log::error!("Teach word failed: {}", e);
        }
    });

    Ok(TeachWordResult {
        success: true,
        error: None,
    })
}

/// 停止 AI 响应参数
#[derive(Deserialize)]
pub struct StopAiResponseParams {
    pub session_id: i64,
}

/// 停止 AI 响应结果
#[derive(Serialize)]
pub struct StopAiResponseResult {
    pub success: bool,
}

/// 中断当前 AI 回复
#[tauri::command]
pub async fn stop_ai_response(
    _params: StopAiResponseParams,
) -> Result<StopAiResponseResult, String> {
    log::info!("Stopping AI response");

    // TODO: 实现中断逻辑

    Ok(StopAiResponseResult {
        success: true,
    })
}
