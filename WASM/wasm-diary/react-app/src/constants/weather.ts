import type { Weather } from "../types/diary";

interface WeatherInfo {
  label: string;
  emoji: string;
}

export const WEATHER_MAP: Record<Weather, WeatherInfo> = {
  Sunny: { label: "맑음", emoji: "\u2600\uFE0F" },
  Cloudy: { label: "흐림", emoji: "\u2601\uFE0F" },
  Rainy: { label: "비", emoji: "\uD83C\uDF27\uFE0F" },
  Snowy: { label: "눈", emoji: "\u2744\uFE0F" },
  Windy: { label: "바람", emoji: "\uD83C\uDF2C\uFE0F" },
  Foggy: { label: "안개", emoji: "\uD83C\uDF2B\uFE0F" },
};

export const ALL_WEATHERS: Weather[] = Object.keys(WEATHER_MAP) as Weather[];
