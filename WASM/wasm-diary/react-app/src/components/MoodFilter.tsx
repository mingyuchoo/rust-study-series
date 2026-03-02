import { ALL_MOODS, MOOD_MAP } from "../constants/mood";
import type { Mood } from "../types/diary";

interface Props {
  selected: Mood | null;
  onSelect: (mood: Mood | null) => void;
}

export function MoodFilter({ selected, onSelect }: Props) {
  return (
    <div className="mood-filter">
      <button
        className={`mood-filter-btn ${selected === null ? "active" : ""}`}
        onClick={() => onSelect(null)}
      >
        전체
      </button>
      {ALL_MOODS.map((mood) => (
        <button
          key={mood}
          className={`mood-filter-btn ${selected === mood ? "active" : ""}`}
          onClick={() => onSelect(mood)}
          title={MOOD_MAP[mood].label}
        >
          {MOOD_MAP[mood].emoji}
        </button>
      ))}
    </div>
  );
}
