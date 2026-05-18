import { useState, useEffect } from 'react';
import { ArrowLeft, Save, Volume2, Zap, Brain, Palette } from 'lucide-react';
import { Button } from '../components/ui/Button';
import { getSettings, updateSettings } from '../services/api';
import type { AppSettings } from '../types';

interface SettingsPageProps {
  onBack: () => void;
}

export const SettingsPage: React.FC<SettingsPageProps> = ({ onBack }) => {
  const [settings, setSettings] = useState<Partial<AppSettings>>({});
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const result = await getSettings();
      setSettings(result.settings);
    } catch (error) {
      console.error('Failed to load settings:', error);
      alert('Failed to load settings');
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await updateSettings(settings);
      alert('Settings saved successfully!');
      onBack();
    } catch (error) {
      console.error('Failed to save settings:', error);
      alert('Failed to save settings. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const handleChange = (key: keyof AppSettings, value: any) => {
    setSettings(prev => ({ ...prev, [key]: value }));
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-white">
      {/* Header */}
      <header className="border-b border-gray-200 px-8 py-4">
        <div className="max-w-3xl mx-auto flex items-center gap-4">
          <button
            onClick={onBack}
            className="p-2 hover:bg-primary-100 rounded-full transition-colors"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <h1 className="text-2xl font-bold text-primary-600">Settings</h1>
        </div>
      </header>

      {/* Settings Content */}
      <main className="max-w-3xl mx-auto px-8 py-8">
        <div className="space-y-8">
          {/* AI Model Settings */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <Brain className="w-5 h-5 text-primary-600" />
              AI Model
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Current Model
                </label>
                <select
                  value={settings.ai_model || 'qwen2.5:3b'}
                  onChange={(e) => handleChange('ai_model', e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                >
                  <option value="qwen2.5:3b">Qwen 2.5 3B (Recommended)</option>
                  <option value="llama3.2:3b">Llama 3.2 3B</option>
                  <option value="mistral:7b">Mistral 7B</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Performance Mode
                </label>
                <select
                  value={settings.performance_mode || 'fluent'}
                  onChange={(e) => handleChange('performance_mode', e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                >
                  <option value="fluent">Fluent (Better Quality)</option>
                  <option value="performance">Performance (Faster)</option>
                </select>
              </div>
            </div>
          </section>

          {/* Voice Settings */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <Volume2 className="w-5 h-5 text-primary-600" />
              Voice & Speech
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  TTS Mode
                </label>
                <select
                  value={settings.tts_mode || 'auto'}
                  onChange={(e) => handleChange('tts_mode', e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                >
                  <option value="auto">Auto (Edge TTS + System Fallback)</option>
                  <option value="edge_only">Edge TTS Only</option>
                  <option value="system_only">System TTS Only</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Speech Speed: {settings.speech_speed || 0.85}x
                </label>
                <input
                  type="range"
                  min="0.5"
                  max="2.0"
                  step="0.05"
                  value={settings.speech_speed || 0.85}
                  onChange={(e) => handleChange('speech_speed', parseFloat(e.target.value))}
                  className="w-full"
                />
                <div className="flex justify-between text-xs text-gray-500 mt-1">
                  <span>Slower</span>
                  <span>Normal</span>
                  <span>Faster</span>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Volume: {Math.round((settings.speech_volume || 1.0) * 100)}%
                </label>
                <input
                  type="range"
                  min="0.0"
                  max="1.0"
                  step="0.1"
                  value={settings.speech_volume || 1.0}
                  onChange={(e) => handleChange('speech_volume', parseFloat(e.target.value))}
                  className="w-full"
                />
              </div>
            </div>
          </section>

          {/* Learning Settings */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <Zap className="w-5 h-5 text-primary-600" />
              Learning Preferences
            </h2>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium text-gray-700">
                    Show Grammar Hints
                  </label>
                  <p className="text-xs text-gray-500">Display grammar corrections during conversations</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={settings.show_grammar_hints ?? true}
                    onChange={(e) => handleChange('show_grammar_hints', e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"></div>
                </label>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium text-gray-700">
                    Auto-play TTS
                  </label>
                  <p className="text-xs text-gray-500">Automatically play AI response audio</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={settings.auto_play_tts ?? true}
                    onChange={(e) => handleChange('auto_play_tts', e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"></div>
                </label>
              </div>
            </div>
          </section>

          {/* Theme Settings */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <Palette className="w-5 h-5 text-primary-600" />
              Appearance
            </h2>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Theme
              </label>
              <select
                value={settings.theme || 'light'}
                onChange={(e) => handleChange('theme', e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              >
                <option value="light">Light</option>
                <option value="dark">Dark (Coming Soon)</option>
              </select>
            </div>
          </section>

          {/* Save Button */}
          <div className="flex justify-end pt-4">
            <Button
              variant="primary"
              size="lg"
              onClick={handleSave}
              disabled={saving}
              icon={Save}
            >
              {saving ? 'Saving...' : 'Save Settings'}
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
};