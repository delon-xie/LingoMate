# LingoMate API 接口规范

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | 后端开发团队 |

---

## 1. 架构概述

### 1.1 通信模式

LingoMate 采用 **Tauri IPC (Inter-Process Communication)** 机制实现前后端通信:

```
┌─────────────────┐         Tauri Commands/Events         ┌──────────────────┐
│                 │  ──────────────────────────────────►  │                  │
│   React Frontend│                                       │  Rust Backend    │
│   (WebView)     │  ◄──────────────────────────────────  │  (Native)        │
│                 │         Tauri Events                  │                  │
└─────────────────┘                                       └────────┬─────────┘
                                                                   │
                                                            Local HTTP
                                                                   │
                                                                   ▼
                                                          ┌──────────────────┐
                                                          │   Ollama API     │
                                                          │  (localhost)     │
                                                          └──────────────────┘
```

### 1.2 通信类型

| 类型 | 方向 | 用途 | 示例 |
| :--- | :--- | :--- | :--- |
| **Tauri Command** | Frontend → Backend | 同步请求,等待响应 | 发送消息、查询生词本 |
| **Tauri Event** | Backend → Frontend | 异步通知,流式数据 | AI 流式回复、录音状态变化 |
| **Local HTTP** | Backend → Ollama | 调用本地 AI 服务 | 生成 AI 回复 |

---

## 2. Tauri Commands (前端调用后端)

### 2.1 对话相关命令

#### `start_conversation`

启动新对话会话。

**参数**:
```typescript
interface StartConversationParams {
  scenario: string;        // 情景模式: "coffee_shop" | "restaurant" | "hotel" | "interview" | "airport" | "social"
  proficiency_level: string; // 用户水平: "beginner" | "intermediate" | "advanced"
}
```

**返回**:
```typescript
interface StartConversationResult {
  session_id: number;      // 会话 ID
  greeting: string;        // AI 开场白
  success: boolean;
  error?: string;          // 错误信息(可选)
}
```

**示例**:
```typescript
const result = await invoke('start_conversation', {
  scenario: 'coffee_shop',
  proficiency_level: 'intermediate'
});
```

---

#### `send_message`

发送用户消息(文本)。

**参数**:
```typescript
interface SendMessageParams {
  session_id: number;      // 会话 ID
  text: string;            // 用户输入的文本
}
```

**返回**:
```typescript
interface SendMessageResult {
  success: boolean;
  error?: string;
}
```

**说明**: 
- 此命令触发后端 AI 推理,结果通过 `ai_response_chunk` Event 流式返回
- 不直接返回 AI 回复,避免长时间阻塞

**示例**:
```typescript
await invoke('send_message', {
  session_id: 1,
  text: "I'd like a latte, please."
});
```

---

#### `teach_word`

触发"即点即学"功能。

**参数**:
```typescript
interface TeachWordParams {
  session_id: number;      // 会话 ID
  word: string;            // 被选中的单词
}
```

**返回**:
```typescript
interface TeachWordResult {
  success: boolean;
  error?: string;
}
```

**说明**:
- 后端使用专用 Prompt 进行单词教学
- 结果通过 `ai_response_chunk` Event 流式返回
- 自动将单词加入生词本

**示例**:
```typescript
await invoke('teach_word', {
  session_id: 1,
  word: "procrastinate"
});
```

---

#### `stop_ai_response`

中断当前 AI 回复(用户打断场景)。

**参数**:
```typescript
interface StopAiResponseParams {
  session_id: number;
}
```

**返回**:
```typescript
interface StopAiResponseResult {
  success: boolean;
}
```

**示例**:
```typescript
await invoke('stop_ai_response', { session_id: 1 });
```

---

### 2.2 语音相关命令

#### `start_recording`

开始录音。

**参数**: 无

**返回**:
```typescript
interface StartRecordingResult {
  success: boolean;
  error?: string;          // 如权限被拒绝
}
```

**说明**:
- 后端调用系统原生录音 API
- 录音状态通过 `recording_status_changed` Event 通知前端

**示例**:
```typescript
const result = await invoke('start_recording');
```

---

#### `stop_recording`

停止录音并获取识别结果。

**参数**: 无

**返回**:
```typescript
interface StopRecordingResult {
  success: boolean;
  text?: string;           // STT 识别结果
  error?: string;
}
```

**说明**:
- 后端调用系统 STT API 进行语音转文字
- 识别成功后,前端可选择是否调用 `send_message` 发送文本

**示例**:
```typescript
const result = await invoke('stop_recording');
if (result.success && result.text) {
  console.log("Recognized:", result.text);
}
```

---

#### `play_audio`

播放 TTS 音频。

**参数**:
```typescript
interface PlayAudioParams {
  text: string;            // 要合成的文本
  voice?: string;          // 音色: "default" | "male" | "female"
  speed?: number;          // 语速: 0.5 - 2.0 (默认 1.0)
  volume?: number;         // 音量: 0.0 - 1.0 (默认 1.0)
}
```

**返回**:
```typescript
interface PlayAudioResult {
  success: boolean;
  error?: string;
}
```

**说明**:
- 后端根据网络状态和用户配置选择 Edge TTS 或 System TTS
- 播放状态通过 `audio_playback_status` Event 通知前端

**示例**:
```typescript
await invoke('play_audio', {
  text: "Hello! How can I help you?",
  voice: "female",
  speed: 1.0,
  volume: 0.8
});
```

---

#### `stop_audio`

停止当前音频播放。

**参数**: 无

**返回**:
```typescript
interface StopAudioResult {
  success: boolean;
}
```

**示例**:
```typescript
await invoke('stop_audio');
```

---

### 2.3 数据管理命令

#### `get_sessions`

获取所有对话会话列表。

**参数**:
```typescript
interface GetSessionsParams {
  limit?: number;          // 返回数量限制(默认 50)
  offset?: number;         // 偏移量(默认 0)
}
```

**返回**:
```typescript
interface Session {
  id: number;
  title: string;
  created_at: string;      // ISO 8601 格式
  message_count: number;
}

interface GetSessionsResult {
  sessions: Session[];
  total: number;
}
```

**示例**:
```typescript
const result = await invoke('get_sessions', { limit: 20, offset: 0 });
```

---

#### `get_messages`

获取指定会话的消息历史。

**参数**:
```typescript
interface GetMessagesParams {
  session_id: number;
}
```

**返回**:
```typescript
interface Message {
  id: number;
  session_id: number;
  role: "user" | "assistant";
  content: string;
  created_at: string;      // ISO 8601 格式
}

interface GetMessagesResult {
  messages: Message[];
}
```

**示例**:
```typescript
const result = await invoke('get_messages', { session_id: 1 });
```

---

#### `get_vocabulary`

获取生词本列表。

**参数**:
```typescript
interface GetVocabularyParams {
  sort_by?: string;        // "first_learned" | "last_reviewed" | "mastery_level"
  order?: "asc" | "desc";
}
```

**返回**:
```typescript
interface VocabularyItem {
  id: number;
  word: string;
  first_learned: string;   // ISO 8601 格式
  last_reviewed: string;   // ISO 8601 格式
  review_count: number;
  mastery_level: number;   // 0-5
}

interface GetVocabularyResult {
  vocabulary: VocabularyItem[];
}
```

**示例**:
```typescript
const result = await invoke('get_vocabulary', {
  sort_by: "mastery_level",
  order: "asc"
});
```

---

#### `delete_session`

删除指定会话及其所有消息。

**参数**:
```typescript
interface DeleteSessionParams {
  session_id: number;
}
```

**返回**:
```typescript
interface DeleteSessionResult {
  success: boolean;
  error?: string;
}
```

**示例**:
```typescript
await invoke('delete_session', { session_id: 1 });
```

---

### 2.4 配置管理命令

#### `get_settings`

获取应用设置。

**参数**: 无

**返回**:
```typescript
interface AppSettings {
  ai_model: string;              // "qwen2.5:3b" | "qwen2.5:7b-q4_K_M"
  performance_mode: "fluent" | "performance";
  tts_mode: "auto" | "edge_only" | "system_only";
  current_voice: string;         // "default" | "male" | "female"
  speech_speed: number;          // 0.5 - 2.0
  speech_volume: number;         // 0.0 - 1.0
  show_grammar_hints: boolean;   // 是否显示语法提示
  network_status: "online" | "offline";
}

interface GetSettingsResult {
  settings: AppSettings;
}
```

**示例**:
```typescript
const result = await invoke('get_settings');
```

---

#### `update_settings`

更新应用设置。

**参数**:
```typescript
interface UpdateSettingsParams {
  settings: Partial<AppSettings>; // 只需传递要更新的字段
}
```

**返回**:
```typescript
interface UpdateSettingsResult {
  success: boolean;
  error?: string;
}
```

**示例**:
```typescript
await invoke('update_settings', {
  settings: {
    tts_mode: "edge_only",
    speech_speed: 0.8
  }
});
```

---

#### `switch_ai_model`

切换 AI 模型。

**参数**:
```typescript
interface SwitchAiModelParams {
  model: string;               // "qwen2.5:3b" | "qwen2.5:7b-q4_K_M"
}
```

**返回**:
```typescript
interface SwitchAiModelResult {
  success: boolean;
  error?: string;
}
```

**说明**:
- 后台执行 `ollama stop` 和 `ollama run <new-model>`
- 过程可能需要几秒,前端应显示加载状态

**示例**:
```typescript
const result = await invoke('switch_ai_model', {
  model: "qwen2.5:7b-q4_K_M"
});
```

---

### 2.5 系统管理命令

#### `check_system_health`

检查系统健康状态。

**参数**: 无

**返回**:
```typescript
interface SystemHealth {
  ollama_running: boolean;
  ollama_port_available: boolean;
  model_loaded: boolean;
  current_model: string;
  memory_usage_mb: number;
  disk_space_gb: number;       // 可用磁盘空间
}

interface CheckSystemHealthResult {
  health: SystemHealth;
}
```

**示例**:
```typescript
const result = await invoke('check_system_health');
```

---

#### `restart_ollama`

重启 Ollama 服务(释放内存)。

**参数**: 无

**返回**:
```typescript
interface RestartOllamaResult {
  success: boolean;
  error?: string;
}
```

**说明**:
- 用于解决内存占用过高问题
- 重启期间无法使用 AI 功能,前端应显示提示

**示例**:
```typescript
await invoke('restart_ollama');
```

---

#### `export_data`

导出数据(对话记录、生词本)。

**参数**:
```typescript
interface ExportDataParams {
  format: "json" | "csv";
  include_sessions: boolean;
  include_vocabulary: boolean;
}
```

**返回**:
```typescript
interface ExportDataResult {
  success: boolean;
  file_path?: string;          // 导出文件路径
  error?: string;
}
```

**示例**:
```typescript
const result = await invoke('export_data', {
  format: "json",
  include_sessions: true,
  include_vocabulary: true
});
```

---

## 3. Tauri Events (后端推送前端)

### 3.1 AI 响应事件

#### `ai_response_chunk`

AI 流式回复的一个片段。

**数据结构**:
```typescript
interface AiResponseChunkEvent {
  session_id: number;
  chunk: string;               // 文本片段
  is_complete: boolean;        // 是否为最后一个片段
}
```

**前端监听**:
```typescript
listen('ai_response_chunk', (event: AiResponseChunkEvent) => {
  appendToChatBubble(event.payload.chunk);
  if (event.payload.is_complete) {
    finalizeMessage();
  }
});
```

---

#### `ai_response_error`

AI 响应出错。

**数据结构**:
```typescript
interface AiResponseErrorEvent {
  session_id: number;
  error_code: string;          // "model_not_loaded" | "timeout" | "unknown"
  error_message: string;
}
```

**前端监听**:
```typescript
listen('ai_response_error', (event: AiResponseErrorEvent) => {
  showErrorNotification(event.payload.error_message);
});
```

---

### 3.2 录音状态事件

#### `recording_status_changed`

录音状态变化。

**数据结构**:
```typescript
interface RecordingStatusEvent {
  is_recording: boolean;
  duration_seconds?: number;   // 录音时长(仅当 is_recording=true)
}
```

**前端监听**:
```typescript
listen('recording_status_changed', (event: RecordingStatusEvent) => {
  updateRecordingUI(event.payload.is_recording, event.payload.duration_seconds);
});
```

---

#### `stt_result`

语音识别完成。

**数据结构**:
```typescript
interface SttResultEvent {
  text: string;
  confidence: number;          // 识别置信度 0-1
}
```

**前端监听**:
```typescript
listen('stt_result', (event: SttResultEvent) => {
  displayRecognizedText(event.payload.text);
});
```

---

### 3.3 音频播放事件

#### `audio_playback_status`

音频播放状态变化。

**数据结构**:
```typescript
interface AudioPlaybackStatusEvent {
  status: "playing" | "paused" | "stopped" | "error";
  progress?: number;           // 播放进度 0-100
  error_message?: string;
}
```

**前端监听**:
```typescript
listen('audio_playback_status', (event: AudioPlaybackStatusEvent) => {
  updateAudioPlayerUI(event.payload.status, event.payload.progress);
});
```

---

### 3.4 网络状态事件

#### `network_status_changed`

网络连通性变化。

**数据结构**:
```typescript
interface NetworkStatusEvent {
  status: "online" | "offline";
}
```

**前端监听**:
```typescript
listen('network_status_changed', (event: NetworkStatusEvent) => {
  updateNetworkIndicator(event.payload.status);
  if (event.payload.status === "offline") {
    showOfflineModeWarning();
  }
});
```

---

### 3.5 系统事件

#### `ollama_status_changed`

Ollama 服务状态变化。

**数据结构**:
```typescript
interface OllamaStatusEvent {
  status: "starting" | "running" | "stopped" | "error";
  message?: string;
}
```

**前端监听**:
```typescript
listen('ollama_status_changed', (event: OllamaStatusEvent) => {
  updateServiceStatusIndicator(event.payload.status);
});
```

---

#### `memory_warning`

内存占用警告。

**数据结构**:
```typescript
interface MemoryWarningEvent {
  usage_percentage: number;    // 内存使用百分比
  recommendation: string;      // 建议操作
}
```

**前端监听**:
```typescript
listen('memory_warning', (event: MemoryWarningEvent) => {
  showMemoryWarning(event.payload.usage_percentage, event.payload.recommendation);
});
```

---

#### `model_download_progress`

模型下载进度。

**数据结构**:
```typescript
interface ModelDownloadProgressEvent {
  model_name: string;
  downloaded_mb: number;
  total_mb: number;
  percentage: number;
  speed_mbps: number;          // 下载速度 MB/s
}
```

**前端监听**:
```typescript
listen('model_download_progress', (event: ModelDownloadProgressEvent) => {
  updateDownloadProgressBar(event.payload);
});
```

---

## 4. Ollama HTTP API 封装

### 4.1 基础配置

```rust
// Rust 后端配置
const OLLAMA_BASE_URL: &str = "http://localhost:11434";
const OLLAMA_API_TIMEOUT: Duration = Duration::from_secs(30);
```

### 4.2 Generate API (流式)

**请求**:
```http
POST http://localhost:11434/api/generate
Content-Type: application/json

{
  "model": "qwen2.5:3b",
  "prompt": "[完整 Prompt + 对话历史 + 用户输入]",
  "stream": true,
  "options": {
    "temperature": 0.7,
    "top_p": 0.9,
    "num_predict": 512
  }
}
```

**响应** (Server-Sent Events):
```json
{"response":"Hello","done":false}
{"response":"! How","done":false}
{"response":" can I help","done":false}
{"response":" you?","done":true}
```

**Rust 实现要点**:
```rust
use reqwest::Client;
use tokio_stream::StreamExt;

async fn stream_generate(prompt: String) -> Result<(), Error> {
    let client = Client::new();
    let mut stream = client
        .post(format!("{}/api/generate", OLLAMA_BASE_URL))
        .json(&GenerateRequest {
            model: "qwen2.5:3b".to_string(),
            prompt,
            stream: true,
            options: Some(GenerationOptions {
                temperature: 0.7,
                top_p: 0.9,
                num_predict: 512,
            }),
        })
        .send()
        .await?
        .bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let response: GenerateResponse = serde_json::from_slice(&chunk)?;
        
        // 通过 Tauri Event 推送到前端
        app.emit_all("ai_response_chunk", AiResponseChunkEvent {
            session_id: current_session_id,
            chunk: response.response,
            is_complete: response.done,
        })?;
        
        if response.done {
            break;
        }
    }
    
    Ok(())
}
```

---

### 4.3 Chat API (可选,支持对话历史)

**请求**:
```http
POST http://localhost:11434/api/chat
Content-Type: application/json

{
  "model": "qwen2.5:3b",
  "messages": [
    {"role": "system", "content": "[System Prompt]"},
    {"role": "user", "content": "Hello"},
    {"role": "assistant", "content": "Hi there!"},
    {"role": "user", "content": "How are you?"}
  ],
  "stream": true
}
```

**说明**: 
- MVP 阶段使用 `generate` API 即可
- 未来可扩展为 `chat` API 以更好地管理对话历史

---

## 5. Edge TTS API 封装

### 5.1 在线 Edge TTS 调用

**Rust 依赖**:
```toml
# Cargo.toml
[dependencies]
edge-tts = "0.5"
tokio = { version = "1", features = ["full"] }
```

**实现**:
```rust
use edge_tts::{EdgeTTS, Voice};

async fn synthesize_edge_tts(text: String, voice: Voice) -> Result<Vec<u8>, Error> {
    let mut tts = EdgeTTS::new();
    
    tts.set_text(&text);
    tts.set_voice(voice);
    tts.set_rate(1.0);      // 语速
    tts.set_volume(1.0);    // 音量
    
    let audio_data = tts.synthesize().await?;
    Ok(audio_data)          // MP3 格式音频数据
}
```

**缓存策略**:
```rust
use std::collections::HashMap;
use tokio::sync::Mutex;

static TTS_CACHE: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

async fn get_or_synthesize(text: String) -> Result<Vec<u8>, Error> {
    let cache_key = format!("{}_{}", text, "en-US-AriaNeural");
    
    // 检查缓存
    {
        let cache = TTS_CACHE.lock().await;
        if let Some(audio) = cache.get(&cache_key) {
            return Ok(audio.clone());
        }
    }
    
    // 合成并缓存
    let audio = synthesize_edge_tts(text, Voice::EnUsAriaNeural).await?;
    
    {
        let mut cache = TTS_CACHE.lock().await;
        cache.insert(cache_key, audio.clone());
    }
    
    Ok(audio)
}
```

---

### 5.2 系统 TTS 降级

**Windows**:
```rust
#[cfg(target_os = "windows")]
async fn synthesize_system_tts(text: String) -> Result<Vec<u8>, Error> {
    use windows::Media::SpeechSynthesis::SpeechSynthesizer;
    
    let synthesizer = SpeechSynthesizer::new()?;
    let stream = synthesizer.SynthesizeTextToStreamAsync(&text)?.await?;
    
    // 转换为 MP3/WAV
    let audio_data = extract_audio_from_stream(stream)?;
    Ok(audio_data)
}
```

**macOS**:
```rust
#[cfg(target_os = "macos")]
async fn synthesize_system_tts(text: String) -> Result<Vec<u8>, Error> {
    use cocoa::foundation::NSString;
    use objc::runtime::Class;
    
    // 调用 AVSpeechSynthesizer
    let audio_data = call_avspeech_synthesizer(&text)?;
    Ok(audio_data)
}
```

**Linux**:
```rust
#[cfg(target_os = "linux")]
async fn synthesize_system_tts(text: String) -> Result<Vec<u8>, Error> {
    // 使用 espeak 或 festival
    let output = Command::new("espeak")
        .args(&["-w", "/tmp/tts_output.wav", &text])
        .output()?;
    
    let audio_data = tokio::fs::read("/tmp/tts_output.wav").await?;
    Ok(audio_data)
}
```

---

## 6. 数据格式规范

### 6.1 JSON Schema

#### Message 对象
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Message",
  "type": "object",
  "properties": {
    "id": { "type": "integer" },
    "session_id": { "type": "integer" },
    "role": { "type": "string", "enum": ["user", "assistant"] },
    "content": { "type": "string" },
    "created_at": { "type": "string", "format": "date-time" }
  },
  "required": ["id", "session_id", "role", "content", "created_at"]
}
```

#### VocabularyItem 对象
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "VocabularyItem",
  "type": "object",
  "properties": {
    "id": { "type": "integer" },
    "word": { "type": "string" },
    "first_learned": { "type": "string", "format": "date-time" },
    "last_reviewed": { "type": "string", "format": "date-time" },
    "review_count": { "type": "integer", "minimum": 0 },
    "mastery_level": { "type": "integer", "minimum": 0, "maximum": 5 }
  },
  "required": ["id", "word", "first_learned", "review_count", "mastery_level"]
}
```

---

### 6.2 错误码定义

| 错误码 | 含义 | 前端处理建议 |
| :--- | :--- | :--- |
| `MODEL_NOT_LOADED` | AI 模型未加载 | 提示用户等待模型加载或重新下载 |
| `STT_PERMISSION_DENIED` | 麦克风权限被拒绝 | 引导用户在系统设置中授权 |
| `TTS_NETWORK_ERROR` | Edge TTS 网络错误 | 自动降级到系统 TTS |
| `OLLAMA_CONNECTION_FAILED` | 无法连接 Ollama | 尝试重启 Ollama 服务 |
| `DATABASE_ERROR` | 数据库操作失败 | 显示错误详情,建议重启应用 |
| `INVALID_PARAMS` | 参数无效 | 检查调用代码,修复 bug |
| `TIMEOUT` | 请求超时 | 提示用户重试 |
| `UNKNOWN_ERROR` | 未知错误 | 记录日志,上报反馈 |

---

## 7. 最佳实践

### 7.1 错误处理

**前端**:
```typescript
try {
  const result = await invoke('send_message', params);
  if (!result.success) {
    handleError(result.error);
  }
} catch (error) {
  console.error('Command failed:', error);
  showGenericError();
}
```

**后端**:
```rust
#[tauri::command]
async fn send_message(session_id: i32, text: String) -> Result<SendMessageResult, String> {
    // 验证参数
    if text.is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    
    // 执行业务逻辑
    match process_message(session_id, text).await {
        Ok(_) => Ok(SendMessageResult { success: true, error: None }),
        Err(e) => Ok(SendMessageResult { success: false, error: Some(e.to_string()) }),
    }
}
```

---

### 7.2 性能优化

1. **流式响应**: AI 回复必须流式推送,避免等待完整响应
2. **音频缓存**: Edge TTS 结果缓存,减少重复网络请求
3. **懒加载**: 首次启动时延迟加载 Ollama,优先显示 UI
4. **批量操作**: 生词本查询使用分页,避免一次性加载全部数据

---

### 7.3 安全考虑

1. **输入验证**: 所有前端传入的参数必须验证
2. **SQL 注入防护**: 使用参数化查询,禁止字符串拼接 SQL
3. **路径遍历防护**: 文件导出时验证路径合法性
4. **权限检查**: 敏感操作(如删除会话)需二次确认

---

## 8. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义所有核心 API | LingoMate Team |

---

**文档结束**
