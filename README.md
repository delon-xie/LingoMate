# LingoMate - AI English Tutor

LingoMate 是一款"下载即用、对话即学"的 AI 英语私教桌面应用，通过启发式对话，让用户像拥有私人外教一样自然提升英语能力。

> 想想，世界有时真是一个巨大的草台班子！
> 连老师都是AI在扮演，一些人会为老师的下课而兴奋
> 另外一些人会觉得AI令这个世界太沮丧了，连沟通都找不到真人
> 
> 这个APP真实目的，英文自然对话渠道难以获得的人们，找一个替代品
> 当前APP的状态完全是实验室产品，不具备商用能力，未经严格设计，这也是它的魅力所在
> 实际上你是在进行一个简单限定条件下的AI对话
>
> 这个项目除了创意部分是人为假设的之外，其他绝大部分都是借助AI或完全由AI完成的
> 人只起到一个监督的作用

## 技术栈

### 核心框架
- **前端**: React + TypeScript + Vite + Tailwind CSS
- **后端**: Rust (Tauri 2.0)
- **AI 服务**: Ollama (qwen2.5:3b)
- **数据库**: SQLite
- **桌面框架**: Tauri

### 语音交互
- **语音合成 (TTS)**:
  - **Edge TTS** (首选): 微软在线语音合成服务，提供自然流畅的英语发音
    - 默认声音: `en-US-AriaNeural` (女声，自然清晰)
    - 语速设置: 0.85倍速（更慢、更清晰）
    - 支持 SSML 标记语言，可控制语调、停顿等
  - **System TTS** (备选): macOS 原生 `say` 命令，离线可用
    - 自动降级：网络不可用时切换到系统 TTS
    - 支持多种系统声音（Samantha, Alex, Kate 等）
  
- **语音识别 (STT)**:
  - **Whisper.cpp**: OpenAI Whisper 模型的 C++ 实现
    - 完全离线运行，保护隐私
    - 支持多语言识别
    - 高质量转录准确率
    - 使用模型：**ggml-small.bin** (约 466MB)
      - 下载地址：https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin
      - 自动下载脚本：`./download-whisper-model.sh`
      - 存储位置：`~/.lingomate/models/ggml-small.bin`
      - small 模型在速度和准确度之间取得良好平衡，适合实时对话场景
    - 当前状态：已集成，支持离线语音识别
  - ~~macOS Speech Framework~~ (已弃用): Apple 原生语音识别服务
    - 因稳定性问题（Retry 错误 code=203）已停用
    - 依赖云端服务，网络不稳定时体验差

### 音频处理
- **录音库**: cpal (跨平台音频录制)
- **音频格式**: WAV, 16-bit PCM, 16kHz 采样率
- **音频播放**: rodio (Rust 音频播放库)
- **WAV 处理**: hound (WAV 文件读写)

## 项目结构

```
LingoMate/
├── doc/                    # 设计文档
│   ├── LingoMate-API接口规范.md
│   ├── LingoMate-UI-UX设计规范.md
│   ├── LingoMate-数据库详细设计.md
│   └── ...
├── frontend/               # React 前端
│   ├── src/
│   │   ├── components/     # UI 组件
│   │   ├── pages/          # 页面组件
│   │   ├── services/       # API 服务
│   │   ├── types/          # TypeScript 类型
│   │   └── ...
│   └── ...
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── commands/       # Tauri Commands
│   │   ├── database/       # 数据库模块
│   │   ├── ollama.rs       # Ollama API 封装
│   │   ├── tts.rs          # TTS 模块
│   │   └── lib.rs          # 入口文件
│   ├── Cargo.toml
│   └── tauri.conf.json
└── README.md
```

## 快速开始

### 前置要求

1. **Node.js** >= 18.0
2. **Rust** >= 1.70
3. **Ollama** (用于本地 AI 服务)

### 安装 Ollama 和模型

```bash
# 安装 Ollama (macOS)
brew install ollama

# 或者从官网下载: https://ollama.com

# 启动 Ollama 服务
ollama serve &

# 拉取 qwen2.5:3b 模型
ollama pull qwen2.5:3b
```

### 安装依赖

```bash
# 安装前端依赖
cd frontend && npm install

# 确保 Rust 工具链已安装
rustup update stable
```

### 开发模式

```bash
#重载环境
source $HOME/.cargo/env

#检查环境是否已安装
rustc --version
cargo --version

#清理target缓存，如有必要
cargo clean
cd ..

# 启动 Tauri 开发模式（同时启动前端和后端）
npm run tauri:dev

# 或者单独启动前端
npm run dev
```

### 构建生产版本

```bash
# 构建桌面应用
npm run tauri:build

# 产物位于 src-tauri/target/release/bundle/
```

### 启动正式环境测试

```bash
# 1. 确保 Ollama 正在运行
ollama list

# 2. 构建前端
cd frontend && npm run build

# 3. 启动 Tauri 应用
cd .. && npm run tauri:dev
```

## 核心功能

### MVP 功能清单

- [✅] 情景选择页面 (24个场景，支持自定义扩展)
- [✅] 聊天对话界面
- [✅] 流式 AI 回复
- [✅] 单词"即点即学"
- [✅] 语音输入 (STT) - Whisper.cpp 已集成，支持离线语音识别
- [✅] 语音输出 (TTS) - Edge TTS + System TTS 双引擎
- [✅] 生词本管理
- [✅] 对话历史记录
- [✅] AI 生成个性化开场白
- [✅] 动态场景扩展（无数据库限制）

### API 命令

| 命令 | 描述 |
|------|------|
| `start_conversation` | 启动新对话会话 |
| `send_message` | 发送用户消息 |
| `teach_word` | 触发单词教学 |
| `stop_ai_response` | 中断 AI 回复 |
| `start_recording` | 开始语音录音 |
| `stop_recording` | 停止录音并识别 |
| `play_audio` | 播放 TTS 音频 |
| `stop_audio` | 停止音频播放 |
| `get_sessions` | 获取会话列表 |
| `get_messages` | 获取消息历史 |
| `get_vocabulary` | 获取生词本 |
| `get_vocabulary_detail` | 获取生词详情 |
| `update_vocabulary_review` | 更新复习状态 |
| `delete_session` | 删除会话 |
| `get_settings` | 获取应用设置 |
| `update_settings` | 更新应用设置 |
| `switch_ai_model` | 切换 AI 模型 |
| `check_system_health` | 检查系统状态 |
| `restart_ollama` | 重启 Ollama |
| `export_data` | 导出数据 |

完整 API 文档见 [doc/LingoMate-API接口规范.md](doc/LingoMate-API接口规范.md)

## 设计文档

- [Prompt 工程规范](doc/LingoMate-Prompt工程规范.md)
- [API 接口规范](doc/LingoMate-API接口规范.md)
- [UI/UX 设计规范](doc/LingoMate-UI-UX设计规范.md)
- [数据库详细设计](doc/LingoMate-数据库详细设计.md)
- [测试计划与用例](doc/LingoMate-测试计划与用例.md)
- [部署与打包指南](doc/LingoMate-部署与打包指南.md)
- [错误处理与日志规范](doc/LingoMate-错误处理与日志规范.md)
- [安全与隐私合规](doc/LingoMate-安全与隐私合规.md)

## 开发指南

### 添加新的 Tauri Command

1. 在 `src-tauri/src/commands/` 下创建或编辑模块文件
2. 使用 `#[tauri::command]` 装饰器定义命令
3. 在 `src-tauri/src/lib.rs` 中注册命令

```rust
// src-tauri/src/commands/my_module.rs
#[tauri::command]
pub async fn my_command() -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}

// src-tauri/src/lib.rs
.invoke_handler(tauri::generate_handler![
    my_command,
    // ...其他命令
])
```

### 添加新的前端组件

1. 在 `frontend/src/components/` 下创建组件
2. 使用 Tailwind CSS 进行样式设计
3. 通过 `frontend/src/services/api.ts` 调用后端命令

```tsx
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('my_command');
```

## 许可证

MIT

---

**LingoMate Team** - Making English learning effortless.
