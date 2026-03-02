import type { Mood } from "../types/diary";

interface MoodInfo {
  label: string;
  emoji: string;
}

export const MOOD_MAP: Record<Mood, MoodInfo> = {
  Happy: { label: "행복", emoji: "😊" },
  Sad: { label: "슬픔", emoji: "😢" },
  Angry: { label: "화남", emoji: "😡" },
  Anxious: { label: "불안", emoji: "😰" },
  Calm: { label: "평온", emoji: "😌" },
  Excited: { label: "신남", emoji: "🤩" },
  Tired: { label: "피곤", emoji: "😴" },
  Grateful: { label: "감사", emoji: "🙏" },
};

export const ALL_MOODS: Mood[] = Object.keys(MOOD_MAP) as Mood[];
