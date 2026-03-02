import { useCallback, useState } from "react";
import "./App.css";
import { DateRangeFilter } from "./components/DateRangeFilter";
import { DiaryEntryForm } from "./components/DiaryEntryForm";
import { DiaryEntryList } from "./components/DiaryEntryList";
import { MoodFilter } from "./components/MoodFilter";
import { WeatherFilter } from "./components/WeatherFilter";
import { SearchBar } from "./components/SearchBar";
import { StatsDashboard } from "./components/StatsDashboard";
import { useDiary } from "./hooks/useDiary";
import { useWasm } from "./hooks/useWasm";
import type { DiaryEntry, Mood, Weather } from "./types/diary";

type View = "list" | "create" | "edit" | "stats";

function App() {
  const { ready, error } = useWasm();
  const diary = useDiary(ready);

  const [view, setView] = useState<View>("list");
  const [editingEntry, setEditingEntry] = useState<DiaryEntry | null>(null);
  const [filteredEntries, setFilteredEntries] = useState<DiaryEntry[] | null>(
    null
  );
  const [moodFilter, setMoodFilter] = useState<Mood | null>(null);
  const [weatherFilter, setWeatherFilter] = useState<Weather | null>(null);
  const [dateFrom, setDateFrom] = useState("");
  const [dateTo, setDateTo] = useState("");

  const displayEntries = filteredEntries ?? diary.entries;

  const handleSearch = useCallback(
    (keyword: string) => {
      if (!keyword.trim()) {
        setFilteredEntries(null);
        return;
      }
      setFilteredEntries(diary.searchByKeyword(keyword));
    },
    [diary]
  );

  const handleMoodFilter = useCallback(
    (mood: Mood | null) => {
      setMoodFilter(mood);
      if (mood === null) {
        setFilteredEntries(null);
      } else {
        setFilteredEntries(diary.filterByMood(mood));
      }
    },
    [diary]
  );

  const handleWeatherFilter = useCallback(
    (weather: Weather | null) => {
      setWeatherFilter(weather);
      if (weather === null) {
        setFilteredEntries(null);
      } else {
        setFilteredEntries(diary.filterByWeather(weather));
      }
    },
    [diary]
  );

  const handleDateFilter = useCallback(
    (from: string, to: string) => {
      if (from && to) {
        setFilteredEntries(diary.filterByDateRange(from, to));
      } else {
        setFilteredEntries(null);
      }
    },
    [diary]
  );

  const handleCreate = useCallback(
    (title: string, content: string, mood: Mood, weather: Weather) => {
      diary.createEntry(title, content, mood, weather);
      setView("list");
      setFilteredEntries(null);
    },
    [diary]
  );

  const handleEdit = useCallback(
    (id: string) => {
      const entry = diary.getEntry(id);
      if (entry) {
        setEditingEntry(entry);
        setView("edit");
      }
    },
    [diary]
  );

  const handleUpdate = useCallback(
    (title: string, content: string, mood: Mood, weather: Weather) => {
      if (editingEntry) {
        diary.updateEntry(editingEntry.id, title, content, mood, weather);
        setEditingEntry(null);
        setView("list");
        setFilteredEntries(null);
      }
    },
    [diary, editingEntry]
  );

  const handleDelete = useCallback(
    (id: string) => {
      if (window.confirm("정말 삭제하시겠습니까?")) {
        diary.deleteEntry(id);
        setFilteredEntries(null);
      }
    },
    [diary]
  );

  if (error) {
    return <div className="App error">WASM 로드 오류: {error}</div>;
  }

  if (!ready) {
    return <div className="App loading">로딩 중...</div>;
  }

  return (
    <div className="App">
      <header className="app-header">
        <h1>나의 일기장</h1>
        <nav className="app-nav">
          <button
            className={`nav-btn ${view === "list" ? "active" : ""}`}
            onClick={() => {
              setView("list");
              setFilteredEntries(null);
              setMoodFilter(null);
              setWeatherFilter(null);
            }}
          >
            목록
          </button>
          <button
            className={`nav-btn ${view === "create" ? "active" : ""}`}
            onClick={() => setView("create")}
          >
            새 일기
          </button>
          <button
            className={`nav-btn ${view === "stats" ? "active" : ""}`}
            onClick={() => setView("stats")}
          >
            통계
          </button>
        </nav>
      </header>

      <main className="app-main">
        {view === "list" && (
          <>
            <SearchBar onSearch={handleSearch} />
            <MoodFilter selected={moodFilter} onSelect={handleMoodFilter} />
            <WeatherFilter selected={weatherFilter} onSelect={handleWeatherFilter} />
            <DateRangeFilter
              from={dateFrom}
              to={dateTo}
              onFromChange={(d) => {
                setDateFrom(d);
                handleDateFilter(d, dateTo);
              }}
              onToChange={(d) => {
                setDateTo(d);
                handleDateFilter(dateFrom, d);
              }}
            />
            <DiaryEntryList
              entries={displayEntries}
              onEdit={handleEdit}
              onDelete={handleDelete}
            />
          </>
        )}

        {view === "create" && (
          <DiaryEntryForm
            onSubmit={handleCreate}
            onCancel={() => setView("list")}
            validate={diary.validate}
          />
        )}

        {view === "edit" && (
          <DiaryEntryForm
            editingEntry={editingEntry}
            onSubmit={handleUpdate}
            onCancel={() => {
              setEditingEntry(null);
              setView("list");
            }}
            validate={diary.validate}
          />
        )}

        {view === "stats" && diary.statistics && (
          <StatsDashboard
            statistics={diary.statistics}
            onBack={() => setView("list")}
          />
        )}
      </main>
    </div>
  );
}

export default App;
