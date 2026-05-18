use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use rusqlite::Connection;

// 模块声明
mod commands;
mod database;
mod ollama;
mod tts;
mod stt;

use commands::*;

/// 应用状态管理
pub struct AppState {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
    pub db: Arc<Mutex<Option<Connection>>>,
    pub stt_manager: Arc<stt::STTManager>,
    pub tts_manager: Arc<tts::TTSManager>,
}

impl AppState {
    pub fn get_handle(&self) -> Result<AppHandle, String> {
        let guard = self.app_handle.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or_else(|| "App handle not initialized".to_string())
    }

    pub fn get_db(&self) -> Result<Arc<Mutex<Option<Connection>>>, String> {
        Ok(self.db.clone())
    }
}

/// 初始化 Tauri 应用
pub fn run() {
    // 初始化日志
    env_logger::init();

    let state = AppState {
        app_handle: Arc::new(Mutex::new(None)),
        db: Arc::new(Mutex::new(None)),
        stt_manager: Arc::new(stt::STTManager::new()),
        tts_manager: Arc::new(tts::TTSManager::new()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(|app| {
            // 保存 app_handle
            let app_handle = app.handle().clone();
            let state = app.state::<AppState>();
            if let Ok(mut guard) = state.app_handle.lock() {
                *guard = Some(app_handle.clone());
            }

            // 同步初始化数据库（确保在用户操作前完成）
            let db_state = state.db.clone();
            match database::init_database_with_connection(&app_handle) {
                Ok(conn) => {
                    if let Ok(mut db_guard) = db_state.lock() {
                        *db_guard = Some(conn);
                    }
                    log::info!("Database initialized successfully");
                }
                Err(e) => {
                    log::error!("Failed to initialize database: {}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 对话相关命令
            start_conversation,
            send_message,
            teach_word,
            stop_ai_response,
            // 语音相关命令
            start_recording,
            stop_recording,
            play_audio,
            stop_audio,
            // 数据管理命令
            get_sessions,
            get_messages,
            get_vocabulary,
            get_vocabulary_detail,
            update_vocabulary_review,
            delete_session,
            // 配置管理命令
            get_settings,
            update_settings,
            switch_ai_model,
            // 系统管理命令
            check_system_health,
            restart_ollama,
            export_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}