import { useCallback, useEffect, useRef, useState } from "react";
import { DiaryManager, Mood as WasmMood, Weather as WasmWeather } from "wasm-lib";
import type {
  DiaryEntry,
  DiaryStatistics,
  Mood,
  Weather,
  ValidationResult,
} from "../types/diary";

const STORAGE_KEY = "diary-entries";

// Mood 문자열을 WASM Mood enum 값으로 변환
const MOOD_TO_WASM: Record<Mood, number> = {
  Happy: WasmMood.Happy,
  Sad: WasmMood.Sad,
  Angry: WasmMood.Angry,
  Anxious: WasmMood.Anxious,
  Calm: WasmMood.Calm,
  Excited: WasmMood.Excited,
  Tired: WasmMood.Tired,
  Grateful: WasmMood.Grateful,
};

function toWasmMood(mood: Mood): number {
  return MOOD_TO_WASM[mood];
}

const WEATHER_TO_WASM: Record<Weather, number> = {
  Sunny: WasmWeather.Sunny,
  Cloudy: WasmWeather.Cloudy,
  Rainy: WasmWeather.Rainy,
  Snowy: WasmWeather.Snowy,
  Windy: WasmWeather.Windy,
  Foggy: WasmWeather.Foggy,
};

function toWasmWeather(weather: Weather): number {
  return WEATHER_TO_WASM[weather];
}

export interface UseDiaryReturn {
  entries: DiaryEntry[];
  statistics: DiaryStatistics | null;
  createEntry: (title: string, content: string, mood: Mood, weather: Weather) => DiaryEntry;
  updateEntry: (
    id: string,
    title: string,
    content: string,
    mood: Mood,
    weather: Weather
  ) => DiaryEntry | null;
  deleteEntry: (id: string) => boolean;
  getEntry: (id: string) => DiaryEntry | null;
  searchByKeyword: (keyword: string) => DiaryEntry[];
  filterByMood: (mood: Mood) => DiaryEntry[];
  filterByWeather: (weather: Weather) => DiaryEntry[];
  filterByDateRange: (from: string, to: string) => DiaryEntry[];
  validate: (title: string, content: string) => ValidationResult;
  refreshEntries: () => void;
}

export function useDiary(
  wasmReady: boolean,
  userId?: string,
  isAdmin?: boolean
): UseDiaryReturn {
  const managerRef = useRef<DiaryManager | null>(null);
  const [entries, setEntries] = useState<DiaryEntry[]>([]);
  const [statistics, setStatistics] = useState<DiaryStatistics | null>(null);

  const getEntries = useCallback(
    (mgr: DiaryManager): DiaryEntry[] => {
      if (isAdmin) {
        return JSON.parse(mgr.get_all_entries());
      }
      if (userId) {
        return JSON.parse(mgr.get_entries_by_owner(userId));
      }
      return [];
    },
    [userId, isAdmin]
  );

  // WASM 초기화 후 매니저 생성 및 localStorage에서 복원
  useEffect(() => {
    if (!wasmReady) return;

    const manager = new DiaryManager();
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      manager.load_from_json(saved);
    }
    managerRef.current = manager;

    setEntries(getEntries(manager));
    setStatistics(JSON.parse(manager.get_statistics()));

    return () => {
      manager.free();
      managerRef.current = null;
    };
  }, [wasmReady, getEntries]);

  const persist = useCallback(() => {
    const mgr = managerRef.current;
    if (!mgr) return;
    localStorage.setItem(STORAGE_KEY, mgr.save_to_json());
    setEntries(getEntries(mgr));
    setStatistics(JSON.parse(mgr.get_statistics()));
  }, [getEntries]);

  const createEntry = useCallback(
    (title: string, content: string, mood: Mood, weather: Weather): DiaryEntry => {
      const mgr = managerRef.current!;
      const json = mgr.create_entry(userId ?? "", title, content, toWasmMood(mood), toWasmWeather(weather));
      persist();
      return JSON.parse(json);
    },
    [persist, userId]
  );

  const updateEntry = useCallback(
    (
      id: string,
      title: string,
      content: string,
      mood: Mood,
      weather: Weather
    ): DiaryEntry | null => {
      const mgr = managerRef.current!;
      const json = mgr.update_entry(id, title, content, toWasmMood(mood), toWasmWeather(weather));
      if (!json) return null;
      persist();
      return JSON.parse(json);
    },
    [persist]
  );

  const deleteEntry = useCallback(
    (id: string): boolean => {
      const mgr = managerRef.current!;
      const result = mgr.delete_entry(id);
      if (result) persist();
      return result;
    },
    [persist]
  );

  const getEntry = useCallback((id: string): DiaryEntry | null => {
    const mgr = managerRef.current!;
    const json = mgr.get_entry(id);
    return json ? JSON.parse(json) : null;
  }, []);

  const searchByKeyword = useCallback((keyword: string): DiaryEntry[] => {
    const mgr = managerRef.current!;
    return JSON.parse(mgr.search_by_keyword(keyword));
  }, []);

  const filterByMood = useCallback((mood: Mood): DiaryEntry[] => {
    const mgr = managerRef.current!;
    return JSON.parse(mgr.filter_by_mood(toWasmMood(mood)));
  }, []);

  const filterByWeather = useCallback((weather: Weather): DiaryEntry[] => {
    const mgr = managerRef.current!;
    return JSON.parse(mgr.filter_by_weather(toWasmWeather(weather)));
  }, []);

  const filterByDateRange = useCallback(
    (from: string, to: string): DiaryEntry[] => {
      const mgr = managerRef.current!;
      return JSON.parse(mgr.filter_by_date_range(from, to));
    },
    []
  );

  const validate = useCallback(
    (title: string, content: string): ValidationResult => {
      return JSON.parse(DiaryManager.validate(title, content));
    },
    []
  );

  const refreshEntries = useCallback(() => {
    const mgr = managerRef.current;
    if (!mgr) return;
    setEntries(getEntries(mgr));
    setStatistics(JSON.parse(mgr.get_statistics()));
  }, [getEntries]);

  return {
    entries,
    statistics,
    createEntry,
    updateEntry,
    deleteEntry,
    getEntry,
    searchByKeyword,
    filterByMood,
    filterByWeather,
    filterByDateRange,
    validate,
    refreshEntries,
  };
}
