use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::database;

/// 应用设置
#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub ai_model: String,
    pub performance_mode: String,
    pub tts_mode: String,
    pub current_voice: String,
    pub speech_speed: f64,
    pub speech_volume: f64,
    pub show_grammar_hints: bool,
    pub network_status: String,
    pub user_nickname: String,
    pub user_level: String,
    pub theme: String,
    pub auto_play_tts: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_model: "qwen2.5:3b".to_string(),
            performance_mode: "fluent".to_string(),
            tts_mode: "auto".to_string(),
            current_voice: "default".to_string(),
            speech_speed: 1.0,
            speech_volume: 1.0,
            show_grammar_hints: true,
            network_status: "online".to_string(),
            user_nickname: "".to_string(),
            user_level: "intermediate".to_string(),
            theme: "light".to_string(),
            auto_play_tts: true,
        }
    }
}

/// 获取设置结果
#[derive(Serialize)]
pub struct GetSettingsResult {
    pub settings: AppSettings,
}

/// 获取应用设置
#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<GetSettingsResult, String> {
    log::info!("Getting settings");

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    let settings_json = database::get_all_settings(conn)
        .map_err(|e| format!("Failed to get settings: {}", e))?;

    // 从 JSON 解析为 AppSettings
    let settings = AppSettings {
        ai_model: settings_json["ai_model"].as_str().unwrap_or("qwen2.5:3b").to_string(),
        performance_mode: settings_json["performance_mode"].as_str().unwrap_or("fluent").to_string(),
        tts_mode: settings_json["tts_mode"].as_str().unwrap_or("auto").to_string(),
        current_voice: settings_json["current_voice"].as_str().unwrap_or("default").to_string(),
        speech_speed: settings_json["speech_speed"].as_f64().unwrap_or(1.0),
        speech_volume: settings_json["speech_volume"].as_f64().unwrap_or(1.0),
        show_grammar_hints: settings_json["show_grammar_hints"].as_bool().unwrap_or(true),
        network_status: settings_json["network_status"].as_str().unwrap_or("online").to_string(),
        user_nickname: settings_json["user_nickname"].as_str().unwrap_or("").to_string(),
        user_level: settings_json["user_level"].as_str().unwrap_or("intermediate").to_string(),
        theme: settings_json["theme"].as_str().unwrap_or("light").to_string(),
        auto_play_tts: settings_json["auto_play_tts"].as_bool().unwrap_or(true),
    };

    Ok(GetSettingsResult {
        settings,
    })
}

/// 更新设置参数
#[derive(Deserialize)]
pub struct UpdateSettingsParams {
    pub settings: serde_json::Value,
}

/// 更新设置结果
#[derive(Serialize)]
pub struct UpdateSettingsResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 更新应用设置
#[tauri::command]
pub async fn update_settings(
    params: UpdateSettingsParams,
    state: State<'_, AppState>,
) -> Result<UpdateSettingsResult, String> {
    log::info!("Updating settings: {:?}", params.settings);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    // 更新每个设置项
    if let Some(obj) = params.settings.as_object() {
        for (key, value) in obj {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };

            database::update_setting(conn, key, &value_str)
                .map_err(|e| format!("Failed to update setting {}: {}", key, e))?;
        }
    }

    Ok(UpdateSettingsResult {
        success: true,
        error: None,
    })
}

/// 切换 AI 模型参数
#[derive(Deserialize)]
pub struct SwitchAiModelParams {
    pub model: String,
}

/// 切换 AI 模型结果
#[derive(Serialize)]
pub struct SwitchAiModelResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 切换 AI 模型
#[tauri::command]
pub async fn switch_ai_model(
    params: SwitchAiModelParams,
    state: State<'_, AppState>,
) -> Result<SwitchAiModelResult, String> {
    log::info!("Switching AI model: {}", params.model);

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;

    database::update_setting(conn, "ai_model", &params.model)
        .map_err(|e| format!("Failed to update model setting: {}", e))?;

    Ok(SwitchAiModelResult {
        success: true,
        error: None,
    })
}
