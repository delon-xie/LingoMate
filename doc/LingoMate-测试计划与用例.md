# LingoMate 测试计划与用例

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | QA 团队 |

---

## 1. 测试策略概述

### 1.1 测试金字塔

```
        /\
       /  \      E2E Tests (Playwright) - 10%
      /____\
     /      \    Integration Tests - 20%
    /________\
   /          \  Unit Tests - 70%
  /____________\
```

### 1.2 测试类型分布

| 测试类型 | 占比 | 工具 | 目标 |
| :--- | :--- | :--- | :--- |
| **单元测试** | 70% | Rust `cargo test`, Jest | 验证单个函数/组件逻辑 |
| **集成测试** | 20% | Rust, Playwright | 验证模块间交互 |
| **端到端测试** | 10% | Playwright | 验证完整用户流程 |

### 1.3 测试环境

| 环境 | 用途 | 配置 |
| :--- | :--- | :--- |
| **Local Dev** | 开发时快速测试 | 开发者本地机器 |
| **CI/CD** | 自动化回归测试 | GitHub Actions (Ubuntu, macOS, Windows) |
| **Staging** | 发布前最终验证 | 真实硬件 (8GB RAM 笔记本) |

---

## 2. 单元测试

### 2.1 Rust 后端单元测试

#### 测试文件结构

```
src-tauri/src/
├── lib.rs
├── commands/
│   ├── conversation.rs
│   ├── vocabulary.rs
│   └── settings.rs
├── services/
│   ├── ollama_service.rs
│   ├── tts_service.rs
│   └── stt_service.rs
└── tests/
    ├── conversation_tests.rs
    ├── vocabulary_tests.rs
    └── tts_tests.rs
```

#### 测试用例示例

**测试 1: Prompt 构造逻辑**

```rust
// src/services/prompt_builder.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_coffee_shop_prompt() {
        let prompt = build_scenario_prompt("coffee_shop", "intermediate");
        
        assert!(prompt.contains("Coffee Shop Ordering"));
        assert!(prompt.contains("barista"));
        assert!(prompt.contains("latte"));
    }

    #[test]
    fn test_build_word_teaching_prompt() {
        let prompt = build_word_teaching_prompt("procrastinate");
        
        assert!(prompt.contains("procrastinate"));
        assert!(prompt.contains("SIMPLE EXPLANATION"));
        assert!(prompt.contains("CONTEXTUAL EXAMPLE"));
        assert!(prompt.contains("ENGAGING QUESTION"));
    }

    #[test]
    fn test_prompt_includes_base_rules() {
        let prompt = build_scenario_prompt("restaurant", "beginner");
        
        assert!(prompt.contains("ONLY use English"));
        assert!(prompt.contains("maximum 3 sentences"));
        assert!(prompt.contains("A1/A2 level"));
    }
}
```

**测试 2: 生词本遗忘曲线算法**

```rust
// src/services/vocabulary_service.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_next_review_date_level_0() {
        let next_date = calculate_next_review(0);
        let expected = chrono::Utc::now().date_naive();
        
        assert_eq!(next_date, expected);
    }

    #[test]
    fn test_calculate_next_review_date_level_2() {
        let next_date = calculate_next_review(2);
        let expected = chrono::Utc::now().date_naive() + chrono::Days::new(3);
        
        assert_eq!(next_date, expected);
    }

    #[test]
    fn test_calculate_next_review_date_level_5() {
        let next_date = calculate_next_review(5);
        let expected = chrono::Utc::now().date_naive() + chrono::Days::new(30);
        
        assert_eq!(next_date, expected);
    }

    #[test]
    fn test_update_mastery_level_increase() {
        let mut word = VocabularyItem {
            mastery_level: 2,
            review_count: 3,
            ..Default::default()
        };
        
        update_mastery(&mut word, true); // 正确回答
        
        assert_eq!(word.mastery_level, 3);
        assert_eq!(word.review_count, 4);
    }

    #[test]
    fn test_update_mastery_level_decrease() {
        let mut word = VocabularyItem {
            mastery_level: 3,
            review_count: 5,
            ..Default::default()
        };
        
        update_mastery(&mut word, false); // 错误回答
        
        assert_eq!(word.mastery_level, 2); // 需要连续2次错误才降级
    }
}
```

**测试 3: TTS 引擎选择逻辑**

```rust
// src/services/tts_service.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_tts_engine_auto_online() {
        let config = TtsConfig {
            mode: TtsMode::Auto,
            network_status: NetworkStatus::Online,
        };
        
        let engine = select_tts_engine(&config);
        
        assert_eq!(engine, TtsEngine::Edge);
    }

    #[test]
    fn test_select_tts_engine_auto_offline() {
        let config = TtsConfig {
            mode: TtsMode::Auto,
            network_status: NetworkStatus::Offline,
        };
        
        let engine = select_tts_engine(&config);
        
        assert_eq!(engine, TtsEngine::System);
    }

    #[test]
    fn test_select_tts_engine_edge_only() {
        let config = TtsConfig {
            mode: TtsMode::EdgeOnly,
            network_status: NetworkStatus::Offline,
        };
        
        let engine = select_tts_engine(&config);
        
        assert_eq!(engine, TtsEngine::Edge); // 即使用户强制,也要检查网络
    }
}
```

---

### 2.2 前端 React 单元测试

#### 测试文件结构

```
src/
├── components/
│   ├── ChatBubble.test.tsx
│   ├── RecordButton.test.tsx
│   └── ScenarioCard.test.tsx
├── hooks/
│   ├── useConversation.test.ts
│   └── useVocabulary.test.ts
└── utils/
    ├── formatters.test.ts
    └── validators.test.ts
```

#### 测试用例示例

**测试 1: ChatBubble 组件渲染**

```tsx
// src/components/ChatBubble.test.tsx
import { render, screen } from '@testing-library/react';
import { ChatBubble } from './ChatBubble';

describe('ChatBubble', () => {
  test('renders AI message correctly', () => {
    render(
      <ChatBubble 
        role="assistant" 
        content="Hello! How can I help you?" 
      />
    );
    
    expect(screen.getByText('Hello! How can I help you?')).toBeInTheDocument();
    expect(screen.getByRole('img')).toHaveAttribute('aria-label', 'AI avatar');
  });

  test('renders user message with different style', () => {
    render(
      <ChatBubble 
        role="user" 
        content="I'd like a latte." 
      />
    );
    
    const bubble = screen.getByText("I'd like a latte.").closest('.message-user');
    expect(bubble).toHaveClass('message-user');
  });

  test('makes words clickable for teaching', () => {
    render(
      <ChatBubble 
        role="assistant" 
        content="That's fascinating!" 
        enableWordClick={true}
      />
    );
    
    const word = screen.getByText('fascinating');
    expect(word).toHaveClass('clickable-word');
  });
});
```

**测试 2: RecordButton 状态管理**

```tsx
// src/components/RecordButton.test.tsx
import { render, fireEvent } from '@testing-library/react';
import { RecordButton } from './RecordButton';

describe('RecordButton', () => {
  test('starts recording on mouse down', async () => {
    const onStart = jest.fn();
    const { container } = render(<RecordButton onStart={onStart} />);
    
    const button = container.querySelector('.record-button');
    fireEvent.mouseDown(button);
    
    expect(onStart).toHaveBeenCalled();
    expect(button).toHaveClass('recording');
  });

  test('stops recording on mouse up', async () => {
    const onStop = jest.fn();
    const { container } = render(<RecordButton onStop={onStop} />);
    
    const button = container.querySelector('.record-button');
    fireEvent.mouseDown(button);
    fireEvent.mouseUp(button);
    
    expect(onStop).toHaveBeenCalled();
  });

  test('shows processing state', () => {
    const { container } = render(
      <RecordButton status="processing" />
    );
    
    const button = container.querySelector('.record-button');
    expect(button).toHaveClass('processing');
    expect(screen.getByText('Processing...')).toBeInTheDocument();
  });
});
```

---

## 3. 集成测试

### 3.1 后端集成测试

#### 测试 1: 完整对话流程

```rust
// src-tauri/tests/integration/conversation_flow.rs
use tokio;

#[tokio::test]
async fn test_full_conversation_flow() {
    // 1. 启动 Ollama 服务 (mock)
    let mock_server = MockOllamaServer::start().await;
    
    // 2. 创建新会话
    let session_id = create_session("coffee_shop", "intermediate").await;
    
    // 3. 发送用户消息
    send_message(session_id, "I'd like a latte.").await;
    
    // 4. 等待 AI 响应 (流式)
    let mut response_chunks = Vec::new();
    listen_for_events("ai_response_chunk", |chunk| {
        response_chunks.push(chunk);
    }).await;
    
    // 5. 验证响应
    assert!(!response_chunks.is_empty());
    let full_response = response_chunks.join("");
    assert!(full_response.contains("latte") || full_response.contains("coffee"));
    
    // 6. 验证消息已保存到数据库
    let messages = get_messages(session_id).await;
    assert_eq!(messages.len(), 2); // user + assistant
    assert_eq!(messages[0].role, "user");
    assert_eq!(messages[1].role, "assistant");
    
    mock_server.stop().await;
}
```

#### 测试 2: TTS 降级逻辑

```rust
#[tokio::test]
async fn test_tts_fallback_when_edge_unavailable() {
    // Mock Edge TTS 服务不可用
    let mock_edge = MockEdgeTtsService::unavailable().await;
    
    let config = TtsConfig {
        mode: TtsMode::Auto,
        network_status: NetworkStatus::Online,
    };
    
    // 尝试合成语音
    let result = synthesize_speech("Hello", &config).await;
    
    // 应该自动降级到系统 TTS
    assert!(result.is_ok());
    assert_eq!(result.unwrap().engine, TtsEngine::System);
    
    mock_edge.stop().await;
}
```

---

### 3.2 前端集成测试

#### 测试 1: 情景选择到对话跳转

```tsx
// tests/integration/scenario_selection.test.tsx
import { test, expect } from '@playwright/test';

test('select scenario and start conversation', async ({ page }) => {
  // 1. 访问首页
  await page.goto('/');
  
  // 2. 点击咖啡店情景
  await page.click('[data-testid="scenario-coffee-shop"]');
  
  // 3. 验证跳转到聊天页面
  await expect(page).toHaveURL(/\/chat\/\d+/);
  
  // 4. 验证 AI 发送开场白
  await expect(page.locator('.message-ai').first()).toContainText('What can I get for you');
});
```

---

## 4. 端到端测试 (E2E)

### 4.1 测试工具: Playwright

**安装**:
```bash
npm install -D @playwright/test
npx playwright install
```

**配置文件** `playwright.config.ts`:
```typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30000,
  retries: process.env.CI ? 2 : 0,
  use: {
    baseURL: 'http://localhost:1420',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { browserName: 'chromium' },
    },
    {
      name: 'webkit',
      use: { browserName: 'webkit' }, // macOS Safari
    },
  ],
});
```

---

### 4.2 核心用户流程测试

#### 测试 1: 首次启动与模型下载

```typescript
// tests/e2e/first_launch.spec.ts
import { test, expect } from '@playwright/test';

test('first launch downloads model and starts chat', async ({ page }) => {
  // 1. 首次启动显示欢迎向导
  await page.goto('/');
  await expect(page.getByText('Welcome to LingoMate')).toBeVisible();
  
  // 2. 设置用户信息
  await page.fill('input[name="nickname"]', 'Alex');
  await page.selectOption('select[name="level"]', 'intermediate');
  await page.click('button:has-text("Get Started")');
  
  // 3. 显示模型下载进度
  await expect(page.getByText('Downloading AI Model')).toBeVisible();
  
  // 4. 等待下载完成 (mock 加速)
  await page.waitForSelector('.progress-bar', { state: 'hidden', timeout: 60000 });
  
  // 5. 自动进入情景选择页
  await expect(page.getByText('Choose a Scenario')).toBeVisible();
});
```

#### 测试 2: 完整语音对话流程

```typescript
// tests/e2e/voice_conversation.spec.ts
import { test, expect } from '@playwright/test';

test('complete voice conversation flow', async ({ page }) => {
  // 1. 选择情景
  await page.goto('/');
  await page.click('[data-testid="scenario-coffee-shop"]');
  
  // 2. 模拟录音 (mock STT)
  await page.evaluate(() => {
    window.__MOCK_STT_RESULT__ = "I'd like a latte, please.";
  });
  
  await page.click('.record-button');
  await page.waitForTimeout(1000); // 模拟录音 1 秒
  await page.dispatchEvent('.record-button', 'mouseup');
  
  // 3. 验证识别文本显示
  await expect(page.locator('.message-user')).toContainText("I'd like a latte");
  
  // 4. 等待 AI 流式回复
  await page.waitForSelector('.message-ai:last-child');
  
  // 5. 验证 AI 回复包含相关内容
  const aiMessage = await page.locator('.message-ai:last-child').textContent();
  expect(aiMessage).toMatch(/latte|coffee|hot|iced/i);
  
  // 6. 验证音频播放按钮出现
  await expect(page.locator('.audio-play-btn').last()).toBeVisible();
});
```

#### 测试 3: 单词即点即学

```typescript
// tests/e2e/word_teaching.spec.ts
import { test, expect } from '@playwright/test';

test('double-click word triggers teaching', async ({ page }) => {
  // 1. 进入对话并等待 AI 回复
  await page.goto('/chat/1');
  await page.waitForSelector('.message-ai');
  
  // 2. 双击单词 "fascinating"
  const word = page.locator('.clickable-word', { hasText: 'fascinating' });
  await word.dblclick();
  
  // 3. 验证单词高亮
  await expect(word).toHaveClass(/selected/);
  
  // 4. 等待 AI 教学回复
  await page.waitForSelector('.message-ai:last-child');
  
  // 5. 验证教学内容包含解释、例句、提问
  const teaching = await page.locator('.message-ai:last-child').textContent();
  expect(teaching).toMatch(/means|example|question/i);
  
  // 6. 验证单词加入生词本
  await page.goto('/vocabulary');
  await expect(page.getByText('fascinating')).toBeVisible();
});
```

#### 测试 4: 生词本智能复习

```typescript
// tests/e2e/vocabulary_review.spec.ts
import { test, expect } from '@playwright/test';

test('AI naturally reviews vocabulary in new conversation', async ({ page }) => {
  // 1. 先在生词本添加单词
  await page.goto('/vocabulary');
  await page.fill('input[name="new-word"]', 'procrastinate');
  await page.click('button:has-text("Add")');
  
  // 2. 开始新对话
  await page.goto('/');
  await page.click('[data-testid="scenario-social"]');
  
  // 3. 进行几轮对话
  await page.fill('.input-text', "Hi! How are you?");
  await page.click('.send-button');
  
  await page.waitForSelector('.message-ai:last-child');
  
  // 4. 再次发送消息触发复习
  await page.fill('.input-text', "I'm good, thanks!");
  await page.click('.send-button');
  
  // 5. 验证 AI 在回复中自然使用了 "procrastinate"
  await page.waitForSelector('.message-ai:last-child');
  const response = await page.locator('.message-ai:last-child').textContent();
  expect(response).toMatch(/procrastinate/i);
});
```

---

### 4.3 网络切换测试

#### 测试 5: Edge TTS 离线降级

```typescript
// tests/e2e/network_fallback.spec.ts
import { test, expect } from '@playwright/test';

test('TTS falls back to system engine when offline', async ({ page, context }) => {
  // 1. 模拟在线状态
  await context.route('**/edge-tts-api/**', route => route.abort('failed'));
  
  // 2. 进入对话
  await page.goto('/chat/1');
  
  // 3. 触发 AI 回复 (会调用 TTS)
  await page.fill('.input-text', "Hello");
  await page.click('.send-button');
  
  // 4. 验证降级提示
  await expect(page.getByText('Using offline voice')).toBeVisible({ timeout: 5000 });
  
  // 5. 验证音频仍然播放 (系统 TTS)
  await expect(page.locator('.audio-playing')).toBeVisible();
});
```

---

## 5. 性能测试

### 5.1 关键性能指标

| 指标 | 目标值 | 测试方法 |
| :--- | :--- | :--- |
| **冷启动时间** | ≤ 5 秒 | 从点击图标到主界面加载完成 |
| **STT 响应延迟** | ≤ 1.5 秒 | 松开录音按钮到显示文本 |
| **AI 首字延迟** | ≤ 2 秒 | 发送消息到看到第一个字符 |
| **TTS 合成延迟** | ≤ 1 秒 | 收到文本到开始播放音频 |
| **总响应时间** | ≤ 2.5 秒 | 松开录音到听到 AI 语音 |
| **内存占用** | ≤ 6 GB | 持续对话 30 分钟后的峰值 |
| **崩溃率** | < 0.1% | 1000 次会话的崩溃次数 |

---

### 5.2 性能测试脚本

#### 测试 1: 启动时间测量

```typescript
// tests/performance/startup_time.spec.ts
import { test, expect } from '@playwright/test';

test('cold startup time under 5 seconds', async ({ page }) => {
  const startTime = Date.now();
  
  await page.goto('/');
  await page.waitForSelector('.scenario-grid');
  
  const endTime = Date.now();
  const startupTime = (endTime - startTime) / 1000;
  
  console.log(`Startup time: ${startupTime}s`);
  expect(startupTime).toBeLessThan(5);
});
```

#### 测试 2: 内存泄漏检测

```typescript
// tests/performance/memory_leak.spec.ts
import { test, expect } from '@playwright/test';

test('no memory leak after 30 minutes of conversation', async ({ page }) => {
  await page.goto('/chat/1');
  
  // 记录初始内存
  const initialMemory = await getProcessMemory(page);
  
  // 模拟 50 轮对话
  for (let i = 0; i < 50; i++) {
    await page.fill('.input-text', `Message ${i}`);
    await page.click('.send-button');
    await page.waitForSelector('.message-ai:last-child');
  }
  
  // 记录最终内存
  const finalMemory = await getProcessMemory(page);
  const memoryIncrease = finalMemory - initialMemory;
  
  console.log(`Memory increase: ${memoryIncrease} MB`);
  expect(memoryIncrease).toBeLessThan(500); // 不超过 500MB
});

async function getProcessMemory(page: any): Promise<number> {
  const metrics = await page.evaluate(() => performance.memory);
  return metrics.usedJSHeapSize / 1024 / 1024; // 转换为 MB
}
```

---

## 6. 兼容性测试

### 6.1 操作系统矩阵

| 操作系统 | 版本 | 测试优先级 |
| :--- | :--- | :--- |
| **Windows** | 10, 11 | P0 (必须通过) |
| **macOS** | 12 Monterey, 13 Ventura, 14 Sonoma | P0 |
| **Linux** | Ubuntu 20.04, 22.04 | P1 |

### 6.2 硬件配置测试

| 配置 | CPU | 内存 | GPU | 预期表现 |
| :--- | :--- | :--- | :--- | :--- |
| **最低配置** | Intel i5-8代 | 8 GB | 集成显卡 | 流畅模式可用 |
| **推荐配置** | Intel i7-10代 | 16 GB | GTX 1650 | 性能模式可用 |
| **高端配置** | Apple M2 | 16 GB | M2 GPU | 所有功能流畅 |

---

## 7. 安全测试

### 7.1 SQL 注入测试

```typescript
// tests/security/sql_injection.spec.ts
import { test, expect } from '@playwright/test';

test('prevents SQL injection in search', async ({ page }) => {
  await page.goto('/vocabulary');
  
  // 尝试 SQL 注入
  await page.fill('.search-input', "' OR '1'='1");
  await page.press('.search-input', 'Enter');
  
  // 应该返回空结果或错误,而不是所有数据
  const results = await page.locator('.vocabulary-card').count();
  expect(results).toBeLessThan(100); // 不应该返回所有单词
});
```

### 7.2 权限测试

```typescript
// tests/security/microphone_permission.spec.ts
import { test, expect } from '@playwright/test';

test('handles microphone permission denial gracefully', async ({ page, context }) => {
  // 拒绝麦克风权限
  await context.grantPermissions([]); // 不授予任何权限
  
  await page.goto('/chat/1');
  await page.click('.record-button');
  
  // 应显示友好提示
  await expect(page.getByText('Microphone access denied')).toBeVisible();
  await expect(page.getByText('Please enable microphone')).toBeVisible();
});
```

---

## 8. 验收标准

### 8.1 MVP 功能验收清单

#### F1. 主体对话功能

- [ ] F1.1 语音输入能准确转文字 (准确率 > 90%)
- [ ] F1.2 文字输入正常发送
- [ ] F1.3 AI 回复流式显示无卡顿
- [ ] F1.4 TTS 语音清晰可听
- [ ] F1.5 对话历史正确保存和回看

#### F2. 教学功能

- [ ] F2.1 6 个情景对话正常工作
- [ ] F2.2 双击单词触发教学
- [ ] F2.3 生词本自动记录查询过的单词
- [ ] F2.4 AI 在新对话中自然复习生词

#### F3. 系统功能

- [ ] F3.1 用户档案正确保存
- [ ] F3.2 可切换 TTS 音色
- [ ] F3.3 流畅模式和性能模式切换有效
- [ ] F3.4 可查看和切换 AI 模型
- [ ] F3.5 数据导出功能正常

---

### 8.2 非功能性需求验收

#### NFR1. 性能

- [ ] 冷启动时间 ≤ 5 秒 (10 次测试平均值)
- [ ] 语音响应延迟 ≤ 2.5 秒 (100 次测试 P95)
- [ ] 8GB 设备持续对话 30 分钟无卡顿

#### NFR2. 可靠性

- [ ] 崩溃率 < 0.1% (1000 次会话)
- [ ] 数据零丢失 (断电恢复测试)
- [ ] 弱网环境下优雅降级

#### NFR3. 易用性

- [ ] 99% 用户无需技术配置即可使用
- [ ] 主要功能 3 次点击内可达
- [ ] 新用户 10 分钟内掌握所有核心操作

#### NFR4. 兼容性

- [ ] Windows 10/11 正常运行
- [ ] macOS 12+ 正常运行
- [ ] Ubuntu 20.04+ 正常运行
- [ ] 8GB 内存设备流畅运行

---

## 9. 测试执行计划

### 9.1 测试阶段

| 阶段 | 时间 | 负责人 | 内容 |
| :--- | :--- | :--- | :--- |
| **Sprint 测试** | 每个 Sprint 结束 | 开发团队 | 单元测试 + 集成测试 |
| **Alpha 测试** | Sprint 6 结束 | QA 团队 | 功能测试 + 性能测试 |
| **Beta 测试** | Sprint 7 结束 | 外部用户 (50人) | 真实场景测试 |
| **RC 测试** | Sprint 8 结束 | QA + 产品 | 回归测试 + 验收测试 |

### 9.2 每日构建测试

**GitHub Actions 配置** `.github/workflows/test.yml`:

```yaml
name: Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run unit tests
        run: |
          npm run test:unit
          cargo test
      
      - name: Run E2E tests
        run: npx playwright test
      
      - name: Upload test results
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test-results/
```

---

## 10. 缺陷管理

### 10.1 缺陷等级定义

| 等级 | 名称 | 描述 | 修复时限 |
| :--- | :--- | :--- | :--- |
| **P0** | Critical | 应用崩溃、数据丢失、核心功能完全失效 | 24 小时内 |
| **P1** | High | 主要功能异常,严重影响用户体验 | 3 天内 |
| **P2** | Medium | 次要功能异常,有替代方案 | 下个 Sprint |
| **P3** | Low | UI 小问题、文案错误 | 有空闲时 |

### 10.2 缺陷报告模板

```markdown
## 缺陷标题
[简短描述问题]

## 环境信息
- 操作系统: Windows 11 / macOS 13 / Ubuntu 22.04
- 应用版本: v0.9.0
- 硬件配置: 8GB RAM, Intel i5

## 复现步骤
1. 打开应用
2. 点击...
3. 输入...
4. 观察到...

## 预期行为
[描述应该发生什么]

## 实际行为
[描述实际发生了什么]

## 截图/日志
[附加截图或日志片段]

## 严重程度
P0 / P1 / P2 / P3

## 备注
[其他相关信息]
```

---

## 11. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义完整测试计划 | LingoMate Team |

---

**文档结束**
