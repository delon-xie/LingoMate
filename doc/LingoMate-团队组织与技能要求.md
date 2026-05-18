# LingoMate 团队组织与技能要求

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用阶段** | MVP v0.9 开发 |
| **维护者** | 项目管理团队 |

---

## 1. 团队规模建议

### 1.1 MVP 阶段 (8周)

根据技术架构文档和任务拆解,MVP 阶段推荐 **3-4 人核心团队**:

```
┌─────────────────────────────────────┐
│      LingoMate MVP 团队结构          │
├─────────────────────────────────────┤
│                                     │
│  Tech Lead / Full-Stack (1人)       │
│  ├─ Rust/Tauri 后端                 │
│  ├─ 架构设计与技术决策               │
│  └─ 代码审查                         │
│                                     │
│  Frontend Developer (1人)           │
│  ├─ React/TypeScript 前端           │
│  ├─ UI/UX 实现                      │
│  └─ 状态管理                         │
│                                     │
│  AI/Prompt Engineer (0.5-1人)       │
│  ├─ Prompt 设计与优化               │
│  ├─ 教学效果验证                     │
│  └─ 模型调优                         │
│                                     │
│  QA/DevOps (0.5人,可兼职)           │
│  ├─ 测试用例编写                    │
│  ├─ CI/CD 配置                      │
│  └─ 多平台打包                      │
│                                     │
└─────────────────────────────────────┘
```

**总人力投入**: 约 **3-3.5 人月**

---

### 1.2 扩展阶段 (v1.0+, 可选)

如果预算允许,可增加:

| 角色 | 人数 | 职责 |
| :--- | :--- | :--- |
| **UI/UX Designer** | 1人 (前期介入) | 视觉设计、原型制作、用户研究 |
| **Backend Developer** | 1人 | 分担 Rust 后端工作,加速开发 |
| **Content Specialist** | 1人 (兼职) | 情景对话脚本、教学内容审核 |
| **Product Manager** | 0.5人 | 需求管理、进度跟踪、用户反馈收集 |

---

## 2. 核心岗位详细定义

### 2.1 Tech Lead / Full-Stack Developer (技术负责人/全栈开发)

#### 岗位职责

1. **架构设计**
   - 设计 Tauri + Rust 后端架构
   - 定义前后端通信协议 (Tauri Commands/Events)
   - 数据库 Schema 设计与优化

2. **核心功能开发**
   - Ollama 进程管理 (启动、停止、健康检查)
   - 语音服务集成 (STT/TTS)
   - 教学引擎实现 (Prompt 构造、流式响应)
   - 数据持久化层 (SQLite CRUD)

3. **技术决策**
   - 技术选型与评估
   - 性能优化策略
   - 安全方案设计

4. **代码质量**
   - 代码审查 (Code Review)
   - 制定编码规范
   - 技术指导与知识分享

---

#### 必需技能

**核心技术栈**:

| 技能 | 熟练度 | 说明 |
| :--- | :--- | :--- |
| **Rust** | ⭐⭐⭐⭐⭐ | 2年以上经验,熟悉异步编程 (tokio)、错误处理、生命周期 |
| **Tauri** | ⭐⭐⭐⭐ | 1年以上经验,理解 IPC 机制、插件系统、打包流程 |
| **SQLite** | ⭐⭐⭐⭐ | 熟悉 SQL、索引优化、事务处理、迁移策略 |
| **TypeScript** | ⭐⭐⭐⭐ | 能阅读和修改前端代码,理解 React 组件生命周期 |
| **React** | ⭐⭐⭐ | 理解组件化、Hooks、状态管理基本概念 |

**系统编程**:

- ✅ 进程管理 (spawn、kill、信号处理)
- ✅ 网络编程 (HTTP client、WebSocket、SSE)
- ✅ 文件系统操作 (读写、权限、路径处理)
- ✅ 跨平台开发 (Windows/macOS/Linux 差异处理)

**AI 集成经验**:

- ✅ 调用 LLM API (OpenAI、Ollama、Anthropic 等)
- ✅ 流式响应处理 (Server-Sent Events)
- ✅ Prompt 工程基础理解

**DevOps**:

- ✅ Git 工作流 (branching strategy、merge conflict 解决)
- ✅ CI/CD 配置 (GitHub Actions、GitLab CI)
- ✅ 多平台构建 (cross-compilation)

---

#### 加分技能

- 🎯 教育科技产品经验
- 🎯 TTS/STT 系统集成经验
- 🎯 性能优化经验 (内存管理、CPU  profiling)
- 🎯 开源项目贡献经历

---

#### 面试评估要点

**技术面试题目示例**:

1. **Rust 基础**:
   ```rust
   // 问题: 解释这段代码的问题并修复
   fn get_first_element(vec: &Vec<String>) -> &str {
       &vec[0]
   }
   ```

2. **Tauri 架构**:
   - "如何在 Tauri 中实现前端调用后端 Rust 函数?"
   - "Tauri Command 和 Event 的区别是什么?"

3. **系统设计**:
   - "设计一个支持流式响应的 AI 对话系统"
   - "如何实现 Ollama 进程的自动重启和恢复?"

4. **实际问题**:
   - "如果 Ollama 在 8GB 内存设备上崩溃,如何诊断和解决?"
   - "如何优化 SQLite 查询性能?"

---

### 2.2 Frontend Developer (前端开发工程师)

#### 岗位职责

1. **UI 实现**
   - 根据 UI/UX 设计规范实现界面
   - 情景选择页、聊天对话页、生词本页
   - 响应式布局 (桌面端固定宽度 1024px)

2. **交互逻辑**
   - 录音按钮状态管理 (Idle → Recording → Processing)
   - 流式文本动画 (打字机效果)
   - 单词双击教学交互

3. **状态管理**
   - 使用 Zustand 管理全局状态
   - 对话历史、生词本、用户设置同步

4. **Tauri 集成**
   - 调用 Tauri Commands (发送消息、查询数据)
   - 监听 Tauri Events (AI 响应、录音状态)
   - 错误处理与用户提示

---

#### 必需技能

**核心技术栈**:

| 技能 | 熟练度 | 说明 |
| :--- | :--- | :--- |
| **React 18** | ⭐⭐⭐⭐⭐ | 2年以上经验,熟悉 Hooks、Context、Suspense |
| **TypeScript** | ⭐⭐⭐⭐⭐ | 严格模式、类型守卫、泛型、工具类型 |
| **Tailwind CSS** | ⭐⭐⭐⭐ | 1年以上经验,自定义主题、响应式设计 |
| **Zustand** | ⭐⭐⭐⭐ | 或 Redux/Jotai,理解状态管理原理 |
| **Vite** | ⭐⭐⭐ | 构建配置、插件开发、性能优化 |

**UI/UX 实现**:

- ✅ CSS 动画 (transition、animation、keyframes)
- ✅ 无障碍设计 (ARIA labels、键盘导航、对比度)
- ✅ 组件化设计 (可复用、可测试)
- ✅ 设计系统实施 (CSS 变量、语义化类名)

**Tauri 前端集成**:

- ✅ `@tauri-apps/api` 使用 (invoke、listen、convertFileSrc)
- ✅ 文件系统访问 (读写本地文件)
- ✅ 对话框、通知系统

**测试**:

- ✅ Jest/Vitest 单元测试
- ✅ React Testing Library 组件测试
- ✅ Playwright E2E 测试基础

---

#### 加分技能

- 🎯 教育类产品 UI 经验
- 🎯 音频可视化 (Web Audio API)
- 🎯 国际化 (i18n) 实现
- 🎯 性能优化 (懒加载、代码分割、虚拟滚动)

---

#### 面试评估要点

**技术面试题目示例**:

1. **React 基础**:
   ```tsx
   // 问题: 这个组件有什么性能问题?如何优化?
   function ChatList({ messages }) {
     return (
       <div>
         {messages.map(msg => (
           <ChatBubble key={msg.id} message={msg} />
         ))}
       </div>
     );
   }
   ```

2. **状态管理**:
   - "如何使用 Zustand 实现对话历史的全局状态?"
   - "如何处理异步操作的状态 (loading、error、success)?"

3. **UI 实现**:
   - "实现一个流式文本动画 (逐字显示)"
   - "如何确保黄色系配色的对比度符合 WCAG AA 标准?"

4. **Tauri 集成**:
   - "前端如何接收后端的流式响应?"
   - "如何处理麦克风权限被拒绝的情况?"

---

### 2.3 AI/Prompt Engineer (AI 工程师/Prompt 专家)

#### 岗位职责

1. **Prompt 设计**
   - 编写 6 个情景的 System Prompt
   - 设计"即点即学"专用 Prompt
   - 智能复习 Prompt 策略

2. **教学效果验证**
   - 测试不同难度级别的教学效果
   - A/B 测试 Prompt 变体
   - 收集用户反馈并迭代优化

3. **模型调优**
   - 选择合适的量化模型 (qwen2.5:3b vs 7b)
   - 调整生成参数 (temperature、top_p、max_tokens)
   - 性能与质量平衡

4. **内容质量把控**
   - 确保 AI 回复符合启发式教学法
   - 避免直接翻译、保持英语沉浸
   - 文化敏感性检查

---

#### 必需技能

**Prompt 工程**:

| 技能 | 熟练度 | 说明 |
| :--- | :--- | :--- |
| **LLM 原理** | ⭐⭐⭐⭐ | 理解 Transformer、tokenization、上下文窗口 |
| **Prompt 设计** | ⭐⭐⭐⭐⭐ | Chain-of-Thought、Few-Shot、Role-Playing |
| **指令遵循优化** | ⭐⭐⭐⭐ | 防止 jailbreak、增强约束力 |
| **评估方法** | ⭐⭐⭐⭐ | 人工评估、自动化指标 (BLEU、ROUGE) |

**语言学背景**:

- ✅ 英语教学法 (启发式、交际法、任务型教学)
- ✅ CEFR 等级标准 (A1-C2)
- ✅ 二语习得理论 (i+1、输出假说)
- ✅ 常见中式英语错误模式

**技术能力**:

- ✅ Python 基础 (数据处理、脚本编写)
- ✅ Ollama/LangChain 使用
- ✅ JSON/YAML 配置文件管理
- ✅ Git 版本控制

**数据分析**:

- ✅ 用户行为分析 (对话轮次、停留时间)
- ✅ A/B 测试设计
- ✅ 学习效果量化 (词汇掌握率、流利度提升)

---

#### 加分技能

- 🎯 应用语言学硕士/博士学历
- 🎯 TESOL/TEFL 教师资格证
- 🎯 教育产品研发经验
- 🎯 NLP 项目经验 (fine-tuning、RAG)

---

#### 面试评估要点

**实际测试任务**:

1. **Prompt 设计**:
   ```
   任务: 为"咖啡店点单"情景设计 System Prompt
   
   要求:
   - AI 扮演 barista 角色
   - 使用启发式教学法
   - 针对中级水平 (B1) 用户
   - 包含目标词汇: latte, cappuccino, size, hot/iced
   
   提交: 完整的 Prompt 文本 + 3 轮示例对话
   ```

2. **错误纠正**:
   ```
   用户输入: "I want coffee."
   
   任务: 设计 AI 的回复,要求:
   - 不直接说"That's wrong"
   - 引导用户使用更礼貌的表达
   - 保持对话自然流畅
   ```

3. **模型对比**:
   - "比较 qwen2.5:3b 和 qwen2.5:7b 在教学场景下的优劣"
   - "如何选择 temperature 参数以平衡创造性和准确性?"

---

### 2.4 QA/DevOps Engineer (测试/运维工程师,可兼职)

#### 岗位职责

1. **测试计划执行**
   - 编写单元测试 (Rust + React)
   - 集成测试 (Tauri Commands)
   - E2E 测试 (Playwright)

2. **CI/CD 配置**
   - GitHub Actions 流水线
   - 自动化构建 (三平台)
   - 代码签名与公证 (macOS)

3. **性能测试**
   - 启动时间测量
   - 内存泄漏检测
   - 响应延迟基准测试

4. **发布管理**
   - 版本号管理
   - Release Notes 编写
   - 更新器配置

---

#### 必需技能

**测试框架**:

| 技能 | 熟练度 | 说明 |
| :--- | :--- | :--- |
| **Jest/Vitest** | ⭐⭐⭐⭐ | React 组件测试、Mock、Snapshot |
| **Playwright** | ⭐⭐⭐⭐ | E2E 测试、跨浏览器、截图对比 |
| **cargo test** | ⭐⭐⭐ | Rust 单元测试、集成测试 |
| **Test Design** | ⭐⭐⭐⭐ | 边界值、等价类、错误推测 |

**DevOps**:

- ✅ GitHub Actions/GitLab CI
- ✅ Docker 基础 (可选)
- ✅ Shell 脚本 (bash/powershell)
- ✅ 版本管理 (Semantic Versioning)

**多平台构建**:

- ✅ Windows NSIS 打包
- ✅ macOS DMG + 代码签名
- ✅ Linux AppImage/DEB

**监控与日志**:

- ✅ 日志分析 (grep、awk)
- ✅ 性能 profiling (Chrome DevTools、cargo-flamegraph)
- ✅ 错误追踪 (Sentry 可选)

---

#### 加分技能

- 🎯 安全测试经验 (OWASP Top 10)
- 🎯 负载测试 (k6、JMeter)
- 🎯 容器化部署 (Docker、Kubernetes)
- 🎯 云服务平台 (AWS、Azure、GCP)

---

#### 面试评估要点

**实际测试任务**:

1. **测试用例设计**:
   ```
   功能: 用户双击单词触发教学
   
   任务: 设计 10 个测试用例,覆盖:
   - 正常流程
   - 边界情况 (空单词、特殊字符)
   - 错误场景 (网络断开、模型未加载)
   ```

2. **CI/CD 配置**:
   ```yaml
   # 任务: 编写 GitHub Actions workflow
   # 要求:
   # - 在 push 时触发
   # - 运行单元测试
   # - 构建 Windows 安装包
   # - 上传 artifacts
   ```

3. **性能测试**:
   - "如何测量应用冷启动时间?"
   - "如何检测内存泄漏?"

---

## 3. 团队协作与工作流

### 3.1 沟通工具

| 工具 | 用途 | 推荐方案 |
| :--- | :--- | :--- |
| **即时通讯** | 日常沟通、快速问答 | Slack / Discord / 飞书 |
| **视频会议** | 每日站会、评审会议 | Zoom / Google Meet / 腾讯会议 |
| **文档协作** | 需求文档、技术规范 | Notion / Confluence / 语雀 |
| **任务管理** | Sprint 规划、进度跟踪 | Jira / Linear / GitHub Projects |
| **设计协作** | UI/UX 评审 | Figma / Sketch |

---

### 3.2 Git 工作流

**分支策略** (Git Flow 简化版):

```
main (生产分支)
  ↑
  └── release/v0.9.0 (发布分支)
        ↑
        └── develop (开发分支)
              ↑
              ├── feature/chat-interface
              ├── feature/voice-recording
              ├── feature/vocabulary-list
              └── bugfix/tts-fallback
```

**Commit 规范** (Conventional Commits):

```
feat: add voice recording button
fix: handle microphone permission denial
docs: update API specification
test: add E2E test for scenario selection
refactor: simplify prompt builder logic
```

**Code Review 流程**:

1. 开发者创建 PR (Pull Request)
2. 至少 1 人审查 (Tech Lead 必须审查核心模块)
3. CI 测试全部通过
4. 审查通过后合并到 develop

---

### 3.3 Sprint 规划

**8周 MVP 开发计划**:

| Sprint | 周期 | 目标 | 关键交付物 |
| :--- | :--- | :--- | :--- |
| **Sprint 1** | Week 1-2 | 基础设施 | Tauri 项目框架、Ollama 集成、基础对话 |
| **Sprint 2** | Week 3-4 | 语音功能 | STT/TTS 集成、完整语音对话流 |
| **Sprint 3** | Week 5-6 | 教学功能 | 情景模式、单词教学、生词本 |
| **Sprint 4** | Week 7-8 | 打磨发布 | 性能优化、多平台打包、测试验收 |

**每日站会** (15分钟):

```
每人回答:
1. 昨天完成了什么?
2. 今天计划做什么?
3. 遇到什么阻碍?
```

**每周评审** (1小时):

- 演示本周完成功能
- 收集反馈
- 调整下周计划

---

## 4. 招聘建议

### 4.1 招聘渠道

| 渠道 | 适用角色 | 成本 | 周期 |
| :--- | :--- | :--- | :--- |
| **LinkedIn** | Tech Lead、Frontend | 中 | 4-8周 |
| **GitHub Jobs** | Rust 开发、Full-Stack | 低 | 4-6周 |
| **Reddit (r/rust)** | Rust 专家 | 低 | 2-4周 |
| **Upwork/Freelancer** | QA/DevOps (兼职) | 低 | 1-2周 |
| **内推** | 所有角色 | 低 | 2-4周 |
| **高校合作** | AI/Prompt Engineer | 中 | 4-8周 |

---

### 4.2 薪资参考 (2026年市场水平)

**中国大陆**:

| 角色 | 月薪 (人民币) | 备注 |
| :--- | :--- | :--- |
| Tech Lead (Rust) | ¥35,000 - ¥50,000 | 5年以上经验 |
| Frontend Developer | ¥25,000 - ¥35,000 | 3年以上经验 |
| AI/Prompt Engineer | ¥30,000 - ¥45,000 | 需语言学背景 |
| QA/DevOps (兼职) | ¥15,000 - ¥25,000 | 50% 时间投入 |

**远程/国际**:

| 角色 | 月薪 (美元) | 备注 |
| :--- | :--- | :--- |
| Tech Lead | $6,000 - $9,000 | 全球远程 |
| Frontend Developer | $4,000 - $6,000 | 全球远程 |
| AI Engineer | $5,000 - $8,000 | 需英语流利 |
| QA/DevOps | $2,000 - $4,000 | 兼职 |

---

### 4.3 面试流程

**标准流程** (4轮):

1. **初筛** (30分钟)
   - HR 电话面试
   - 了解候选人背景、薪资期望
   - 确认基本资格

2. **技术笔试** (2-4小时)
   - 在线编程测试 (HackerRank、LeetCode)
   - 或实际任务 (如: 实现一个简单的 Tauri Command)

3. **技术面试** (60-90分钟)
   - 深入技术问题
   - 系统设计讨论
   - 代码审查练习

4. **文化契合度面试** (30-45分钟)
   - 团队协作风格
   - 工作态度与价值观
   - 提问环节

---

## 5. 培训计划

### 5.1 入职培训 (第1周)

**Day 1-2: 项目介绍**
- 产品愿景与核心价值
- 技术架构概览
- 文档阅读 (需求文档、技术方案)

**Day 3-4: 环境搭建**
- 开发环境配置 (Rust、Node.js、Tauri)
- 运行第一个 Demo
- Git 工作流培训

**Day 5: 任务分配**
- 认领第一个小任务 (bug fix 或 small feature)
- 配对编程 (Pair Programming)
- 答疑环节

---

### 5.2 持续学习

**每周技术分享** (1小时):

- 轮流主讲
- 主题示例:
  - "Rust 异步编程最佳实践"
  - "Tauri vs Electron 性能对比"
  - "Prompt Engineering 技巧"
  - "React 性能优化案例"

**外部资源**:

- Rust: The Book (doc.rust-lang.org/book)
- Tauri: 官方文档 (tauri.app)
- React: 官方教程 (react.dev)
- AI: Coursera "AI For Everyone" (Andrew Ng)

---

## 6. 风险管理

### 6.1 人员风险

| 风险 | 概率 | 影响 | 缓解措施 |
| :--- | :--- | :--- | :--- |
| **核心成员离职** | 中 | 高 | 交叉培训、文档完善、代码审查 |
| **技能不匹配** | 低 | 中 | 严格面试、试用期评估 |
| **沟通障碍** (远程) | 中 | 中 | 定期视频会、清晰文档、异步沟通规范 |
| **工作量过载** | 高 | 中 | 合理 Sprint 规划、优先级排序 |

---

### 6.2 应对措施

**知识共享**:
- 所有代码必须有注释和文档
- 关键技术决策记录在 ADR (Architecture Decision Records)
- 每周代码审查会议

**备份计划**:
- 关键岗位至少有 2 人了解核心代码
- 外包备选 (Upwork 紧急支援)
- 延长 Sprint 缓冲期 (1周)

---

## 7. 附录

### 7.1 技能自评表

候选人可使用此表自评 (1-5分):

```markdown
## Rust 技能
- [ ] 1: 听说过 Rust
- [ ] 2: 写过 Hello World
- [ ] 3: 完成过小项目
- [ ] 4: 有生产环境经验
- [ ] 5: 精通异步、宏、unsafe

## Tauri 技能
- [ ] 1: 听说过 Tauri
- [ ] 2: 跑过 Demo
- [ ] 3: 开发过简单应用
- [ ] 4: 理解插件系统
- [ ] 5: 贡献过源码

## React 技能
- [ ] 1: 了解 JSX
- [ ] 2: 写过 Todo List
- [ ] 3: 有商业项目经验
- [ ] 4: 精通 Hooks、性能优化
- [ ] 5: 写过自定义 Hooks 库

## Prompt Engineering
- [ ] 1: 用过 ChatGPT
- [ ] 2: 写过简单 Prompt
- [ ] 3: 系统化设计过 Prompt
- [ ] 4: 有 A/B 测试经验
- [ ] 5: 发表过相关论文/文章
```

---

### 7.2 推荐学习资源

**Rust**:
- 📚 《The Rust Programming Language》(免费电子书)
- 🎥 Rust Official YouTube Channel
- 💻 Rustlings (交互式练习)

**Tauri**:
- 📖 官方文档 (tauri.app)
- 🎯 Awesome Tauri (GitHub)
- 💬 Discord 社区

**React**:
- 📚 《React 官方文档》(react.dev)
- 🎥 Epic React (Kent C. Dodds)
- 💻 Build a React App (freeCodeCamp)

**AI/Prompt**:
- 📖 Prompt Engineering Guide (promptengineering.org)
- 🎯 OpenAI Cookbook
- 📚 《Language Models are Few-Shot Learners》(论文)

---

## 8. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义团队结构与技能要求 | LingoMate Team |

---

**文档结束**
