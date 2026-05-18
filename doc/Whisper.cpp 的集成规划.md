# 集成 Whisper.cpp 进行离线语音识别

## 当前问题
- macOS Speech Framework 返回 "Retry" 错误 (code=203)
- 依赖 Apple 云服务，稳定性不可控
- 需要网络连接

## 解决方案：Whisper.cpp

### 优势
- ✅ 完全离线运行
- ✅ 支持多种语言（包括英语）
- ✅ 高质量识别
- ✅ 开源免费
- ✅ 可在本地运行，保护隐私

### 实现步骤

#### 1. 添加 Whisper.cpp 依赖
在 `src-tauri/Cargo.toml` 中添加：
```toml
[dependencies]
whisper-rs = "0.10"  # Rust binding for whisper.cpp