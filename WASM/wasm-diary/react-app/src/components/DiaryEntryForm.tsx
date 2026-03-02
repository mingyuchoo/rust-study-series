import { useEffect, useState } from "react";
import { ALL_MOODS, MOOD_MAP } from "../constants/mood";
import type { DiaryEntry, Mood, ValidationResult } from "../types/diary";

interface Props {
  editingEntry?: DiaryEntry | null;
  onSubmit: (title: string, content: string, mood: Mood) => void;
  onCancel: () => void;
  validate: (title: string, content: string) => ValidationResult;
}

export function DiaryEntryForm({
  editingEntry,
  onSubmit,
  onCancel,
  validate,
}: Props) {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [mood, setMood] = useState<Mood>("Calm");
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (editingEntry) {
      setTitle(editingEntry.title);
      setContent(editingEntry.content);
      setMood(editingEntry.mood);
    }
  }, [editingEntry]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const result = validate(title, content);
    if (!result.valid) {
      const errorMap: Record<string, string> = {};
      for (const err of result.errors) {
        errorMap[err.field] = err.message;
      }
      setErrors(errorMap);
      return;
    }
    setErrors({});
    onSubmit(title, content, mood);
  };

  return (
    <form className="diary-form" onSubmit={handleSubmit}>
      <h2>{editingEntry ? "일기 수정" : "새 일기 작성"}</h2>

      <div className="form-field">
        <label htmlFor="title">제목</label>
        <input
          id="title"
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="오늘의 제목을 입력하세요"
        />
        {errors.title && <span className="field-error">{errors.title}</span>}
      </div>

      <div className="form-field">
        <label htmlFor="mood">감정</label>
        <div className="mood-selector">
          {ALL_MOODS.map((m) => (
            <button
              key={m}
              type="button"
              className={`mood-btn ${mood === m ? "active" : ""}`}
              onClick={() => setMood(m)}
              title={MOOD_MAP[m].label}
            >
              {MOOD_MAP[m].emoji}
            </button>
          ))}
        </div>
      </div>

      <div className="form-field">
        <label htmlFor="content">내용</label>
        <textarea
          id="content"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="오늘 하루를 기록해보세요"
          rows={8}
        />
        {errors.content && (
          <span className="field-error">{errors.content}</span>
        )}
      </div>

      <div className="form-actions">
        <button type="submit" className="btn btn-primary">
          {editingEntry ? "수정 완료" : "작성 완료"}
        </button>
        <button type="button" className="btn btn-secondary" onClick={onCancel}>
          취소
        </button>
      </div>
    </form>
  );
}
