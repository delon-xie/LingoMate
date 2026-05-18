import React, { useState, useEffect } from 'react';
import { ArrowLeft, Plus } from 'lucide-react';
import { VocabularyCard } from '../components/ui/VocabularyCard';
import type { VocabularyItem } from '../types';
import { getVocabulary } from '../services/api';

interface VocabularyPageProps {
  onBack: () => void;
}

export const VocabularyPage: React.FC<VocabularyPageProps> = ({ onBack }) => {
  const [vocabulary, setVocabulary] = useState<VocabularyItem[]>([]);
  const [sortBy, setSortBy] = useState<'first_learned' | 'mastery_level'>('first_learned');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadVocabulary();
  }, [sortBy]);

  const loadVocabulary = async () => {
    try {
      setLoading(true);
      const result = await getVocabulary(sortBy, 'desc');
      setVocabulary(result.vocabulary);
    } catch (error) {
      console.error('Failed to load vocabulary:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-white">
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
            <h2 className="text-xl font-semibold text-gray-900">My Vocabulary</h2>
          </div>
          <button className="px-4 py-2 bg-primary-500 text-primary-900 rounded-lg font-medium hover:bg-primary-400 transition-colors flex items-center gap-2">
            <Plus className="w-4 h-4" />
            Add Word
          </button>
        </div>
      </header>

      {/* Filter & Sort */}
      <section className="max-w-4xl mx-auto px-6 py-4">
        <div className="flex gap-4">
          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as any)}
            className="px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:border-primary-400"
          >
            <option value="first_learned">Sort by: Recently Learned</option>
            <option value="mastery_level">Sort by: Mastery Level</option>
          </select>
        </div>
      </section>

      {/* Word List */}
      <main className="max-w-4xl mx-auto px-6 py-4">
        {loading ? (
          <div className="space-y-3">
            {[1, 2, 3].map((i) => (
              <div key={i} className="bg-gray-50 rounded-md p-4 animate-pulse">
                <div className="h-6 bg-gray-200 rounded w-1/3 mb-2" />
                <div className="h-4 bg-gray-200 rounded w-1/4 mb-2" />
                <div className="h-4 bg-gray-200 rounded w-2/3" />
              </div>
            ))}
          </div>
        ) : vocabulary.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-gray-500 text-lg">No words learned yet.</p>
            <p className="text-gray-400 text-sm mt-2">
              Start a conversation and double-click words to learn them!
            </p>
          </div>
        ) : (
          vocabulary.map((item) => (
            <VocabularyCard key={item.id} item={item} />
          ))
        )}
      </main>
    </div>
  );
};
