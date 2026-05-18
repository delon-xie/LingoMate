#!/bin/bash

# LingoMate 可用性测试脚本

echo "=========================================="
echo "LingoMate 可用性测试"
echo "=========================================="
echo ""

# 1. 检查 Ollama 状态
echo "1. 检查 Ollama 服务..."
if ollama list &>/dev/null; then
    echo "   ✓ Ollama 正在运行"
    ollama list | grep qwen2.5
else
    echo "   ✗ Ollama 未运行"
    echo "   请执行: ollama serve &"
    exit 1
fi
echo ""

# 2. 检查前端依赖
echo "2. 检查前端依赖..."
if [ -d "frontend/node_modules" ]; then
    echo "   ✓ 前端依赖已安装"
else
    echo "   ✗ 前端依赖未安装"
    echo "   请执行: cd frontend && npm install"
    exit 1
fi
echo ""

# 3. 检查 Tauri 依赖
echo "3. 检查 Tauri 依赖..."
if [ -d "node_modules/@tauri-apps" ]; then
    echo "   ✓ Tauri 依赖已安装"
else
    echo "   ✗ Tauri 依赖未安装"
    echo "   请执行: npm install"
    exit 1
fi
echo ""

# 4. 构建前端
echo "4. 构建前端..."
cd frontend
npm run build > /tmp/build.log 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ 前端构建成功"
else
    echo "   ✗ 前端构建失败"
    cat /tmp/build.log
    exit 1
fi
cd ..
echo ""

# 5. 启动 Tauri 应用
echo "5. 启动 Tauri 应用..."
echo "   提示: 应用将在新窗口中打开"
echo "   如果看到空白页面，请检查:"
echo "   - 是否在 Tauri 环境中运行（不是浏览器）"
echo "   - 浏览器控制台是否有错误"
echo ""

# 6. 测试清单
echo "=========================================="
echo "手工测试步骤"
echo "=========================================="
echo ""
echo "请按以下步骤测试应用："
echo ""
echo "步骤 1: 启动应用"
echo "   命令: npm run tauri:dev"
echo "   预期: 应用窗口打开，显示情景选择页面"
echo ""
echo "步骤 2: 测试情景卡片点击"
echo "   操作: 点击任意情景卡片（如 Coffee Shop）"
echo "   预期: "
echo "   - 控制台显示 'Starting conversation for scenario: ...'"
echo "   - 控制台显示 'Conversation started: {session_id: ..., greeting: ...}'"
echo "   - 页面切换到聊天界面"
echo "   - 显示 AI 的问候消息"
echo ""
echo "步骤 3: 测试发送消息"
echo "   操作: 在输入框中输入文字并点击发送按钮"
echo "   预期:"
echo "   - 用户消息显示在聊天区域（右侧）"
echo "   - AI 开始流式回复（左侧）"
echo "   - 回复内容逐字显示"
echo ""
echo "步骤 4: 测试单词点击学习"
echo "   操作: 双击 AI 回复中的英文单词"
echo "   预期:"
echo "   - 控制台显示 'teach_word' 调用"
echo "   - 单词被添加到生词本"
echo ""
echo "步骤 5: 测试返回按钮"
echo "   操作: 点击左上角的返回箭头"
echo "   预期: 返回情景选择页面"
echo ""
echo "步骤 6: 测试语音输入（可选）"
echo "   操作: 按住 'Hold to Speak' 按钮说话"
echo "   预期:"
echo "   - 按钮变为红色并显示 'Recording...'"
echo "   - 松开后显示识别的文字"
echo ""
echo "=========================================="
echo "常见问题排查"
echo "=========================================="
echo ""
echo "问题 1: 空白页面"
echo "   原因: 在浏览器中运行而非 Tauri 环境"
echo "   解决: 使用 'npm run tauri:dev' 而不是 'npm run dev'"
echo ""
echo "问题 2: 点击无响应"
echo "   原因: Tauri API 调用失败"
echo "   解决: 确保在 Tauri 环境中运行"
echo "   检查: 打开开发者工具查看控制台错误"
echo ""
echo "问题 3: AI 不回复"
echo "   原因: Ollama 服务未运行或模型未加载"
echo "   解决: ollama pull qwen2.5:3b && ollama serve"
echo ""
echo "=========================================="
