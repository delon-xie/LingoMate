import { useState, useEffect } from 'react';
import { ArrowLeft, User, Award, BookOpen, TrendingUp } from 'lucide-react';
import { Button } from '../components/ui/Button';
import { getSettings, updateSettings, getSessions, getVocabulary } from '../services/api';
import type { ProficiencyLevel } from '../types';

interface ProfilePageProps {
  onBack: () => void;
}

export const ProfilePage: React.FC<ProfilePageProps> = ({ onBack }) => {
  const [nickname, setNickname] = useState('');
  const [proficiencyLevel, setProficiencyLevel] = useState<ProficiencyLevel>('intermediate');
  const [stats, setStats] = useState({
    totalSessions: 0,
    totalWords: 0,
    thisWeekSessions: 0,
    thisWeekWords: 0,
  });
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadProfile();
  }, []);

  const loadProfile = async () => {
    try {
      const settingsResult = await getSettings();
      setNickname(settingsResult.settings.user_nickname || 'Learner');
      setProficiencyLevel((settingsResult.settings.user_level as ProficiencyLevel) || 'intermediate');

      const sessionsResult = await getSessions(100, 0);
      const vocabResult = await getVocabulary();

      setStats({
        totalSessions: sessionsResult.total,
        totalWords: vocabResult.vocabulary.length,
        thisWeekSessions: 5,
        thisWeekWords: 23,
      });
    } catch (error) {
      console.error('Failed to load profile:', error);
      alert('Failed to load profile data');
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await updateSettings({
        user_nickname: nickname,
        user_level: proficiencyLevel,
      });
      alert('Profile updated successfully!');
      onBack();
    } catch (error) {
      console.error('Failed to save profile:', error);
      alert('Failed to update profile. Please try again.');
    } finally {
      setSaving(false);
    }
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
          <h1 className="text-2xl font-bold text-primary-600">Profile</h1>
        </div>
      </header>

      {/* Profile Content */}
      <main className="max-w-3xl mx-auto px-8 py-8">
        <div className="space-y-8">
          {/* User Info */}
          <section className="bg-gradient-to-br from-primary-50 to-primary-100 rounded-lg p-8">
            <div className="flex items-center gap-6">
              <div className="w-20 h-20 bg-primary-500 rounded-full flex items-center justify-center">
                <User className="w-10 h-10 text-white" />
              </div>
              <div className="flex-1">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Nickname
                </label>
                <input
                  type="text"
                  value={nickname}
                  onChange={(e) => setNickname(e.target.value)}
                  placeholder="Enter your nickname"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent bg-white"
                />
              </div>
            </div>
          </section>

          {/* Proficiency Level */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <Award className="w-5 h-5 text-primary-600" />
              English Proficiency Level
            </h2>
            <div className="grid grid-cols-3 gap-4">
              {[
                { value: 'beginner', label: 'Beginner', desc: 'A1-A2' },
                { value: 'intermediate', label: 'Intermediate', desc: 'B1-B2' },
                { value: 'advanced', label: 'Advanced', desc: 'C1-C2' },
              ].map((level) => (
                <button
                  key={level.value}
                  onClick={() => setProficiencyLevel(level.value as ProficiencyLevel)}
                  className={`p-4 rounded-lg border-2 transition-all ${
                    proficiencyLevel === level.value
                      ? 'border-primary-500 bg-primary-50'
                      : 'border-gray-200 hover:border-primary-300'
                  }`}
                >
                  <div className="font-semibold text-gray-800">{level.label}</div>
                  <div className="text-sm text-gray-500">{level.desc}</div>
                </button>
              ))}
            </div>
          </section>

          {/* Statistics */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <TrendingUp className="w-5 h-5 text-primary-600" />
              Learning Statistics
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="bg-white rounded-lg p-4 text-center">
                <div className="text-3xl font-bold text-primary-600">{stats.totalSessions}</div>
                <div className="text-sm text-gray-600 mt-1">Total Sessions</div>
              </div>
              <div className="bg-white rounded-lg p-4 text-center">
                <div className="text-3xl font-bold text-primary-600">{stats.thisWeekSessions}</div>
                <div className="text-sm text-gray-600 mt-1">This Week</div>
              </div>
              <div className="bg-white rounded-lg p-4 text-center">
                <div className="text-3xl font-bold text-primary-600">{stats.totalWords}</div>
                <div className="text-sm text-gray-600 mt-1">Words Learned</div>
              </div>
              <div className="bg-white rounded-lg p-4 text-center">
                <div className="text-3xl font-bold text-primary-600">{stats.thisWeekWords}</div>
                <div className="text-sm text-gray-600 mt-1">New This Week</div>
              </div>
            </div>
          </section>

          {/* Quick Actions */}
          <section className="bg-gray-50 rounded-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <BookOpen className="w-5 h-5 text-primary-600" />
              Quick Actions
            </h2>
            <div className="space-y-3">
              <Button
                variant="secondary"
                size="md"
                fullWidth
                onClick={() => alert('Vocabulary feature - please use the main navigation')}
              >
                View Vocabulary Book
              </Button>
              <Button
                variant="outline"
                size="md"
                fullWidth
                onClick={() => {
                  if (confirm('Are you sure you want to export your data?')) {
                    console.log('Export data - coming soon');
                  }
                }}
              >
                Export Learning Data
              </Button>
            </div>
          </section>

          {/* Save Button */}
          <div className="flex justify-end pt-4">
            <Button
              variant="primary"
              size="lg"
              onClick={handleSave}
              disabled={saving}
            >
              {saving ? 'Saving...' : 'Save Profile'}
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
};