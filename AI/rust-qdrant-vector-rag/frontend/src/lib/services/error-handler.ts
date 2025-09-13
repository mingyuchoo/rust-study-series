/**
 * Comprehensive Error Handling Service
 * Provides centralized error handling, recovery mechanisms, and user feedback
 */

import { toastActions } from '../stores/toast.store.js';
import { appActions } from '../stores/app.store.js';
import type {
  AppError,
  ErrorContext,
  ErrorRecovery,
  NetworkError,
  ApiError,
  UploadError,
  SearchError
} from '../types/errors.js';
import { ErrorTypeValues, ErrorSeverityValues } from '../types/errors.js';

// Network status monitoring
class NetworkMonitor {
  private isOnline = navigator.onLine;
  private listeners: Array<(isOnline: boolean) => void> = [];
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;

  constructor() {
    this.setupEventListeners();
  }

  private setupEventListeners(): void {
    window.addEventListener('online', this.handleOnline.bind(this));
    window.addEventListener('offline', this.handleOffline.bind(this));
  }

  private handleOnline(): void {
    this.isOnline = true;
    this.reconnectAttempts = 0;
    appActions.setOnlineStatus(true);
    this.notifyListeners(true);
    
    toastActions.success('Connection restored', {
      duration: 3000
    });
  }

  private handleOffline(): void {
    this.isOnline = false;
    appActions.setOnlineStatus(false);
    this.notifyListeners(false);
    
    toastActions.warning('You are currently offline. Some features may not be available.', {
      duration: 0 // Don't auto-dismiss offline notifications
    });
  }

  private notifyListeners(isOnline: boolean): void {
    this.listeners.forEach(listener => listener(isOnline));
  }

  public addListener(listener: (isOnline: boolean) => void): () => void {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  public getStatus(): boolean {
    return this.isOnline;
  }

  public async checkConnectivity(): Promise<boolean> {
    try {
      const response = await fetch('/api/health', {
        method: 'HEAD',
        cache: 'no-cache'
      });
      return response.ok;
    } catch {
      return false;
    }
  }
}

// Error message parser for user-friendly messages
class ErrorMessageParser {
  private static readonly USER_FRIENDLY_MESSAGES: Record<string, string> = {
    // Network errors
    'Failed to fetch': 'Unable to connect to the server. Please check your internet connection.',
    'NetworkError': 'Network connection failed. Please try again.',
    'TypeError: Failed to fetch': 'Connection failed. Please check your internet connection and try again.',
    
    // Timeout errors
    'Request timed out': 'The request took too long to complete. Please try again.',
    'AbortError': 'The request was cancelled due to timeout. Please try again.',
    
    // API errors
    'Internal Server Error': 'The server encountered an error. Please try again later.',
    'Service Unavailable': 'The service is temporarily unavailable. Please try again later.',
    'Bad Gateway': 'Server communication error. Please try again.',
    'Gateway Timeout': 'The server took too long to respond. Please try again.',
    
    // File upload errors
    'File too large': 'The selected file is too large. Please choose a smaller file.',
    'Invalid file type': 'Please select a valid PDF file.',
    'Upload failed': 'File upload failed. Please try again.',
    
    // Search errors
    'No results found': 'No relevant information found for your query. Try rephrasing your question.',
    'Query too short': 'Please enter a longer search query.',
    'Query too long': 'Your search query is too long. Please shorten it.',
    
    // Authentication errors
    'Unauthorized': 'You are not authorized to perform this action.',
    'Forbidden': 'Access denied. You do not have permission to access this resource.',
    
    // Default fallbacks
    'Unknown error': 'An unexpected error occurred. Please try again.'
  };

  public static parseError(error: AppError): string {
    // Check for exact message matches first
    if (this.USER_FRIENDLY_MESSAGES[error.message]) {
      return this.USER_FRIENDLY_MESSAGES[error.message];
    }

    // Check for partial message matches
    for (const [key, value] of Object.entries(this.USER_FRIENDLY_MESSAGES)) {
      if (error.message.toLowerCase().includes(key.toLowerCase())) {
        return value;
      }
    }

    // Handle specific error types
    switch (error.type) {
      case ErrorTypeValues.NETWORK_ERROR:
        return 'Network connection failed. Please check your internet connection and try again.';
      
      case ErrorTypeValues.TIMEOUT_ERROR:
        return 'The request timed out. Please try again.';
      
      case ErrorTypeValues.API_ERROR:
        const apiError = error as ApiError;
        if (apiError.statusCode >= 500) {
          return 'Server error occurred. Please try again later.';
        } else if (apiError.statusCode >= 400) {
          return 'Request failed. Please check your input and try again.';
        }
        break;
      
      case ErrorTypeValues.UPLOAD_ERROR:
        const uploadError = error as UploadError;
        switch (uploadError.reason) {
          case 'file_too_large':
            return 'The selected file is too large. Please choose a smaller file.';
          case 'invalid_type':
            return 'Please select a valid PDF file.';
          case 'upload_failed':
            return 'File upload failed. Please try again.';
          case 'processing_failed':
            return 'File processing failed. Please try uploading again.';
        }
        break;
      
      case ErrorTypeValues.SEARCH_ERROR:
        const searchError = error as SearchError;
        switch (searchError.reason) {
          case 'no_results':
            return 'No relevant information found. Try rephrasing your question.';
          case 'query_too_short':
            return 'Please enter a longer search query.';
          case 'query_too_long':
            return 'Your search query is too long. Please shorten it.';
          case 'service_unavailable':
            return 'Search service is temporarily unavailable. Please try again later.';
        }
        break;
    }

    // Fallback to original message or generic error
    return error.message || 'An unexpected error occurred. Please try again.';
  }
}

// Error recovery strategies
class ErrorRecoveryManager {
  private static createRetryRecovery(
    action: () => Promise<void>,
    label = 'Try Again'
  ): ErrorRecovery {
    return {
      action: 'retry',
      label,
      handler: action
    };
  }

  private static createRefreshRecovery(): ErrorRecovery {
    return {
      action: 'refresh',
      label: 'Refresh Page',
      handler: () => {
        window.location.reload();
      }
    };
  }

  private static createNavigateHomeRecovery(): ErrorRecovery {
    return {
      action: 'navigate_home',
      label: 'Go Home',
      handler: () => {
        appActions.setCurrentPage('upload');
        appActions.clearError();
      }
    };
  }

  private static createContactSupportRecovery(): ErrorRecovery {
    return {
      action: 'contact_support',
      label: 'Contact Support',
      handler: () => {
        // In a real app, this would open a support form or email
        window.open('mailto:support@example.com?subject=Error Report', '_blank');
      }
    };
  }

  private static createDismissRecovery(): ErrorRecovery {
    return {
      action: 'dismiss',
      label: 'Dismiss',
      handler: () => {
        appActions.clearError();
      }
    };
  }

  public static getRecoveryOptions(
    error: AppError,
    retryAction?: () => Promise<void>
  ): ErrorRecovery[] {
    const options: ErrorRecovery[] = [];

    // Add retry option for retryable errors
    if (error.retryable && retryAction) {
      options.push(this.createRetryRecovery(retryAction));
    }

    // Add specific recovery options based on error type
    switch (error.type) {
      case ErrorTypeValues.NETWORK_ERROR:
        if (retryAction) {
          options.push(this.createRetryRecovery(retryAction, 'Retry Connection'));
        }
        options.push(this.createRefreshRecovery());
        break;

      case ErrorTypeValues.API_ERROR:
        const apiError = error as ApiError;
        if (apiError.statusCode >= 500 && retryAction) {
          options.push(this.createRetryRecovery(retryAction));
        }
        if (apiError.statusCode >= 400 && apiError.statusCode < 500) {
          options.push(this.createNavigateHomeRecovery());
        }
        break;

      case ErrorTypeValues.UPLOAD_ERROR:
        if (retryAction) {
          options.push(this.createRetryRecovery(retryAction, 'Try Upload Again'));
        }
        options.push(this.createNavigateHomeRecovery());
        break;

      case ErrorTypeValues.SEARCH_ERROR:
        if (retryAction) {
          options.push(this.createRetryRecovery(retryAction, 'Search Again'));
        }
        break;

      case ErrorTypeValues.TIMEOUT_ERROR:
        if (retryAction) {
          options.push(this.createRetryRecovery(retryAction));
        }
        options.push(this.createRefreshRecovery());
        break;

      default:
        if (retryAction) {
          options.push(this.createRetryRecovery(retryAction));
        }
        options.push(this.createRefreshRecovery());
        break;
    }

    // Add contact support for critical errors
    if (error.severity === ErrorSeverityValues.CRITICAL) {
      options.push(this.createContactSupportRecovery());
    }

    // Always add dismiss option
    options.push(this.createDismissRecovery());

    return options;
  }
}

// Main error handler service
export class ErrorHandlerService {
  private networkMonitor: NetworkMonitor;
  private errorLog: AppError[] = [];
  private maxErrorLogSize = 100;

  constructor() {
    this.networkMonitor = new NetworkMonitor();
    this.setupGlobalErrorHandlers();
  }

  private setupGlobalErrorHandlers(): void {
    // Handle unhandled promise rejections
    window.addEventListener('unhandledrejection', (event) => {
      const error = this.createErrorFromRejection(event.reason);
      this.handleError(error);
      event.preventDefault(); // Prevent console logging
    });

    // Handle global JavaScript errors
    window.addEventListener('error', (event) => {
      const error = this.createErrorFromEvent(event);
      this.handleError(error);
    });
  }

  private createErrorFromRejection(reason: unknown): AppError {
    if (reason instanceof Error) {
      return {
        type: ErrorTypeValues.UNKNOWN_ERROR,
        message: reason.message,
        details: { stack: reason.stack },
        retryable: false,
        severity: ErrorSeverityValues.HIGH,
        timestamp: new Date()
      };
    }

    return {
      type: ErrorTypeValues.UNKNOWN_ERROR,
      message: String(reason) || 'An unexpected error occurred',
      details: reason,
      retryable: false,
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date()
    };
  }

  private createErrorFromEvent(event: ErrorEvent): AppError {
    return {
      type: ErrorTypeValues.UNKNOWN_ERROR,
      message: event.message || 'JavaScript error occurred',
      details: {
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
        stack: event.error?.stack
      },
      retryable: false,
      severity: ErrorSeverityValues.HIGH,
      timestamp: new Date()
    };
  }

  private logError(error: AppError): void {
    // Add to error log
    this.errorLog.unshift(error);
    
    // Maintain log size
    if (this.errorLog.length > this.maxErrorLogSize) {
      this.errorLog = this.errorLog.slice(0, this.maxErrorLogSize);
    }

    // Log to console in development
    if (import.meta.env.DEV) {
      console.error('Error logged:', error);
    }

    // In production, send to error reporting service
    if (import.meta.env.PROD) {
      this.reportError(error);
    }
  }

  private async reportError(error: AppError): Promise<void> {
    try {
      // This would send to an error reporting service like Sentry
      const errorReport = {
        type: error.type,
        message: error.message,
        details: error.details,
        severity: error.severity,
        timestamp: error.timestamp.toISOString(),
        userAgent: navigator.userAgent,
        url: window.location.href,
        userId: 'anonymous' // Would be actual user ID in real app
      };

      // Placeholder for actual error reporting
      console.log('Error report (would be sent to service):', errorReport);
    } catch (reportingError) {
      console.error('Failed to report error:', reportingError);
    }
  }

  public handleError(
    error: AppError,
    retryAction?: () => Promise<void>,
    showToast = true
  ): ErrorContext {
    // Log the error
    this.logError(error);

    // Parse user-friendly message
    const userMessage = ErrorMessageParser.parseError(error);

    // Get recovery options
    const recoveryOptions = ErrorRecoveryManager.getRecoveryOptions(error, retryAction);

    // Create error context
    const errorContext: ErrorContext = {
      error,
      recoveryOptions,
      userMessage,
      technicalMessage: error.message
    };

    // Show toast notification based on severity
    if (showToast) {
      this.showErrorToast(error, userMessage, recoveryOptions);
    }

    // Update global error state for critical errors
    if (error.severity === ErrorSeverityValues.CRITICAL) {
      appActions.setError(userMessage);
    }

    return errorContext;
  }

  private showErrorToast(
    error: AppError,
    userMessage: string,
    recoveryOptions: ErrorRecovery[]
  ): void {
    const toastType = error.severity === ErrorSeverityValues.CRITICAL ? 'error' : 'warning';
    const duration = error.severity === ErrorSeverityValues.LOW ? 5000 : 0; // Don't auto-dismiss serious errors

    toastActions.add({
      type: toastType,
      message: userMessage,
      duration,
      dismissible: true
    });
  }

  public addNetworkListener(listener: (isOnline: boolean) => void): () => void {
    return this.networkMonitor.addListener(listener);
  }

  public isOnline(): boolean {
    return this.networkMonitor.getStatus();
  }

  public async checkConnectivity(): Promise<boolean> {
    return this.networkMonitor.checkConnectivity();
  }

  public getErrorLog(): AppError[] {
    return [...this.errorLog];
  }

  public clearErrorLog(): void {
    this.errorLog = [];
  }

  // Utility method to create specific error types
  public createNetworkError(
    message: string,
    details?: { url?: string; method?: string; statusCode?: number }
  ): NetworkError {
    return {
      type: ErrorTypeValues.NETWORK_ERROR,
      message,
      details,
      retryable: true,
      severity: ErrorSeverityValues.HIGH,
      timestamp: new Date(),
      statusCode: details?.statusCode,
      url: details?.url,
      method: details?.method
    };
  }

  public createUploadError(
    message: string,
    filename: string,
    reason: UploadError['reason'],
    details?: { fileSize?: number; fileType?: string }
  ): UploadError {
    return {
      type: ErrorTypeValues.UPLOAD_ERROR,
      message,
      filename,
      reason,
      details,
      retryable: reason === 'upload_failed' || reason === 'processing_failed',
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date(),
      fileSize: details?.fileSize,
      fileType: details?.fileType
    };
  }

  public createSearchError(
    message: string,
    query: string,
    reason: SearchError['reason']
  ): SearchError {
    return {
      type: ErrorTypeValues.SEARCH_ERROR,
      message,
      query,
      reason,
      retryable: reason === 'service_unavailable',
      severity: reason === 'service_unavailable' ? ErrorSeverityValues.HIGH : ErrorSeverityValues.LOW,
      timestamp: new Date()
    };
  }
}

// Export singleton instance
export const errorHandler = new ErrorHandlerService();

// Export utility functions
export { ErrorMessageParser, ErrorRecoveryManager };