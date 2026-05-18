// LingoMate 类型定义

// 会话
export interface Session {
  id: number;
  title: string;
  created_at: string;
  message_count: number;
}

// 消息
export interface Message {
  id: number;
  session_id: number;
  role: 'user' | 'assistant' | 'system';
  content: string;
  created_at: string;
}

// 生词项
export interface VocabularyItem {
  id: number;
  word: string;
  phonetic?: string;
  definition?: string;
  example_sentence?: string;
  first_learned: string;
  last_reviewed: string;
  review_count: number;
  mastery_level: number; // 0-5
}

// 应用设置
export interface AppSettings {
  ai_model: string;
  performance_mode: 'fluent' | 'performance';
  tts_mode: 'auto' | 'edge_only' | 'system_only';
  current_voice: string;
  speech_speed: number;
  speech_volume: number;
  show_grammar_hints: boolean;
  network_status: 'online' | 'offline';
  user_nickname?: string;
  user_level?: string;
  theme?: string;
  auto_play_tts?: boolean;
}

// 情景模式 - 扩展到24个典型场景
export type Scenario =
  // 日常生活 (Daily Life)
  | 'coffee_shop'
  | 'restaurant'
  | 'grocery_shopping'
  | 'pharmacy'
  | 'banking'
  | 'post_office'
  
  // 旅行交通 (Travel & Transportation)
  | 'hotel_checkin'
  | 'airport'
  | 'taxi_rideshare'
  | 'train_station'
  | 'car_rental'
  | 'directions_asking'
  
  // 工作商务 (Work & Business)
  | 'job_interview'
  | 'office_meeting'
  | 'business_call'
  | 'email_writing'
  | 'presentation'
  | 'networking_event'
  
  // 社交娱乐 (Social & Entertainment)
  | 'social_gathering'
  | 'dating'
  | 'movie_theater'
  | 'gym_fitness'
  | 'doctor_appointment'
  | 'shopping_clothes';

// 用户水平
export type ProficiencyLevel = 'beginner' | 'intermediate' | 'advanced';

// 系统健康状态
export interface SystemHealth {
  ollama_running: boolean;
  ollama_port_available: boolean;
  model_loaded: boolean;
  current_model: string;
  memory_usage_mb: number;
  disk_space_gb: number;
}
