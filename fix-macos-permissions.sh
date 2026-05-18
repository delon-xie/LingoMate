#!/bin/bash
# fix-macos-permissions.sh

# 找到 Tauri 应用的 Info.plist 位置
APP_PATH="$(pwd)/src-tauri/target/debug/lingomate.app"

if [ -d "$APP_PATH" ]; then
    INFO_PLIST="$APP_PATH/Contents/Info.plist"
    
    echo "Adding permissions to Info.plist at: $INFO_PLIST"
    
    # 使用 PlistBuddy 添加权限描述
    /usr/libexec/PlistBuddy -c "Add :NSMicrophoneUsageDescription string 'LingoMate needs access to your microphone for speech recognition and voice practice.'" "$INFO_PLIST" 2>/dev/null || \
    /usr/libexec/PlistBuddy -c "Set :NSMicrophoneUsageDescription 'LingoMate needs access to your microphone for speech recognition and voice practice.'" "$INFO_PLIST"
    
    /usr/libexec/PlistBuddy -c "Add :NSSpeechRecognitionUsageDescription string 'LingoMate uses speech recognition to convert your voice to text for English learning.'" "$INFO_PLIST" 2>/dev/null || \
    /usr/libexec/PlistBuddy -c "Set :NSSpeechRecognitionUsageDescription 'LingoMate uses speech recognition to convert your voice to text for English learning.'" "$INFO_PLIST"
    
    echo "Permissions added successfully!"
else
    echo "App bundle not found. Please run the app first."
fi