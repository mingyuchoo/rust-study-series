import type { DiaryEntry } from "../types/diary";
import { DiaryEntryCard } from "./DiaryEntryCard";

interface Props {
  entries: DiaryEntry[];
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
}

export function DiaryEntryList({ entries, onEdit, onDelete }: Props) {
  if (entries.length === 0) {
    return <p className="empty-message">작성된 일기가 없습니다.</p>;
  }

  return (
    <div className="diary-list">
      {entries.map((entry) => (
        <DiaryEntryCard
          key={entry.id}
          entry={entry}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
}
