export type Mood =
  | "Happy"
  | "Sad"
  | "Angry"
  | "Anxious"
  | "Calm"
  | "Excited"
  | "Tired"
  | "Grateful";

export type Weather =
  | "Sunny"
  | "Cloudy"
  | "Rainy"
  | "Snowy"
  | "Windy"
  | "Foggy";

export interface DiaryEntry {
  id: string;
  title: string;
  content: string;
  mood: Mood;
  weather: Weather;
  created_at: string;
  updated_at: string;
}

export interface ValidationError {
  field: string;
  message: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export interface DiaryStatistics {
  total_entries: number;
  total_characters: number;
  total_words: number;
  mood_distribution: Record<string, number>;
  weather_distribution: Record<string, number>;
}
