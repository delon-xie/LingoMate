// Ollama API 封装模块

use reqwest;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio_stream::StreamExt;

const OLLAMA_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "qwen2.5:3b";

/// Ollama 生成请求
#[derive(Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GenerationOptions>,
}

/// 生成选项
#[derive(Serialize)]
pub struct GenerationOptions {
    pub temperature: f32,
    pub top_p: f32,
    pub num_predict: i32,
}

/// Ollama 生成响应
#[derive(Deserialize)]
pub struct GenerateResponse {
    pub response: String,
    pub done: bool,
}

/// Ollama 模型列表响应
#[derive(Deserialize)]
pub struct ModelListResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
pub struct ModelInfo {
    pub name: String,
}

/// 检查 Ollama 服务是否运行
pub async fn check_ollama_status() -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let response = client
        .get(format!("{}/api/tags", OLLAMA_BASE_URL))
        .send()
        .await?;

    Ok(response.status().is_success())
}

/// 获取当前加载的模型
pub async fn get_current_model() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/tags", OLLAMA_BASE_URL))
        .send()
        .await?;

    if response.status().is_success() {
        let list: ModelListResponse = response.json().await?;
        // 返回第一个可用模型
        if let Some(model) = list.models.first() {
            return Ok(model.name.clone());
        }
    }

    Ok(DEFAULT_MODEL.to_string())
}

/// 生成场景开场白
pub async fn generate_greeting(scenario: &str, proficiency_level: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let prompt = format!(
        r#"You are an AI English tutor creating a natural opening greeting for a conversation practice session.

Scenario: {}
Student Level: {}

Generate a short, natural, and friendly opening greeting (1-2 sentences) that a native speaker would say in this scenario. The greeting should:
- Be conversational and welcoming
- Match the context of the scenario
- Be appropriate for the student's level
- Encourage the student to respond
- NOT include any explanations or meta-commentary

Just provide the greeting text, nothing else."#,
        scenario.replace('_', " "),
        proficiency_level
    );
    
    let request = serde_json::json!({
        "model": get_current_model().await?,
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": 0.7,
            "num_predict": 100
        }
    });
    
    let response = client
        .post(format!("{}/api/generate", OLLAMA_BASE_URL))
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to generate greeting: {}", response.status()).into());
    }
    
    let json: serde_json::Value = response.json().await?;
    let greeting = json["response"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string();
    
    if greeting.is_empty() {
        //  fallback 默认开场白
        Ok(get_default_greeting(scenario))
    } else {
        Ok(greeting)
    }
}

/// 获取默认开场白（fallback）
fn get_default_greeting(scenario: &str) -> String {
    match scenario {
        "coffee_shop" => "Hi there! Welcome to our coffee shop. What can I get for you today?".to_string(),
        "restaurant" => "Good evening! Welcome to our restaurant. Do you have a reservation?".to_string(),
        "hotel_checkin" => "Hello! Welcome to our hotel. How may I assist you with your check-in?".to_string(),
        "job_interview" => "Good morning! Thank you for coming in today. Please have a seat.".to_string(),
        "airport" => "Hello! Welcome to the airport. Are you checking in for a flight?".to_string(),
        "social_gathering" => "Hi! Nice to meet you! How do you know the host?".to_string(),
        _ => format!("Hello! Let's practice English in this {} scenario. How can I help you?", scenario.replace('_', " ")),
    }
}

/// 系统 Prompt - 英语私教角色设定
fn build_system_prompt(proficiency_level: &str, scenario: &str) -> String {
    format!(
        r#"You are LingoMate, a patient and friendly AI English tutor. Your teaching philosophy follows the "i+1" comprehensible input theory.

**User Profile:**
- Proficiency Level: {level}
- Current Scenario: {scenario}

**CRITICAL RESPONSE RULES (MUST FOLLOW):**
1. ALWAYS respond in English, unless the user explicitly asks for Chinese explanation
2. **STRICT LENGTH LIMIT**: Each response MUST be 2-5 sentences maximum. NEVER exceed 5 sentences.
3. When the user makes grammar mistakes, gently correct them by rephrasing, not by explicit correction
4. If the user seems stuck, provide hints or simpler alternatives
5. Ask ONE follow-up question to keep the conversation going
6. Use simple vocabulary appropriate for {level} level learners
7. Be encouraging and positive
8. **DO NOT** provide definitions, examples, or practice questions during normal conversation - only do this when user explicitly clicks on a word to learn it

**Scenario Context:**
You are currently in a "{scenario}" scenario. Act naturally within this context as a real person would.

**Examples of GOOD responses (2-5 sentences):**
- "Great choice! A cappuccino sounds perfect. Would you like any cream with it? How would you like the foam?"
- "I'd recommend our house blend. It's smooth and not too strong. Would you like to try it?"
- "Sure thing! One cappuccino, no sugar. That'll be $4.50. Would you like anything else?"

**Examples of BAD responses (too long, includes definitions):**
- NEVER include dictionary-style definitions during conversation
- NEVER include CEFR levels, example sentences lists, or practice questions unless teaching a specific word
- NEVER write more than 5 sentences in one response

**Important:**
- Keep responses SHORT and NATURAL like real conversation
- Focus on keeping the dialogue flowing
- Save detailed explanations for when user explicitly asks to learn a word
"#,
        level = proficiency_level,
        scenario = scenario
    )
}

/// 单词教学 Prompt
fn build_teach_word_prompt(word: &str) -> String {
    format!(
        r#"The user wants to learn the word "{word}". Please teach it using the following structure:

1. **Simple Definition**: Explain the meaning in simple English (CEFR A2-B1 level)
2. **Example Sentence**: Give 1-2 natural example sentences
3. **Practice Question**: Ask the user to create their own sentence using this word

Keep your explanation concise and engaging. Do NOT provide Chinese translation unless asked.

Word: {word}
"#,
        word = word
    )
}

/// 流式生成并推送事件到前端
pub async fn stream_generate_with_events(
    app_handle: &AppHandle,
    model: &str,
    prompt: &str,
    session_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let request = GenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: true,
        options: Some(GenerationOptions {
            temperature: 0.7,
            top_p: 0.9,
            num_predict: 512,
        }),
    };

    let mut stream = client
        .post(format!("{}/api/generate", OLLAMA_BASE_URL))
        .json(&request)
        .send()
        .await?
        .bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);

        // 解析 JSON 响应
        if let Ok(response) = serde_json::from_str::<GenerateResponse>(&text) {
            // 通过 Tauri Event 推送到前端
            let _ = app_handle.emit("ai_response_chunk", serde_json::json!({
                "session_id": session_id,
                "chunk": response.response,
                "is_complete": response.done
            }));

            if response.done {
                break;
            }
        }
    }

    Ok(())
}

/// 完整的对话生成流程
pub async fn generate_conversation_response(
    app_handle: &AppHandle,
    session_id: i64,
    user_message: &str,
    conversation_history: &[(String, String)], // (role, content)
    scenario: &str,
    proficiency_level: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 构建完整 Prompt
    let mut prompt = build_system_prompt(proficiency_level, scenario);
    prompt.push_str("\n\n**Conversation History:**\n");

    for (role, content) in conversation_history {
        prompt.push_str(&format!("{}: {}\n", role, content));
    }

    prompt.push_str(&format!("User: {}\nAssistant:", user_message));

    // 获取当前模型
    let model = get_current_model().await.unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    // 流式生成
    stream_generate_with_events(app_handle, &model, &prompt, session_id).await
}

/// 单词教学流程
pub async fn teach_word_with_events(
    app_handle: &AppHandle,
    session_id: i64,
    word: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = build_teach_word_prompt(word);
    let model = get_current_model().await.unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    stream_generate_with_events(app_handle, &model, &prompt, session_id).await
}
