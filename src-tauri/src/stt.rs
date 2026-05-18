// STT (Speech-to-Text) 模块

use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::ffi::CStr;
use std::os::raw::c_char;

// TODO: macOS Speech Framework 桥接（暂时禁用）
// #[cfg(target_os = "macos")]
// extern "C" {
//     fn stt_init() -> i32;
//     fn stt_check_permission() -> i32;
//     fn stt_request_permission();
//     fn stt_recognize_from_file(audio_file_path: *const c_char) -> *mut c_char;
//     fn stt_free_string(str: *mut c_char);
// }

/// 录音状态
#[derive(Clone, Debug)]
pub struct RecordingState {
    pub is_recording: bool,
    pub duration_seconds: f64,
    pub audio_buffer: Option<Vec<u8>>,
    pub sample_rate: u32,  // 添加采样率字段
}

/// STT 管理器
pub struct STTManager {
    state: Arc<Mutex<RecordingState>>,
    stream_handle: Arc<Mutex<Option<cpal::Stream>>>,
    sample_rate: u32,  // 保存采样率
}

// 手动实现 Send + Sync
unsafe impl Send for STTManager {}
unsafe impl Sync for STTManager {}

impl STTManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(RecordingState {
                is_recording: false,
                duration_seconds: 0.0,
                audio_buffer: None,
                sample_rate: 16000,  // 默认值
            })),
            stream_handle: Arc::new(Mutex::new(None)),
            sample_rate: 16000,  // 默认值
        }
    }

    /// 开始录音
    pub fn start_recording(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;

        if state.is_recording {
            return Err("Already recording".to_string());
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        log::info!("Using input device: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));

        let config = device
            .default_input_config()
            .map_err(|e| format!("Failed to get default config: {}", e))?;

        log::info!("Audio config: channels={}, sample_rate={:?}, format={:?}", 
                   config.channels(), config.sample_rate(), config.sample_format());

        // 保存采样率
        let actual_sample_rate = config.sample_rate().0;
        
        let start_time = std::time::Instant::now();
        
        state.is_recording = true;
        state.duration_seconds = 0.0;
        state.audio_buffer = Some(Vec::new());
        state.sample_rate = actual_sample_rate;

        let state_clone = self.state.clone();
        drop(state);

        // 更新管理器的采样率
        unsafe {
            let ptr = self as *const STTManager as *mut STTManager;
            (*ptr).sample_rate = actual_sample_rate;
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::I8 => self.build_input_stream::<i8>(
                &device,
                &config,
                state_clone,
                start_time,
            )?,
            cpal::SampleFormat::I16 => self.build_input_stream::<i16>(
                &device,
                &config,
                state_clone,
                start_time,
            )?,
            cpal::SampleFormat::I32 => self.build_input_stream::<i32>(
                &device,
                &config,
                state_clone,
                start_time,
            )?,
            cpal::SampleFormat::F32 => self.build_input_stream::<f32>(
                &device,
                &config,
                state_clone,
                start_time,
            )?,
            _ => return Err("Unsupported sample format".to_string()),
        };

        {
            let mut stream_guard = self.stream_handle.lock().map_err(|e| e.to_string())?;
            *stream_guard = Some(stream);
        }

        log::info!("Recording started at {} Hz", actual_sample_rate);
        Ok(())
    }

    /// 停止录音
    pub fn stop_recording(&self) -> Result<Vec<u8>, String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;

        if !state.is_recording {
            return Err("Not recording".to_string());
        }

        {
            let mut stream_guard = self.stream_handle.lock().map_err(|e| e.to_string())?;
            if let Some(stream) = stream_guard.take() {
                drop(stream);
            }
        }

        state.is_recording = false;
        let audio_data = state.audio_buffer.take().unwrap_or_default();

        log::info!("Recording stopped, {} bytes captured", audio_data.len());
        Ok(audio_data)
    }

    /// 获取录音状态
    pub fn get_state(&self) -> Result<RecordingState, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        Ok(state.clone())
    }

    /// 将音频数据转换为 WAV 格式并保存到临时文件
    pub fn save_to_wav_file(&self, audio_data: &[u8], sample_rate: u32, channels: u16) -> Result<String, String> {
        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("lingomate_stt_{}.wav", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        ));

        let path_str = temp_path.to_str()
            .ok_or("Invalid temp file path")?
            .to_string();

        {
            let mut writer = WavWriter::create(&path_str, spec)
                .map_err(|e| format!("Failed to create WAV writer: {}", e))?;

            let samples: Vec<i16> = audio_data
                .chunks_exact(2)
                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            for sample in samples {
                writer.write_sample(sample)
                    .map_err(|e| format!("Failed to write sample: {}", e))?;
            }

            writer.flush()
                .map_err(|e| format!("Failed to flush WAV writer: {}", e))?;
        }

        log::info!("WAV file saved to: {}", path_str);
        Ok(path_str)
    }

    /// 构建输入流
    fn build_input_stream<T>(
        &self,
        device: &cpal::Device,
        config: &cpal::SupportedStreamConfig,
        state: Arc<Mutex<RecordingState>>,
        start_time: std::time::Instant,
    ) -> Result<cpal::Stream, String>
    where
        T: cpal::Sample + cpal::SizedSample + Into<f64>,
    {
        let err_fn = move |err| {
            log::error!("Audio stream error: {}", err);
        };

        let stream = device
            .build_input_stream(
                &config.config(),
                move |data: &[T], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut state_guard) = state.lock() {
                        state_guard.duration_seconds = start_time.elapsed().as_secs_f64();
                        
                        if let Some(ref mut buffer) = state_guard.audio_buffer {
                            for sample in data {
                                let normalized = (*sample).into();
                                let scaled = (normalized * 32767.0) as i16;
                                buffer.extend_from_slice(&scaled.to_le_bytes());
                            }
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| format!("Failed to build input stream: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Failed to play stream: {}", e))?;

        Ok(stream)
    }
}

/// Whisper STT 识别器（离线）
pub struct WhisperSTT {
    ctx: std::sync::Arc<std::sync::Mutex<Option<whisper_rs::WhisperContext>>>,
    model_path: String,
}

impl WhisperSTT {
    /// 创建 Whisper STT 实例
    pub fn new(model_path: &str) -> Result<Self, String> {
        log::info!("Loading Whisper model from: {}", model_path);
        
        // 检查模型文件是否存在
        if !std::path::Path::new(model_path).exists() {
            return Err(format!("Whisper model not found: {}. Please download it from https://huggingface.co/ggerganov/whisper.cpp", model_path));
        }
        
        let ctx_params = whisper_rs::WhisperContextParameters::default();
        
        let ctx = whisper_rs::WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| format!("Failed to load Whisper model: {}", e))?;
        
        log::info!("Whisper model loaded successfully");
        
        Ok(Self {
            ctx: std::sync::Arc::new(std::sync::Mutex::new(Some(ctx))),
            model_path: model_path.to_string(),
        })
    }
    
    /// 识别音频数据
    pub fn recognize(&self, audio_data: &[u8]) -> Result<String, String> {
        log::info!("Starting Whisper recognition with {} bytes", audio_data.len());
        
        let ctx_guard = self.ctx.lock().map_err(|e| e.to_string())?;
        let ctx = ctx_guard.as_ref().ok_or("Whisper context not initialized")?;
        
        // 将 16-bit PCM 转换为 i16 样本
        let samples_i16: Vec<i16> = audio_data
            .chunks_exact(2)
            .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        
        log::info!("Loaded {} i16 samples from {} bytes", samples_i16.len(), audio_data.len());
        
        // 关键：Whisper 需要 16kHz 采样率
        // cpal 录制的可能是 44100Hz 或 48000Hz，需要重采样到 16000Hz
        // 
        // 假设录音设备使用 48kHz（常见默认值）
        // 重采样比例：16000 / 48000 = 1/3
        // 即每 3 个样本取 1 个
        
        // 更精确的做法：使用线性插值重采样
        let source_sample_rate = 48000; // 假设源采样率
        let target_sample_rate = 16000;
        
        let resampled_samples = if source_sample_rate != target_sample_rate {
            let ratio = source_sample_rate as f64 / target_sample_rate as f64;
            let target_len = (samples_i16.len() as f64 / ratio) as usize;
            
            log::info!("Resampling from {}Hz to {}Hz (ratio: {:.2}, {} -> {} samples)", 
                       source_sample_rate, target_sample_rate, ratio, samples_i16.len(), target_len);
            
            let mut resampled = Vec::with_capacity(target_len);
            for i in 0..target_len {
                let src_idx = (i as f64 * ratio) as usize;
                if src_idx < samples_i16.len() {
                    resampled.push(samples_i16[src_idx]);
                }
            }
            resampled
        } else {
            samples_i16.clone()
        };
        
        // 转换为 f32 并归一化到 [-1.0, 1.0]
        let samples_f32: Vec<f32> = resampled_samples.iter()
            .map(|&s| s as f32 / 32767.0)
            .collect();
        
        log::info!("Converted {} samples to f32", samples_f32.len());
        
        // 检查音频是否有声音（避免静音导致识别失败）
        let avg_amplitude = samples_f32.iter()
            .map(|&s| s.abs())
            .sum::<f32>() / samples_f32.len() as f32;
        
        log::info!("Average amplitude: {:.4} (should be > 0.01 for speech)", avg_amplitude);
        
        if avg_amplitude < 0.01 {
            log::warn!("Audio seems to be silent or very quiet");
        }
        
        // 创建 state
        let mut state = ctx.create_state()
            .map_err(|e| format!("Failed to create state: {}", e))?;
        
        // 配置参数 - 优化识别质量
        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::BeamSearch { 
            beam_size: 5,
            patience: -1.0,
        });
        
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_single_segment(false);
        
        // 提高识别质量的参数
        params.set_temperature(0.0);
        params.set_entropy_thold(2.4);
        params.set_logprob_thold(-1.0);
        params.set_no_speech_thold(0.6);
        
        log::info!("Running Whisper inference...");
        
        // 运行识别
        state.full(params, &samples_f32)
            .map_err(|e| format!("Whisper inference failed: {}", e))?;
        
        // 获取结果
        let num_segments = state.full_n_segments()
            .map_err(|e| format!("Failed to get segment count: {}", e))?;
        
        log::info!("Got {} segments from Whisper", num_segments);
        
        if num_segments == 0 {
            log::warn!("No segments found - audio may be too short, silent, or wrong format");
            return Ok(String::new());
        }
        
        let mut text = String::new();
        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                log::info!("Segment {}: '{}'", i, segment);
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(&segment);
            }
        }
        
        log::info!("✓ Whisper recognition completed: '{}'", text);
        Ok(text)
    }
    
    /// 识别音频数据（指定采样率）
    pub fn recognize_with_sample_rate(&self, audio_data: &[u8], source_sample_rate: u32) -> Result<String, String> {
        log::info!("Starting Whisper recognition with {} bytes (source: {}Hz)", 
                   audio_data.len(), source_sample_rate);
        
        let ctx_guard = self.ctx.lock().map_err(|e| e.to_string())?;
        let ctx = ctx_guard.as_ref().ok_or("Whisper context not initialized")?;
        
        // 将 16-bit PCM 转换为 i16 样本
        let samples_i16: Vec<i16> = audio_data
            .chunks_exact(2)
            .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        
        log::info!("Loaded {} i16 samples", samples_i16.len());
        
        // 重采样到 16kHz
        let target_sample_rate = 16000;
        
        let resampled_samples = if source_sample_rate != target_sample_rate {
            let ratio = source_sample_rate as f64 / target_sample_rate as f64;
            let target_len = (samples_i16.len() as f64 / ratio) as usize;
            
            log::info!("Resampling from {}Hz to {}Hz (ratio: {:.3}, {} -> {} samples)", 
                       source_sample_rate, target_sample_rate, ratio, samples_i16.len(), target_len);
            
            let mut resampled = Vec::with_capacity(target_len);
            for i in 0..target_len {
                let src_idx_float = i as f64 * ratio;
                let src_idx = src_idx_float as usize;
                
                // 简单的线性插值
                if src_idx + 1 < samples_i16.len() {
                    let fraction = src_idx_float - src_idx as f64;
                    let s0 = samples_i16[src_idx] as f64;
                    let s1 = samples_i16[src_idx + 1] as f64;
                    let interpolated = (s0 * (1.0 - fraction) + s1 * fraction) as i16;
                    resampled.push(interpolated);
                } else if src_idx < samples_i16.len() {
                    resampled.push(samples_i16[src_idx]);
                }
            }
            resampled
        } else {
            log::info!("No resampling needed (already 16kHz)");
            samples_i16.clone()
        };
        
        // 转换为 f32 并归一化
        let samples_f32: Vec<f32> = resampled_samples.iter()
            .map(|&s| s as f32 / 32767.0)
            .collect();
        
        log::info!("Converted {} samples to f32", samples_f32.len());
        
        // 检查音频质量
        let avg_amplitude = samples_f32.iter()
            .map(|&s| s.abs())
            .sum::<f32>() / samples_f32.len() as f32;
        
        log::info!("Average amplitude: {:.4}", avg_amplitude);
        
        if avg_amplitude < 0.01 {
            log::warn!("⚠ Audio seems to be silent or very quiet!");
        }
        
        // 创建 state
        let mut state = ctx.create_state()
            .map_err(|e| format!("Failed to create state: {}", e))?;
        
        // 配置参数
        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::BeamSearch { 
            beam_size: 5,
            patience: -1.0,
        });
        
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_single_segment(false);
        params.set_temperature(0.0);
        params.set_entropy_thold(2.4);
        params.set_logprob_thold(-1.0);
        params.set_no_speech_thold(0.6);
        
        log::info!("Running Whisper inference...");
        
        state.full(params, &samples_f32)
            .map_err(|e| format!("Whisper inference failed: {}", e))?;
        
        let num_segments = state.full_n_segments()
            .map_err(|e| format!("Failed to get segment count: {}", e))?;
        
        log::info!("Got {} segments", num_segments);
        
        if num_segments == 0 {
            log::warn!("No segments found");
            return Ok(String::new());
        }
        
        let mut text = String::new();
        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                log::info!("Segment {}: '{}'", i, segment);
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(&segment);
            }
        }
        
        log::info!("✓ Recognition result: '{}'", text);
        Ok(text)
    }
}

/// 跨平台 STT 接口
pub async fn recognize_audio(audio_data: &[u8], sample_rate: u32) -> Result<String, String> {
    // 使用 Whisper 进行离线识别
    let model_path = std::env::var("WHISPER_MODEL_PATH")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.lingomate/models/ggml-base.en.bin", home)
        });
    
    log::info!("Using Whisper model: {}, sample_rate: {}", model_path, sample_rate);
    
    let audio_data = audio_data.to_vec();
    tokio::task::spawn_blocking(move || {
        let whisper = WhisperSTT::new(&model_path)?;
        whisper.recognize_with_sample_rate(&audio_data, sample_rate)
    })
    .await
    .map_err(|e| format!("STT task failed: {}", e))?
}