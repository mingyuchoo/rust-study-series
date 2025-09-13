/**
 * Error Handling Types and Enums
 * Comprehensive error handling system for the application
 */

// Error Type Enumeration
export const ErrorTypeValues = {
  NETWORK_ERROR: 'network_error',
  VALIDATION_ERROR: 'validation_error',
  API_ERROR: 'api_error',
  UPLOAD_ERROR: 'upload_error',
  SEARCH_ERROR: 'search_error',
  AUTHENTICATION_ERROR: 'authentication_error',
  PERMISSION_ERROR: 'permission_error',
  TIMEOUT_ERROR: 'timeout_error',
  UNKNOWN_ERROR: 'unknown_error'
} as const;

export type ErrorType = typeof ErrorTypeValues[keyof typeof ErrorTypeValues];

// Error Severity Levels
export const ErrorSeverityValues = {
  LOW: 'low',
  MEDIUM: 'medium',
  HIGH: 'high',
  CRITICAL: 'critical'
} as const;

export type ErrorSeverity = typeof ErrorSeverityValues[keyof typeof ErrorSeverityValues];

// Base Application Error Interface
export interface AppError {
  type: ErrorType;
  message: string;
  details?: unknown;
  retryable: boolean;
  severity: ErrorSeverity;
  timestamp: Date;
  code?: string;
  stack?: string;
}

// Network Error Specific
export interface NetworkError extends AppError {
  type: typeof ErrorTypeValues.NETWORK_ERROR;
  statusCode?: number;
  url?: string;
  method?: string;
}

// Validation Error Specific
export interface ValidationError extends AppError {
  type: typeof ErrorTypeValues.VALIDATION_ERROR;
  field: string;
  value?: unknown;
  constraints?: string[];
}

// API Error Specific
export interface ApiError extends AppError {
  type: typeof ErrorTypeValues.API_ERROR;
  statusCode: number;
  endpoint: string;
  method: string;
  requestId?: string;
}

// Upload Error Specific
export interface UploadError extends AppError {
  type: typeof ErrorTypeValues.UPLOAD_ERROR;
  filename: string;
  fileSize?: number;
  fileType?: string;
  reason: 'file_too_large' | 'invalid_type' | 'upload_failed' | 'processing_failed';
}

// Search Error Specific
export interface SearchError extends AppError {
  type: typeof ErrorTypeValues.SEARCH_ERROR;
  query: string;
  reason: 'no_results' | 'query_too_short' | 'query_too_long' | 'service_unavailable';
}

// Error Recovery Actions
export const ErrorActionValues = {
  RETRY: 'retry',
  REFRESH: 'refresh',
  NAVIGATE_HOME: 'navigate_home',
  CONTACT_SUPPORT: 'contact_support',
  DISMISS: 'dismiss'
} as const;

export type ErrorAction = typeof ErrorActionValues[keyof typeof ErrorActionValues];

// Error Recovery Strategy
export interface ErrorRecovery {
  action: ErrorAction;
  label: string;
  handler: () => void | Promise<void>;
}

// Complete Error Context
export interface ErrorContext {
  error: AppError;
  recoveryOptions: ErrorRecovery[];
  userMessage: string;
  technicalMessage?: string;
}

// Error Handler Configuration
export interface ErrorHandlerConfig {
  enableRetry: boolean;
  maxRetries: number;
  retryDelay: number;
  enableLogging: boolean;
  enableUserNotification: boolean;
}

// Retry Configuration
export interface RetryConfig {
  maxAttempts: number;
  baseDelay: number;
  maxDelay: number;
  backoffMultiplier: number;
  retryableErrors: ErrorType[];
}