
import clsx from 'clsx';
import { Volume2 } from 'lucide-react';
import type { Message } from '../../types';

interface ChatBubbleProps {
  message: Message;
  onWordClick?: (word: string) => void;
  onPlayClick?: () => void;
}

export const ChatBubble: React.FC<ChatBubbleProps> = ({
  message,
  onWordClick,
  onPlayClick,
}) => {
  const isUser = message.role === 'user';
  const isAI = message.role === 'assistant';

  // 将文本拆分为可点击的单词
  const renderContent = (content: string) => {
    if (!onWordClick || !isAI) {
      return content;
    }

    const words = content.split(/(\s+)/);
    return words.map((word, index) => {
      // 只使英文单词可点击
      if (/^[a-zA-Z']+$/.test(word)) {
        return (
          <span
            key={index}
            onClick={() => onWordClick(word)}
            className="cursor-pointer border-b-2 border-dotted border-primary-500 hover:bg-primary-200 transition-colors"
          >
            {word}
          </span>
        );
      }
      return word;
    });
  };

  return (
    <div
      className={clsx(
        'flex gap-3 mb-4',
        isUser && 'flex-row-reverse'
      )}
    >
      {/* Avatar */}
      <div
        className={clsx(
          'w-10 h-10 rounded-full flex items-center justify-center flex-shrink-0',
          isUser
            ? 'bg-primary-500 text-white font-semibold text-sm'
            : 'bg-primary-200 text-2xl'
        )}
      >
        {isUser ? 'You' : '🤖'}
      </div>

      {/* Bubble */}
      <div
        className={clsx(
          'rounded-lg px-4 py-3 max-w-[70%]',
          isUser
            ? 'bg-primary-100 border border-primary-300'
            : 'bg-gray-100'
        )}
      >
        <div className="flex items-start justify-between gap-2">
          <p className="text-base leading-relaxed text-gray-900 flex-1">
            {renderContent(message.content)}
          </p>
          
          {/* 播放按钮（仅 AI 消息） */}
          {isAI && onPlayClick && (
            <button
              onClick={onPlayClick}
              className="p-1.5 text-gray-400 hover:text-primary-600 hover:bg-primary-100 rounded-full transition-colors flex-shrink-0"
              title="播放语音"
            >
              <Volume2 className="w-4 h-4" />
            </button>
          )}
        </div>
        <p className="text-xs text-gray-400 mt-1">
          {new Date(message.created_at).toLocaleTimeString()}
        </p>
      </div>
    </div>
  );
};