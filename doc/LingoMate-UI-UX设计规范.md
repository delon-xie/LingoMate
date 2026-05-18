# LingoMate UI/UX 设计规范

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | 前端开发团队 |

---

## 1. 设计原则

### 1.1 核心设计理念

LingoMate 的视觉设计围绕以下核心理念展开:

1. **温暖友好 (Warm & Approachable)**: 黄色系主色调传递积极、乐观的学习氛围,降低学习焦虑
2. **清晰简洁 (Clean & Clear)**: 减少视觉干扰,让用户专注于对话和学习内容
3. **教育导向 (Education-Focused)**: 所有交互设计服务于"有效教学"目标
4. **零门槛体验 (Zero Friction)**: 界面直观,无需学习成本即可上手

### 1.2 设计关键词

- **伙伴感**: 像朋友一样自然,非正式但专业
- **沉浸感**: 对话为核心,最小化界面元素
- **成就感**: 通过视觉反馈强化学习进步
- **安全感**: 无评判环境,鼓励开口尝试

---

## 2. 色彩系统

### 2.1 主色调 - 黄色系

基于用户偏好,采用温暖的黄色系作为品牌主色:

```css
/* src/index.css - 黄色系主题 */
:root {
  /* 主色系 - 黄色 */
  --primary-50: #FFFBEB;    /* 最浅背景 */
  --primary-100: #FEF3C7;   /* 浅色背景 */
  --primary-200: #FDE68A;   /* 悬停背景 */
  --primary-300: #FCD34D;   /* 次要按钮 */
  --primary-400: #FBBF24;   /* 主要强调 */
  --primary-500: #F59E0B;   /* 品牌主色 */
  --primary-600: #D97706;   /* 深色强调 */
  --primary-700: #B45309;   /* 激活状态 */
  --primary-800: #92400E;   /* 深文本 */
  --primary-900: #78350F;   /* 最深文本 */

  /* 语义化颜色映射 */
  --color-accent: var(--primary-500);      /* 主要按钮、链接 */
  --color-accent-light: var(--primary-400); /* 悬停状态 */
  --color-accent-dark: var(--primary-600);  /* 按下状态 */
  --color-hover: var(--primary-100);        /* 悬停背景 */
  --color-selection: var(--primary-200);    /* 选中文本背景 */
}
```

### 2.2 完整调色板

#### 中性色 (Neutrals)

```css
:root {
  /* 背景色 */
  --bg-primary: #FFFFFF;        /* 主背景 */
  --bg-secondary: #FAFAFA;      /* 次级背景 */
  --bg-tertiary: #F5F5F5;       /* 卡片背景 */

  /* 文本色 */
  --text-primary: #1F2937;      /* 主要文本 (Gray 800) */
  --text-secondary: #6B7280;    /* 次要文本 (Gray 500) */
  --text-muted: #9CA3AF;        /* 弱化文本 (Gray 400) */
  --text-inverse: #FFFFFF;      /* 反色文本 */

  /* 边框色 */
  --border-light: #E5E7EB;      /* 浅色边框 (Gray 200) */
  --border-medium: #D1D5DB;     /* 中等边框 (Gray 300) */
  --border-focus: var(--primary-400); /* 聚焦边框 */
}
```

#### 功能色 (Functional Colors)

```css
:root {
  /* 成功 - 绿色 */
  --success-light: #D1FAE5;
  --success-main: #10B981;
  --success-dark: #059669;

  /* 警告 - 橙色 */
  --warning-light: #FEF3C7;
  --warning-main: #F59E0B;
  --warning-dark: #D97706;

  /* 错误 - 红色 */
  --error-light: #FEE2E2;
  --error-main: #EF4444;
  --error-dark: #DC2626;

  /* 信息 - 蓝色 */
  --info-light: #DBEAFE;
  --info-main: #3B82F6;
  --info-dark: #2563EB;
}
```

### 2.3 暗色模式 (可选,MVP暂不实现)

```css
.dark {
  --bg-primary: #111827;
  --bg-secondary: #1F2937;
  --bg-tertiary: #374151;

  --text-primary: #F9FAFB;
  --text-secondary: #D1D5DB;
  --text-muted: #9CA3AF;

  --border-light: #374151;
  --border-medium: #4B5563;
}
```

---

## 3. 字体系统

### 3.1 字体家族

```css
:root {
  /* 主字体 - 优先使用系统字体 */
  --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, 
               "Helvetica Neue", Arial, "Noto Sans SC", sans-serif;
  
  /* 等宽字体 - 用于代码示例 */
  --font-mono: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", 
               Consolas, "Courier New", monospace;
}
```

### 3.2 字号层级

遵循 1.25 比例缩放 (Major Third Scale):

| 层级 | 字号 | 行高 | 字重 | 用途 |
| :--- | :--- | :--- | :--- | :--- |
| H1 | 32px / 2rem | 1.2 | 700 | 页面标题 |
| H2 | 24px / 1.5rem | 1.3 | 600 | 区块标题 |
| H3 | 20px / 1.25rem | 1.4 | 600 | 卡片标题 |
| Body Large | 16px / 1rem | 1.6 | 400 | 对话正文 |
| Body | 14px / 0.875rem | 1.6 | 400 | 普通文本 |
| Small | 12px / 0.75rem | 1.5 | 400 | 辅助说明 |
| Caption | 11px / 0.6875rem | 1.4 | 400 | 标签、提示 |

```css
/* Tailwind 配置扩展 */
theme: {
  extend: {
    fontSize: {
      'h1': ['32px', { lineHeight: '1.2', fontWeight: '700' }],
      'h2': ['24px', { lineHeight: '1.3', fontWeight: '600' }],
      'h3': ['20px', { lineHeight: '1.4', fontWeight: '600' }],
      'body-lg': ['16px', { lineHeight: '1.6', fontWeight: '400' }],
      'body': ['14px', { lineHeight: '1.6', fontWeight: '400' }],
      'small': ['12px', { lineHeight: '1.5', fontWeight: '400' }],
      'caption': ['11px', { lineHeight: '1.4', fontWeight: '400' }],
    }
  }
}
```

### 3.3 字体颜色应用

```css
.text-primary { color: var(--text-primary); }
.text-secondary { color: var(--text-secondary); }
.text-muted { color: var(--text-muted); }
.text-accent { color: var(--color-accent); }
.text-success { color: var(--success-main); }
.text-error { color: var(--error-main); }
```

---

## 4. 间距系统

### 4.1 基础间距单位

采用 **4px 基准网格系统**,所有间距为 4 的倍数:

```css
:root {
  --space-1: 4px;    /* xs */
  --space-2: 8px;    /* sm */
  --space-3: 12px;   /* md */
  --space-4: 16px;   /* lg */
  --space-5: 20px;   /* xl */
  --space-6: 24px;   /* 2xl */
  --space-8: 32px;   /* 3xl */
  --space-10: 40px;  /* 4xl */
  --space-12: 48px;  /* 5xl */
  --space-16: 64px;  /* 6xl */
}
```

### 4.2 应用场景

| 场景 | 间距值 | Tailwind 类 |
| :--- | :--- | :--- |
| 组件内元素间距 | 8px | `gap-2` / `p-2` |
| 卡片内边距 | 16px | `p-4` |
| 区块间距 | 24px | `gap-6` / `my-6` |
| 页面边距 | 32px | `p-8` |
| 对话框内边距 | 24px | `p-6` |

---

## 5. 圆角与阴影

### 5.1 圆角规范

```css
:root {
  --radius-sm: 4px;    /* 小按钮、标签 */
  --radius-md: 8px;    /* 输入框、卡片 */
  --radius-lg: 12px;   /* 大卡片、模态框 */
  --radius-xl: 16px;   /* 情景卡片 */
  --radius-full: 9999px; /* 圆形按钮、头像 */
}
```

### 5.2 阴影系统 (扁平设计,极少使用)

MVP 采用**扁平设计风格**,仅在必要时使用轻微阴影:

```css
:root {
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.07);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);
  
  /* 黄色系专属阴影 */
  --shadow-accent: 0 4px 12px rgba(245, 158, 11, 0.25);
}
```

**使用原则**:
- 默认不使用阴影,通过边框和背景色区分层级
- 仅悬浮卡片、下拉菜单使用 `shadow-md`
- 模态框、Toast 使用 `shadow-lg`

---

## 6. 布局系统

### 6.1 响应式断点

```css
/* Tailwind 默认断点 */
sm: 640px   /* 手机横屏 */
md: 768px   /* 平板竖屏 */
lg: 1024px  /* 平板横屏 / 小笔记本 */
xl: 1280px  /* 桌面 */
2xl: 1536px /* 大桌面 */
```

**MVP 目标**: 桌面应用固定宽度 **1024px**,不支持移动端响应式

### 6.2 栅格系统

采用 **12 列栅格**,间距 16px:

```css
.grid-container {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 16px;
  max-width: 1024px;
  margin: 0 auto;
  padding: 32px;
}
```

---

## 7. 核心页面布局

### 7.1 情景选择页 (Scenario Selection)

**布局结构**:

```
┌──────────────────────────────────────────────┐
│  Header                                      │
│  ┌────────────────────────────────────────┐  │
│  │ Logo          Settings    Profile      │  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Hero Section                                │
│  ┌────────────────────────────────────────┐  │
│  │  Welcome back, [Name]!                 │  │
│  │  Ready to practice English today?      │  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Scenario Cards (2x3 Grid)                   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐    │
│  │ ☕ Coffee │ │ 🍽️ Rest. │ │ 🏨 Hotel │    │
│  └──────────┘ └──────────┘ └──────────┘    │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐    │
│  │ 💼 Interview│ │ ✈️ Airport│ │ 🎉 Social│  │
│  └──────────┘ └──────────┘ └──────────┘    │
├──────────────────────────────────────────────┤
│  Footer Stats                                │
│  ┌────────────────────────────────────────┐  │
│  │ This Week: 5 sessions | 23 new words  │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

**关键样式**:

```css
.scenario-card {
  background: var(--bg-tertiary);
  border: 2px solid transparent;
  border-radius: var(--radius-xl);
  padding: 24px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.scenario-card:hover {
  border-color: var(--color-accent);
  background: var(--primary-50);
  transform: translateY(-2px);
}

.scenario-card .icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.scenario-card .title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.scenario-card .description {
  font-size: 14px;
  color: var(--text-secondary);
}
```

---

### 7.2 聊天对话页 (Chat Interface)

**布局结构**:

```
┌──────────────────────────────────────────────┐
│  Chat Header                                 │
│  ┌────────────────────────────────────────┐  │
│  │ ← Back    Coffee Shop       ⚙️ Settings│  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Message List (Scrollable)                   │
│  ┌────────────────────────────────────────┐  │
│  │                                        │  │
│  │  AI: Hi! What can I get for you?      │  │
│  │                                        │  │
│  │  You: I'd like a latte, please.       │  │
│  │                                        │  │
│  │  AI: Great choice! Hot or iced?       │  │
│  │                                        │  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Input Area                                  │
│  ┌────────────────────────────────────────┐  │
│  │ [Type your message...]         [🎤] 📤 │  │
│  └────────────────────────────────────────┘  │
│  Hold to Speak Button (Large, Centered)      │
│  ┌────────────────────────────────────────┐  │
│  │          🎤 Hold to Speak              │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

**聊天气泡样式**:

```css
/* AI 消息 - 左侧,白色背景 */
.message-ai {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.message-ai .avatar {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-full);
  background: var(--primary-200);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.message-ai .bubble {
  background: var(--bg-tertiary);
  border-radius: var(--radius-lg);
  padding: 12px 16px;
  max-width: 70%;
  font-size: 16px;
  line-height: 1.6;
  color: var(--text-primary);
}

/* 用户消息 - 右侧,黄色背景 */
.message-user {
  display: flex;
  flex-direction: row-reverse;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.message-user .avatar {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-full);
  background: var(--primary-500);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 600;
}

.message-user .bubble {
  background: var(--primary-100);
  border: 1px solid var(--primary-300);
  border-radius: var(--radius-lg);
  padding: 12px 16px;
  max-width: 70%;
  font-size: 16px;
  line-height: 1.6;
  color: var(--text-primary);
}

/* 可点击单词样式 */
.clickable-word {
  cursor: pointer;
  border-bottom: 2px dotted var(--color-accent);
  transition: background 0.2s;
}

.clickable-word:hover {
  background: var(--color-selection);
}

/* 打字机动画 */
.typing-indicator::after {
  content: '▋';
  animation: blink 1s infinite;
  color: var(--color-accent);
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}
```

**录音按钮样式**:

```css
.record-button {
  width: 100%;
  padding: 16px 24px;
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-lg);
  font-size: 18px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.record-button:hover {
  background: var(--color-accent-light);
  transform: scale(1.02);
}

.record-button:active,
.record-button.recording {
  background: var(--color-accent-dark);
  transform: scale(0.98);
}

.record-button.recording {
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(245, 158, 11, 0.4); }
  50% { box-shadow: 0 0 0 20px rgba(245, 158, 11, 0); }
}
```

---

### 7.3 生词本页 (Vocabulary List)

**布局结构**:

```
┌──────────────────────────────────────────────┐
│  Vocabulary Header                           │
│  ┌────────────────────────────────────────┐  │
│  │ ← Back    My Vocabulary    [+ Add]    │  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Filter & Sort                               │
│  ┌────────────────────────────────────────┐  │
│  │ [All Words ▼]  [Sort by: Mastery ▼]   │  │
│  └────────────────────────────────────────┘  │
├──────────────────────────────────────────────┤
│  Word List                                   │
│  ┌────────────────────────────────────────┐  │
│  │ procrastinate                          │  │
│  │ /prəˈkræstɪneɪt/  •  Learned 3 days ago│  │
│  │ Mastery: ★★★☆☆  Review: 2 times      │  │
│  └────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────┐  │
│  │ fascinating                            │  │
│  │ /ˈfæsɪneɪtɪŋ/  •  Learned 1 week ago  │  │
│  │ Mastery: ★★★★☆  Review: 4 times      │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

**生词卡片样式**:

```css
.vocabulary-card {
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  padding: 16px;
  margin-bottom: 12px;
  transition: all 0.2s;
}

.vocabulary-card:hover {
  background: var(--primary-50);
  border-left: 4px solid var(--color-accent);
}

.vocabulary-card .word {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.vocabulary-card .phonetic {
  font-size: 14px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  margin-bottom: 8px;
}

.vocabulary-card .meta {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: var(--text-muted);
}

.mastery-stars {
  display: inline-flex;
  gap: 2px;
}

.mastery-stars .star {
  color: var(--primary-300);
}

.mastery-stars .star.filled {
  color: var(--color-accent);
}
```

---

## 8. 组件库

### 8.1 按钮 (Buttons)

#### 主要按钮 (Primary Button)

```css
.btn-primary {
  padding: 12px 24px;
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover {
  background: var(--color-accent-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-accent);
}

.btn-primary:active {
  background: var(--color-accent-dark);
  transform: translateY(0);
}

.btn-primary:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
  opacity: 0.5;
}
```

#### 次要按钮 (Secondary Button)

```css
.btn-secondary {
  padding: 12px 24px;
  background: transparent;
  color: var(--color-accent);
  border: 2px solid var(--color-accent);
  border-radius: var(--radius-md);
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: var(--primary-50);
}
```

#### 幽灵按钮 (Ghost Button)

```css
.btn-ghost {
  padding: 8px 16px;
  background: transparent;
  color: var(--text-secondary);
  border: none;
  border-radius: var(--radius-md);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-ghost:hover {
  background: var(--color-hover);
  color: var(--text-primary);
}
```

#### 图标按钮 (Icon Button)

```css
.btn-icon {
  width: 40px;
  height: 40px;
  padding: 0;
  background: transparent;
  border: none;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--color-hover);
}

.btn-icon svg {
  width: 20px;
  height: 20px;
  color: var(--text-secondary);
}
```

---

### 8.2 输入框 (Input Fields)

#### 文本输入框

```css
.input-text {
  width: 100%;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: 2px solid var(--border-medium);
  border-radius: var(--radius-md);
  font-size: 16px;
  color: var(--text-primary);
  transition: all 0.2s;
}

.input-text:focus {
  outline: none;
  border-color: var(--color-focus);
  box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
}

.input-text::placeholder {
  color: var(--text-muted);
}

.input-text.error {
  border-color: var(--error-main);
}

.input-text.success {
  border-color: var(--success-main);
}
```

#### 搜索输入框

```css
.input-search {
  position: relative;
}

.input-search input {
  padding-left: 40px;
}

.input-search .icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
}
```

---

### 8.3 卡片 (Cards)

#### 基础卡片

```css
.card {
  background: var(--bg-tertiary);
  border-radius: var(--radius-lg);
  padding: 24px;
  border: 1px solid var(--border-light);
}

.card-header {
  margin-bottom: 16px;
}

.card-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.card-description {
  font-size: 14px;
  color: var(--text-secondary);
}

.card-content {
  /* 内容由子组件定义 */
}

.card-footer {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-light);
  display: flex;
  justify-content: space-between;
  align-items: center;
}
```

#### 可点击卡片

```css
.card-clickable {
  cursor: pointer;
  transition: all 0.2s;
}

.card-clickable:hover {
  border-color: var(--color-accent);
  background: var(--primary-50);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}
```

---

### 8.4 徽章 (Badges)

```css
.badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
}

.badge-primary {
  background: var(--primary-100);
  color: var(--primary-800);
}

.badge-success {
  background: var(--success-light);
  color: var(--success-dark);
}

.badge-warning {
  background: var(--warning-light);
  color: var(--warning-dark);
}

.badge-error {
  background: var(--error-light);
  color: var(--error-dark);
}
```

---

### 8.5 进度条 (Progress Bar)

```css
.progress-bar {
  width: 100%;
  height: 8px;
  background: var(--border-light);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-bar .fill {
  height: 100%;
  background: var(--color-accent);
  border-radius: var(--radius-full);
  transition: width 0.3s ease;
}

/* 下载进度特殊样式 */
.progress-bar.download .fill {
  background: linear-gradient(90deg, var(--primary-400), var(--color-accent));
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% { opacity: 1; }
  50% { opacity: 0.7; }
  100% { opacity: 1; }
}
```

---

### 8.6 Toast 通知

```css
.toast {
  position: fixed;
  bottom: 24px;
  right: 24px;
  background: var(--bg-primary);
  border: 1px solid var(--border-medium);
  border-radius: var(--radius-md);
  padding: 16px 20px;
  box-shadow: var(--shadow-lg);
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 300px;
  max-width: 400px;
  animation: slideIn 0.3s ease;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.toast.success {
  border-left: 4px solid var(--success-main);
}

.toast.error {
  border-left: 4px solid var(--error-main);
}

.toast.warning {
  border-left: 4px solid var(--warning-main);
}

.toast.info {
  border-left: 4px solid var(--info-main);
}
```

---

## 9. 交互模式

### 9.1 录音按钮交互

**状态流转**:

```
Idle → Pressing → Recording → Released → Processing → Result
```

**视觉反馈**:

```css
/* Idle 状态 */
.record-button {
  background: var(--color-accent);
}

/* Pressing 状态 (按住未松) */
.record-button.pressing {
  background: var(--color-accent-dark);
  transform: scale(0.95);
}

/* Recording 状态 (录音中) */
.record-button.recording {
  background: var(--error-main);
  animation: pulse 1.5s infinite;
}

/* Processing 状态 (STT 识别中) */
.record-button.processing {
  background: var(--text-muted);
  cursor: wait;
}

/* Result 状态 (显示识别结果) */
.record-button.result {
  background: var(--success-main);
}
```

**音频波形可视化** (可选增强):

```css
.waveform {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 3px;
  height: 40px;
}

.waveform .bar {
  width: 4px;
  background: white;
  border-radius: var(--radius-full);
  animation: wave 1s ease-in-out infinite;
}

.waveform .bar:nth-child(1) { animation-delay: 0s; height: 20%; }
.waveform .bar:nth-child(2) { animation-delay: 0.1s; height: 40%; }
.waveform .bar:nth-child(3) { animation-delay: 0.2s; height: 60%; }
.waveform .bar:nth-child(4) { animation-delay: 0.3s; height: 80%; }
.waveform .bar:nth-child(5) { animation-delay: 0.4s; height: 100%; }

@keyframes wave {
  0%, 100% { transform: scaleY(0.5); }
  50% { transform: scaleY(1); }
}
```

---

### 9.2 单词双击交互

**交互流程**:

```
1. 用户双击单词
2. 单词高亮 (黄色背景)
3. 弹出工具提示 "Teaching..."
4. AI 回复教学解释
5. 自动加入生词本
6. 显示确认徽章 ✓
```

**视觉实现**:

```css
/* 可点击单词 */
.clickable-word {
  cursor: pointer;
  border-bottom: 2px dotted var(--color-accent);
  transition: all 0.2s;
  position: relative;
}

.clickable-word:hover {
  background: var(--color-selection);
}

/* 选中状态 */
.clickable-word.selected {
  background: var(--color-accent);
  color: white;
  border-bottom-color: transparent;
  border-radius: 2px;
  padding: 0 2px;
}

/* 教学工具提示 */
.teaching-tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: var(--text-primary);
  color: white;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  white-space: nowrap;
  animation: fadeIn 0.2s;
}

.teaching-tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-top-color: var(--text-primary);
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateX(-50%) translateY(5px); }
  to { opacity: 1; transform: translateX(-50%) translateY(0); }
}

/* 已学习徽章 */
.learned-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  background: var(--success-light);
  color: var(--success-dark);
  padding: 2px 8px;
  border-radius: var(--radius-full);
  font-size: 11px;
  font-weight: 500;
  margin-left: 8px;
  animation: popIn 0.3s ease;
}

@keyframes popIn {
  0% { transform: scale(0); }
  70% { transform: scale(1.2); }
  100% { transform: scale(1); }
}
```

---

### 9.3 流式文本动画

**打字机效果**:

```css
.streaming-text {
  position: relative;
}

/* 光标闪烁 */
.streaming-text::after {
  content: '▋';
  color: var(--color-accent);
  animation: blink 1s infinite;
  margin-left: 2px;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}

/* 逐字淡入效果 (可选) */
.streaming-text .char {
  opacity: 0;
  animation: charFadeIn 0.1s forwards;
}

@keyframes charFadeIn {
  from { opacity: 0; transform: translateY(5px); }
  to { opacity: 1; transform: translateY(0); }
}
```

**React 实现示例**:

```tsx
import { useState, useEffect } from 'react';

function StreamingText({ text }: { text: string }) {
  const [displayedText, setDisplayedText] = useState('');
  
  useEffect(() => {
    let index = 0;
    const interval = setInterval(() => {
      if (index < text.length) {
        setDisplayedText(text.slice(0, index + 1));
        index++;
      } else {
        clearInterval(interval);
      }
    }, 30); // 每 30ms 显示一个字符
    
    return () => clearInterval(interval);
  }, [text]);
  
  return <span className="streaming-text">{displayedText}</span>;
}
```

---

### 9.4 加载状态

**Skeleton Loading**:

```css
.skeleton {
  background: linear-gradient(90deg, 
    var(--border-light) 25%, 
    var(--bg-secondary) 50%, 
    var(--border-light) 75%
  );
  background-size: 200% 100%;
  animation: skeleton-loading 1.5s infinite;
  border-radius: var(--radius-sm);
}

@keyframes skeleton-loading {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* 使用示例 */
.skeleton-text {
  height: 16px;
  margin-bottom: 8px;
}

.skeleton-text.short {
  width: 40%;
}

.skeleton-text.medium {
  width: 70%;
}

.skeleton-text.long {
  width: 100%;
}
```

**Spinner**:

```css
.spinner {
  width: 24px;
  height: 24px;
  border: 3px solid var(--border-light);
  border-top-color: var(--color-accent);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

---

## 10. 无障碍设计 (Accessibility)

### 10.1 WCAG 2.1 AA 标准合规

#### 对比度要求

- **正常文本**: 至少 4.5:1
- **大文本 (≥18px 或 ≥14px bold)**: 至少 3:1
- **UI 组件和图形对象**: 至少 3:1

**当前配色对比度检查**:

| 组合 | 对比度 | 是否达标 |
| :--- | :--- | :--- |
| `--text-primary` on `--bg-primary` | 12.6:1 | ✅ AAA |
| `--text-secondary` on `--bg-primary` | 5.3:1 | ✅ AA |
| `--color-accent` on white | 2.9:1 | ❌ 需调整 |
| white on `--color-accent` | 2.9:1 | ❌ 需调整 |

**修正方案**: 按钮文字使用深色而非白色

```css
.btn-primary {
  background: var(--color-accent);
  color: var(--primary-900); /* 深棕色,对比度 4.6:1 */
}
```

---

### 10.2 键盘导航

**焦点样式**:

```css
*:focus {
  outline: 2px solid var(--color-focus);
  outline-offset: 2px;
}

*:focus:not(:focus-visible) {
  outline: none;
}

*:focus-visible {
  outline: 2px solid var(--color-focus);
  outline-offset: 2px;
}
```

**Tab 顺序**: 确保 DOM 顺序与视觉顺序一致

---

### 10.3 屏幕阅读器支持

**ARIA 标签**:

```tsx
<button 
  aria-label="Start recording"
  aria-pressed={isRecording}
  className="record-button"
>
  <MicIcon />
  {isRecording ? 'Recording...' : 'Hold to Speak'}
</button>

<div 
  role="log" 
  aria-live="polite"
  aria-label="Chat messages"
>
  {messages.map(msg => <Message key={msg.id} {...msg} />)}
</div>
```

**语义化 HTML**:

```html
<header role="banner">...</header>
<nav role="navigation">...</nav>
<main role="main">...</main>
<footer role="contentinfo">...</footer>

<article aria-label="AI message">...</article>
<article aria-label="Your message">...</article>
```

---

### 10.4 运动减少

**尊重用户系统设置**:

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## 11. 性能优化

### 11.1 渲染优化

**虚拟滚动** (长列表):

```tsx
import { FixedSizeList } from 'react-window';

function MessageList({ messages }) {
  return (
    <FixedSizeList
      height={600}
      itemCount={messages.length}
      itemSize={80}
      width="100%"
    >
      {({ index, style }) => (
        <Message message={messages[index]} style={style} />
      )}
    </FixedSizeList>
  );
}
```

**图片懒加载**:

```tsx
<img 
  src={scenario.image} 
  alt={scenario.name}
  loading="lazy"
/>
```

---

### 11.2 CSS 优化

**避免重排**:

```css
/* ❌ 避免 */
.element {
  transition: width 0.3s; /* 触发重排 */
}

/* ✅ 推荐 */
.element {
  transition: transform 0.3s, opacity 0.3s; /* 仅合成 */
}
```

**使用 CSS 变量**:

```css
/* 主题切换无需重新计算样式 */
:root[data-theme="dark"] {
  --bg-primary: #111827;
  --text-primary: #F9FAFB;
}
```

---

## 12. 设计资源

### 12.1 图标库

推荐使用 **Lucide Icons** (轻量、现代):

```bash
npm install lucide-react
```

**常用图标**:

```tsx
import { 
  Mic, Send, Settings, User, ArrowLeft, 
  BookOpen, Coffee, Utensils, Bed, Briefcase,
  Plane, Users, Star, Clock, Trash2
} from 'lucide-react';
```

---

### 12.2 插图资源

MVP 阶段使用 **Emoji** 作为情景图标:

- ☕ Coffee Shop
- 🍽️ Restaurant
- 🏨 Hotel
- 💼 Job Interview
- ✈️ Airport
- 🎉 Social Gathering

未来可替换为自定义 SVG 插图。

---

### 12.3 设计工具推荐

- **Figma**: UI 设计和原型
- **Coolors.co**: 调色板生成
- **Realtime Colors**: 实时预览配色方案
- **Contrast Checker**: 对比度验证

---

## 13. 实施清单

### 13.1 MVP 必需组件

- [ ] 情景选择卡片 (6个)
- [ ] 聊天气泡 (AI + User)
- [ ] 录音按钮 (带状态)
- [ ] 文本输入框
- [ ] 发送按钮
- [ ] 生词本列表
- [ ] 生词卡片
- [ ] Toast 通知
- [ ] 加载 Spinner
- [ ] Skeleton Loading

### 13.2 样式文件结构

```
src/
├── styles/
│   ├── globals.css          # 全局样式、CSS 变量
│   ├── components/
│   │   ├── button.css
│   │   ├── card.css
│   │   ├── input.css
│   │   ├── chat-bubble.css
│   │   └── ...
│   └── animations.css       # 所有动画定义
├── components/
│   ├── ui/
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   ├── Input.tsx
│   │   └── ...
│   ├── ChatBubble.tsx
│   ├── RecordButton.tsx
│   ├── ScenarioCard.tsx
│   └── VocabularyCard.tsx
```

---

## 14. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义完整 UI/UX 规范 | LingoMate Team |

---

**文档结束**
