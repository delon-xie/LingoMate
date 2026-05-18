// TTS (Text-to-Speech) 模块

use std::sync::{Arc, Mutex};
use reqwest;
use std::io::Write;

/// TTS 引擎类型
#[derive(Clone, Debug)]
pub enum TTSEngine {
    Edge,      // Edge TTS (在线,高质量)
    System,    // 系统 TTS (离线)
}

/// TTS 配置
#[derive(Clone, Debug)]
pub struct TTSConfig {
    pub voice: String,
    pub speed: f32,
    pub volume: f32,
    pub engine: TTSEngine,
}

impl Default for TTSConfig {
    fn default() -> Self {
        Self {
            // 使用更自然的声音：Aria (女声) 或 Guy (男声)
            voice: "en-US-AriaNeural".to_string(),
            // 降低语速到 0.85，让语音更清晰自然
            speed: 0.85,
            volume: 1.0,
            engine: TTSEngine::Edge,
        }
    }
}

/// 音频播放器状态
#[derive(Clone, Debug)]
pub struct AudioPlayerState {
    pub is_playing: bool,
    pub current_text: Option<String>,
}

/// TTS 管理器
pub struct TTSManager {
    state: Arc<Mutex<AudioPlayerState>>,
    client: reqwest::Client,
}

unsafe impl Send for TTSManager {}
unsafe impl Sync for TTSManager {}

impl TTSManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AudioPlayerState {
                is_playing: false,
                current_text: None,
            })),
            client: reqwest::Client::new(),
        }
    }

    /// 合成并播放 TTS 音频
    pub async fn synthesize_and_play(
        &self,
        text: &str,
        config: &TTSConfig,
    ) -> Result<(), String> {
        log::info!("Synthesizing TTS: text={}, engine={:?}", text, config.engine);

        // 更新状态
        {
            let mut state = self.state.lock().unwrap();
            state.is_playing = true;
            state.current_text = Some(text.to_string());
        }

        // 在 macOS 上直接使用 say 命令播放
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            log::info!("Using macOS say command for direct playback");
            
            // 直接使用 say 命令播放，不生成文件
            let output = Command::new("say")
                .arg("-v")
                .arg(self.get_system_voice(&config.voice))
                .arg("-r")
                .arg(format!("{}", (config.speed * 200.0) as u32))
                .arg(text)
                .output()
                .map_err(|e| format!("Failed to execute say command: {}", e))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                return Err(format!("System TTS failed: {}", error_msg));
            }
            
            log::info!("TTS playback completed via say command");
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 非 macOS 平台使用原有逻辑
            // 根据引擎类型选择合成方式
            let audio_data = match config.engine {
                TTSEngine::Edge => {
                    if is_online() {
                        self.edge_tts_synthesize(text, config).await
                            .map_err(|e| format!("Edge TTS failed: {}", e))?
                    } else {
                        log::warn!("Edge TTS requires online connection, falling back to system TTS");
                        self.system_tts_synthesize(text, config)
                            .map_err(|e| format!("System TTS failed: {}", e))?
                    }
                }
                TTSEngine::System => {
                    self.system_tts_synthesize(text, config)
                        .map_err(|e| format!("System TTS failed: {}", e))?
                }
            };

            // 播放音频
            if !audio_data.is_empty() {
                self.play_audio(&audio_data).await
                    .map_err(|e| format!("Audio playback failed: {}", e))?;
            }
        }

        // 更新状态
        {
            let mut state = self.state.lock().unwrap();
            state.is_playing = false;
        }

        Ok(())
    }

    /// 停止当前播放
    pub fn stop_playback(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.is_playing = false;
        state.current_text = None;
        log::info!("Audio playback stopped");
        Ok(())
    }

    /// 获取播放器状态
    pub fn get_state(&self) -> Result<AudioPlayerState, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        Ok(state.clone())
    }

    /// Edge TTS 合成 (使用 edge-tts API)
    async fn edge_tts_synthesize(
        &self,
        text: &str,
        config: &TTSConfig,
    ) -> Result<Vec<u8>, String> {
        log::info!("Using Edge TTS with voice: {}, speed: {}", config.voice, config.speed);

        // 使用 Edge TTS API
        let url = "https://speech.platform.bing.com/consumer/speech/synthesize/readaloud/edge/v1";

        // 计算语速百分比（0.85 = -15%）
        let rate_percent = ((config.speed - 1.0) * 100.0) as i32;
        let volume_percent = (config.volume * 100.0) as i32;

        // 构建优化的 SSML，添加自然的停顿和语调
        let ssml = format!(
            r#"<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'>
                <voice name='{}'>
                    <prosody rate='{}%' volume='{}%' pitch='+0Hz'>
                        {}
                    </prosody>
                </voice>
            </speak>"#,
            config.voice,
            rate_percent,
            volume_percent,
            escape_xml(text)
        );

        log::debug!("Edge TTS SSML: {}", ssml);

        // 发送请求
        let response = self.client
            .post(url)
            .header("Content-Type", "application/ssml+xml")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edge/120.0.0.0")
            .header("X-Microsoft-OutputFormat", "audio-24khz-48kbitrate-mono-mp3")
            .query(&[("TrustedClientToken", "6A5AA1D4EAFF4E9FB37E23D68491D6F4")])
            .body(ssml)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        log::info!("Edge TTS response status: {}", response.status());

        if response.status().is_success() {
            let audio_bytes = response.bytes().await
                .map_err(|e| format!("Failed to read response: {}", e))?;
            
            log::info!("Edge TTS received {} bytes of audio data", audio_bytes.len());
            Ok(audio_bytes.to_vec())
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            log::error!("Edge TTS failed with status: {}, body: {}", status, error_body);
            Err(format!("Edge TTS request failed: {} - {}", status, error_body))
        }
    }

    /// 系统 TTS 合成 (macOS say 命令)
    fn system_tts_synthesize(
        &self,
        text: &str,
        config: &TTSConfig,
    ) -> Result<Vec<u8>, String> {
        log::info!("Using System TTS");

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // 创建临时文件（使用 mp3 格式以获得更好的兼容性）
            let temp_file = std::env::temp_dir().join(format!(
                "lingomate_tts_{}.mp3",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            ));

            // 使用 macOS say 命令生成音频
            let output = Command::new("say")
                .arg("-v")
                .arg(self.get_system_voice(&config.voice))
                .arg("-r")
                .arg(format!("{}", (config.speed * 200.0) as u32))
                .arg("-o")
                .arg(&temp_file)
                .arg(text)
                .output()
                .map_err(|e| format!("Failed to execute say command: {}", e))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                return Err(format!("System TTS failed: {}", error_msg));
            }

            // 读取音频文件
            let audio_data = std::fs::read(&temp_file)
                .map_err(|e| format!("Failed to read audio file: {}", e))?;

            log::info!("System TTS generated {} bytes of audio data", audio_data.len());

            // 删除临时文件
            let _ = std::fs::remove_file(&temp_file);

            Ok(audio_data)
        }

        #[cfg(not(target_os = "macos"))]
        {
            log::warn!("System TTS not implemented for this platform");
            Ok(vec![])
        }
    }

    /// 播放音频数据
    async fn play_audio(&self, audio_data: &[u8]) -> Result<(), String> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            use tempfile::NamedTempFile;
            
            log::info!("Playing audio using afplay: {} bytes", audio_data.len());
            
            // 创建临时文件
            let mut temp_file = NamedTempFile::new()
                .map_err(|e| format!("Failed to create temp file: {}", e))?;
            
            temp_file.write_all(audio_data)
                .map_err(|e| format!("Failed to write audio data: {}", e))?;
            
            let temp_path = temp_file.path();
            
            // 使用 afplay 播放
            let output = Command::new("afplay")
                .arg(temp_path)
                .output()
                .map_err(|e| format!("Failed to execute afplay: {}", e))?;
            
            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Audio playback failed: {}", error_msg));
            }
            
            log::info!("Audio playback completed via afplay");
            Ok(())
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            use rodio::{Decoder, Sink};
            use std::io::Cursor;

            log::info!("Playing audio using rodio: {} bytes", audio_data.len());

            // 创建音频设备
            let (_stream, stream_handle) = rodio::OutputStream::try_default()
                .map_err(|e| {
                    log::error!("Failed to create audio output: {}", e);
                    format!("Failed to create audio output: {}", e)
                })?;

            // 解码音频
            let cursor = Cursor::new(audio_data.to_vec());
            let source = Decoder::new(cursor).map_err(|e| {
                log::error!("Failed to decode audio: {}", e);
                format!("Failed to decode audio: {}", e)
            })?;

            // 创建播放器
            let sink = Sink::try_new(&stream_handle).map_err(|e| {
                log::error!("Failed to create audio sink: {}", e);
                format!("Failed to create audio sink: {}", e)
            })?;

            // 设置音量
            sink.set_volume(1.0);

            // 添加音频源并播放
            sink.append(source);

            log::info!("Audio playback started");

            // 等待播放完成
            sink.sleep_until_end();

            log::info!("Audio playback completed");

            Ok(())
        }
    }

    /// 获取系统语音名称 (macOS)
    #[cfg(target_os = "macos")]
    fn get_system_voice(&self, voice_name: &str) -> String {
        // 使用更自然的声音
        match voice_name {
            // Edge TTS 声音映射到 macOS 系统声音
            "en-US-AriaNeural" => "Samantha".to_string(),      // 女声，自然流畅
            "en-US-GuyNeural" => "Alex".to_string(),           // 男声，清晰
            "en-GB-SoniaNeural" => "Kate".to_string(),         // 英式女声
            "en-AU-NatashaNeural" => "Karen".to_string(),      // 澳式女声
            // 兼容旧的声音名称
            "male" => "Alex".to_string(),
            "female" => "Samantha".to_string(),
            "default" => "Samantha".to_string(),
            // 如果用户指定了其他声音，直接使用
            other => other.to_string(),
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn get_system_voice(&self, voice_name: &str) -> String {
        voice_name.to_string()
    }
}

/// XML 转义
fn escape_xml(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

/// 检查网络连接状态
pub fn is_online() -> bool {
    // 简单检测: 尝试连接一个域名
    std::net::TcpStream::connect_timeout(
        &"8.8.8.8:53".parse().unwrap(),
        std::time::Duration::from_secs(2)
    ).is_ok()
}

/// 自动选择 TTS 引擎
pub fn select_tts_engine() -> TTSEngine {
    // 优先使用 Edge TTS（音质更好）
    if is_online() {
        TTSEngine::Edge
    } else {
        log::warn!("No network connection, falling back to System TTS");
        TTSEngine::System
    }
}

/// 创建优化的 TTS 配置（用于 AI 回复）
pub fn create_optimal_config() -> TTSConfig {
    TTSConfig {
        // 使用 Aria 女声，自然流畅
        voice: "en-US-AriaNeural".to_string(),
        // 语速稍慢，更清晰易懂
        speed: 0.85,
        volume: 1.0,
        engine: select_tts_engine(),
    }
}
