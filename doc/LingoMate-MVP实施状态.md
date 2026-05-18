# LingoMate MVP 实施状态报告

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.1 |
| **维护者** | LingoMate Team |

---

## 1. 当前完成状态

### 1.1 已完成 (✅)

#### 项目框架
- ✅ Tauri 2.0 + React + TypeScript 项目结构
- ✅ Rust 后端命令框架 (17个 commands 定义)
- ✅ SQLite 数据库 Schema (4张表完整定义)
- ✅ 前端组件库 (5个UI组件 + 3个页面)
- ✅ Tailwind CSS 黄色系主题配置
- ✅ TypeScript 类型定义完整
- ✅ API 服务封装完整

#### 文档
- ✅ 框架构建指南 (含问题排查记录)
- ✅ API 接口规范
- ✅ UI/UX 设计规范
- ✅ 数据库详细设计
- ✅ README 项目说明

#### 环境验证
- ✅ Node.js v26.0.0 已安装
- ✅ Rust 1.95.0 已安装
- ✅ Ollama 已安装, qwen2.5:3b 模型已下载
- ✅ 前端构建成功 (`npm run build`)
- ✅ Rust 编译通过 (`cargo check`)

---

### 1.2 部分完成 (⚠️)

#### Ollama 集成模块
- ✅ API 请求/响应结构定义
- ✅ System Prompt 模板 (英语私教角色设定)
- ✅ 单词教学 Prompt 模板
- ✅ 流式响应解析框架
- ⚠️ **待实现**: 与 Tauri Event 系统的完整集成
- ⚠️ **待实现**: 对话历史管理

#### 数据库操作
- ✅ Schema 迁移脚本
- ✅ CRUD 函数实现 (sessions, messages, vocabulary, settings)
- ⚠️ **待实现**: Commands 中调用数据库操作的实际代码

---

### 1.3 未完成 (❌)

#### 语音功能
- ❌ STT (语音转文字) - macOS/iOS 原生 API 集成
- ❌ TTS (文字转语音) - Edge TTS / 系统 TTS 集成
- ❌ 音频流播放控制

#### 应用图标
- ❌ 正式应用图标设计
- ⚠️ 当前使用占位 PNG (32x32 黄色方块)

#### 前后端联调
- ❌ 完整的功能测试
- ❌ 性能优化
- ❌ 错误处理完善

---

## 2. 下一步工作计划

### Phase 1: 核心对话功能 (优先级 P0)

**目标**: 实现基本的文字对话功能

**任务清单**:

1. **更新 conversation commands** (预计 2小时)
   ```rust
   // src-tauri/src/commands/conversation.rs

   #[tauri::command]
   pub async fn start_conversation(
       params: StartConversationParams,
       state: State<'_, AppState>,
       app_handle: AppHandle,
   ) -> Result<StartConversationResult, String> {
       // 1. 获取数据库连接
       let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
       let conn = db_guard.as_ref().ok_or("Database not initialized")?;

       // 2. 创建会话
       let session_id = database::create_session(
           conn,
           &params.scenario,
           &params.proficiency_level
       ).map_err(|e| e.to_string())?;

       // 3. 生成 AI 开场白
       let greeting = ollama::generate_greeting(
           &app_handle,
           session_id,
           &params.scenario,
           &params.proficiency_level
       ).await.map_err(|e| e.to_string())?;

       Ok(StartConversationResult {
           session_id,
           greeting,
           success: true,
           error: None,
       })
   }
   ```

2. **实现 send_message command** (预计 3小时)
   - 保存用户消息到数据库
   - 调用 Ollama 生成 AI 回复
   - 通过 Event 流式推送回复到前端
   - 保存 AI 回复到数据库

3. **实现 teach_word command** (预计 2小时)
   - 调用专用 Prompt 进行单词教学
   - 自动添加单词到生词本

**验收标准**:
- [ ] 用户可以选择情景并开始对话
- [ ] 用户可以发送文字消息
- [ ] AI 能够流式返回回复
- [ ] 对话历史保存到数据库

---

### Phase 2: 数据管理功能 (优先级 P0)

**任务清单**:

1. **实现 get_sessions command** (预计 1小时)
2. **实现 get_messages command** (预计 1小时)
3. **实现 get_vocabulary command** (预计 1小时)
4. **实现 delete_session command** (预计 0.5小时)

**验收标准**:
- [ ] 前端可以获取会话列表
- [ ] 前端可以加载历史消息
- [ ] 前端可以查看生词本

---

### Phase 3: 语音功能 (优先级 P1)

**技术方案选择**:

| 功能 | macOS | Windows | Linux |
| :--- | :--- | :--- | :--- |
| **STT** | SFSpeechRecognizer | Windows Speech API | Google STT (在线) |
| **TTS** | AVSpeechSynthesizer | Windows SAPI | espeak/festival |

**任务清单**:

1. **实现 STT** (预计 4小时)
   ```rust
   // macOS 示例
   #[cfg(target_os = "macos")]
   pub fn start_recording() -> Result<(), String> {
       // 使用 CoreAudio + SFSpeechRecognizer
   }
   ```

2. **实现 TTS** (预计 4小时)
   - Edge TTS (在线,高质量)
   - 系统 TTS (离线,降级方案)

3. **音频播放控制** (预计 2小时)

**验收标准**:
- [ ] 用户可以按住按钮说话
- [ ] 语音自动转为文字发送
- [ ] AI 回复可以语音播放
- [ ] 支持音量/语速调节

---

### Phase 4: 应用图标与打包 (优先级 P2)

**任务清单**:

1. **设计应用图标** (预计 2小时)
   - 主图标: 1024x1024 PNG
   - 多尺寸导出: 32x32, 128x128, 256x256
   - macOS: .icns 格式
   - Windows: .ico 格式

2. **更新 tauri.conf.json** (预计 0.5小时)

3. **测试打包** (预计 1小时)
   ```bash
   npm run tauri:build
   ```

**验收标准**:
- [ ] 应用有正式图标
- [ ] 可以生成 DMG/MSI 安装包
- [ ] 安装包大小 < 20MB

---

### Phase 5: 测试与优化 (优先级 P1)

**任务清单**:

1. **端到端测试** (预计 4小时)
   - 情景选择 → 开始对话 → 发送消息 → 查看回复
   - 单词双击 → 教学流程 → 生词本添加
   - 会话历史加载

2. **性能优化** (预计 2小时)
   - Ollama 响应延迟优化
   - 数据库查询优化
   - 前端渲染优化

3. **错误处理** (预计 2小时)
   - Ollama 服务未启动提示
   - 网络断开处理
   - 数据库损坏恢复

---

## 3. 技术债务与注意事项

### 3.1 已知问题

1. **数据库连接管理**: 当前使用 `Mutex<Option<Connection>>`,在高并发场景可能成为瓶颈
   - **解决方案**: 考虑使用 r2d2 连接池

2. **Ollama 超时处理**: 未设置请求超时,可能导致长时间等待
   - **解决方案**: 添加 30秒超时,显示加载状态

3. **内存占用**: Ollama 模型加载后占用 ~2GB 内存
   - **解决方案**: 提供"释放内存"按钮,调用 `ollama stop`

### 3.2 安全考虑

1. **SQL 注入**: 已使用参数化查询,安全性良好
2. **XSS 防护**: 前端需对用户输入进行转义
3. **文件权限**: 数据库文件应设置为仅当前用户可读

### 3.3 可扩展性

1. **多语言支持**: 当前仅支持英语教学,未来可扩展其他语言
2. **云端同步**: 当前数据本地存储,未来可添加云同步功能
3. **插件系统**: 情景包可以设计为可下载插件

---

## 4. 快速开始指南

### 4.1 开发环境启动

```bash
# 1. 确保 Ollama 运行
ollama serve &

# 2. 确认模型已下载
ollama list
# 应显示: qwen2.5:3b

# 3. 启动开发模式
npm run tauri:dev
```

### 4.2 测试核心功能

```bash
# 前端单独测试
cd frontend && npm run dev
# 访问 http://localhost:5173

# Rust 单元测试
cargo test --manifest-path src-tauri/Cargo.toml
```

### 4.3 构建生产版本

```bash
# 构建安装包
npm run tauri:build

# macOS 产物
ls src-tauri/target/release/bundle/dmg/
# LingoMate_0.1.0_x64.dmg
```

---

## 5. 资源链接

- [Tauri 官方文档](https://v2.tauri.app/)
- [Ollama API 文档](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [Rusqlite 文档](https://docs.rs/rusqlite/)
- [React 文档](https://react.dev/)

---

## 6. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始状态报告,记录框架搭建完成情况 | LingoMate Team |

---

**文档结束**
