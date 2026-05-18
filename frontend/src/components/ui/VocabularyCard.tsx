
import type { VocabularyItem } from '../../types';

interface VocabularyCardProps {
  item: VocabularyItem;
}

const MasteryStars: React.FC<{ level: number }> = ({ level }) => {
  return (
    <div className="inline-flex gap-0.5">
      {[1, 2, 3, 4, 5].map((star) => (
        <span
          key={star}
          className={star <= level ? 'text-primary-500' : 'text-primary-300'}
        >
          ★
        </span>
      ))}
    </div>
  );
};

export const VocabularyCard: React.FC<VocabularyCardProps> = ({ item }) => {
  return (
    <div className="bg-gray-50 rounded-md p-4 mb-3 transition-all duration-200 hover:bg-primary-50 hover:border-l-4 hover:border-l-primary-500">
      <h3 className="text-xl font-semibold text-gray-900 mb-1">{item.word}</h3>
      {item.phonetic && (
        <p className="text-sm text-gray-500 font-mono mb-2">{item.phonetic}</p>
      )}
      {item.definition && (
        <p className="text-sm text-gray-700 mb-2">{item.definition}</p>
      )}
      <div className="flex items-center gap-4 text-xs text-gray-400">
        <span>Mastery:</span>
        <MasteryStars level={item.mastery_level} />
        <span>Review: {item.review_count} times</span>
        <span>Learned: {new Date(item.first_learned).toLocaleDateString()}</span>
      </div>
    </div>
  );
};
