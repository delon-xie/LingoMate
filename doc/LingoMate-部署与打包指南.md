# LingoMate 部署与打包指南

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.9+ |
| **维护者** | DevOps 团队 |

---

## 1. 构建环境准备

### 1.1 前置依赖

#### 所有平台通用

```bash
# Node.js (v18+)
node --version  # 应显示 v18.x.x 或更高

# Rust (最新稳定版)
rustc --version  # 应显示 rustc 1.x.x

# Tauri CLI
cargo install tauri-cli --version "^2.0.0"
```

#### Windows 额外依赖

```powershell
# Visual Studio Build Tools 2019 或 2022
# 安装时勾选 "Desktop development with C++"

# WebView2 Runtime (Windows 10/11 通常已预装)
```

#### macOS 额外依赖

```bash
# Xcode Command Line Tools
xcode-select --install

# 需要 Apple Developer Account (用于代码签名和公证)
```

#### Linux 额外依赖

```bash
# Ubuntu/Debian
sudo apt-get install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

---

## 2. 项目结构

### 2.1 完整目录结构

```
lingomate/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   ├── services/
│   │   └── models/
│   ├── Cargo.toml
│   ├── tauri.conf.json     # Tauri 配置
│   └── icons/              # 应用图标 (多尺寸)
├── src/                    # React 前端
│   ├── components/
│   ├── pages/
│   ├── hooks/
│   └── styles/
├── public/                 # 静态资源
├── resources/              # 打包资源
│   ├── ollama/             # Ollama 二进制文件
│   │   ├── windows/ollama.exe
│   │   ├── macos/ollama
│   │   └── linux/ollama
│   └── models/             # 初始模型 (可选)
├── package.json
├── vite.config.ts
└── README.md
```

---

## 3. Tauri 配置

### 3.1 `src-tauri/tauri.conf.json`

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "LingoMate",
    "version": "0.9.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.lingomate.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "resources": [
        "resources/ollama/*",
        "resources/models/*"
      ],
      "externalBin": [
        "binaries/ollama"
      ],
      "copyright": "© 2026 LingoMate Team",
      "category": "Education",
      "shortDescription": "AI English Tutor",
      "longDescription": "Your personal AI English teacher, available anytime.",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "webviewInstallMode": {
          "type": "embedBootstrapper"
        }
      },
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "linux": {
        "deb": {
          "depends": []
        }
      }
    },
    "security": {
      "csp": "default-src 'self'; img-src 'self' data:; media-src 'self'"
    },
    "updater": {
      "active": false,
      "endpoints": [
        "https://releases.lingomate.com/{{target}}/{{arch}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": ""
    },
    "allowlist": {
      "all": false,
      "shell": {
        "all": true,
        "execute": true,
        "sidecar": true,
        "scope": [
          {
            "name": "ollama",
            "sidecar": true,
            "args": true
          }
        ]
      },
      "fs": {
        "all": true,
        "scope": ["$APPDATA/**", "$HOME/.lingomate/**"]
      },
      "dialog": {
        "all": true
      },
      "notification": {
        "all": true
      }
    }
  }
}
```

---

## 4. Ollama 集成

### 4.1 下载 Ollama 二进制文件

**官方下载地址**: https://ollama.com/download

```bash
# Windows
curl -L https://ollama.com/download/OllamaSetup.exe -o resources/ollama/windows/ollama.exe

# macOS
curl -L https://ollama.com/download/Ollama-darwin.zip -o resources/ollama/macos/ollama.zip
unzip resources/ollama/macos/ollama.zip -d resources/ollama/macos/

# Linux
curl -L https://ollama.com/download/ollama-linux-amd64 -o resources/ollama/linux/ollama
chmod +x resources/ollama/linux/ollama
```

### 4.2 首次启动模型下载流程

**Rust 实现** (`src-tauri/src/services/model_manager.rs`):

```rust
use std::process::Command;
use tokio::fs;

pub async fn ensure_model_exists(model_name: &str) -> Result<(), Error> {
    let model_path = get_model_path(model_name);
    
    if !model_path.exists() {
        // 触发下载
        download_model(model_name).await?;
    }
    
    Ok(())
}

async fn download_model(model_name: &str) -> Result<(), Error> {
    let output = Command::new("ollama")
        .args(&["pull", model_name])
        .output()?;
    
    if !output.status.success() {
        return Err(Error::ModelDownloadFailed(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    Ok(())
}
```

**前端进度监听**:

```typescript
// 监听模型下载进度
listen('model_download_progress', (event) => {
  const { downloaded_mb, total_mb, percentage } = event.payload;
  
  updateProgressBar({
    current: downloaded_mb,
    total: total_mb,
    percentage: percentage
  });
  
  setStatusText(`Downloading: ${Math.round(percentage)}%`);
});
```

---

## 5. 构建命令

### 5.1 开发模式

```bash
# 启动开发服务器 (热重载)
npm run tauri dev

# 等价于
npm run dev         # 启动 Vite 前端
tauri dev           # 启动 Tauri 后端
```

### 5.2 生产构建

```bash
# 构建所有平台
npm run tauri build

# 仅构建特定平台
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### 5.3 构建输出位置

```
src-tauri/target/release/bundle/
├── nsis/                          # Windows NSIS 安装包
│   └── LingoMate_0.9.0_x64-setup.exe
├── dmg/                           # macOS DMG
│   └── LingoMate_0.9.0_x64.dmg
├── appimage/                      # Linux AppImage
│   └── LingoMate_0.9.0_amd64.AppImage
└── deb/                           # Linux DEB
    └── lingomate_0.9.0_amd64.deb
```

---

## 6. 各平台打包详解

### 6.1 Windows

#### NSIS 安装包配置

**`src-tauri/tauri.windows.conf.json`**:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com",
        "webviewInstallMode": {
          "type": "embedBootstrapper",
          "silent": true
        },
        "nsis": {
          "displayLanguageSelector": true,
          "installerIcon": "icons/icon.ico",
          "headerImage": "assets/nsis-header.bmp",
          "sidebarImage": "assets/nsis-sidebar.bmp",
          "license": "LICENSE.txt",
          "languages": ["English", "SimpChinese"]
        }
      }
    }
  }
}
```

#### 代码签名 (可选但推荐)

```powershell
# 需要购买代码签名证书 (如 DigiCert, Sectigo)

# 使用 signtool 签名
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 `
  /a LingoMate_0.9.0_x64-setup.exe
```

---

### 6.2 macOS

#### DMG 打包配置

**`src-tauri/tauri.macos.conf.json`**:

```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "frameworks": [],
        "minimumSystemVersion": "10.15",
        "exceptionDomain": "",
        "entitlements": "entitlements.plist",
        "providerShortName": "YOUR_TEAM_ID",
        "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

#### 代码签名与公证

```bash
# 1. 导出签名证书 (从 Keychain Access)
# 需要 Apple Developer Program 会员资格 ($99/年)

# 2. 签名应用
codesign --force --deep --sign "Developer ID Application: Your Name (TEAM_ID)" \
  src-tauri/target/release/bundle/macos/LingoMate.app

# 3. 创建 DMG
create-dmg --volname "LingoMate" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "LingoMate.app" 200 190 \
  --hide-extension "LingoMate.app" \
  --app-drop-link 600 185 \
  "LingoMate_0.9.0.dmg" \
  "src-tauri/target/release/bundle/macos/"

# 4. 公证 (Notarization)
xcrun notarytool submit "LingoMate_0.9.0.dmg" \
  --apple-id "your@apple.id" \
  --password "@keychain:AC_PASSWORD" \
  --team-id "YOUR_TEAM_ID" \
  --wait

# 5. 贴票 (Staple)
xcrun stapler staple "LingoMate_0.9.0.dmg"
```

**环境变量设置**:

```bash
# ~/.zshrc 或 ~/.bash_profile
export APPLE_ID="your@apple.id"
export AC_PASSWORD="@keychain:AC_PASSWORD"  # 存储在 Keychain
export TEAM_ID="YOUR_TEAM_ID"
```

---

### 6.3 Linux

#### AppImage 配置

**`src-tauri/tauri.linux.conf.json`**:

```json
{
  "tauri": {
    "bundle": {
      "linux": {
        "deb": {
          "depends": [
            "libwebkit2gtk-4.0-37",
            "libgtk-3-0",
            "libayatana-appindicator3-1"
          ],
          "files": {
            "/usr/share/doc/lingomate/README.md": "README.md"
          }
        },
        "rpm": {
          "depends": []
        },
        "appimage": {
          "bundleMediaFramework": true
        }
      }
    }
  }
}
```

#### 构建 AppImage

```bash
# 安装 appimagetool
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage
sudo mv appimagetool-x86_64.AppImage /usr/local/bin/appimagetool

# 构建
npm run tauri build -- --target x86_64-unknown-linux-gnu

# 输出: src-tauri/target/release/bundle/appimage/LingoMate_0.9.0_amd64.AppImage
```

---

## 7. CI/CD 自动化构建

### 7.1 GitHub Actions 配置

**`.github/workflows/build.yml`**:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            target: 'x86_64-apple-darwin'
          - platform: 'ubuntu-20.04'
            target: 'x86_64-unknown-linux-gnu'
          - platform: 'windows-latest'
            target: 'x86_64-pc-windows-msvc'

    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      
      - name: Install frontend dependencies
        run: npm ci
      
      - name: Build frontend
        run: npm run build
      
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ENABLE_CODE_SIGNING: ${{ matrix.platform == 'macos-latest' }}
          APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
          APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
          APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
          APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
        with:
          tagName: v__VERSION__
          releaseName: "LingoMate v__VERSION__"
          releaseBody: "See the assets to download this version and install."
          releaseDraft: true
          prerelease: false
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}-build
          path: src-tauri/target/release/bundle/
```

### 7.2 密钥管理

**GitHub Secrets 设置**:

```
Settings → Secrets and variables → Actions → New repository secret

APPLE_CERTIFICATE: Base64 编码的 .p12 证书文件
APPLE_CERTIFICATE_PASSWORD: 证书密码
APPLE_SIGNING_IDENTITY: "Developer ID Application: Your Name (TEAM_ID)"
APPLE_ID: your@apple.id
APPLE_PASSWORD: App-Specific Password (从 appleid.apple.com 生成)
APPLE_TEAM_ID: YOUR_TEAM_ID
```

**生成 Apple Certificate**:

```bash
# 1. 从 Keychain 导出证书
# Keychain Access → Login → Certificates → 右键 "Developer ID Application" → Export

# 2. 转换为 Base64
base64 -i DeveloperID.p12 -o certificate_base64.txt

# 3. 复制内容到 GitHub Secret
```

---

## 8. 自动更新机制

### 8.1 Tauri Updater 配置

**启用更新器** (`tauri.conf.json`):

```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.lingomate.com/{{target}}/{{arch}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6 ...\n"
    }
  }
}
```

### 8.2 发布服务器配置

**`update.json` 示例**:

```json
{
  "version": "v0.9.0",
  "notes": "- Initial MVP release\n- 6 scenario dialogs\n- Voice conversation support",
  "pub_date": "2026-05-17T10:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "Content-Length: 12345678\n...",
      "url": "https://releases.lingomate.com/darwin/x86_64/0.9.0/LingoMate_0.9.0.dmg"
    },
    "windows-x86_64": {
      "signature": "...",
      "url": "https://releases.lingomate.com/windows/x86_64/0.9.0/LingoMate_0.9.0-setup.exe"
    },
    "linux-x86_64": {
      "signature": "...",
      "url": "https://releases.lingomate.com/linux/x86_64/0.9.0/LingoMate_0.9.0.AppImage"
    }
  }
}
```

### 8.3 前端检查更新

```typescript
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';

async function checkForUpdates() {
  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    
    if (shouldUpdate) {
      const confirmed = await confirm(
        `New version ${manifest.version} is available. Update now?`,
        { title: 'Update Available' }
      );
      
      if (confirmed) {
        await installUpdate();
        // 应用会自动重启
      }
    }
  } catch (error) {
    console.error('Update check failed:', error);
  }
}
```

---

## 9. 分发渠道

### 9.1 官方网站

**下载页面设计**:

```html
<!-- downloads.html -->
<section class="downloads">
  <h2>Download LingoMate</h2>
  
  <div class="platform-cards">
    <div class="card">
      <img src="/icons/windows.svg" alt="Windows" />
      <h3>Windows</h3>
      <p>Windows 10/11 (64-bit)</p>
      <a href="/download/windows" class="btn-primary">Download .exe</a>
      <small>Size: ~150 MB</small>
    </div>
    
    <div class="card">
      <img src="/icons/macos.svg" alt="macOS" />
      <h3>macOS</h3>
      <p>macOS 10.15+ (Intel & Apple Silicon)</p>
      <a href="/download/macos" class="btn-primary">Download .dmg</a>
      <small>Size: ~180 MB</small>
    </div>
    
    <div class="card">
      <img src="/icons/linux.svg" alt="Linux" />
      <h3>Linux</h3>
      <p>Ubuntu 20.04+ / Debian</p>
      <a href="/download/linux" class="btn-primary">Download .AppImage</a>
      <small>Size: ~160 MB</small>
    </div>
  </div>
  
  <div class="system-requirements">
    <h3>System Requirements</h3>
    <ul>
      <li>Memory: 8 GB RAM minimum</li>
      <li>Storage: 5 GB free space</li>
      <li>Internet: Required for initial setup</li>
    </ul>
  </div>
</section>
```

### 9.2 软件分发平台

**推荐平台**:

| 平台 | 类型 | 费用 | 链接 |
| :--- | :--- | :--- | :--- |
| **GitHub Releases** | 免费 | $0 | github.com/lingomate/lingomate/releases |
| **Softpedia** | 软件下载站 | 免费 | softpedia.com |
| **MajorGeeks** | 软件下载站 | 免费 | majorgeeks.com |
| **MacUpdate** | macOS 专用 | 免费 | macupdate.com |
| **AlternativeTo** | 软件发现 | 免费 | alternativeto.net |

---

## 10. 安装后验证

### 10.1 首次启动检查清单

```typescript
// 应用启动时执行
async function onFirstLaunch() {
  const checks = [
    { name: 'Ollama binary exists', check: checkOllamaBinary },
    { name: 'Model downloaded', check: checkModelExists },
    { name: 'Database initialized', check: checkDatabase },
    { name: 'Permissions granted', check: checkPermissions },
  ];
  
  for (const check of checks) {
    const result = await check.check();
    if (!result.ok) {
      showSetupWizard(check.name, result.error);
      return;
    }
  }
  
  // 所有检查通过,进入主界面
  navigateTo('/scenarios');
}
```

### 10.2 健康检查 API

**Rust 实现** (`src-tauri/src/commands/system.rs`):

```rust
#[tauri::command]
async fn check_system_health() -> Result<SystemHealth, String> {
    let ollama_running = is_ollama_running().await;
    let model_loaded = is_model_loaded("qwen2.5:3b").await;
    let memory_usage = get_memory_usage();
    
    Ok(SystemHealth {
        ollama_running,
        model_loaded,
        memory_usage_mb: memory_usage,
        disk_space_gb: get_available_disk_space(),
    })
}
```

---

## 11. 故障排查

### 11.1 常见问题

#### 问题 1: Windows 安装失败

**症状**: NSIS 安装程序报错 "WebView2 Runtime not found"

**解决方案**:
```powershell
# 手动安装 WebView2 Runtime
winget install Microsoft.EdgeWebView2Runtime

# 或在安装包中嵌入 Bootstrapper (已在 tauri.conf.json 配置)
```

#### 问题 2: macOS Gatekeeper 阻止运行

**症状**: "LingoMate cannot be opened because the developer cannot be verified"

**解决方案**:
```bash
# 方法 1: 系统偏好设置中允许
# System Preferences → Security & Privacy → General → Allow Anyway

# 方法 2: 命令行移除隔离属性
xattr -d com.apple.quarantine /Applications/LingoMate.app

# 方法 3: 正确签名和公证 (推荐)
# 参考第 6.2 节
```

#### 问题 3: Linux AppImage 无法运行

**症状**: `./LingoMate.AppImage: permission denied`

**解决方案**:
```bash
chmod +x LingoMate_0.9.0_amd64.AppImage
./LingoMate_0.9.0_amd64.AppImage
```

#### 问题 4: Ollama 启动失败

**症状**: 应用卡在 "Starting AI service..."

**排查步骤**:
```bash
# 1. 检查 Ollama 日志
tail -f ~/.ollama/logs/server.log

# 2. 手动测试 Ollama
ollama run qwen2.5:3b

# 3. 检查端口占用
lsof -i :11434

# 4. 重启 Ollama 服务
ollama serve
```

---

### 11.2 日志收集

**用户反馈时需要的信息**:

```markdown
## 系统信息
- 操作系统: [Windows 11 / macOS 13 / Ubuntu 22.04]
- 应用版本: [v0.9.0]
- 硬件配置: [CPU, RAM, GPU]

## 日志文件位置
Windows: %APPDATA%\com.lingomate.app\logs\latest.log
macOS:   ~/Library/Logs/com.lingomate.app/latest.log
Linux:   ~/.local/share/com.lingomate.app/logs/latest.log

## 复现步骤
1. ...
2. ...
3. ...

## 截图
[附加截图]
```

---

## 12. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,定义完整部署流程 | LingoMate Team |

---

**文档结束**
