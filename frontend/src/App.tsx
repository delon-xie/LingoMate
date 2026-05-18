import { useState, useEffect } from 'react';
import type { Scenario } from './types';
import { ScenarioSelection } from './pages/ScenarioSelection';
import { ChatPage } from './pages/ChatPage';
import { VocabularyPage } from './pages/VocabularyPage';
import { isTauri } from './services/api';

type Page = 'scenario' | 'chat' | 'vocabulary';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('scenario');
  const [selectedScenario, setSelectedScenario] = useState<Scenario | null>(null);
  const [sessionId, setSessionId] = useState<number | null>(null);
  const [initialGreeting, setInitialGreeting] = useState<string | null>(null);

  useEffect(() => {
    const inTauri = isTauri();
    if (!inTauri) {
      console.warn(
        '%c️ LingoMate 警告: 应用未在 Tauri 环境中运行',
        'color: orange; font-size: 14px; font-weight: bold;'
      );
      console.warn(
        '某些功能（如语音输入、TTS）可能无法正常工作。'
      );
      console.warn(
        '请使用 "npm run tauri:dev" 启动完整应用。'
      );
    } else {
      console.log('✓ LingoMate running in Tauri environment');
    }
  }, []);

  const handleScenarioSelect = async (scenario: Scenario, sessionId: number, greeting: string) => {
    console.log('=== App.handleScenarioSelect called ===');
    console.log('scenario:', scenario);
    console.log('sessionId:', sessionId);
    console.log('greeting:', greeting);
    
    try {
      setSelectedScenario(scenario);
      setSessionId(sessionId);
      setInitialGreeting(greeting);
      setCurrentPage('chat');
      console.log('State updated, switching to chat page');
    } catch (error) {
      console.error('Failed to select scenario:', error);
    }
  };

  const handleBackToScenarios = () => {
    setCurrentPage('scenario');
    setSelectedScenario(null);
    setSessionId(null);
    setInitialGreeting(null);
  };

  return (
    <div className="min-h-screen bg-white">
      {currentPage === 'scenario' && (
        <ScenarioSelection
          userNickname="Learner"
          proficiencyLevel="intermediate"
          onScenarioSelect={handleScenarioSelect}
        />
      )}

      {currentPage === 'chat' && selectedScenario && sessionId && (
        <ChatPage
          scenario={selectedScenario}
          sessionId={sessionId}
          onBack={handleBackToScenarios}
          greeting={initialGreeting}
        />
      )}

      {currentPage === 'vocabulary' && (
        <VocabularyPage onBack={handleBackToScenarios} />
      )}
    </div>
  );
}

export default App;
