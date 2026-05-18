// Tauri API 服务封装
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  Session,
  Message,
  VocabularyItem,
  AppSettings,
  SystemHealth,
  Scenario,
  ProficiencyLevel,
} from '../types';

// 检测是否在 Tauri 环境中
export function isTauri(): boolean {
  // Tauri v2 使用 __TAURI__ 全局变量
  return typeof window !== 'undefined' && (
    '__TAURI__' in window ||
    '__TAURI_INTERNALS__' in window ||
    (window as any).__TAURI_POST_MESSAGE__ !== undefined
  );
}

// ============ 对话相关 ============

export async function startConversation(
  scenario: Scenario,
  proficiencyLevel: ProficiencyLevel
) {
  console.log('Calling Tauri invoke: start_conversation', { scenario, proficiencyLevel });
  
  // 检查是否在 Tauri 环境中
  if (!isTauri()) {
    console.error('Not running in Tauri environment! Please use "npm run tauri:dev" to start the app.');
    throw new Error('此功能需要在 Tauri 桌面应用中运行。请使用 "npm run tauri:dev" 启动应用。');
  }
  
  try {
    // Tauri v2 converts snake_case Rust params to camelCase in JS
    const result = await invoke<{ session_id: number; greeting: string; success: boolean }>(
      'start_conversation',
      {
        scenario,
        proficiencyLevel
      }
    );
    console.log('Tauri invoke succeeded:', result);
    return result;
  } catch (error) {
    console.error('Tauri invoke failed:', error);
    throw error;
  }
}

export async function sendMessage(sessionId: number, text: string) {
  return invoke<{ success: boolean }>('send_message', {
    params: {
      session_id: sessionId,
      text,
    }
  });
}

export async function teachWord(sessionId: number, word: string) {
  return invoke<{ success: boolean }>('teach_word', {
    params: {
      session_id: sessionId,
      word,
    }
  });
}

export async function stopAiResponse(sessionId: number) {
  return invoke<{ success: boolean }>('stop_ai_response', {
    params: {
      session_id: sessionId,
    }
  });
}

// ============ 语音相关 ============

export async function startRecording() {
  return invoke<{ success: boolean }>('start_recording');
}

export async function stopRecording() {
  return invoke<{ success: boolean; text?: string }>('stop_recording');
}

export async function playAudio(
  text: string,
  voice?: string,
  speed?: number,
  volume?: number
) {
  return invoke<{ success: boolean }>('play_audio', {
    params: {
      text,
      voice,
      speed,
      volume,
    }
  });
}

export async function stopAudio() {
  return invoke<{ success: boolean }>('stop_audio');
}

// ============ 数据管理 ============

export async function getSessions(limit = 50, offset = 0) {
  return invoke<{ sessions: Session[]; total: number }>('get_sessions', {
    params: {
      limit,
      offset,
    }
  });
}

export async function getMessages(sessionId: number) {
  return invoke<{ messages: Message[] }>('get_messages', {
    params: {
      session_id: sessionId,
    }
  });
}

export async function getVocabulary(sortBy = 'first_learned', order = 'desc') {
  return invoke<{ vocabulary: VocabularyItem[] }>('get_vocabulary', {
    params: {
      sort_by: sortBy,
      order,
    }
  });
}

export async function deleteSession(sessionId: number) {
  return invoke<{ success: boolean }>('delete_session', {
    params: {
      session_id: sessionId,
    }
  });
}

export async function getVocabularyDetail(word: string) {
  return invoke<{ vocabulary: VocabularyItem | null }>('get_vocabulary_detail', {
    params: {
      word,
    }
  });
}

export async function updateVocabularyReview(wordId: number, masteryLevel: number) {
  return invoke<{ success: boolean }>('update_vocabulary_review', {
    params: {
      word_id: wordId,
      mastery_level: masteryLevel,
    }
  });
}

// ============ 配置管理 ============

export async function getSettings() {
  return invoke<{ settings: AppSettings }>('get_settings');
}

export async function updateSettings(settings: Partial<AppSettings>) {
  return invoke<{ success: boolean }>('update_settings', {
    params: {
      settings,
    }
  });
}

export async function switchAiModel(model: string) {
  return invoke<{ success: boolean }>('switch_ai_model', {
    params: {
      model,
    }
  });
}

// ============ 系统管理 ============

export async function checkSystemHealth() {
  return invoke<{ health: SystemHealth }>('check_system_health');
}

export async function restartOllama() {
  return invoke<{ success: boolean }>('restart_ollama');
}

export async function exportData(
  format: 'json' | 'csv',
  includeSessions: boolean,
  includeVocabulary: boolean
) {
  return invoke<{ success: boolean; file_path?: string }>('export_data', {
    params: {
      format,
      include_sessions: includeSessions,
      include_vocabulary: includeVocabulary,
    }
  });
}

// ============ Event 监听 ============

export interface AiResponseChunkEvent {
  session_id: number;
  chunk: string;
  is_complete: boolean;
}

export interface RecordingStatusEvent {
  is_recording: boolean;
  duration_seconds?: number;
}

export interface AudioPlaybackStatusEvent {
  status: 'playing' | 'paused' | 'stopped' | 'error';
  progress?: number;
  error_message?: string;
}

export async function onAiResponseChunk(
  callback: (event: AiResponseChunkEvent) => void
) {
  return listen<AiResponseChunkEvent>('ai_response_chunk', (e) => callback(e.payload));
}

export async function onRecordingStatusChanged(
  callback: (event: RecordingStatusEvent) => void
) {
  return listen<RecordingStatusEvent>('recording_status_changed', (e) => callback(e.payload));
}

export async function onAudioPlaybackStatus(
  callback: (event: AudioPlaybackStatusEvent) => void
) {
  return listen<AudioPlaybackStatusEvent>('audio_playback_status', (e) => callback(e.payload));
}
