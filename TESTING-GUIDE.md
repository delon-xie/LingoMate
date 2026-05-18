# LingoMate 可用性测试指南

## 快速启动

```bash
# 1. 确保 Ollama 正在运行
ollama serve &

# 2. 启动应用
npm run tauri:dev
```

## 已修复的问题

### 问题 1: 空白页面
**原因**: `isTauri()` 环境检测过于严格，导致在 Tauri 环境中也显示警告页面而非正常界面。

**修复**:
- 移除了阻塞式的环境检测警告页面
- 改为在控制台输出友好提示
- 改进 `isTauri()` 检测逻辑，支持多种 Tauri v2 检测方式

### 问题 2: sessionId 硬编码
**原因**: `App.tsx` 中 sessionId 被硬编码为 `1`，忽略了 API 返回的真实 session_id。

**修复**:
- `ScenarioSelection` 现在正确传递 API 返回的 `session_id`
- `App.tsx` 使用真实的 session_id 进行状态管理

### 问题 3: ScenarioCard 参数冗余
**原因**: 调用时传递了空的 `icon`, `title`, `description` 参数。

**修复**: 移除未使用的 props，组件内部使用配置对象。

## 手工测试步骤

### 测试 1: 应用启动
```bash
npm run tauri:dev
```

**预期结果**:
- [ ] 应用窗口打开（1024x768）
- [ ] 显示情景选择页面
- [ ] 顶部显示 "LingoMate" 标题
- [ ] 显示 6 个情景卡片
- [ ] 控制台无错误

### 测试 2: 情景卡片点击
**操作**: 点击任意情景卡片（如 "Coffee Shop"）

**预期结果**:
- [ ] 控制台显示: `Starting conversation for scenario: coffee_shop`
- [ ] 控制台显示: `Conversation started: {session_id: X, greeting: "...", success: true}`
- [ ] 页面切换到聊天界面
- [ ] 顶部显示情景名称（如 "Coffee Shop"）
- [ ] 显示 AI 的问候消息（左侧气泡）

**如果失败**:
- 检查 Ollama 是否运行: `ollama list`
- 检查控制台错误信息

### 测试 3: 发送文本消息
**操作**:
1. 在底部输入框中输入英文（如 "Hello!"）
2. 点击发送按钮或按 Enter

**预期结果**:
- [ ] 用户消息显示在右侧（黄色背景）
- [ ] 输入框清空
- [ ] AI 开始回复（左侧灰色背景）
- [ ] AI 回复逐字显示（流式效果）
- [ ] 自动滚动到底部

**如果失败**:
- 检查 Ollama 模型: `ollama pull qwen2.5:3b`
- 查看网络请求是否成功

### 测试 4: 单词点击学习
**操作**: 双击 AI 回复中的任意英文单词

**预期结果**:
- [ ] 单词下方有虚线下划线
- [ ] 鼠标悬停时高亮
- [ ] 点击后控制台显示 teach_word 调用
- [ ] 单词被添加到生词本

### 测试 5: 返回情景选择
**操作**: 点击左上角的返回箭头

**预期结果**:
- [ ] 返回情景选择页面
- [ ] 会话状态重置

### 测试 6: 语音输入（需要 Tauri 环境）
**操作**:
1. 按住 "Hold to Speak" 按钮
2. 说话
3. 松开按钮

**预期结果**:
- [ ] 按下时按钮变为红色
- [ ] 显示 "Recording..."
- [ ] 松开后显示识别的文字
- [ ] 文字填入输入框

### 测试 7: 生词本页面
**操作**: （需要从代码中添加导航入口）

**预期结果**:
- [ ] 显示已学习的单词列表
- [ ] 显示掌握程度
- [ ] 可以排序

## 常见问题排查

### 问题: 点击情景卡片无响应
**可能原因**:
1. Ollama 未运行
2. 模型未加载

**解决**:
```bash
ollama serve &
ollama pull qwen2.5:3b
```

### 问题: AI 不回复
**可能原因**:
1. Ollama API 连接失败
2. 模型加载缓慢

**解决**:
```bash
# 检查 Ollama 状态
curl http://localhost:11434/api/tags

# 测试模型
ollama run qwen2.5:3b "Hello"
```

### 问题: 空白页面
**可能原因**:
1. 前端构建失败
2. React 渲染错误

**解决**:
```bash
# 重新构建前端
cd frontend && npm run build

# 检查控制台错误
# 打开开发者工具查看错误
```

### 问题: 语音功能不可用
**说明**: 语音功能需要在 Tauri 桌面环境中运行，浏览器中不可用。

**解决**: 使用 `npm run tauri:dev` 启动应用

## 开发者调试

### 查看日志
```bash
# Rust 后端日志
RUST_LOG=debug npm run tauri:dev

# 前端控制台
# 打开开发者工具 -> Console
```

### 数据库位置
```
~/Library/Application Support/com.lingomate.app/lingomate.db
```

### 查看数据库
```bash
sqlite3 ~/Library/Application\ Support/com.lingomate.app/lingomate.db ".tables"
```

## 测试清单总结

| 功能 | 状态 | 备注 |
|------|------|------|
| 应用启动 | ✓ | 窗口正常打开 |
| 情景选择页面 | ✓ | 6个卡片显示正常 |
| 情景卡片点击 | ✓ | 触发 API 调用 |
| 会话创建 | ✓ | 返回真实 session_id |
| 聊天页面 | ✓ | 正确渲染 |
| 发送消息 | ✓ | 用户消息显示 |
| AI 流式回复 | ✓ | 逐字显示 |
| 单词点击 | ✓ | 可点击学习 |
| 返回导航 | ✓ | 正常切换页面 |
| 语音输入 | - | 需要 macOS 权限 |
| TTS 播放 | - | 需要 Tauri 环境 |

## 下一步改进

1. 添加单元测试
2. 添加 E2E 测试（Playwright）
3. 完善错误处理和用户反馈
4. 添加加载状态指示器
5. 优化首屏加载速度
