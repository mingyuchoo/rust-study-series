/**
 * Application Constants and Default Values
 * Centralized constants used throughout the application
 */

import type { QueryConfig } from './api.js';
import type { ErrorHandlerConfig, RetryConfig } from './errors.js';

// File Upload Constants
export const FILE_UPLOAD_CONSTANTS = {
  MAX_FILE_SIZE: 10 * 1024 * 1024, // 10MB in bytes
  ACCEPTED_TYPES: ['application/pdf'],
  ACCEPTED_EXTENSIONS: ['.pdf'],
  CHUNK_SIZE: 1024 * 1024, // 1MB chunks for upload
} as const;

// Search Configuration Defaults
export const DEFAULT_QUERY_CONFIG: Required<QueryConfig> = {
  max_chunks: 5,
  similarity_threshold: 0.7,
  max_response_tokens: 1000,
  temperature: 0.3,
  include_low_confidence: false,
} as const;

// Search Configuration Limits
export const QUERY_CONFIG_LIMITS = {
  MAX_CHUNKS: { min: 1, max: 20 },
  SIMILARITY_THRESHOLD: { min: 0.0, max: 1.0 },
  MAX_RESPONSE_TOKENS: { min: 50, max: 4000 },
  TEMPERATURE: { min: 0.0, max: 1.0 },
  QUERY_LENGTH: { min: 3, max: 500 },
} as const;

// UI Constants
export const UI_CONSTANTS = {
  TOAST_DURATION: {
    SUCCESS: 3000,
    ERROR: 5000,
    WARNING: 4000,
    INFO: 3000,
  },
  DEBOUNCE_DELAY: 300,
  ANIMATION_DURATION: 200,
  POLLING_INTERVAL: 30000, // 30 seconds for health checks
} as const;

// API Constants
export const API_CONSTANTS = {
  ENDPOINTS: {
    UPLOAD: '/upload',
    QUERY: '/query',
    HEALTH: '/health',
  },
  TIMEOUT: 30000, // 30 seconds
  MAX_RETRIES: 3,
} as const;

// Error Handling Configuration
export const DEFAULT_ERROR_CONFIG: ErrorHandlerConfig = {
  enableRetry: true,
  maxRetries: 3,
  retryDelay: 1000,
  enableLogging: true,
  enableUserNotification: true,
} as const;

// Retry Configuration
export const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxAttempts: 3,
  baseDelay: 1000,
  maxDelay: 10000,
  backoffMultiplier: 2,
  retryableErrors: [
    'network_error' as const,
    'timeout_error' as const,
    'api_error' as const,
  ],
} as const;

// Responsive Breakpoints
export const BREAKPOINTS = {
  MOBILE: 320,
  TABLET: 768,
  DESKTOP: 1024,
  LARGE: 1440,
} as const;

// Theme Constants
export const THEME_CONSTANTS = {
  COLORS: {
    PRIMARY: '#2196f3',
    SECONDARY: '#9c27b0',
    SUCCESS: '#4caf50',
    WARNING: '#ff9800',
    ERROR: '#f44336',
    INFO: '#2196f3',
  },
  SPACING: {
    XS: '0.5rem',
    SM: '0.75rem',
    MD: '1rem',
    LG: '1.5rem',
    XL: '2rem',
  },
} as const;

// Validation Messages
export const VALIDATION_MESSAGES = {
  REQUIRED: 'This field is required',
  INVALID_EMAIL: 'Please enter a valid email address',
  INVALID_FILE_TYPE: 'Only PDF files are allowed',
  FILE_TOO_LARGE: 'File size must be less than 10MB',
  QUERY_TOO_SHORT: 'Query must be at least 3 characters long',
  QUERY_TOO_LONG: 'Query cannot exceed 500 characters',
  INVALID_NUMBER: 'Please enter a valid number',
  OUT_OF_RANGE: 'Value is out of allowed range',
} as const;

// Route Constants
export const ROUTES = {
  HOME: '/',
  UPLOAD: '/upload',
  SEARCH: '/search',
  DASHBOARD: '/dashboard',
} as const;

// Local Storage Keys
export const STORAGE_KEYS = {
  SEARCH_HISTORY: 'rag_search_history',
  USER_PREFERENCES: 'rag_user_preferences',
  QUERY_CONFIG: 'rag_query_config',
  THEME: 'rag_theme',
} as const;

// Performance Constants
export const PERFORMANCE_CONSTANTS = {
  LAZY_LOAD_THRESHOLD: 100, // pixels
  VIRTUAL_SCROLL_ITEM_HEIGHT: 50, // pixels
  MAX_SEARCH_HISTORY_ITEMS: 50,
  CACHE_TTL: 5 * 60 * 1000, // 5 minutes
} as const;