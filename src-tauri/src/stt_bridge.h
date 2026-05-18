#ifndef STT_BRIDGE_H
#define STT_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

// 初始化语音识别器
int stt_init();

// 检查麦克风权限
int stt_check_permission();

// 检查语音识别权限
int stt_check_speech_recognition_permission();

// 请求麦克风权限（异步，需要回调）
void stt_request_permission();

// 请求语音识别权限
void stt_request_speech_recognition_permission();

// 识别音频文件中的语音
char* stt_recognize_from_file(const char* audio_file_path);

// 释放字符串内存
void stt_free_string(char* str);

#ifdef __cplusplus
}
#endif

#endif // STT_BRIDGE_H