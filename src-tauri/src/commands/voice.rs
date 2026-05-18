use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle, Emitter};
use crate::AppState;

/// 开始录音结果
#[derive(Serialize)]
pub struct StartRecordingResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 开始录音
#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<StartRecordingResult, String> {
    log::info!("Starting recording");

    // 开始录音
    if let Err(e) = state.stt_manager.start_recording() {
        return Ok(StartRecordingResult {
            success: false,
            error: Some(e),
        });
    }

    // 通知前端录音已开始
    let _ = app_handle.emit("recording_status_changed", serde_json::json!({
        "is_recording": true
    }));

    Ok(StartRecordingResult {
        success: true,
        error: None,
    })
}

/// 停止录音结果
#[derive(Serialize)]
pub struct StopRecordingResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 停止录音并获取识别结果
#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<StopRecordingResult, String> {
    log::info!("Stopping recording");

    // 获取录音状态（包含采样率）
    let recording_state = state.stt_manager.get_state()
        .map_err(|e| format!("Failed to get recording state: {}", e))?;
    
    let sample_rate = recording_state.sample_rate;
    log::info!("Recording sample rate: {} Hz", sample_rate);

    // 停止录音并获取音频数据
    let audio_data = match state.stt_manager.stop_recording() {
        Ok(data) => data,
        Err(e) => {
            return Ok(StopRecordingResult {
                success: false,
                text: None,
                error: Some(e),
            });
        }
    };

    // 通知前端录音已停止
    let _ = app_handle.emit("recording_status_changed", serde_json::json!({
        "is_recording": false
    }));

    // 如果音频数据为空,返回错误
    if audio_data.is_empty() {
        return Ok(StopRecordingResult {
            success: false,
            text: None,
            error: Some("No audio data recorded".to_string()),
        });
    }

    log::info!("Audio data captured: {} bytes (~{:.1} seconds at {}Hz)", 
               audio_data.len(), 
               audio_data.len() as f64 / (sample_rate as f64 * 2.0),
               sample_rate);

    // 检查音频是否太短或太长
    let duration_seconds = audio_data.len() as f64 / (sample_rate as f64 * 2.0);
    if duration_seconds < 0.5 {
        return Ok(StopRecordingResult {
            success: false,
            text: None,
            error: Some("Recording too short. Please speak for at least 1 second.".to_string()),
        });
    }
    
    if duration_seconds > 60.0 {
        return Ok(StopRecordingResult {
            success: false,
            text: None,
            error: Some("Recording too long. Please keep it under 60 seconds.".to_string()),
        });
    }

    // 进行语音识别 (异步)，传入采样率
    let recognized_text = crate::stt::recognize_audio(&audio_data, sample_rate).await?;

    log::info!("Speech recognized: {}", recognized_text);

    Ok(StopRecordingResult {
        success: true,
        text: Some(recognized_text),
        error: None,
    })
}

/// 播放音频参数
#[derive(Deserialize)]
pub struct PlayAudioParams {
    pub text: String,
    #[serde(default = "default_voice")]
    pub voice: Option<String>,
    #[serde(default)]
    pub speed: Option<f32>,
    #[serde(default)]
    pub volume: Option<f32>,
}

fn default_voice() -> Option<String> { 
    Some("en-US-AriaNeural".to_string()) 
}

/// 播放音频结果
#[derive(Serialize)]
pub struct PlayAudioResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 播放 TTS 音频
#[tauri::command]
pub async fn play_audio(
    params: PlayAudioParams,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<PlayAudioResult, String> {
    log::info!("Playing audio: text={}", params.text);

    // 构建 TTS 配置，使用优化的默认值
    let config = crate::tts::TTSConfig {
        voice: params.voice.unwrap_or_else(|| "en-US-AriaNeural".to_string()),
        speed: params.speed.unwrap_or(0.85),  // 默认较慢语速
        volume: params.volume.unwrap_or(1.0),
        engine: crate::tts::select_tts_engine(),
    };
    
    log::info!("TTS config: voice={}, speed={}, volume={}, engine={:?}", 
               config.voice, config.speed, config.volume, config.engine);

    // 通知前端开始播放
    let _ = app_handle.emit("audio_playback_status", serde_json::json!({
        "status": "playing",
        "text": params.text
    }));

    // 合成并播放 (在 blocking task 中运行以避免阻塞 async runtime)
    let tts_manager = state.tts_manager.clone();
    let text = params.text.clone();

    let result = tokio::task::spawn_blocking(move || {
        // 使用 block_on 来运行 async 函数
        let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;
        rt.block_on(tts_manager.synthesize_and_play(&text, &config))
    }).await.map_err(|e| format!("TTS task failed: {}", e))?;

    match result {
        Ok(_) => {
            log::info!("TTS playback completed successfully");
            // 通知前端播放完成
            let _ = app_handle.emit("audio_playback_status", serde_json::json!({
                "status": "stopped"
            }));

            Ok(PlayAudioResult {
                success: true,
                error: None,
            })
        }
        Err(e) => {
            log::error!("TTS playback failed: {}", e);
            // 通知前端播放错误
            let _ = app_handle.emit("audio_playback_status", serde_json::json!({
                "status": "error",
                "error_message": e.to_string()
            }));

            Ok(PlayAudioResult {
                success: false,
                error: Some(e.to_string()),
            })
        }
    }
}

/// 停止音频结果
#[derive(Serialize)]
pub struct StopAudioResult {
    pub success: bool,
}

/// 停止当前音频播放
#[tauri::command]
pub async fn stop_audio(
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<StopAudioResult, String> {
    log::info!("Stopping audio");

    // 停止播放
    if let Err(e) = state.tts_manager.stop_playback() {
        return Err(e);
    }

    // 通知前端已停止
    let _ = app_handle.emit("audio_playback_status", serde_json::json!({
        "status": "stopped"
    }));

    Ok(StopAudioResult {
        success: true,
    })
}
