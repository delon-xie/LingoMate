
import { Settings, User } from 'lucide-react';
import { ScenarioCard } from '../components/ui/ScenarioCard';
import type { Scenario, ProficiencyLevel } from '../types';
import { startConversation } from '../services/api';

interface ScenarioSelectionProps {
  userNickname?: string;
  proficiencyLevel?: ProficiencyLevel;
  onScenarioSelect: (scenario: Scenario, sessionId: number, greeting: string) => void;
}

const scenarios: Scenario[] = [
  // Daily Life
  'coffee_shop',
  'restaurant',
  'grocery_shopping',
  'pharmacy',
  'banking',
  'post_office',
  
  // Travel & Transportation
  'hotel_checkin',
  'airport',
  'taxi_rideshare',
  'train_station',
  'car_rental',
  'directions_asking',
  
  // Work & Business
  'job_interview',
  'office_meeting',
  'business_call',
  'email_writing',
  'presentation',
  'networking_event',
  
  // Social & Entertainment
  'social_gathering',
  'dating',
  'movie_theater',
  'gym_fitness',
  'doctor_appointment',
  'shopping_clothes',
];

// 场景分类
const scenarioCategories = [
  { id: 'daily', name: 'Daily Life', scenarios: scenarios.slice(0, 6) },
  { id: 'travel', name: 'Travel & Transportation', scenarios: scenarios.slice(6, 12) },
  { id: 'work', name: 'Work & Business', scenarios: scenarios.slice(12, 18) },
  { id: 'social', name: 'Social & Entertainment', scenarios: scenarios.slice(18, 24) },
];

export const ScenarioSelection: React.FC<ScenarioSelectionProps> = ({
  userNickname = 'Learner',
  proficiencyLevel = 'intermediate',
  onScenarioSelect,
}) => {
  const handleScenarioClick = async (scenario: Scenario) => {
    console.log('=== Scenario Card Clicked ===');
    console.log('Scenario:', scenario);
    console.log('Proficiency Level:', proficiencyLevel);
    
    try {
      console.log('Calling startConversation API...');
      const result = await startConversation(scenario, proficiencyLevel);
      console.log('startConversation returned:', result);
      
      if (!result || !result.session_id) {
        console.error('Invalid result from startConversation:', result);
        alert('启动失败：服务器返回无效数据');
        return;
      }
      
      console.log('Calling onScenarioSelect callback...');
      onScenarioSelect(scenario, result.session_id, result.greeting);
      console.log('onScenarioSelect completed');
    } catch (error) {
      console.error('=== Error in handleScenarioClick ===');
      console.error('Error type:', typeof error);
      console.error('Error message:', error instanceof Error ? error.message : String(error));
      console.error('Full error:', error);
      
      const errorMessage = error instanceof Error ? error.message : String(error);
      alert(`启动失败: ${errorMessage}`);
    }
  };

  return (
    <div className="min-h-screen bg-white">
      {/* Header */}
      <header className="border-b border-gray-200 px-8 py-4">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <h1 className="text-2xl font-bold text-primary-600">LingoMate</h1>
          <div className="flex items-center gap-4">
            <button className="p-2 hover:bg-primary-100 rounded-full transition-colors">
              <Settings className="w-5 h-5 text-gray-600" />
            </button>
            <button className="p-2 hover:bg-primary-100 rounded-full transition-colors">
              <User className="w-5 h-5 text-gray-600" />
            </button>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="max-w-5xl mx-auto px-8 py-8">
        <div className="text-center mb-8">
          <h2 className="text-3xl font-bold text-gray-900 mb-2">
            Welcome back, {userNickname}!
          </h2>
          <p className="text-lg text-gray-600">
            Choose a scenario to practice English today
          </p>
        </div>

        {/* Scenario Categories */}
        <div className="space-y-8">
          {scenarioCategories.map((category) => (
            <div key={category.id}>
              <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
                <span className="w-2 h-2 bg-primary-500 rounded-full"></span>
                {category.name}
              </h3>
              <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                {category.scenarios.map((scenario) => (
                  <ScenarioCard
                    key={scenario}
                    scenario={scenario}
                    onClick={() => handleScenarioClick(scenario)}
                  />
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Footer Stats */}
      <footer className="max-w-5xl mx-auto px-8 py-6 border-t border-gray-200 mt-8">
        <div className="text-center text-sm text-gray-500">
          This Week: 5 sessions | 23 new words learned
        </div>
      </footer>
    </div>
  );
};
