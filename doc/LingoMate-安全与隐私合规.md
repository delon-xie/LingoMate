# LingoMate 安全与隐私合规文档

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | 安全团队 |

---

## 1. 隐私设计原则

### 1.1 Privacy by Design

LingoMate 遵循以下隐私保护原则:

1. **数据最小化**: 仅收集实现功能所必需的最少数据
2. **本地优先**: 所有用户数据存储在本地,不上传云端
3. **用户控制**: 用户可随时查看、导出、删除自己的数据
4. **透明性**: 清晰告知用户数据收集和使用方式
5. **安全性**: 采用行业标准保护用户数据安全

---

## 2. 数据收集清单

### 2.1 收集的数据类型

| 数据类型 | 用途 | 存储位置 | 是否必需 | 用户可控 |
| :--- | :--- | :--- | :--- | :--- |
| **对话记录** | 提供连续对话体验 | 本地 SQLite | 是 | ✅ 可删除 |
| **生词本** | 智能复习功能 | 本地 SQLite | 否 | ✅ 可清空 |
| **用户设置** | 个性化配置 | 本地 SQLite | 是 | ✅ 可修改 |
| **麦克风音频** | 语音识别 (实时处理,不存储) | 不存储 | 是 | ✅ 可禁用 |
| **使用统计** | 产品改进 (匿名,可选) | 本地 | 否 | ✅ 可选择退出 |

### 2.2 不收集的数据

❌ **明确不收集**:
- 用户真实身份 (姓名、邮箱、电话)
- 设备唯一标识符 (MAC 地址、IMEI)
- 位置信息
- 联系人列表
- 浏览历史
- 第三方应用数据

---

## 3. 权限管理

### 3.1 所需系统权限

| 权限 | 用途 | 何时请求 | 拒绝后果 |
| :--- | :--- | :--- | :--- |
| **麦克风** | 语音输入 (STT) | 首次点击录音按钮时 | 无法使用语音功能,仍可用文字输入 |
| **文件系统** | 保存对话记录和设置 | 应用启动时自动授予 | 应用无法运行 |
| **网络访问** | Edge TTS、模型下载 | 需要时自动使用 | 降级到离线功能 |

---

### 3.2 权限请求实现

**macOS** (`Info.plist`):

```xml
<key>NSMicrophoneUsageDescription</key>
<string>LingoMate needs microphone access to convert your speech to text for English practice.</string>
```

**Windows** (Package.appxmanifest):

```xml
<Capabilities>
  <DeviceCapability Name="microphone" />
</Capabilities>
```

**Linux** (Flatpak manifest):

```json
{
  "finish-args": [
    "--socket=pulseaudio",
    "--device=all"
  ]
}
```

---

### 3.3 权限被拒绝的处理

**前端实现**:

```typescript
async function requestMicrophonePermission(): Promise<boolean> {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    stream.getTracks().forEach(track => track.stop()); // 立即释放
    return true;
  } catch (error) {
    if (error.name === 'NotAllowedError') {
      showPermissionDeniedDialog();
      return false;
    }
    throw error;
  }
}

function showPermissionDeniedDialog() {
  showDialog({
    title: 'Microphone Access Denied',
    message: 'LingoMate needs microphone access for voice input.\n\n' +
             'Please enable it in System Settings:',
    type: 'warning',
    buttons: [
      {
        label: 'Open Settings',
        action: () => openSystemSettings('microphone'),
      },
      {
        label: 'Use Text Input',
        action: () => switchToTextInputMode(),
      },
    ],
  });
}
```

---

## 4. 数据安全

### 4.1 本地数据加密 (MVP暂不实现,规划中)

**未来方案**: 使用 SQLCipher 加密数据库

```toml
# Cargo.toml
[dependencies]
rusqlite = { version = "0.29", features = ["bundled-sqlcipher"] }
```

```rust
use rusqlite::Connection;

fn open_encrypted_db(db_path: &str, password: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(db_path)?;
    
    // 设置加密密钥
    conn.execute_batch(&format!("PRAGMA key = '{}';", password))?;
    
    // 验证密钥
    let result: Result<i64, _> = conn.query_row(
        "SELECT count(*) FROM sqlite_master",
        [],
        |row| row.get(0)
    );
    
    if result.is_err() {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_NOTADB),
            Some("Invalid password".to_string())
        ));
    }
    
    Ok(conn)
}
```

**密钥管理**: 使用操作系统密钥链

```rust
#[cfg(target_os = "macos")]
fn get_encryption_key() -> Result<String, Error> {
    use keychain_services;
    
    let query = keychain_services::ItemQueryOptions::default()
        .label("com.lingomate.db_key")
        .account("lingomate_user");
    
    match keychain_services::find_generic_password(query) {
        Ok(password) => Ok(String::from_utf8_lossy(&password).to_string()),
        Err(_) => {
            // 生成新密钥并存储
            let key = generate_random_key();
            keychain_services::add_generic_password(
                keychain_services::ItemAddOptions::default()
                    .label("com.lingomate.db_key")
                    .account("lingomate_user")
                    .password(key.as_bytes())
            )?;
            Ok(key)
        }
    }
}
```

---

### 4.2 数据传输安全

**Edge TTS HTTPS**:

```rust
use reqwest::Client;

async fn call_edge_tts(text: &str) -> Result<Vec<u8>, Error> {
    let client = Client::builder()
        .https_only(true)  // 强制 HTTPS
        .build()?;
    
    let response = client
        .post("https://speech.platform.bing.com/consumer/speech/synthesize/readaloud")
        .header("Authorization", "Bearer <token>")
        .body(build_ssml_request(text))
        .send()
        .await?;
    
    // ...
}
```

**Ollama 本地通信**:

```rust
// Ollama 仅在 localhost 监听,不暴露到外网
const OLLAMA_URL: &str = "http://127.0.0.1:11434";
```

---

### 4.3 内存安全

**Rust 优势**:
- ✅ 编译时防止空指针解引用
- ✅ 防止数据竞争
- ✅ 自动内存管理 (无 GC)
- ✅ 缓冲区溢出保护

**最佳实践**:

```rust
// ❌ 避免: 未检查的 unwrap
let value = some_option.unwrap(); // 可能 panic

// ✅ 推荐: 错误处理
let value = some_option.ok_or(AppError::Unknown("Value missing".to_string()))?;

// ❌ 避免: 字符串拼接 SQL
let sql = format!("SELECT * FROM users WHERE name = '{}'", user_input);

// ✅ 推荐: 参数化查询
conn.execute("SELECT * FROM users WHERE name = ?", params![user_input])?;
```

---

## 5. 内容安全

### 5.1 AI 输出过滤

**System Prompt 约束**:

```
CONTENT SAFETY RULES:
1. Never generate harmful, illegal, or explicit content
2. If user asks inappropriate questions, politely redirect to educational topics
3. For sensitive topics (politics, religion), remain neutral and factual
4. If user expresses distress, suggest seeking professional help
5. Always maintain a respectful and supportive tone
```

**关键词过滤** (Rust 实现):

```rust
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROFANITY_FILTER: Regex = Regex::new(
        r"(?i)(badword1|badword2|badword3)"
    ).unwrap();
}

fn filter_content(text: &str) -> String {
    PROFANITY_FILTER.replace_all(text, "***").to_string()
}

// 在 AI 回复后过滤
let filtered_response = filter_content(&ai_response);
```

---

### 5.2 用户输入验证

**防止注入攻击**:

```rust
fn validate_user_input(input: &str) -> Result<String, AppError> {
    // 长度限制
    if input.len() > 1000 {
        return Err(AppError::InvalidParameter {
            field: "input".to_string(),
            reason: "Input too long (max 1000 characters)".to_string(),
        });
    }
    
    // 移除危险字符
    let sanitized = input
        .replace("'", "''")  // SQL 转义
        .replace("<", "&lt;") // HTML 转义
        .replace(">", "&gt;");
    
    // 检测潜在注入
    if contains_sql_injection(&sanitized) {
        log_warn!("Potential SQL injection detected");
        return Err(AppError::InvalidParameter {
            field: "input".to_string(),
            reason: "Invalid characters detected".to_string(),
        });
    }
    
    Ok(sanitized)
}

fn contains_sql_injection(input: &str) -> bool {
    let patterns = [
        "' OR '1'='1",
        "; DROP TABLE",
        "UNION SELECT",
        "--",
    ];
    
    patterns.iter().any(|p| input.to_uppercase().contains(p))
}
```

---

## 6. 青少年保护

### 6.1 家长控制功能

**护眼模式**:

```typescript
interface ParentalControls {
  session_time_limit: number;     // 单次使用时长限制 (分钟)
  daily_time_limit: number;       // 每日总时长限制 (分钟)
  break_reminder_interval: number; // 休息提醒间隔 (分钟)
  content_filter_level: 'strict' | 'moderate' | 'off';
}

// 定时器实现
let sessionTimer: NodeJS.Timeout;
let sessionStartTime: Date;

function startSession() {
  sessionStartTime = new Date();
  
  sessionTimer = setInterval(() => {
    const elapsed = (Date.now() - sessionStartTime.getTime()) / 60000; // 分钟
    
    if (elapsed >= settings.session_time_limit) {
      showBreakReminder();
      pauseSession();
    }
  }, 60000); // 每分钟检查
}

function showBreakReminder() {
  showToast({
    type: 'info',
    title: 'Time for a Break!',
    message: 'You have been practicing for 30 minutes. Take a short break to rest your eyes.',
    duration: 10000,
  });
}
```

---

### 6.2 家长概览 (不涉及具体对话内容)

**统计数据 API**:

```rust
#[tauri::command]
async fn get_parental_summary() -> CommandResult<ParentalSummary> {
    let total_sessions = count_sessions_this_week().await?;
    let total_duration = calculate_total_duration_this_week().await?;
    let words_learned = count_new_vocabulary_this_week().await?;
    
    Ok(ParentalSummary {
        total_sessions,
        total_duration_minutes: total_duration,
        words_learned,
        last_session_date: get_last_session_date().await?,
        // 不包含任何对话内容
    })
}
```

**前端展示**:

```tsx
function ParentalDashboard() {
  const summary = useParentalSummary();
  
  return (
    <div className="parental-dashboard">
      <h2>Weekly Learning Summary</h2>
      
      <div className="stats-grid">
        <StatCard 
          icon={<ClockIcon />}
          label="Total Practice Time"
          value={`${summary.total_duration_minutes} min`}
        />
        <StatCard 
          icon={<BookIcon />}
          label="Sessions Completed"
          value={summary.total_sessions}
        />
        <StatCard 
          icon={<StarIcon />}
          label="New Words Learned"
          value={summary.words_learned}
        />
      </div>
      
      <p className="privacy-note">
        Note: Specific conversation content is private and not visible here.
      </p>
    </div>
  );
}
```

---

## 7. 合规性

### 7.1 GDPR (欧盟通用数据保护条例)

**适用性**: LingoMate 面向全球用户,需遵守 GDPR

**合规措施**:

#### 数据主体权利

| 权利 | 实现方式 |
| :--- | :--- |
| **访问权** | 用户可导出所有数据 (JSON/CSV) |
| **更正权** | 用户可修改或删除任何数据 |
| **删除权** ("被遗忘权") | 一键删除所有数据 |
| **可携带权** | 提供标准格式的数据导出 |
| **反对权** | 可选择退出匿名统计 |

#### 数据保护官 (DPO)

```
联系方式: privacy@lingomate.com
响应时间: 30 天内
```

#### 数据处理记录

```rust
// 记录所有数据处理活动
struct DataProcessingLog {
    timestamp: DateTime<Utc>,
    action: String,       // "create", "read", "update", "delete"
    data_type: String,    // "conversation", "vocabulary", "settings"
    user_id: String,      // 匿名用户 ID
    purpose: String,      // 处理目的
}
```

---

### 7.2 COPPA (美国儿童在线隐私保护法)

**适用性**: 如果用户包含 13 岁以下儿童

**合规措施**:

1. **家长同意**: 首次启动时要求家长确认
2. **数据最小化**: 不收集儿童个人信息
3. **家长控制**: 提供完整的家长监控工具
4. **透明度**: 清晰的隐私政策 (儿童友好语言)

**家长同意对话框**:

```tsx
function ParentalConsentDialog() {
  return (
    <Dialog title="Parental Consent Required">
      <p>
        LingoMate is an educational app that may be used by children under 13.
      </p>
      <p>
        We do NOT collect any personal information from children. All data is 
        stored locally on your device.
      </p>
      <p>
        As a parent/guardian, please confirm that you have read our 
        <a href="/privacy-policy">Privacy Policy</a> and consent to your 
        child's use of this app.
      </p>
      
      <div className="dialog-actions">
        <Button variant="secondary" onClick={() => exitApp()}>
          Decline
        </Button>
        <Button variant="primary" onClick={() => grantConsent()}>
          I Consent
        </Button>
      </div>
    </Dialog>
  );
}
```

---

### 7.3 中国网络安全法

**适用性**: 如果在中国大陆分发

**合规措施**:

1. **数据本地化**: 所有数据存储在中国境内服务器 (如有云端功能)
2. **实名认证**: MVP 无需 (纯本地应用)
3. **内容审核**: AI 输出经过过滤
4. **备案**: 应用商店上架时需 ICP 备案

---

## 8. 隐私政策

### 8.1 隐私政策模板

```markdown
# LingoMate Privacy Policy

**Last Updated: May 17, 2026**

## 1. Introduction

LingoMate ("we", "our", or "us") is committed to protecting your privacy. This Privacy Policy explains how we handle your information when you use our desktop application.

## 2. Information We Collect

### 2.1 Information Stored Locally

All data is stored **only on your device** and never transmitted to our servers:

- **Conversation History**: Your chat messages with the AI tutor
- **Vocabulary List**: Words you have learned
- **App Settings**: Your preferences (voice, speed, etc.)

### 2.2 Information We Do NOT Collect

We do **NOT** collect:
- Personal identification (name, email, phone)
- Device identifiers
- Location data
- Usage analytics (unless you opt-in)
- Any data transmitted to third parties

## 3. How We Use Your Information

Your data is used solely to provide the core functionality of LingoMate:
- Maintaining conversation context
- Tracking vocabulary progress
- Personalizing your learning experience

## 4. Data Security

- All data is stored locally on your device
- No cloud synchronization (MVP version)
- Future versions may offer optional encrypted backup

## 5. Your Rights

You have full control over your data:
- **Access**: View all your data within the app
- **Export**: Download your data as JSON/CSV
- **Delete**: Remove any or all data at any time
- **Opt-out**: Disable optional features like usage statistics

## 6. Children's Privacy

LingoMate may be used by children under parental supervision. We do not knowingly collect personal information from children under 13. Parents can monitor usage through the Parental Dashboard.

## 7. Changes to This Policy

We may update this Privacy Policy from time to time. We will notify you of any changes by posting the new policy in the app.

## 8. Contact Us

If you have questions about this Privacy Policy, please contact us at:

- Email: privacy@lingomate.com
- GitHub: github.com/lingomate/lingomate/issues

---

**Summary**: LingoMate is a privacy-first application. Your data stays on your device, and you have complete control over it.
```

---

### 8.2 隐私政策展示

**应用内访问**:

```tsx
function SettingsPage() {
  return (
    <div className="settings">
      {/* ... other settings ... */}
      
      <section className="privacy-section">
        <h3>Privacy</h3>
        <ul>
          <li>
            <Button variant="ghost" onClick={() => openPrivacyPolicy()}>
              View Privacy Policy
            </Button>
          </li>
          <li>
            <Button variant="ghost" onClick={() => exportAllData()}>
              Export My Data
            </Button>
          </li>
          <li>
            <Button variant="ghost" className="text-danger" onClick={() => deleteAllData()}>
              Delete All Data
            </Button>
          </li>
        </ul>
      </section>
    </div>
  );
}
```

---

## 9. 安全审计清单

### 9.1 发布前安全检查

#### 代码安全

- [ ] 所有用户输入经过验证和清理
- [ ] 使用参数化查询,无 SQL 注入风险
- [ ] 敏感信息未硬编码在代码中
- [ ] 依赖库无已知高危漏洞 (`cargo audit`, `npm audit`)
- [ ] 错误消息不泄露系统细节

#### 数据安全

- [ ] 本地数据库文件权限正确 (仅当前用户可读)
- [ ] 临时文件使用后删除
- [ ] 日志文件不包含敏感信息
- [ ] 数据导出功能需要用户确认

#### 网络安全

- [ ] 所有外部请求使用 HTTPS
- [ ] 证书验证启用 (无 `verify_ssl=false`)
- [ ] 超时设置合理 (防止 DoS)
- [ ] 本地服务仅绑定 localhost

#### 权限安全

- [ ] 仅请求必需的权限
- [ ] 权限被拒绝时优雅降级
- [ ] 权限使用说明清晰

---

### 9.2 依赖漏洞扫描

**Rust**:

```bash
# 安装 cargo-audit
cargo install cargo-audit

# 扫描依赖漏洞
cargo audit

# 输出示例:
# Scanning Cargo.lock for vulnerabilities...
# No vulnerabilities found!
```

**Node.js**:

```bash
# 扫描 npm 依赖
npm audit

# 自动修复
npm audit fix

# 输出示例:
# found 0 vulnerabilities
```

**定期执行**: 每周 CI/CD 流水线中自动扫描

---

## 10. 事件响应

### 10.1 安全事件分类

| 级别 | 类型 | 响应时限 | 通知对象 |
| :--- | :--- | :--- | :--- |
| **Critical** | 数据泄露、远程代码执行 | 24 小时内 | 用户 + 监管机构 |
| **High** | 权限提升、SQL 注入 | 48 小时内 | 受影响用户 |
| **Medium** | XSS、CSRF | 7 天内 | 内部团队 |
| **Low** | 信息泄露 (非敏感) | 30 天内 | 内部团队 |

---

### 10.2 应急响应流程

```mermaid
graph TD
    A[发现安全事件] --> B{评估严重程度}
    B -->|Critical/High| C[立即隔离受影响系统]
    B -->|Medium/Low| D[记录并计划修复]
    C --> E[调查根本原因]
    E --> F[制定修复方案]
    F --> G[部署补丁]
    G --> H[通知受影响用户]
    H --> I[提交监管报告 (如需要)]
    I --> J[事后复盘]
    J --> K[更新安全策略]
    D --> L[定期审查]
```

---

### 10.3 用户通知模板

**数据泄露通知邮件**:

```
Subject: Important Security Notice - LingoMate

Dear LingoMate User,

We are writing to inform you of a security incident that may have affected your data.

What Happened:
On [DATE], we discovered [BRIEF DESCRIPTION]. Our investigation found that [DETAILS].

What Information Was Involved:
[AFFECTED DATA TYPES]

What We Are Doing:
- [ACTION 1]
- [ACTION 2]
- [ACTION 3]

What You Can Do:
- [RECOMMENDATION 1]
- [RECOMMENDATION 2]

For More Information:
Visit [LINK] or contact us at security@lingomate.com.

We sincerely apologize for this incident and are committed to protecting your privacy.

Sincerely,
The LingoMate Team
```

---

## 11. 附录

### 11.1 参考法规

- **GDPR**: Regulation (EU) 2016/679
- **COPPA**: 16 CFR Part 312
- **CCPA**: California Consumer Privacy Act
- **PIPL**: 中华人民共和国个人信息保护法
- **Cybersecurity Law**: 中华人民共和国网络安全法

### 11.2 安全资源

- **OWASP Top 10**: owasp.org/www-project-top-ten
- **Rust Security Advisory**: rustsec.org
- **Node Security Platform**: nodesecurity.io
- **CVE Database**: cve.mitre.org

---

## 12. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义安全与隐私规范 | LingoMate Team |

---

**文档结束**
