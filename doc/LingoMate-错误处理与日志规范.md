# LingoMate 错误处理与日志规范

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | 后端开发团队 |

---

## 1. 错误处理原则

### 1.1 核心设计理念

1. **用户友好**: 向用户展示清晰、易懂的错误提示,避免技术术语
2. **开发者友好**: 记录详细的错误日志,便于快速定位问题
3. **优雅降级**: 遇到非致命错误时,尽可能提供替代方案而非完全中断
4. **安全优先**: 不泄露敏感信息 (API 密钥、用户数据、系统路径)

### 1.2 错误分类

| 类别 | 严重程度 | 处理方式 | 示例 |
| :--- | :--- | :--- | :--- |
| **Critical** | P0 | 立即终止操作,显示错误页 | 数据库损坏、模型文件丢失 |
| **Error** | P1 | 中断当前操作,提示用户重试 | 网络超时、权限拒绝 |
| **Warning** | P2 | 继续执行,显示警告信息 | TTS 降级、低内存警告 |
| **Info** | P3 | 仅记录日志 | 功能使用统计、性能数据 |

---

## 2. 错误码定义

### 2.1 错误码规范

**格式**: `DOMAIN_ERROR_CODE`

- **DOMAIN**: 错误所属模块 (大写)
- **ERROR_CODE**: 具体错误标识 (大写蛇形命名)

**示例**:
- `STT_PERMISSION_DENIED`
- `TTS_NETWORK_ERROR`
- `OLLAMA_MODEL_NOT_FOUND`

---

### 2.2 完整错误码列表

#### STT (语音识别) 错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `STT_PERMISSION_DENIED` | 403 | "麦克风访问被拒绝。请在系统设置中允许 LingoMate 使用麦克风。" | "Microphone permission denied by user" |
| `STT_HARDWARE_UNAVAILABLE` | 503 | "麦克风不可用。请检查设备连接。" | "No microphone device detected" |
| `STT_RECOGNITION_FAILED` | 500 | "语音识别失败。请再试一次。" | "STT engine returned error: {details}" |
| `STT_TIMEOUT` | 408 | "录音超时。请缩短单次发言长度。" | "STT processing timeout after 30s" |

---

#### TTS (语音合成) 错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `TTS_NETWORK_ERROR` | 503 | "网络连接异常,切换到离线语音。" | "Edge TTS request failed: {error}" |
| `TTS_ENGINE_UNAVAILABLE` | 503 | "语音引擎不可用。" | "No TTS engine available (Edge + System both failed)" |
| `TTS_SYNTHESIS_FAILED` | 500 | "语音合成失败。" | "TTS synthesis error: {details}" |
| `TTS_PLAYBACK_ERROR` | 500 | "音频播放失败。" | "Audio playback error: {device_error}" |

---

#### Ollama (AI 服务) 错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `OLLAMA_NOT_RUNNING` | 503 | "AI 服务未启动。正在尝试重启..." | "Ollama process not found on port 11434" |
| `OLLAMA_MODEL_NOT_FOUND` | 404 | "AI 模型未下载。开始下载模型 (约 2GB)..." | "Model 'qwen2.5:3b' not found in local registry" |
| `OLLAMA_OUT_OF_MEMORY` | 507 | "内存不足。建议关闭其他程序或使用流畅模式。" | "Ollama memory usage exceeded 80% of system RAM" |
| `OLLAMA_TIMEOUT` | 408 | "AI 响应超时。请重试或切换至更小的模型。" | "Generate request timeout after 60s" |
| `OLLAMA_GENERATION_ERROR` | 500 | "AI 生成失败。请重试。" | "Ollama generate error: {error_message}" |

---

#### 数据库错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `DB_CONNECTION_FAILED` | 500 | "数据库连接失败。请重启应用。" | "SQLite connection error: {details}" |
| `DB_QUERY_FAILED` | 500 | "数据查询失败。" | "SQL query error: {query}, error: {details}" |
| `DB_CONSTRAINT_VIOLATION` | 409 | "数据冲突。" | "Unique constraint violated: {constraint_name}" |
| `DB_MIGRATION_FAILED` | 500 | "数据库升级失败。请联系技术支持。" | "Migration v{from}_to_v{to} failed: {error}" |

---

#### 文件系统错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `FS_PERMISSION_DENIED` | 403 | "文件访问被拒绝。" | "Permission denied: {path}" |
| `FS_DISK_FULL` | 507 | "磁盘空间不足。请清理空间后重试。" | "No space left on device: {available_bytes} bytes remaining" |
| `FS_FILE_NOT_FOUND` | 404 | "文件不存在。" | "File not found: {path}" |
| `FS_WRITE_FAILED` | 500 | "文件写入失败。" | "Write error: {path}, reason: {details}" |

---

#### 网络错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `NETWORK_OFFLINE` | 503 | "网络已断开。部分功能可能不可用。" | "Network connectivity check failed" |
| `NETWORK_DNS_ERROR` | 503 | "DNS 解析失败。请检查网络设置。" | "DNS resolution failed for {domain}" |
| `NETWORK_TIMEOUT` | 408 | "请求超时。请检查网络连接。" | "Request timeout after {timeout_ms}ms" |
| `NETWORK_SSL_ERROR` | 500 | "安全连接失败。" | "SSL/TLS handshake error: {details}" |

---

#### 参数验证错误

| 错误码 | HTTP 状态 | 用户提示 | 开发者信息 |
| :--- | :--- | :--- | :--- |
| `INVALID_PARAMS` | 400 | "输入参数无效。" | "Validation failed: {field} - {reason}" |
| `MISSING_REQUIRED_FIELD` | 400 | "缺少必填字段。" | "Required field missing: {field_name}" |
| `INVALID_ENUM_VALUE` | 400 | "选项值无效。" | "Invalid enum value: {value}, expected: {valid_values}" |

---

## 3. Rust 后端错误处理

### 3.1 自定义错误类型

**`src-tauri/src/errors.rs`**:

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    // STT Errors
    #[error("STT permission denied")]
    SttPermissionDenied,
    
    #[error("STT recognition failed: {0}")]
    SttRecognitionFailed(String),
    
    // TTS Errors
    #[error("TTS network error: {0}")]
    TtsNetworkError(String),
    
    #[error("TTS engine unavailable")]
    TtsEngineUnavailable,
    
    // Ollama Errors
    #[error("Ollama not running")]
    OllamaNotRunning,
    
    #[error("Ollama model not found: {0}")]
    OllamaModelNotFound(String),
    
    #[error("Ollama out of memory")]
    OllamaOutOfMemory,
    
    // Database Errors
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    // File System Errors
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Disk space insufficient")]
    DiskSpaceInsufficient,
    
    // Network Errors
    #[error("Network offline")]
    NetworkOffline,
    
    #[error("Request timeout")]
    RequestTimeout,
    
    // Validation Errors
    #[error("Invalid parameter: {field} - {reason}")]
    InvalidParameter { field: String, reason: String },
    
    // Generic Error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl AppError {
    /// 获取用户友好的错误消息
    pub fn user_message(&self) -> String {
        match self {
            AppError::SttPermissionDenied => 
                "麦克风访问被拒绝。请在系统设置中允许 LingoMate 使用麦克风。".to_string(),
            AppError::TtsNetworkError(_) => 
                "网络连接异常,已切换到离线语音。".to_string(),
            AppError::OllamaModelNotFound(model) => 
                format!("AI 模型 {} 未下载。开始下载模型 (约 2GB)...", model),
            AppError::OllamaOutOfMemory => 
                "内存不足。建议关闭其他程序或使用流畅模式。".to_string(),
            AppError::DiskSpaceInsufficient => 
                "磁盘空间不足。请清理空间后重试。".to_string(),
            AppError::NetworkOffline => 
                "网络已断开。部分功能可能不可用。".to_string(),
            _ => "发生未知错误。请重试或联系技术支持。".to_string(),
        }
    }
    
    /// 获取错误码
    pub fn error_code(&self) -> String {
        match self {
            AppError::SttPermissionDenied => "STT_PERMISSION_DENIED".to_string(),
            AppError::TtsNetworkError(_) => "TTS_NETWORK_ERROR".to_string(),
            AppError::OllamaModelNotFound(_) => "OLLAMA_MODEL_NOT_FOUND".to_string(),
            // ... 其他映射
            _ => "UNKNOWN_ERROR".to_string(),
        }
    }
    
    /// 判断是否可恢复
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AppError::TtsNetworkError(_) |
            AppError::NetworkOffline |
            AppError::RequestTimeout
        )
    }
}

/// Tauri Command 返回类型
pub type CommandResult<T> = Result<T, AppError>;
```

---

### 3.2 错误转换

**从第三方库错误转换为 AppError**:

```rust
use rusqlite;
use reqwest;

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::RequestTimeout
        } else if err.is_connect() {
            AppError::NetworkOffline
        } else {
            AppError::Unknown(format!("HTTP error: {}", err))
        }
    }
}
```

---

### 3.3 Tauri Command 错误处理

**示例: 发送消息命令**:

```rust
#[tauri::command]
async fn send_message(
    app_handle: AppHandle,
    session_id: i32,
    text: String,
) -> CommandResult<SendMessageResult> {
    // 1. 参数验证
    if text.trim().is_empty() {
        return Err(AppError::InvalidParameter {
            field: "text".to_string(),
            reason: "Message cannot be empty".to_string(),
        });
    }
    
    // 2. 检查会话是否存在
    let session = get_session(session_id).await?;
    if session.is_none() {
        return Err(AppError::InvalidParameter {
            field: "session_id".to_string(),
            reason: "Session not found".to_string(),
        });
    }
    
    // 3. 保存用户消息
    save_message(session_id, "user", &text).await?;
    
    // 4. 调用 Ollama
    match generate_ai_response(&text).await {
        Ok(response_stream) => {
            // 流式推送响应
            stream_to_frontend(app_handle, response_stream).await?;
            Ok(SendMessageResult { success: true })
        }
        Err(AppError::OllamaNotRunning) => {
            // 尝试重启 Ollama
            restart_ollama().await?;
            // 重试
            let response_stream = generate_ai_response(&text).await?;
            stream_to_frontend(app_handle, response_stream).await?;
            Ok(SendMessageResult { success: true })
        }
        Err(e) => Err(e),
    }
}
```

---

## 4. 前端错误处理

### 4.1 统一错误处理器

**`src/utils/errorHandler.ts`**:

```typescript
interface AppError {
  code: string;
  message: string;
  details?: string;
  recoverable: boolean;
}

class ErrorHandler {
  private static instance: ErrorHandler;
  
  static getInstance(): ErrorHandler {
    if (!ErrorHandler.instance) {
      ErrorHandler.instance = new ErrorHandler();
    }
    return ErrorHandler.instance;
  }
  
  /**
   * 处理 Tauri Command 错误
   */
  handleCommandError(error: any): AppError {
    console.error('Command error:', error);
    
    // 解析错误对象
    const appError: AppError = {
      code: error.code || 'UNKNOWN_ERROR',
      message: error.user_message || '发生未知错误',
      details: error.details,
      recoverable: error.recoverable || false,
    };
    
    // 根据错误类型显示不同提示
    this.showErrorNotification(appError);
    
    // 记录日志
    this.logError(appError);
    
    return appError;
  }
  
  /**
   * 显示错误通知
   */
  private showErrorNotification(error: AppError) {
    const toastType = error.recoverable ? 'warning' : 'error';
    
    showToast({
      type: toastType,
      title: '错误',
      message: error.message,
      action: error.recoverable ? {
        label: '重试',
        onClick: () => this.retryLastOperation()
      } : undefined,
    });
  }
  
  /**
   * 记录错误日志
   */
  private logError(error: AppError) {
    invoke('log_error', {
      level: 'error',
      code: error.code,
      message: error.message,
      details: error.details,
      timestamp: new Date().toISOString(),
    }).catch(console.error);
  }
  
  /**
   * 重试最后一次操作
   */
  private retryLastOperation() {
    // 实现重试逻辑
    console.log('Retrying last operation...');
  }
}

export const errorHandler = ErrorHandler.getInstance();
```

---

### 4.2 Try-Catch 包装器

**`src/hooks/useAsyncCommand.ts`**:

```typescript
import { invoke } from '@tauri-apps/api';
import { errorHandler } from '../utils/errorHandler';

export function useAsyncCommand() {
  const execute = async <T>(command: string, params?: any): Promise<T | null> => {
    try {
      const result = await invoke<T>(command, params);
      return result;
    } catch (error) {
      errorHandler.handleCommandError(error);
      return null;
    }
  };
  
  const executeWithRetry = async <T>(
    command: string,
    params?: any,
    maxRetries: number = 3
  ): Promise<T | null> => {
    for (let i = 0; i < maxRetries; i++) {
      try {
        const result = await invoke<T>(command, params);
        return result;
      } catch (error) {
        const appError = errorHandler.handleCommandError(error);
        
        if (!appError.recoverable || i === maxRetries - 1) {
          return null;
        }
        
        // 等待后重试 (指数退避)
        const delay = Math.pow(2, i) * 1000;
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }
    
    return null;
  };
  
  return { execute, executeWithRetry };
}
```

---

### 4.3 错误边界组件

**`src/components/ErrorBoundary.tsx`**:

```tsx
import React, { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

class ErrorBoundary extends Component<Props, State> {
  state: State = {
    hasError: false,
    error: null,
  };
  
  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }
  
  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught:', error, errorInfo);
    
    // 记录到后端日志
    invoke('log_error', {
      level: 'critical',
      code: 'REACT_RENDER_ERROR',
      message: error.message,
      details: errorInfo.componentStack,
    }).catch(console.error);
  }
  
  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="error-fallback">
          <h2>Something went wrong</h2>
          <p>{this.state.error?.message}</p>
          <button onClick={() => window.location.reload()}>
            Reload Application
          </button>
        </div>
      );
    }
    
    return this.props.children;
  }
}

export default ErrorBoundary;
```

**使用**:

```tsx
import ErrorBoundary from './components/ErrorBoundary';

function App() {
  return (
    <ErrorBoundary>
      <ChatInterface />
    </ErrorBoundary>
  );
}
```

---

## 5. 日志系统

### 5.1 日志级别

| 级别 | 用途 | 示例 |
| :--- | :--- | :--- |
| **ERROR** | 系统错误,需要立即关注 | 崩溃、数据丢失、服务不可用 |
| **WARN** | 潜在问题,不影响当前功能 | TTS 降级、低内存警告 |
| **INFO** | 重要事件,用于审计和分析 | 用户登录、功能使用、模型下载 |
| **DEBUG** | 详细调试信息 (仅开发环境) | API 请求详情、函数调用栈 |

---

### 5.2 Rust 日志配置

**`Cargo.toml`**:

```toml
[dependencies]
log = "0.4"
env_logger = "0.10"
tauri-plugin-log = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "dev" }
```

**`src-tauri/src/main.rs`**:

```rust
use log::{info, warn, error, debug};

fn main() {
    // 初始化日志
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 使用示例
fn start_ollama() -> Result<(), Error> {
    info!("Starting Ollama service...");
    
    match Command::new("ollama").arg("serve").spawn() {
        Ok(child) => {
            info!("Ollama started with PID: {}", child.id());
            Ok(())
        }
        Err(e) => {
            error!("Failed to start Ollama: {}", e);
            Err(Error::OllamaStartFailed(e.to_string()))
        }
    }
}

fn select_tts_engine(config: &TtsConfig) -> TtsEngine {
    if config.mode == TtsMode::Auto {
        if is_network_available() {
            info!("Using Edge TTS (online)");
            TtsEngine::Edge
        } else {
            warn!("Network unavailable, falling back to System TTS");
            TtsEngine::System
        }
    } else {
        debug!("TTS mode: {:?}", config.mode);
        // ...
    }
}
```

---

### 5.3 日志文件配置

**`src-tauri/tauri.conf.json`**:

```json
{
  "plugins": {
    "log": {
      "logPath": "$APPDATA/com.lingomate.app/logs",
      "rotationStrategy": "daily",
      "maxLogFiles": 7,
      "levels": {
        "default": "info",
        "tauri": "error"
      }
    }
  }
}
```

**日志文件结构**:

```
~/Library/Logs/com.lingomate.app/
├── 2026-05-17.log
├── 2026-05-16.log
├── 2026-05-15.log
└── latest.log -> 2026-05-17.log (symlink)
```

---

### 5.4 前端日志上报

**`src/utils/logger.ts`**:

```typescript
import { invoke } from '@tauri-apps/api';

enum LogLevel {
  DEBUG = 'debug',
  INFO = 'info',
  WARN = 'warn',
  ERROR = 'error',
}

class Logger {
  private static instance: Logger;
  
  static getInstance(): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }
  
  debug(message: string, meta?: any) {
    this.log(LogLevel.DEBUG, message, meta);
  }
  
  info(message: string, meta?: any) {
    this.log(LogLevel.INFO, message, meta);
  }
  
  warn(message: string, meta?: any) {
    this.log(LogLevel.WARN, message, meta);
  }
  
  error(message: string, error?: any) {
    this.log(LogLevel.ERROR, message, {
      error: error?.message,
      stack: error?.stack,
    });
  }
  
  private async log(level: LogLevel, message: string, meta?: any) {
    // 控制台输出 (开发环境)
    if (import.meta.env.DEV) {
      console[level](message, meta);
    }
    
    // 上报到后端日志文件
    try {
      await invoke('log_message', {
        level,
        message,
        meta: meta ? JSON.stringify(meta) : null,
        timestamp: new Date().toISOString(),
      });
    } catch (e) {
      // 日志失败不应影响主流程
      console.error('Failed to log:', e);
    }
  }
}

export const logger = Logger.getInstance();

// 使用示例
logger.info('User started recording');
logger.error('TTS synthesis failed', error);
```

---

## 6. 监控与告警

### 6.1 关键指标监控

| 指标 | 阈值 | 告警方式 |
| :--- | :--- | :--- |
| **崩溃率** | > 0.1% | 邮件 + Slack |
| **平均响应延迟** | > 3 秒 | Slack |
| **错误率 (5xx)** | > 1% | Slack |
| **磁盘空间** | < 1 GB | 本地通知 |
| **内存占用** | > 80% | 本地通知 |

---

### 6.2 错误上报服务 (可选,MVP暂不实现)

**集成 Sentry**:

```toml
# Cargo.toml
[dependencies]
sentry = "0.31"
```

```rust
use sentry;

fn main() {
    let _guard = sentry::init(("https://xxx@sentry.io/xxx", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    
    // 自动捕获 panic
    sentry::configure_scope(|scope| {
        scope.set_tag("app", "lingomate");
    });
}

// 手动上报错误
fn handle_critical_error(error: &AppError) {
    sentry::capture_message(
        &format!("Critical error: {:?}", error),
        sentry::Level::Error,
    );
}
```

---

## 7. 用户反馈机制

### 7.1 错误报告对话框

**当发生 Critical 错误时**:

```tsx
import { showDialog } from '@tauri-apps/api/dialog';

function showCrashReportDialog(error: AppError) {
  showDialog({
    title: 'Application Error',
    message: `LingoMate encountered a critical error:\n\n${error.message}`,
    type: 'error',
    buttons: [
      {
        label: 'Report Issue',
        action: () => openIssueTracker(error),
      },
      {
        label: 'Restart',
        action: () => restartApp(),
      },
    ],
  });
}

async function openIssueTracker(error: AppError) {
  // 收集诊断信息
  const diagnostics = await collectDiagnostics();
  
  // 打开 GitHub Issue 页面
  const issueUrl = `https://github.com/lingomate/lingomate/issues/new?title=${encodeURIComponent(
    `Bug: ${error.code}`
  )}&body=${encodeURIComponent(diagnostics)}`;
  
  open(issueUrl);
}
```

---

### 7.2 诊断信息收集

**`src/utils/diagnostics.ts`**:

```typescript
export async function collectDiagnostics(): Promise<string> {
  const systemInfo = await invoke('get_system_info');
  const appVersion = await invoke('get_app_version');
  const recentLogs = await invoke('get_recent_logs', { count: 50 });
  
  return `
## System Information
- OS: ${systemInfo.os}
- Memory: ${systemInfo.memory_gb} GB
- App Version: ${appVersion}

## Error Details
- Error Code: ${lastError.code}
- Message: ${lastError.message}
- Timestamp: ${lastError.timestamp}

## Recent Logs
\`\`\`
${recentLogs.join('\n')}
\`\`\`

## Steps to Reproduce
1. ...
2. ...
3. ...
  `.trim();
}
```

---

## 8. 最佳实践

### 8.1 Do's and Don'ts

#### ✅ Do

- 使用具体的错误码,而非通用错误
- 提供可操作的错误提示
- 记录足够的上下文信息
- 实现优雅降级
- 定期审查日志文件

#### ❌ Don't

- 不要向用户展示原始错误堆栈
- 不要泄露敏感信息 (API 密钥、密码)
- 不要忽略异步错误
- 不要在生产环境使用 DEBUG 级别日志
- 不要让日志文件无限增长

---

### 8.2 错误处理检查清单

在提交代码前,确认:

- [ ] 所有可能失败的操作都有错误处理
- [ ] 错误消息对用户友好
- [ ] 错误日志包含足够调试信息
- [ ] 可恢复错误提供了重试机制
- [ ] Critical 错误有适当的回退方案
- [ ] 没有泄露敏感信息

---

## 9. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义错误处理和日志规范 | LingoMate Team |

---

**文档结束**
