use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

/// 系统健康状态
#[derive(Serialize)]
pub struct SystemHealth {
    pub ollama_running: bool,
    pub ollama_port_available: bool,
    pub model_loaded: bool,
    pub current_model: String,
    pub memory_usage_mb: f64,
    pub disk_space_gb: f64,
}

/// 检查系统健康结果
#[derive(Serialize)]
pub struct CheckSystemHealthResult {
    pub health: SystemHealth,
}

/// 检查系统健康状态
#[tauri::command]
pub async fn check_system_health(
    _state: State<'_, AppState>,
) -> Result<CheckSystemHealthResult, String> {
    log::info!("Checking system health");

    // TODO: 检查 Ollama 服务状态
    // TODO: 检查模型加载状态
    // TODO: 检查内存和磁盘使用情况

    Ok(CheckSystemHealthResult {
        health: SystemHealth {
            ollama_running: false,
            ollama_port_available: true,
            model_loaded: false,
            current_model: "qwen2.5:3b".to_string(),
            memory_usage_mb: 0.0,
            disk_space_gb: 100.0,
        },
    })
}

/// 重启 Ollama 结果
#[derive(Serialize)]
pub struct RestartOllamaResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 重启 Ollama 服务
#[tauri::command]
pub async fn restart_ollama() -> Result<RestartOllamaResult, String> {
    log::info!("Restarting Ollama service");

    // TODO: 执行 ollama stop 和 ollama serve

    Ok(RestartOllamaResult {
        success: true,
        error: None,
    })
}

/// 导出数据参数
#[derive(Deserialize)]
pub struct ExportDataParams {
    pub format: String,
    pub include_sessions: bool,
    pub include_vocabulary: bool,
}

/// 导出数据结果
#[derive(Serialize)]
pub struct ExportDataResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 导出数据
#[tauri::command]
pub async fn export_data(
    params: ExportDataParams,
    _state: State<'_, AppState>,
) -> Result<ExportDataResult, String> {
    log::info!(
        "Exporting data: format={}, sessions={}, vocabulary={}",
        params.format,
        params.include_sessions,
        params.include_vocabulary
    );

    // TODO: 导出对话记录和生词本为 JSON 或 CSV

    Ok(ExportDataResult {
        success: true,
        file_path: Some("/path/to/export.json".to_string()),
        error: None,
    })
}
