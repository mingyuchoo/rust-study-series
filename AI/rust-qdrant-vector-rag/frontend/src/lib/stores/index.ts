/**
 * Store Exports
 * Central export point for all application stores
 */

// App Store
export {
  appStore,
  isLoading,
  currentError,
  currentPage,
  isOnline,
  appActions
} from './app.store.js';

// Upload Store
export {
  uploadStore,
  uploadProgress,
  isUploading,
  uploadResult,
  selectedFile,
  dragActive,
  uploadComplete,
  hasUploadError,
  uploadSuccess,
  uploadActions
} from './upload.store.js';

// Search Store
export {
  searchStore,
  searchQuery,
  searchResults,
  isSearching,
  searchConfig,
  searchHistory,
  hasResults,
  hasSearchError,
  queryCharacterCount,
  isQueryValid,
  searchActions
} from './search.store.js';

// Health Store
export {
  healthStore,
  healthStatus,
  isHealthChecking,
  lastHealthCheck,
  healthCheckInterval,
  isSystemHealthy,
  isSystemUnhealthy,
  systemUptime,
  serviceStatuses,
  isQdrantHealthy,
  isOpenAIHealthy,
  timeSinceLastCheck,
  isHealthDataStale,
  healthActions,
  healthAutoRefresh
} from './health.store.js';

// Toast Store
export {
  toastStore,
  activeToasts,
  toastCount,
  hasToasts,
  errorToasts,
  successToasts,
  warningToasts,
  infoToasts,
  toastActions,
  toastQueue,
  toastUtils
} from './toast.store.js';

// Re-export types for convenience
export type {
  AppState,
  UploadState,
  SearchState,
  HealthState,
  ToastMessage,
  SearchHistoryItem
} from '../types/state.js';