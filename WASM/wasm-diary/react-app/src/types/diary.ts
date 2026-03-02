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

export type Role = "Admin" | "User";

export interface DiaryEntry {
  id: string;
  owner_id: string;
  title: string;
  content: string;
  mood: Mood;
  weather: Weather;
  created_at: string;
  updated_at: string;
}

export interface UserView {
  id: string;
  username: string;
  nickname?: string;
  role: Role;
  created_at: string;
}

export interface LoginResult {
  success: boolean;
  user_id?: string;
  username?: string;
  nickname?: string;
  role?: Role;
  error?: string;
}

export interface RegistrationValidation {
  valid: boolean;
  username_error?: string;
  password_error?: string;
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
