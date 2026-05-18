针对 8G 内存/显存的限制，直接运行原版 `qwen2.5:7b`（约 13GB）会直接爆内存。核心思路是**换用 4-bit 量化版本**，并调整运行参数。以下是适配 8G 硬件的完整方案。

### 方案核心：换用量化模型

**必须使用 `qwen2.5:7b-q4_K_M`**。该版本显存占用约 4.5GB，给系统留出了足够的运行空间。

---

### 详细操作命令

#### 1. 拉取并运行（推荐命令）

这是最稳妥的一键命令，适合大多数 8G 设备（含集成显卡或独显）：

```
# 拉取量化模型（约 4.5GB）
ollama pull qwen2.5:7b-q4_K_M

# 运行模型（自动利用 GPU 加速）
ollama run qwen2.5:7b-q4_K_M
```

#### 2. 内存极限优化（如果上述命令仍卡顿）

如果你的设备是 **8G 内存且无独立显存**（或显存共享内存），需要强制限制资源：

**Linux/macOS (终端执行):**

```
# 设置环境变量：限制 CPU 线程，强制关闭 GPU（防止内存溢出）
export OLLAMA_NUM_THREAD=4
export OLLAMA_GPU_LAYERS=0

# 运行模型（纯 CPU 模式，更稳定）
ollama run qwen2.5:7b-q4_K_M
```

**Windows (PowerShell 执行):**

```
# 设置环境变量
$env:OLLAMA_NUM_THREAD=4
$env:OLLAMA_GPU_LAYERS=0

# 运行
ollama run qwen2.5:7b-q4_K_M
```

_注：`OLLAMA_NUM_THREAD=4`表示限制使用 4 个 CPU 核心，可根据你的 CPU 核心数调整（建议设为物理核心数）。_

#### 3. 验证资源占用

运行模型后，打开另一个终端，检查内存是否在安全范围内：

```
# Linux/macOS 查看内存
top

# Windows 查看内存
taskmgr

# 如果有 NVIDIA 显卡，查看显存
nvidia-smi
```

预期内存占用应在 **5GB 以下**。

---

### 8G 环境配置要点

|硬件场景|推荐配置|备注|
|---|---|---|
|**8G 独显**​ (如 RTX 4060)|`qwen2.5:7b-q4_K_M`，默认运行|显存足够，开启 GPU 加速，速度极快|
|**8G 内存 (核显)**​|上述“极限优化”命令|必须关闭 GPU 层 (`GPU_LAYERS=0`)，防止爆内存|
|**Mac (Apple Silicon)**​|`ollama run qwen2.5:7b-q4_K_M`|Ollama 会自动调用 Metal 优化，无需额外设置|

### 常见问题排查

- **报错 `out of memory`**：确认你运行的是 `-q4_K_M`版本，而不是默认的 `:latest`或 `:7b`。
    
- **速度极慢**：在 8G 内存机器上，如果没开 GPU，速度在 1-5 token/s 是正常的，建议降低 `--num_ctx`（如 2048）来提速。
    
- **模型卸载**：长时间不用时，执行 `ollama stop qwen2.5:7b-q4_K_M`释放内存。
    

**总结命令**：对于香港常见的轻薄本或小型台式机，直接复制以下命令即可稳定运行：

```
ollama run qwen2.5:7b-q4_K_M
```

老设备（无独立显卡或显卡太老）跑 Ollama，核心策略是**强制纯 CPU 运行**，避免驱动问题，并选择**更小的量化模型**来保证流畅度。针对 8G 内存的老机器，建议直接降级到 **3B 模型**，7B 模型在纯 CPU 下会非常卡顿。

### 方案一：首选（最稳最快）—— Qwen2.5:3B

这是老设备的最佳选择，内存占用仅 2GB 左右，响应速度极快。

```
# 使用 Homebrew 安装（需提前安装 brew）
brew install ollama

# 启动服务
ollama serve &

# 1. 拉取 3B 量化模型（约 2GB）
ollama pull qwen2.5:3b

# 2. 设置环境变量：强制纯 CPU 运行，限制线程数
# Linux/macOS (终端执行):
export OLLAMA_GPU_LAYERS=0
export OLLAMA_NUM_THREAD=2   # 老 CPU 建议设 2-4 个线程

# Windows (PowerShell 执行):
$env:OLLAMA_GPU_LAYERS=0
$env:OLLAMA_NUM_THREAD=2

# 3. 运行
ollama run qwen2.5:3b

# 4. 退出
/exit
```

### 方案二：极限尝试（如果非要跑 7B）

如果你坚持要跑 7B，必须使用 **Q4 量化**并大幅缩短上下文，否则 8G 内存极易崩溃。

```
# 拉取最小量化版本（约 4.5GB）
ollama pull qwen2.5:7b-q4_K_M

# 运行（强制 CPU + 短上下文）
# Linux/macOS:
export OLLAMA_GPU_LAYERS=0
export OLLAMA_NUM_THREAD=4
ollama run qwen2.5:7b-q4_K_M --num_ctx 2048  # 上下文减半，防爆内存

# Windows:
$env:OLLAMA_GPU_LAYERS=0
$env:OLLAMA_NUM_THREAD=4
ollama run qwen2.5:7b-q4_K_M --num_ctx 2048
```

---

### 老设备专用配置详解

#### 1. 关键参数说明

- **`OLLAMA_GPU_LAYERS=0`**：**最重要**。强制所有计算都在 CPU 进行，避免老显卡驱动不兼容导致的崩溃。
    
- **`OLLAMA_NUM_THREAD=2`**：限制 CPU 线程数。老设备核心少，设为物理核心数（如 2 或 4）比让它自动跑满更稳定。
    
- **`--num_ctx 2048`**：将上下文长度从默认 4096 减半，能显著降低内存峰值压力。
    

#### 2. 预期体验

- **Qwen2.5:3B**：响应速度较快（1-3秒），生成速度约 10-20 token/s，适合日常对话。
    
- **Qwen2.5:7B**：**极慢**。纯 CPU 推理速度可能只有 1-2 token/s（每秒蹦一个字），且内存占用接近 6GB，系统容易卡死。
    

#### 3. 如果还是爆内存（Fallback 方案）

如果连 3B 都跑不动，说明机器实在太老，建议换用 **1.1B/2B**​ 级别的模型：

```
ollama run tinyllama:1.1b
# 或
ollama run gemma2:2b
```

**总结**：老设备求稳，直接复制方案一的命令运行 `qwen2.5:3b`即可，这是体验和性能的最佳平衡点。

考虑到你身处香港且设备较老（无 GPU），安装流程需侧重**网络稳定性**和**纯 CPU 环境适配**。以下是针对 Linux、macOS 和 Windows 的详细安装指南。

### 一、Linux（老设备首选，控制力最强）

针对老设备，建议使用官方脚本安装，并手动配置服务。

**1. 一键安装**

```
# 执行官方安装脚本（自动识别架构）
curl -fsSL https://ollama.com/install.sh | sh
```

**2. 配置服务（重要：防止后台崩溃）**

```
# 启动服务并设置开机自启
sudo systemctl enable ollama
sudo systemctl start ollama

# 检查服务状态（看到 Active: active (running) 即为成功）
sudo systemctl status ollama
```

**3. 验证安装**

```
# 查看版本
ollama --version

# 测试运行（使用之前推荐的轻量模型）
OLLAMA_GPU_LAYERS=0 ollama run qwen2.5:3b
```

_注：`OLLAMA_GPU_LAYERS=0`强制使用 CPU，避免老显卡驱动问题。_

---

### 二、macOS（Apple Silicon / Intel）

**1. 终端安装（推荐）**

```
# 使用 Homebrew 安装（需提前安装 brew）
brew install ollama

# 启动服务
ollama serve &
```

**2. 图形化安装（备用）**

- 访问 [ollama.com/download](https://ollama.com/download)，下载 `Ollama-darwin.zip`。
    
- 解压后将应用拖入 `Applications`文件夹，双击运行。
    

**3. 验证**

```
# 查看版本
ollama --version

# 运行模型（Mac 老设备建议用 CPU）
OLLAMA_GPU_LAYERS=0 ollama run qwen2.5:3b
```

---

### 三、Windows（PowerShell）

**1. 一键安装**

以**管理员身份**打开 PowerShell，执行：

```
irm https://ollama.com/install.ps1 | iex
```

_注意：Windows 10 及以上版本支持，安装后会自动添加环境变量。_

**2. 验证**

关闭 PowerShell 重新打开，执行：

```
# 查看版本
ollama --version

# 运行模型（老设备强制 CPU）
$env:OLLAMA_GPU_LAYERS=0
ollama run qwen2.5:3b
```

---

### 四、针对老设备的特殊优化（必看）

由于你的设备内存仅 8G 且无 GPU，安装后需进行以下设置以防崩溃：

**1. 修改服务配置（Linux/macOS）**

创建或编辑配置文件，限制资源占用：

```
# Linux 编辑服务文件
sudo systemctl edit ollama

# 在打开的编辑器中添加以下内容（限制内存和线程）
[Service]
Environment="OLLAMA_NUM_THREAD=2"
Environment="OLLAMA_MAX_LOADED_MODELS=1"
```

**2. 环境变量（所有平台通用）**

在运行模型前，始终设置以下变量：

- **Linux/macOS (终端)**：
    
    ```
    export OLLAMA_GPU_LAYERS=0    # 强制 CPU
    export OLLAMA_NUM_THREAD=2    # 限制线程数（根据你的 CPU 核心数调整）
    ```
    
- **Windows (PowerShell)**：
    
    ```
    $env:OLLAMA_GPU_LAYERS=0
    $env:OLLAMA_NUM_THREAD=2
    ```
    

**3. 模型存储位置（可选）**

如果系统盘空间不足，可更改模型下载路径：

```
# Linux/macOS
export OLLAMA_MODELS=/path/to/your/large/disk

# Windows
$env:OLLAMA_MODELS="D:\ollama_models"
```

---

### 五、安装后验证

无论哪个平台，安装完成后请执行以下步骤确认环境正常：

1. **检查版本**：`ollama --version`（应输出版本号）。
    
2. **拉取测试模型**：`ollama pull qwen2.5:3b`（约 2GB，适合老设备）。
    
3. **运行测试**：`OLLAMA_GPU_LAYERS=0 ollama run qwen2.5:3b`，输入简单问题（如“你好”），观察是否流畅响应。
    

**总结**：对于你的老设备，**推荐使用 Linux 系统**，安装后务必设置 `OLLAMA_GPU_LAYERS=0`和 `OLLAMA_NUM_THREAD=2`，并优先运行 `qwen2.5:3b`模型以获得最佳体验。