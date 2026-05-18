import React, { useState, useRef, useEffect } from 'react';
import { ArrowLeft, Settings, Send, Volume2 } from 'lucide-react';
import { ChatBubble } from '../components/chat/ChatBubble';
import { RecordButton } from '../components/chat/RecordButton';
import type { Message, Scenario } from '../types';
import { sendMessage, teachWord, onAiResponseChunk, playAudio } from '../services/api';

interface ChatPageProps {
  scenario: Scenario;
  sessionId: number;
  onBack: () => void;
  greeting?: string | null;
}

const scenarioTitles: Record<Scenario, string> = {
  // Daily Life
  coffee_shop: 'Coffee Shop',
  restaurant: 'Restaurant',
  grocery_shopping: 'Grocery Shopping',
  pharmacy: 'Pharmacy',
  banking: 'Banking',
  post_office: 'Post Office',
  
  // Travel & Transportation
  hotel_checkin: 'Hotel Check-in',
  airport: 'Airport',
  taxi_rideshare: 'Taxi & Rideshare',
  train_station: 'Train Station',
  car_rental: 'Car Rental',
  directions_asking: 'Asking Directions',
  
  // Work & Business
  job_interview: 'Job Interview',
  office_meeting: 'Office Meeting',
  business_call: 'Business Call',
  email_writing: 'Email Writing',
  presentation: 'Presentation',
  networking_event: 'Networking Event',
  
  // Social & Entertainment
  social_gathering: 'Social Gathering',
  dating: 'Dating',
  movie_theater: 'Movie Theater',
  gym_fitness: 'Gym & Fitness',
  doctor_appointment: 'Doctor Appointment',
  shopping_clothes: 'Clothing Store',
};

export const ChatPage: React.FC<ChatPageProps> = ({
  scenario,
  sessionId,
  onBack,
  greeting,
}) => {
  console.log('=== ChatPage rendering ===');
  console.log('scenario:', scenario);
  console.log('sessionId:', sessionId);
  console.log('greeting:', greeting);
  
  const [messages, setMessages] = useState<Message[]>(() => {
    if (greeting) {
      return [{
        id: 0,
        session_id: sessionId,
        role: 'assistant' as const,
        content: greeting,
        created_at: new Date().toISOString(),
      }];
    }
    return [];
  });
  const [inputText, setInputText] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [autoPlayTTS, setAutoPlayTTS] = useState(true); // 自动播放开关
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const lastAssistantMessageRef = useRef<string>(''); // 跟踪最后一条 AI 消息

  // 自动滚动到底部
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // 监听 AI 流式响应
  useEffect(() => {
    let streamingTimer: NodeJS.Timeout | null = null;
    
    const unsubscribe = onAiResponseChunk((event) => {
      if (event.session_id === sessionId) {
        setIsStreaming(!event.is_complete);
        
        // 更新消息
        setMessages((prev) => {
          const lastMsg = prev[prev.length - 1];
          if (lastMsg && lastMsg.role === 'assistant') {
            const updatedContent = lastMsg.content + event.chunk;
            
            // 如果是最后一段，保存完整消息用于 TTS
            if (event.is_complete) {
              lastAssistantMessageRef.current = updatedContent;
              
              // 延迟播放 TTS，确保 UI 已更新
              if (autoPlayTTS) {
                streamingTimer = setTimeout(() => {
                  playTTS(updatedContent);
                }, 500);
              }
            }
            
            return [
              ...prev.slice(0, -1),
              { ...lastMsg, content: updatedContent },
            ];
          }
          return [
            ...prev,
            {
              id: Date.now(),
              session_id: sessionId,
              role: 'assistant',
              content: event.chunk,
              created_at: new Date().toISOString(),
            },
          ];
        });
      }
    });

    return () => {
      unsubscribe.then((unsub) => unsub());
      if (streamingTimer) {
        clearTimeout(streamingTimer);
      }
    };
  }, [sessionId, autoPlayTTS]);

  // 播放 TTS
  const playTTS = async (text: string) => {
    if (!text.trim()) return;
    
    try {
      // 清理 markdown 格式
      const plainText = text
        .replace(/\*\*/g, '')  // 移除粗体
        .replace(/\*/g, '')    // 移除斜体
        .replace(/#/g, '')     // 移除标题
        .replace(/`/g, '')     // 移除代码标记
        .trim();
      
      console.log('=== Playing TTS ===');
      console.log('Original text:', text);
      console.log('Plain text:', plainText);
      
      await playAudio(plainText);
      
      console.log('TTS playback initiated successfully');
    } catch (error) {
      console.error('=== TTS Playback Failed ===');
      console.error('Error:', error);
    }
  };

  // 手动播放某条消息的 TTS
  const handlePlayMessage = async (message: Message) => {
    if (message.role === 'assistant') {
      await playTTS(message.content);
    }
  };

  const handleSendMessage = async () => {
    if (!inputText.trim()) return;

    const userMessage: Message = {
      id: Date.now(),
      session_id: sessionId,
      role: 'user',
      content: inputText,
      created_at: new Date().toISOString(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInputText('');

    try {
      await sendMessage(sessionId, inputText);
    } catch (error) {
      console.error('Failed to send message:', error);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const handleWordClick = async (word: string) => {
    try {
      await teachWord(sessionId, word);
    } catch (error) {
      console.error('Failed to teach word:', error);
    }
  };

  const handleRecordingEnd = (text: string) => {
    setInputText(text);
    // 可选: 自动发送
    // handleSendMessage();
  };

  return (
    <div className="min-h-screen bg-white flex flex-col">
      {/* Header */}
      <header className="border-b border-gray-200 px-6 py-4">
        <div className="max-w-4xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-4">
            <button
              onClick={onBack}
              className="p-2 hover:bg-primary-100 rounded-full transition-colors"
            >
              <ArrowLeft className="w-5 h-5 text-gray-600" />
            </button>
            <h2 className="text-xl font-semibold text-gray-900">
              {scenarioTitles[scenario]}
            </h2>
          </div>
          <div className="flex items-center gap-2">
            {/* 自动播放开关 */}
            <button
              onClick={() => setAutoPlayTTS(!autoPlayTTS)}
              className={`p-2 rounded-full transition-colors ${
                autoPlayTTS ? 'bg-primary-100 text-primary-600' : 'hover:bg-gray-100 text-gray-600'
              }`}
              title={autoPlayTTS ? '关闭自动播放' : '开启自动播放'}
            >
              <Volume2 className="w-5 h-5" />
            </button>
            <button className="p-2 hover:bg-primary-100 rounded-full transition-colors">
              <Settings className="w-5 h-5 text-gray-600" />
            </button>
          </div>
        </div>
      </header>

      {/* Message List */}
      <main className="flex-1 overflow-y-auto px-6 py-8">
        <div className="max-w-4xl mx-auto">
          {messages.map((message) => (
            <ChatBubble
              key={message.id}
              message={message}
              onWordClick={handleWordClick}
              onPlayClick={() => handlePlayMessage(message)}
            />
          ))}
          {isStreaming && (
            <div className="flex gap-3 mb-4">
              <div className="w-10 h-10 rounded-full bg-primary-200 flex items-center justify-center text-2xl">
                🤖
              </div>
              <div className="bg-gray-100 rounded-lg px-4 py-3">
                <span className="inline-block w-2 h-5 bg-primary-500 animate-pulse" />
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>
      </main>

      {/* Input Area */}
      <footer className="border-t border-gray-200 bg-white shadow-sm">
        <div className="max-w-4xl mx-auto px-6 py-4 space-y-3">
          {/* Text Input Row */}
          <div className="flex gap-3">
            <input
              type="text"
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Type your message in English..."
              className="flex-1 px-4 py-3 border-2 border-gray-200 rounded-xl focus:outline-none focus:border-primary-400 focus:ring-2 focus:ring-primary-100 transition-all text-gray-800 placeholder:text-gray-400"
            />
            <button
              onClick={handleSendMessage}
              disabled={!inputText.trim()}
              className="px-5 py-3 bg-primary-500 text-primary-900 rounded-xl font-semibold hover:bg-primary-400 disabled:opacity-40 disabled:cursor-not-allowed transition-all active:scale-95 flex items-center justify-center min-w-[48px]"
              title="Send message (Enter)"
            >
              <Send className="w-5 h-5" />
            </button>
          </div>

          {/* Record Button - Full Width */}
          <RecordButton
            onRecordingEnd={handleRecordingEnd}
          />
          
          {/* Helper Text */}
          <p className="text-xs text-gray-400 text-center">
            Press Enter to send • Hold microphone button to speak
          </p>
        </div>
      </footer>
    </div>
  );
};
