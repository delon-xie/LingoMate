fn main() {
    tauri_build::build();
    
    // 在 macOS 上编译 Objective-C 桥接代码
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("src/stt_bridge.m")
            .flag("-fobjc-arc")  // 启用 ARC (Automatic Reference Counting)
            .compile("stt_bridge");
        
        println!("cargo:rerun-if-changed=src/stt_bridge.m");
        println!("cargo:rerun-if-changed=src/stt_bridge.h");
        
        // 链接 macOS 框架
        println!("cargo:rustc-link-lib=framework=Speech");
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=Foundation");
        
        // 生成 Info.plist 文件到目标目录
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let plist_path = format!("{}/Info.plist", out_dir);
        
        let plist_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>NSMicrophoneUsageDescription</key>
    <string>LingoMate needs access to your microphone for speech recognition and voice practice.</string>
    <key>NSSpeechRecognitionUsageDescription</key>
    <string>LingoMate uses speech recognition to convert your voice to text for English learning.</string>
</dict>
</plist>"#;
        
        std::fs::write(&plist_path, plist_content).expect("Failed to write Info.plist");
        println!("cargo:warning=Info.plist generated at: {}", plist_path);
    }
}