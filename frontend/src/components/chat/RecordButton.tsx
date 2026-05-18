import React, { useState, useCallback, useEffect } from 'react';
import clsx from 'clsx';
import { Mic } from 'lucide-react';
import { startRecording, stopRecording } from '../../services/api';

interface RecordButtonProps {
  onRecordingStart?: () => void;
  onRecordingEnd?: (text: string) => void;
  disabled?: boolean;
}

type RecordState = 'idle' | 'pressing' | 'recording' | 'processing';

export const RecordButton: React.FC<RecordButtonProps> = ({
  onRecordingStart,
  onRecordingEnd,
  disabled = false,
}) => {
  const [state, setState] = useState<RecordState>('idle');
  const [duration, setDuration] = useState(0);

  // 更新录音时长
  useEffect(() => {
    let interval: NodeJS.Timeout;
    
    if (state === 'recording') {
      interval = setInterval(() => {
        setDuration(prev => prev + 0.1);
      }, 100);
    } else {
      setDuration(0);
    }
    
    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [state]);

  const handleMouseDown = useCallback(async () => {
    if (state !== 'idle') return;
    
    try {
      // TODO: 检查麦克风权限（暂时跳过，Whisper 不需要特殊权限）
      
      setState('pressing');
      
      const result = await startRecording();
      if (!result.success) {
        console.error('Failed to start recording:', result.error);
        setState('idle');
        return;
      }
      
      setState('recording');
    } catch (error) {
      console.error('Error starting recording:', error);
      setState('idle');
    }
  }, [state]);

  const handleMouseUp = useCallback(async () => {
    if (state === 'pressing') {
      setState('idle');
      return;
    }

    if (state === 'recording') {
      setState('processing');
      
      try {
        const result = await stopRecording();
        if (result.success && result.text) {
          setState('idle');
          onRecordingEnd?.(result.text);
        } else {
          console.error('Failed to recognize speech:', result.error);
          
          // 显示友好的错误提示
          const errorMessage = result.error || 'Speech recognition failed';
          if (errorMessage.includes('Retry') || errorMessage.includes('timed out')) {
            alert('语音识别暂时不可用。请检查网络连接，或使用键盘输入。');
          } else if (errorMessage.includes('permission')) {
            alert('需要麦克风权限。请在系统偏好设置中允许 LingoMate 访问麦克风。');
          } else {
            alert(`语音识别失败: ${errorMessage}`);
          }
          
          setState('idle');
        }
      } catch (error) {
        console.error('Error stopping recording:', error);
        alert('录音过程中发生错误，请重试。');
        setState('idle');
      }
    }
  }, [state, onRecordingEnd]);

  const stateStyles = {
    idle: 'bg-primary-500 hover:bg-primary-400',
    pressing: 'bg-primary-700 scale-95',
    recording: 'bg-red-500 animate-pulse',
    processing: 'bg-gray-400 cursor-wait',
  };

  return (
    <div className="w-full">
      <button
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        className={clsx(
          "w-full px-6 py-4 rounded-xl font-medium transition-all duration-200",
          "flex items-center justify-center gap-3",
          "border-2 border-transparent",
          state === 'recording'
            ? "bg-red-50 border-red-200 text-red-600 animate-pulse shadow-sm"
            : state === 'processing'
            ? "bg-gray-50 border-gray-200 text-gray-400 cursor-wait"
            : state === 'pressing'
            ? "bg-primary-50 border-primary-300 text-primary-700 scale-[0.98] shadow-md"
            : "bg-gradient-to-r from-primary-50 to-primary-100/50 border-primary-200/50 text-primary-700 hover:from-primary-100 hover:to-primary-200/50 hover:border-primary-300 hover:shadow-md active:scale-[0.99]"
        )}
        disabled={state === 'processing'}
        title="Hold to Speak"
      >
        <div className={clsx(
          "p-2 rounded-full transition-colors",
          state === 'recording'
            ? "bg-red-100"
            : state === 'pressing'
            ? "bg-primary-200"
            : "bg-primary-100 group-hover:bg-primary-200"
        )}>
          <Mic className={clsx("w-5 h-5", state === 'recording' && "animate-bounce")} />
        </div>
        
        <span className="text-sm font-medium">
          {state === 'recording' && `Recording... ${duration.toFixed(1)}s`}
          {state === 'processing' && 'Processing speech...'}
          {state === 'idle' && 'Hold to Speak'}
          {state === 'pressing' && 'Release to stop'}
        </span>
      </button>
    </div>
  );
};