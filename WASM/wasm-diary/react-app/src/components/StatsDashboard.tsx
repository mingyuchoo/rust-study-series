import { MOOD_MAP } from "../constants/mood";
import { WEATHER_MAP } from "../constants/weather";
import type { DiaryStatistics, Mood, Weather } from "../types/diary";

interface Props {
  statistics: DiaryStatistics;
  onBack: () => void;
}

export function StatsDashboard({ statistics, onBack }: Props) {
  const moodEntries = Object.entries(statistics.mood_distribution).sort(
    ([, a], [, b]) => b - a
  );
  const weatherEntries = Object.entries(statistics.weather_distribution).sort(
    ([, a], [, b]) => b - a
  );

  return (
    <div className="stats-dashboard">
      <h2>통계</h2>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-value">{statistics.total_entries}</div>
          <div className="stat-label">총 일기 수</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{statistics.total_characters}</div>
          <div className="stat-label">총 글자 수</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{statistics.total_words}</div>
          <div className="stat-label">총 어절 수</div>
        </div>
      </div>

      {moodEntries.length > 0 && (
        <div className="mood-stats">
          <h3>감정 분포</h3>
          <div className="mood-bars">
            {moodEntries.map(([mood, count]) => {
              const info = MOOD_MAP[mood as Mood];
              const percent =
                statistics.total_entries > 0
                  ? Math.round((count / statistics.total_entries) * 100)
                  : 0;
              return (
                <div key={mood} className="mood-bar-row">
                  <span className="mood-bar-label">
                    {info?.emoji ?? mood} {info?.label ?? mood}
                  </span>
                  <div className="mood-bar-track">
                    <div
                      className="mood-bar-fill"
                      style={{ width: `${percent}%` }}
                    />
                  </div>
                  <span className="mood-bar-count">
                    {count}건 ({percent}%)
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {weatherEntries.length > 0 && (
        <div className="weather-stats">
          <h3>날씨 분포</h3>
          <div className="mood-bars">
            {weatherEntries.map(([weather, count]) => {
              const info = WEATHER_MAP[weather as Weather];
              const percent =
                statistics.total_entries > 0
                  ? Math.round((count / statistics.total_entries) * 100)
                  : 0;
              return (
                <div key={weather} className="mood-bar-row">
                  <span className="mood-bar-label">
                    {info?.emoji ?? weather} {info?.label ?? weather}
                  </span>
                  <div className="mood-bar-track">
                    <div
                      className="mood-bar-fill weather-bar-fill"
                      style={{ width: `${percent}%` }}
                    />
                  </div>
                  <span className="mood-bar-count">
                    {count}건 ({percent}%)
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      )}

      <button className="btn btn-secondary" onClick={onBack}>
        돌아가기
      </button>
    </div>
  );
}
