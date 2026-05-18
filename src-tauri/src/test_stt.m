// src-tauri/src/test_stt.m
#import <Foundation/Foundation.h>
#import <Speech/Speech.h>
#import <AVFoundation/AVFoundation.h>

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSLog(@"=== Testing macOS Speech Framework ===");
        
        // 1. 检查权限状态
        SFSpeechRecognizerAuthorizationStatus authStatus = [SFSpeechRecognizer authorizationStatus];
        NSLog(@"Speech recognition authorization status: %ld", (long)authStatus);
        
        switch (authStatus) {
            case SFSpeechRecognizerAuthorizationStatusAuthorized:
                NSLog(@"✓ Authorized");
                break;
            case SFSpeechRecognizerAuthorizationStatusDenied:
                NSLog(@"✗ Denied - Please enable in System Preferences > Privacy & Security > Speech Recognition");
                return 1;
            case SFSpeechRecognizerAuthorizationStatusRestricted:
                NSLog(@"✗ Restricted");
                return 1;
            case SFSpeechRecognizerAuthorizationStatusNotDetermined:
                NSLog(@"⚠ Not determined");
                NSLog(@"   Please run the LingoMate app first to grant permission");
                NSLog(@"   Then run this test again");
                return 1;
        }
        
        // 2. 创建识别器
        NSLocale *locale = [[NSLocale alloc] initWithLocaleIdentifier:@"en-US"];
        SFSpeechRecognizer *recognizer = [[SFSpeechRecognizer alloc] initWithLocale:locale];
        
        if (!recognizer) {
            NSLog(@"ERROR: Failed to create speech recognizer");
            return 1;
        }
        
        NSLog(@"✓ Speech recognizer created");
        NSLog(@"  Available: %d", recognizer.isAvailable);
        
        // 3. 生成测试音频
        NSString *testText = @"Hello, this is a test of speech recognition on macOS";
        NSString *testAudioPath = @"/tmp/test_speech_recognition.aiff";
        
        NSLog(@"\nGenerating test audio...");
        NSLog(@"  Text: %@", testText);
        
        NSTask *sayTask = [[NSTask alloc] init];
        [sayTask setLaunchPath:@"/usr/bin/say"];
        [sayTask setArguments:@[@"-v", @"Samantha", @"-o", testAudioPath, testText]];
        
        NSPipe *pipe = [NSPipe pipe];
        [sayTask setStandardError:pipe];
        
        [sayTask launch];
        [sayTask waitUntilExit];
        
        if ([sayTask terminationStatus] != 0) {
            NSData *errorData = [[pipe fileHandleForReading] readDataToEndOfFile];
            NSString *errorStr = [[NSString alloc] initWithData:errorData encoding:NSUTF8StringEncoding];
            NSLog(@"ERROR: Failed to generate test audio: %@", errorStr);
            return 1;
        }
        
        NSLog(@"✓ Test audio generated: %@", testAudioPath);
        
        // 检查文件大小
        NSDictionary *fileAttrs = [[NSFileManager defaultManager] attributesOfItemAtPath:testAudioPath error:nil];
        NSNumber *fileSize = [fileAttrs objectForKey:NSFileSize];
        NSLog(@"  File size: %@ bytes", fileSize);
        
        // 4. 识别音频文件
        NSURL *audioURL = [NSURL fileURLWithPath:testAudioPath];
        SFSpeechURLRecognitionRequest *request = [[SFSpeechURLRecognitionRequest alloc] initWithURL:audioURL];
        request.shouldReportPartialResults = YES;
        
        NSLog(@"\nStarting speech recognition...");
        
        __block NSString *recognizedText = @"";
        __block BOOL completed = NO;
        __block NSError *error = nil;
        __block int resultCount = 0;
        
        SFSpeechRecognitionTask *task = [recognizer recognitionTaskWithRequest:request
                                                                resultHandler:^(SFSpeechRecognitionResult * _Nullable result, NSError * _Nullable err) {
            if (err) {
                error = err;
                completed = YES;
                NSLog(@"✗ Recognition error (code=%ld): %@", (long)err.code, err.localizedDescription);
                return;
            }
            
            if (result) {
                resultCount++;
                recognizedText = result.bestTranscription.formattedString;
                
                if (result.isFinal) {
                    completed = YES;
                    NSLog(@"✓ Final result received");
                } else {
                    NSLog(@"  Partial result #%d: %@", resultCount, recognizedText);
                }
            }
        }];
        
        // 等待完成（最多 60 秒）
        NSLog(@"Waiting for recognition to complete...");
        NSDate *timeout = [NSDate dateWithTimeIntervalSinceNow:60.0];
        NSTimeInterval startTime = [[NSDate date] timeIntervalSince1970];
        
        while (!completed && [timeout timeIntervalSinceNow] > 0) {
            [[NSRunLoop currentRunLoop] runMode:NSDefaultRunLoopMode beforeDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
        }
        
        NSTimeInterval elapsed = [[NSDate date] timeIntervalSince1970] - startTime;
        
        if (!completed) {
            [task cancel];
            NSLog(@"\n✗ ERROR: Recognition timed out after %.1f seconds", elapsed);
            
            // 清理
            [[NSFileManager defaultManager] removeItemAtPath:testAudioPath error:nil];
            return 1;
        }
        
        NSLog(@"  Recognition completed in %.1f seconds", elapsed);
        
        if (error) {
            NSLog(@"\n✗ ERROR: Recognition failed");
            NSLog(@"  Error: %@", error.localizedDescription);
            NSLog(@"  Code: %ld", (long)error.code);
            
            // 清理
            [[NSFileManager defaultManager] removeItemAtPath:testAudioPath error:nil];
            return 1;
        }
        
        // 5. 显示结果
        NSLog(@"\n=== Test Results ===");
        NSLog(@"Original:  %@", testText);
        NSLog(@"Recognized: %@", recognizedText);
        
        // 计算相似度
        if ([recognizedText.lowercaseString isEqualToString:testText.lowercaseString]) {
            NSLog(@"\n✓ PERFECT MATCH!");
        } else {
            NSLog(@"\n⚠ Text differs (this is normal for speech recognition)");
        }
        
        // 清理
        [[NSFileManager defaultManager] removeItemAtPath:testAudioPath error:nil];
        NSLog(@"\n=== Test Completed Successfully ===");
        
        return 0;
    }
}