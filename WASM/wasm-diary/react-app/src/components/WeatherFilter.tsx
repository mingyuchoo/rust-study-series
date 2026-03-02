import { ALL_WEATHERS, WEATHER_MAP } from "../constants/weather";
import type { Weather } from "../types/diary";

interface Props {
  selected: Weather | null;
  onSelect: (weather: Weather | null) => void;
}

export function WeatherFilter({ selected, onSelect }: Props) {
  return (
    <div className="weather-filter">
      <button
        className={`weather-filter-btn ${selected === null ? "active" : ""}`}
        onClick={() => onSelect(null)}
      >
        전체
      </button>
      {ALL_WEATHERS.map((weather) => (
        <button
          key={weather}
          className={`weather-filter-btn ${selected === weather ? "active" : ""}`}
          onClick={() => onSelect(weather)}
          title={WEATHER_MAP[weather].label}
        >
          {WEATHER_MAP[weather].emoji}
        </button>
      ))}
    </div>
  );
}
