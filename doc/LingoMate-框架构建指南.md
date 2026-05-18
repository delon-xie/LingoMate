# LingoMate 框架构建指南

## 文档状态

| 项目 | 内容 |
| :--- | :--- |
| **文档版本** | 1.0 |
| **创建日期** | 2026-05-17 |
| **适用版本** | MVP v0.1+ |
| **维护者** | LingoMate Team |

---

## 1. 概述

本文档记录 LingoMate 项目的完整框架构建过程,包括技术选型、环境搭建、问题排查和解决方案。旨在为后续开发提供参考,并帮助新成员快速理解项目结构。

---

## 2. 技术栈选择

### 2.1 核心技术

| 层级 | 技术 | 版本 | 选择理由 |
| :--- | :--- | :--- | :--- |
| **前端框架** | React + TypeScript | 18.x | 组件化开发,类型安全,生态丰富 |
| **构建工具** | Vite | 5.x | 极速开发体验,热更新快 |
| **样式方案** | Tailwind CSS | 3.x | 实用优先,快速原型开发 |
| **桌面框架** | Tauri | 2.0 | 轻量级,安全性高,Rust 后端 |
| **后端语言** | Rust | 1.75+ | 内存安全,高性能 |
| **数据库** | SQLite | 3.x | 嵌入式,零配置,单文件 |
| **AI 服务** | Ollama | latest | 本地运行,隐私保护 |

### 2.2 辅助库

| 用途 | 库名 | 说明 |
| :--- | :--- | :--- |
| 图标 | lucide-react | 现代 SVG 图标库 |
| 工具函数 | clsx, tailwind-merge | CSS 类名合并 |
| HTTP 客户端 | reqwest | Rust HTTP 库 |
| JSON 序列化 | serde, serde_json | Rust 序列化框架 |
| 异步运行时 | tokio | Rust 异步运行时 |
| 数据库驱动 | rusqlite | SQLite Rust 绑定 |

---

## 3. 环境准备

### 3.1 系统要求

- **操作系统**: macOS 10.15+, Windows 10+, Linux (Ubuntu 20.04+)
- **内存**: 最低 8GB,推荐 16GB
- **磁盘空间**: 至少 10GB 可用空间

### 3.2 必需软件

#### Node.js 安装

```bash
# macOS (Homebrew)
brew install node

# 验证安装
node --version    # 应输出 v18.0.0 或更高
npm --version     # 应输出 9.0.0 或更高
```

#### Rust 安装

```bash
# 使用 rustup 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装后重启终端或执行
source "$HOME/.cargo/env"

# 验证安装
rustc --version   # 应输出 rustc 1.75.0 或更高
cargo --version   # 应输出 cargo 1.75.0 或更高
```

#### Ollama 安装

```bash
# macOS
brew install ollama

# 启动服务
ollama serve &

# 下载模型
ollama pull qwen2.5:3b

# 验证
ollama run qwen2.5:3b "Hello"
```

---

## 4. 项目初始化步骤

### 4.1 创建前端项目

```bash
# 在项目根目录执行
npm create vite@latest frontend -- --template react-ts

# 进入前端目录并安装依赖
cd frontend
npm install

# 安装 Tauri 相关依赖
npm install @tauri-apps/api @tauri-apps/plugin-shell
npm install -D @tauri-apps/cli

# 安装 UI 相关依赖
npm install lucide-react clsx tailwind-merge
npm install -D tailwindcss @tailwindcss/vite @types/node
```

### 4.2 配置 Tailwind CSS

编辑 `frontend/vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
})
```

在 `frontend/src/index.css` 顶部添加:

```css
@import "tailwindcss";
```

### 4.3 初始化 Rust 后端

```bash
# 在项目根目录执行
cargo init src-tauri --name lingomate --lib
```

### 4.4 创建 Tauri 配置

创建 `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "LingoMate",
  "version": "0.1.0",
  "identifier": "com.lingomate.app",
  "build": {
    "beforeDevCommand": "npm run dev --prefix ../frontend",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build --prefix ../frontend",
    "frontendDist": "../frontend/dist"
  },
  "app": {
    "windows": [
      {
        "title": "LingoMate - AI English Tutor",
        "width": 1024,
        "height": 768,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "center": true
      }
    ]
  }
}
```

### 4.5 配置 Cargo.toml

编辑 `src-tauri/Cargo.toml`:

```toml
[package]
name = "lingomate"
version = "0.1.0"
description = "LingoMate - AI English Tutor Desktop App"
authors = ["LingoMate Team"]
edition = "2021"

[lib]
name = "lingomate_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = [] }
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
rusqlite = { version = "0.31", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
log = "0.4"
env_logger = "0.11"
futures-util = "0.3"
```

创建 `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build()
}
```

---

## 5. 遇到的问题与解决方案

### 问题 1: npm peer dependency 冲突

**现象**:
```
ERESOLVE unable to resolve dependency tree
peer react@"^18.0.0" from @testing-library/react@16.3.0
```

**原因**: React 19 与某些测试库的 peer dependency 不兼容。

**解决方案**:
```bash
npm install --legacy-peer-deps
```

---

### 问题 2: Tauri init 交互式提示失败

**现象**:
```
failed to prompt input: IO error: not a terminal
```

**原因**: 在非交互式环境中,`npx tauri init` 无法接收用户输入。

**解决方案**: 手动创建所有配置文件,包括:
- `tauri.conf.json`
- `Cargo.toml`
- `build.rs`
- `src/main.rs`
- `src/lib.rs`

---

### 问题 3: Rust 编译错误 - Manager trait 未导入

**现象**:
```
error[E0599]: no method named `path` found for reference `&AppHandle`
help: trait `Manager` which provides `path` is implemented but not in scope
```

**原因**: `tauri::Manager` trait 需要显式导入才能使用 `app_handle.path()` 方法。

**解决方案**:
```rust
// 在 src/database/mod.rs 中添加
use tauri::{AppHandle, Manager};
```

---

### 问题 4: Rust 生命周期错误 - borrowed data escapes closure

**现象**:
```
error[E0521]: borrowed data escapes outside of closure
`app` is a reference that is only valid in the closure body
argument requires that `'1` must outlive `'static`
```

**原因**: 在 `.setup()` 闭包中,`app.handle()` 返回的引用不能在 async spawn 中使用,因为闭包的生命周期短于 spawned task。

**解决方案**: 使用 `clone()` 获取 owned handle,并用 `std::thread::spawn` 替代 `async_runtime::spawn`:

```rust
.setup(|app| {
    let app_handle = app.handle().clone();
    std::thread::spawn(move || {
        if let Err(e) = database::init_database(&app_handle) {
            log::error!("Failed to initialize database: {}", e);
        }
    });
    Ok(())
})
```

---

### 问题 5: Tauri 图标文件缺失

**现象**:
```
error: proc macro panicked
failed to open icon /path/to/icons/32x32.png: No such file or directory
```

**原因**: Tauri 默认要求提供应用图标文件,但 MVP 阶段尚未设计正式图标。

**解决方案**:

**方案 A** (临时): 在 `tauri.conf.json` 中禁用 bundle:
```json
{
  "bundle": {
    "active": false,
    "icon": []
  }
}
```

**方案 B** (推荐): 创建最小有效 PNG 作为占位符:
```python
python3 -c "
import struct, zlib

def create_minimal_png(filename, size=32):
    signature = b'\x89PNG\r\n\x1a\n'
    # ... 生成最小 PNG
create_minimal_png('src-tauri/icons/icon.png', 32)
"
```

---

### 问题 6: TypeScript React 导入警告

**现象**:
```
error TS6133: 'React' is declared but its value is never read.
```

**原因**: React 17+ 支持新的 JSX transform,不再需要 `import React from 'react'`。

**解决方案**: 移除未使用的 React 导入:
```typescript
// 修改前
import React, { useState } from 'react';

// 修改后
import { useState } from 'react';
```

批量修复:
```bash
for file in src/**/*.tsx; do
  sed -i '' "s/^import React from 'react';//" "$file"
done
```

---

### 问题 7: Bash 工作目录问题

**现象**:
```
error: could not find `Cargo.toml` in `/Users/admin/codes/LingoMate`
```

**原因**: Bash 工具的工作目录固定在项目根目录,而 `Cargo.toml` 位于 `src-tauri/` 子目录。

**解决方案**: 使用 `--manifest-path` 参数指定 Cargo.toml 位置:
```bash
cargo check --manifest-path /Users/admin/codes/LingoMate/src-tauri/Cargo.toml
```

或使用完整路径调用 cargo:
```bash
/Users/admin/.cargo/bin/cargo check --manifest-path src-tauri/Cargo.toml
```

---

## 6. 项目结构说明

### 6.1 目录树

```
LingoMate/
├── doc/                        # 设计文档
│   ├── LingoMate-API接口规范.md
│   ├── LingoMate-UI-UX设计规范.md
│   ├── LingoMate-数据库详细设计.md
│   ├── LingoMate-框架构建指南.md  # 本文档
│   └── ...
├── frontend/                   # React 前端
│   ├── src/
│   │   ├── components/         # UI 组件
│   │   │   ├── ui/             # 通用 UI 组件
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── ScenarioCard.tsx
│   │   │   │   └── VocabularyCard.tsx
│   │   │   └── chat/           # 聊天相关组件
│   │   │       ├── ChatBubble.tsx
│   │   │       └── RecordButton.tsx
│   │   ├── pages/              # 页面组件
│   │   │   ├── ScenarioSelection.tsx
│   │   │   ├── ChatPage.tsx
│   │   │   └── VocabularyPage.tsx
│   │   ├── services/           # API 服务
│   │   │   └── api.ts          # Tauri Command 封装
│   │   ├── types/              # TypeScript 类型
│   │   │   └── index.ts
│   │   ├── App.tsx             # 主应用组件
│   │   ├── main.tsx            # 入口文件
│   │   └── index.css           # 全局样式
│   ├── package.json
│   ├── tsconfig.json
│   └── vite.config.ts
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/           # Tauri Commands
│   │   │   ├── mod.rs
│   │   │   ├── conversation.rs # 对话相关命令
│   │   │   ├── voice.rs        # 语音相关命令
│   │   │   ├── data.rs         # 数据管理命令
│   │   │   ├── settings.rs     # 配置管理命令
│   │   │   └── system.rs       # 系统管理命令
│   │   ├── database/           # 数据库模块
│   │   │   └── mod.rs          # Schema + 迁移
│   │   ├── ollama.rs           # Ollama API 封装
│   │   ├── tts.rs              # TTS 模块
│   │   ├── lib.rs              # 库入口
│   │   └── main.rs             # 二进制入口
│   ├── capabilities/           # Tauri 权限配置
│   │   └── default.json
│   ├── icons/                  # 应用图标
│   ├── Cargo.toml
│   ├── build.rs
│   └── tauri.conf.json
├── .gitignore
├── package.json                # Monorepo 配置
└── README.md
```

### 6.2 关键文件说明

| 文件 | 作用 |
| :--- | :--- |
| `frontend/src/services/api.ts` | 封装所有 Tauri Command 调用,前端统一从此处调用后端 |
| `frontend/src/types/index.ts` | 定义前后端共享的数据类型 |
| `src-tauri/src/commands/mod.rs` | 导出所有 Tauri Command 供 lib.rs 注册 |
| `src-tauri/src/database/mod.rs` | 数据库初始化 + SQL Schema + 迁移逻辑 |
| `src-tauri/src/lib.rs` | Tauri 应用入口,注册插件、命令、事件处理器 |

---

## 7. 开发工作流

### 7.1 启动开发服务器

```bash
# 方式 1: 使用 Tauri CLI (推荐,同时启动前后端)
npm run tauri:dev

# 方式 2: 单独启动前端
cd frontend && npm run dev

# 方式 3: 单独检查 Rust 代码
cargo check --manifest-path src-tauri/Cargo.toml
```

### 7.2 构建生产版本

```bash
# 构建桌面应用安装包
npm run tauri:build

# 产物位置
# macOS: src-tauri/target/release/bundle/dmg/
# Windows: src-tauri/target/release/bundle/msi/
# Linux: src-tauri/target/release/bundle/deb/
```

### 7.3 添加新的 Tauri Command

1. 在 `src-tauri/src/commands/` 下创建或编辑模块文件
2. 使用 `#[tauri::command]` 装饰器定义函数
3. 在 `src-tauri/src/commands/mod.rs` 中导出
4. 在 `src-tauri/src/lib.rs` 中注册到 `invoke_handler`
5. 在 `frontend/src/services/api.ts` 中添加前端调用封装

示例:
```rust
// src-tauri/src/commands/my_module.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MyParams {
    pub name: String,
}

#[derive(Serialize)]
pub struct MyResult {
    pub message: String,
}

#[tauri::command]
pub async fn my_command(params: MyParams) -> Result<MyResult, String> {
    Ok(MyResult {
        message: format!("Hello, {}!", params.name),
    })
}
```

```typescript
// frontend/src/services/api.ts
export async function myCommand(name: string) {
  return invoke<{ message: string }>('my_command', { name });
}
```

---

## 8. 验证清单

### 8.1 环境验证

```bash
# 检查 Node.js
node --version && npm --version

# 检查 Rust
rustc --version && cargo --version

# 检查 Ollama
ollama --version
ollama list  # 应显示已下载的模型
```

### 8.2 前端构建验证

```bash
cd frontend && npm run build
# 应成功生成 dist/ 目录,无错误
```

### 8.3 Rust 编译验证

```bash
cargo check --manifest-path src-tauri/Cargo.toml
# 应显示 "Finished dev profile [unoptimized + debuginfo] target(s)"
```

### 8.4 数据库初始化验证

启动应用后,检查数据库文件是否创建:
```bash
# macOS
ls -la ~/Library/Application\ Support/com.lingomate.app/lingomate.db

# 验证表结构
sqlite3 ~/Library/Application\ Support/com.lingomate.app/lingomate.db ".tables"
# 应输出: messages  sessions  settings  vocabulary
```

---

## 9. 常见问题 FAQ

### Q1: 为什么选择 Tauri 而不是 Electron?

**A**: Tauri 的优势:
- 打包体积小 (~3MB vs ~150MB)
- 内存占用低
- 使用系统 WebView,无需捆绑 Chromium
- Rust 后端更安全、性能更好

### Q2: 为什么选择 SQLite 而不是其他数据库?

**A**: 
- 单文件存储,易于备份和迁移
- 零配置,无需独立数据库服务
- Tauri 官方插件支持良好
- MVP 阶段性能完全足够

### Q3: 如何调试 Rust 后端代码?

**A**:
```bash
# 启用详细日志
RUST_LOG=debug cargo run --manifest-path src-tauri/Cargo.toml

# 使用 println! 或 log::info! 输出调试信息
log::info!("Debug value: {:?}", some_value);
```

### Q4: 前端如何调试 Tauri API 调用?

**A**:
```typescript
// 在浏览器 DevTools Console 中
import { invoke } from '@tauri-apps/api/core';

// 测试命令
const result = await invoke('get_settings');
console.log(result);
```

---

## 10. 更新日志

| 版本 | 日期 | 变更内容 | 作者 |
| :--- | :--- | :--- | :--- |
| v1.0 | 2026-05-17 | 初始版本,记录完整框架构建过程 | LingoMate Team |

---

**文档结束**
