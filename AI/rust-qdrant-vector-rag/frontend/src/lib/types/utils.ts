/**
 * Utility Types
 * Common utility types and type helpers used throughout the application
 */

// Make all properties optional
export type Partial<T> = {
  [P in keyof T]?: T[P];
};

// Make all properties required
export type Required<T> = {
  [P in keyof T]-?: T[P];
};

// Pick specific properties from a type
export type Pick<T, K extends keyof T> = {
  [P in K]: T[P];
};

// Omit specific properties from a type
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;

// Create a type with some properties optional
export type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

// Create a type with some properties required
export type RequiredBy<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

// Extract the type of array elements
export type ArrayElement<T> = T extends (infer U)[] ? U : never;

// Make a type nullable
export type Nullable<T> = T | null;

// Make a type optional (undefined)
export type Optional<T> = T | undefined;

// Make a type nullable or undefined
export type Maybe<T> = T | null | undefined;

// Deep partial - makes all nested properties optional
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

// Deep required - makes all nested properties required
export type DeepRequired<T> = {
  [P in keyof T]-?: T[P] extends object ? DeepRequired<T[P]> : T[P];
};

// Create a union of all property names of type T
export type Keys<T> = keyof T;

// Create a union of all property values of type T
export type Values<T> = T[keyof T];

// Create a record type with specific keys and value type
export type Record<K extends string | number | symbol, T> = {
  [_P in K]: T;
};

// API Response wrapper
export type ApiResponse<T> = {
  data: T;
  success: boolean;
  message?: string;
  errors?: string[];
};

// Paginated response type
export type PaginatedResponse<T> = {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
    hasNext: boolean;
    hasPrev: boolean;
  };
};

// Form field value type
export type FormFieldValue = string | number | boolean | globalThis.File | null | undefined;

// Form data type
export type FormData<T = Record<string, FormFieldValue>> = T;

// Component ref type
export type ComponentRef<T = globalThis.HTMLElement> = T | null;

// Generic error type
export type GenericError = Error | string | unknown;

// Branded types for type safety
export type Brand<T, B> = T & { __brand: B };

// ID types
export type DocumentId = Brand<string, 'DocumentId'>;
export type ChunkId = Brand<string, 'ChunkId'>;
export type UserId = Brand<string, 'UserId'>;
export type SessionId = Brand<string, 'SessionId'>;

// Timestamp type
export type Timestamp = Brand<string, 'Timestamp'>;

// URL type
export type URL = Brand<string, 'URL'>;

// Email type
export type Email = Brand<string, 'Email'>;

// File size type (in bytes)
export type FileSize = Brand<number, 'FileSize'>;

// Percentage type (0-100)
export type Percentage = Brand<number, 'Percentage'>;

// Score type (0-1)
export type Score = Brand<number, 'Score'>;

// Duration type (in milliseconds)
export type Duration = Brand<number, 'Duration'>;