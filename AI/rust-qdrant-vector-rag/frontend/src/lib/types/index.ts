/**
 * Type Definitions Index
 * Central export point for all type definitions
 */

// API Types
export type {
  QueryConfig,
  QueryRequest,
  UploadResponse,
  SourceReference,
  RAGResponse,
  HealthResponse,
  ApiErrorResponse
} from './api.js';

// State Management Types
export type {
  AppState,
  UploadState,
  SearchState,
  SearchHistoryItem,
  HealthState,
  ToastMessage,
  ValidationError,
  NavigationState,
  FormState,
  FileUploadFormState,
  SearchFormState
} from './state.js';

// Error Handling Types
export type {
  AppError,
  NetworkError,
  ValidationError as ValidationErrorType,
  ApiError,
  UploadError,
  SearchError,
  ErrorContext,
  ErrorRecovery,
  ErrorHandlerConfig,
  RetryConfig
} from './errors.js';

export {
  ErrorTypeValues,
  ErrorSeverityValues,
  ErrorActionValues
} from './errors.js';

// Component Types
export type {
  BaseComponentProps,
  FileUploadProps,
  UploadProgressProps,
  UploadResultProps,
  SearchFormProps,
  SearchConfigProps,
  LoadingSpinnerProps,
  AnswerDisplayProps,
  SourceReferencesProps,
  SearchResultsProps,
  HealthStatusProps,
  SystemMetricsProps,
  NavigationProps,
  ErrorBoundaryProps,
  ToastProps,
  ModalProps,
  FormFieldProps,
  ButtonProps,
  InputProps,
  TextareaProps,
  SelectProps,
  SelectOption,
  SliderProps,
  CheckboxProps,
  ComponentEvents,
  A11yProps
} from './components.js';

// Validation Schema Types
export type {
  FileUploadInput,
  QueryConfigInput,
  SearchQueryInput,
  SearchFormInput,
  UploadFormInput,
  ToastMessageInput,
  ValidationErrorInput,
  EnvConfigInput
} from '../schemas/validation.js';

// Re-export validation schemas
export {
  FileUploadSchema,
  QueryConfigSchema,
  SearchQuerySchema,
  UploadResponseSchema,
  SourceReferenceSchema,
  RAGResponseSchema,
  HealthResponseSchema,
  SearchFormSchema,
  UploadFormSchema,
  EnvConfigSchema,
  ToastMessageSchema,
  ValidationErrorSchema
} from '../schemas/validation.js';

// Utility Types
export type {
  Partial,
  Required,
  Pick,
  Omit,
  PartialBy,
  RequiredBy,
  ArrayElement,
  Nullable,
  Optional,
  Maybe,
  DeepPartial,
  DeepRequired,
  Keys,
  Values,
  Record,
  ApiResponse,
  PaginatedResponse,
  FormFieldValue,
  FormData,
  ComponentRef,
  GenericError,
  Brand,
  DocumentId,
  ChunkId,
  UserId,
  SessionId,
  Timestamp,
  URL,
  Email,
  FileSize,
  Percentage,
  Score,
  Duration
} from './utils.js';

// Constants
export {
  FILE_UPLOAD_CONSTANTS,
  DEFAULT_QUERY_CONFIG,
  QUERY_CONFIG_LIMITS,
  UI_CONSTANTS,
  API_CONSTANTS,
  DEFAULT_ERROR_CONFIG,
  DEFAULT_RETRY_CONFIG,
  BREAKPOINTS,
  THEME_CONSTANTS,
  VALIDATION_MESSAGES,
  ROUTES,
  STORAGE_KEYS,
  PERFORMANCE_CONSTANTS
} from './constants.js';