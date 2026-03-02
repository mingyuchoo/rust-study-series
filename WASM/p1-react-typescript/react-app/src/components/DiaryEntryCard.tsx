import { MOOD_MAP } from "../constants/mood";
import type { DiaryEntry } from "../types/diary";

interface Props {
  entry: DiaryEntry;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
}

export function DiaryEntryCard({ entry, onEdit, onDelete }: Props) {
  const moodInfo = MOOD_MAP[entry.mood];
  const preview =
    entry.content.length > 100
      ? entry.content.slice(0, 100) + "..."
      : entry.content;
  const date = entry.created_at.slice(0, 10);

  return (
    <div className="diary-card">
      <div className="diary-card-header">
        <span className="diary-card-mood" title={moodInfo.label}>
          {moodInfo.emoji}
        </span>
        <h3 className="diary-card-title">{entry.title}</h3>
        <span className="diary-card-date">{date}</span>
      </div>
      <p className="diary-card-content">{preview}</p>
      <div className="diary-card-actions">
        <button className="btn btn-edit" onClick={() => onEdit(entry.id)}>
          수정
        </button>
        <button className="btn btn-delete" onClick={() => onDelete(entry.id)}>
          삭제
        </button>
      </div>
    </div>
  );
}
