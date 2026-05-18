#import <Foundation/Foundation.h>
#import <Speech/Speech.h>
#import <AVFoundation/AVFoundation.h>
#include "stt_bridge.h"
#include <stdlib.h>
#include <string.h>

static SFSpeechRecognizer *speechRecognizer = nil;
static NSOperationQueue *recognitionQueue = nil;
static BOOL permissionRequested = NO;
static BOOL speechPermissionRequested = NO;

// 初始化语音识别器
int stt_init() {
    @autoreleasepool {
        if (speechRecognizer != nil) {
            return 0; // 已初始化
        }
        
        // 设置语言为英语
        NSLocale *locale = [[NSLocale alloc] initWithLocaleIdentifier:@"en-US"];
        speechRecognizer = [[SFSpeechRecognizer alloc] initWithLocale:locale];
        
        if (!speechRecognizer) {
            NSLog(@"Failed to create speech recognizer");
            return -1;
        }
        
        // 配置识别器
        speechRecognizer.delegate = nil;
        speechRecognizer.defaultTaskHint = SFSpeechRecognitionTaskHintDictation;
        
        // 创建操作队列
        recognitionQueue = [[NSOperationQueue alloc] init];
        recognitionQueue.maxConcurrentOperationCount = 1;
        
        // 同步请求语音识别权限（这会触发系统对话框）
        if (!speechPermissionRequested) {
            speechPermissionRequested = YES;
            
            NSLog(@"Requesting speech recognition permission (sync)...");
            
            __block BOOL permissionCompleted = NO;
            __block SFSpeechRecognizerAuthorizationStatus permissionStatus = SFSpeechRecognizerAuthorizationStatusNotDetermined;
            
            [SFSpeechRecognizer requestAuthorization:^(SFSpeechRecognizerAuthorizationStatus status) {
                permissionStatus = status;
                permissionCompleted = YES;
                
                switch (status) {
                    case SFSpeechRecognizerAuthorizationStatusAuthorized:
                        NSLog(@"✓ Speech recognition permission granted");
                        break;
                    case SFSpeechRecognizerAuthorizationStatusDenied:
                        NSLog(@"✗ Speech recognition permission denied");
                        break;
                    case SFSpeechRecognizerAuthorizationStatusRestricted:
                        NSLog(@"✗ Speech recognition permission restricted");
                        break;
                    case SFSpeechRecognizerAuthorizationStatusNotDetermined:
                        NSLog(@"? Speech recognition permission not determined");
                        break;
                }
            }];
            
            // 等待权限请求完成（最多等待 30 秒）
            NSDate *timeout = [NSDate dateWithTimeIntervalSinceNow:30.0];
            while (!permissionCompleted && [timeout timeIntervalSinceNow] > 0) {
                [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
            }
            
            if (!permissionCompleted) {
                NSLog(@" Speech recognition permission request timed out");
            }
            
            if (permissionStatus != SFSpeechRecognizerAuthorizationStatusAuthorized) {
                NSLog(@"⚠ Speech recognition permission not granted. Status: %ld", (long)permissionStatus);
                // 不返回错误，让用户可以在识别时再次触发
            }
        }
        
        NSLog(@"Speech recognizer initialized successfully");
        return 0;
    }
}

// 检查麦克风权限
int stt_check_permission() {
    @autoreleasepool {
        AVAuthorizationStatus status = [AVCaptureDevice authorizationStatusForMediaType:AVMediaTypeAudio];
        
        switch (status) {
            case AVAuthorizationStatusAuthorized:
                return 1; // 已授权
            case AVAuthorizationStatusDenied:
            case AVAuthorizationStatusRestricted:
                return 0; // 未授权
            case AVAuthorizationStatusNotDetermined:
                return -1; // 未确定
            default:
                return 0;
        }
    }
}

// 检查语音识别权限
int stt_check_speech_recognition_permission() {
    @autoreleasepool {
        SFSpeechRecognizerAuthorizationStatus status = [SFSpeechRecognizer authorizationStatus];
        
        switch (status) {
            case SFSpeechRecognizerAuthorizationStatusAuthorized:
                return 1; // 已授权
            case SFSpeechRecognizerAuthorizationStatusDenied:
            case SFSpeechRecognizerAuthorizationStatusRestricted:
                return 0; // 未授权
            case SFSpeechRecognizerAuthorizationStatusNotDetermined:
                return -1; // 未确定
            default:
                return 0;
        }
    }
}

// 请求语音识别权限（异步版本，不阻塞）
void stt_request_speech_recognition_permission() {
    @autoreleasepool {
        // 只在权限未确定时才请求
        int currentStatus = stt_check_speech_recognition_permission();
        if (currentStatus != -1) {
            NSLog(@"Speech recognition permission already determined: %d", currentStatus);
            return;
        }
        
        NSLog(@"Requesting speech recognition permission...");
        
        [SFSpeechRecognizer requestAuthorization:^(SFSpeechRecognizerAuthorizationStatus status) {
            switch (status) {
                case SFSpeechRecognizerAuthorizationStatusAuthorized:
                    NSLog(@"Speech recognition permission granted");
                    break;
                case SFSpeechRecognizerAuthorizationStatusDenied:
                    NSLog(@"Speech recognition permission denied");
                    break;
                case SFSpeechRecognizerAuthorizationStatusRestricted:
                    NSLog(@"Speech recognition permission restricted");
                    break;
                case SFSpeechRecognizerAuthorizationStatusNotDetermined:
                    NSLog(@"Speech recognition permission not determined");
                    break;
            }
        }];
        
        // 注意：这里不等待完成，让系统异步处理
        // macOS 会自动显示权限对话框
    }
}

// 请求麦克风权限（同步等待版本）
void stt_request_permission() {
    @autoreleasepool {
        if (permissionRequested) {
            return; // 已经请求过
        }
        
        permissionRequested = YES;
        
        __block BOOL completed = NO;
        
        [AVCaptureDevice requestAccessForMediaType:AVMediaTypeAudio completionHandler:^(BOOL granted) {
            completed = YES;
            
            if (granted) {
                NSLog(@"Microphone permission granted");
            } else {
                NSLog(@"Microphone permission denied");
            }
        }];
        
        // 等待权限请求完成（最多等待 10 秒）
        NSDate *timeout = [NSDate dateWithTimeIntervalSinceNow:10.0];
        while (!completed && [timeout timeIntervalSinceNow] > 0) {
            [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
        }
        
        if (!completed) {
            NSLog(@"Permission request timed out");
        }
    }
}

// 识别音频文件中的语音
char* stt_recognize_from_file(const char* audio_file_path) {
    @autoreleasepool {
        if (!speechRecognizer) {
            NSLog(@"Speech recognizer not initialized");
            return strdup("Error: Speech recognizer not initialized");
        }
        
        // 检查并请求麦克风权限
        int micPermission = stt_check_permission();
        if (micPermission != 1) {
            if (micPermission == -1) {
                NSLog(@"Microphone permission not determined, requesting...");
                
                // 同步请求麦克风权限（这个通常能正常工作）
                __block BOOL micCompleted = NO;
                __block BOOL micGranted = NO;
                
                [AVCaptureDevice requestAccessForMediaType:AVMediaTypeAudio completionHandler:^(BOOL granted) {
                    micGranted = granted;
                    micCompleted = YES;
                }];
                
                // 等待最多 5 秒
                NSDate *micTimeout = [NSDate dateWithTimeIntervalSinceNow:5.0];
                while (!micCompleted && [micTimeout timeIntervalSinceNow] > 0) {
                    [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
                }
                
                if (!micCompleted || !micGranted) {
                    return strdup("Error: Microphone permission not granted. Please allow microphone access in System Preferences.");
                }
            } else {
                return strdup("Error: Microphone permission denied. Please allow microphone access in System Preferences.");
            }
        }
        
        // 检查语音识别权限
        int speechPermission = stt_check_speech_recognition_permission();
        if (speechPermission != 1) {
            if (speechPermission == -1) {
                NSLog(@"Speech recognition permission not determined, requesting...");
                
                // 异步请求语音识别权限
                stt_request_speech_recognition_permission();
                
                // 等待用户响应（最多 10 秒）
                NSDate *speechTimeout = [NSDate dateWithTimeIntervalSinceNow:10.0];
                while ([speechTimeout timeIntervalSinceNow] > 0) {
                    int newStatus = stt_check_speech_recognition_permission();
                    if (newStatus == 1) {
                        NSLog(@"Speech recognition permission granted");
                        speechPermission = 1;
                        break;
                    } else if (newStatus == 0) {
                        NSLog(@"Speech recognition permission denied");
                        return strdup("Error: Speech recognition permission denied. Please allow speech recognition in System Preferences > Privacy & Security.");
                    }
                    
                    [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
                }
                
                if (speechPermission != 1) {
                    return strdup("Error: Speech recognition permission timeout. Please check System Preferences.");
                }
            } else {
                return strdup("Error: Speech recognition permission denied. Please allow speech recognition in System Preferences > Privacy & Security.");
            }
        }
        
        // 转换 C 字符串为 NSString
        NSString *filePath = [NSString stringWithUTF8String:audio_file_path];
        NSURL *audioURL = [NSURL fileURLWithPath:filePath];
        
        // 检查文件是否存在
        if (![[NSFileManager defaultManager] fileExistsAtPath:filePath]) {
            NSLog(@"Audio file does not exist: %@", filePath);
            return strdup("Error: Audio file not found");
        }
        
        __block NSString *recognizedText = @"";
        __block BOOL completed = NO;
        __block NSError *recognitionError = nil;
        
        // 创建识别请求
        SFSpeechURLRecognitionRequest *request = [[SFSpeechURLRecognitionRequest alloc] initWithURL:audioURL];
        request.shouldReportPartialResults = NO;
        
        // 设置上下文提示，提高识别准确率
        request.taskHint = SFSpeechRecognitionTaskHintDictation;
        
        NSLog(@"Starting speech recognition for file: %@", filePath);
        
        // 记录开始时间
        NSTimeInterval startTime = [[NSDate date] timeIntervalSince1970];
        
        // 执行识别（带重试逻辑）
        __block SFSpeechRecognitionTask *currentTask = nil;
        int attemptNumber = 0;
        int retryCount = 0;
        const int maxRetries = 5;
        
        do {
            attemptNumber++;
            NSLog(@"\n--- Recognition Attempt %d ---", attemptNumber);
            
            // 重置状态
            completed = NO;
            recognitionError = nil;
            
            // 创建识别任务
            currentTask = [speechRecognizer recognitionTaskWithRequest:request
                                                                       resultHandler:^(SFSpeechRecognitionResult * _Nullable result, NSError * _Nullable error) {
                if (error) {
                    recognitionError = error;
                    completed = YES;
                    NSLog(@"  Error: %@ (code=%ld)", error.localizedDescription, (long)error.code);
                    return;
                }
                
                if (result) {
                    recognizedText = result.bestTranscription.formattedString;
                    
                    if (result.isFinal) {
                        completed = YES;
                        NSLog(@"  Final result: %@", recognizedText);
                    } else {
                        NSLog(@"  Partial: %@", recognizedText);
                    }
                }
            }];
            
            // 等待识别完成（最多 30 秒）
            NSDate *attemptTimeout = [NSDate dateWithTimeIntervalSinceNow:30.0];
            while (!completed && [attemptTimeout timeIntervalSinceNow] > 0) {
                [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
            }
            
            // 检查是否超时
            if (!completed) {
                NSLog(@"  Attempt %d timed out", attemptNumber);
                if (currentTask) {
                    [currentTask cancel];
                }
                break;
            }
            
            // 检查是否有错误
            if (recognitionError) {
                // 检查是否是 Retry 错误
                BOOL isRetryError = (recognitionError.code == 203 || 
                                     recognitionError.code == 3006 ||
                                     [recognitionError.localizedDescription containsString:@"Retry"]);
                
                if (isRetryError && retryCount < maxRetries) {
                    retryCount++;
                    NSLog(@"  Retry error detected, will retry (%d/%d)", retryCount, maxRetries);
                    
                    // 取消当前任务
                    if (currentTask) {
                        [currentTask cancel];
                    }
                    
                    // 等待后重试
                    NSTimeInterval delay = 2.0 + (retryCount * 1.0);
                    NSLog(@"  Waiting %.1f seconds...", delay);
                    
                    NSDate *retryDelay = [NSDate dateWithTimeIntervalSinceNow:delay];
                    [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:retryDelay];
                    
                    // 继续循环进行重试
                    continue;
                } else {
                    // 不是 Retry 错误或已达到最大重试次数
                    NSLog(@"  Stopping: isRetryError=%d, retryCount=%d", isRetryError, retryCount);
                    break;
                }
            }
            
            // 成功完成，退出循环
            break;
            
        } while (retryCount < maxRetries);
        
        NSTimeInterval elapsed = [[NSDate date] timeIntervalSince1970] - startTime;

        if (!completed && !recognitionError) {
            NSLog(@"\n✗ Recognition timed out after %.1f seconds (%d attempts)", elapsed, attemptNumber);
            
            if (recognizedText.length > 0) {
                NSLog(@"Returning partial result: %@", recognizedText);
                return strdup([recognizedText UTF8String]);
            }
            
            return strdup("Error: Recognition timed out.");
        }

        NSLog(@"\n✓ Recognition completed in %.1f seconds (%d attempts, %d retries)", 
              elapsed, attemptNumber, retryCount);
        
        if (recognitionError) {
            NSLog(@" Final error: %@", recognitionError.localizedDescription);
            NSString *errorMsg = [NSString stringWithFormat:@"Error: %@", recognitionError.localizedDescription];
            return strdup([errorMsg UTF8String]);
        }
        
        if (recognizedText.length == 0) {
            NSLog(@"⚠ No speech recognized");
            return strdup("");
        }
        
        NSLog(@"✓ Successfully recognized: %@", recognizedText);
        return strdup([recognizedText UTF8String]);
    }
}

// 释放字符串内存
void stt_free_string(char* str) {
    free(str);
}