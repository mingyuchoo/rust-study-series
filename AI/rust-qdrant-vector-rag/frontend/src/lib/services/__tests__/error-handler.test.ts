/**
 * Error Handler Service Tests
 * Tests for comprehensive error handling functionality
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { ErrorHandlerService, ErrorMessageParser, ErrorRecoveryManager } from '../error-handler.js';
import { ErrorTypeValues, ErrorSeverityValues } from '../../types/errors.js';
import type { AppError, NetworkError, UploadError, SearchError } from '../../types/errors.js';

// Mock dependencies
vi.mock('../../stores/toast.store.js', () => ({
  toastActions: {
    add: vi.fn(),
    success: vi.fn(),
    error: vi.fn(),
    warning: vi.fn(),
    info: vi.fn()
  }
}));

vi.mock('../../stores/app.store.js', () => ({
  appActions: {
    setOnlineStatus: vi.fn(),
    setError: vi.fn(),
    clearError: vi.fn()
  }
}));

describe('ErrorHandlerService', () => {
  let errorHandler: ErrorHandlerService;
  let mockFetch: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks();
    
    // Mock fetch for connectivity checks
    mockFetch = vi.fn();
    global.fetch = mockFetch;
    
    // Mock navigator.onLine
    Object.defineProperty(navigator, 'onLine', {
      writable: true,
      value: true
    });

    // Mock window event listeners
    global.window.addEventListener = vi.fn();
    global.window.removeEventListener = vi.fn();

    errorHandler = new ErrorHandlerService();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Network Error Creation', () => {
    it('should create a network error with correct properties', () => {
      const error = errorHandler.createNetworkError(
        'Connection failed',
        { url: '/api/test', method: 'GET', statusCode: 0 }
      );

      expect(error.type).toBe(ErrorTypeValues.NETWORK_ERROR);
      expect(error.message).toBe('Connection failed');
      expect(error.retryable).toBe(true);
      expect(error.severity).toBe(ErrorSeverityValues.HIGH);
      expect(error.url).toBe('/api/test');
      expect(error.method).toBe('GET');
      expect(error.statusCode).toBe(0);
      expect(error.timestamp).toBeInstanceOf(Date);
    });
  });

  describe('Upload Error Creation', () => {
    it('should create an upload error with correct properties', () => {
      const error = errorHandler.createUploadError(
        'File too large',
        'test.pdf',
        'file_too_large',
        { fileSize: 1000000, fileType: 'application/pdf' }
      );

      expect(error.type).toBe(ErrorTypeValues.UPLOAD_ERROR);
      expect(error.message).toBe('File too large');
      expect(error.filename).toBe('test.pdf');
      expect(error.reason).toBe('file_too_large');
      expect(error.retryable).toBe(false);
      expect(error.severity).toBe(ErrorSeverityValues.MEDIUM);
      expect(error.fileSize).toBe(1000000);
      expect(error.fileType).toBe('application/pdf');
    });

    it('should mark upload errors as retryable for certain reasons', () => {
      const retryableError = errorHandler.createUploadError(
        'Upload failed',
        'test.pdf',
        'upload_failed'
      );

      const nonRetryableError = errorHandler.createUploadError(
        'Invalid type',
        'test.txt',
        'invalid_type'
      );

      expect(retryableError.retryable).toBe(true);
      expect(nonRetryableError.retryable).toBe(false);
    });
  });

  describe('Search Error Creation', () => {
    it('should create a search error with correct properties', () => {
      const error = errorHandler.createSearchError(
        'No results found',
        'test query',
        'no_results'
      );

      expect(error.type).toBe(ErrorTypeValues.SEARCH_ERROR);
      expect(error.message).toBe('No results found');
      expect(error.query).toBe('test query');
      expect(error.reason).toBe('no_results');
      expect(error.retryable).toBe(false);
      expect(error.severity).toBe(ErrorSeverityValues.LOW);
    });

    it('should mark service unavailable errors as retryable', () => {
      const error = errorHandler.createSearchError(
        'Service unavailable',
        'test query',
        'service_unavailable'
      );

      expect(error.retryable).toBe(true);
      expect(error.severity).toBe(ErrorSeverityValues.HIGH);
    });
  });

  describe('Error Handling', () => {
    it('should handle errors and return error context', () => {
      const testError: AppError = {
        type: ErrorTypeValues.NETWORK_ERROR,
        message: 'Connection failed',
        retryable: true,
        severity: ErrorSeverityValues.HIGH,
        timestamp: new Date()
      };

      const retryAction = vi.fn();
      const context = errorHandler.handleError(testError, retryAction, false);

      expect(context.error).toBe(testError);
      expect(context.userMessage).toBe('Network connection failed. Please check your internet connection and try again.');
      expect(context.recoveryOptions).toHaveLength(3); // retry, refresh, dismiss
      expect(context.recoveryOptions[0].action).toBe('retry');
    });

    it('should log errors to console in development', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      
      const testError: AppError = {
        type: ErrorTypeValues.UNKNOWN_ERROR,
        message: 'Test error',
        retryable: false,
        severity: ErrorSeverityValues.MEDIUM,
        timestamp: new Date()
      };

      errorHandler.handleError(testError);

      expect(consoleSpy).toHaveBeenCalled();
      consoleSpy.mockRestore();
    });
  });

  describe('Network Status Monitoring', () => {
    it('should return online status', () => {
      expect(errorHandler.isOnline()).toBe(true);
    });

    it('should check connectivity', async () => {
      mockFetch.mockResolvedValueOnce({ ok: true });

      const isConnected = await errorHandler.checkConnectivity();

      expect(isConnected).toBe(true);
      expect(mockFetch).toHaveBeenCalledWith('/api/health', {
        method: 'HEAD',
        cache: 'no-cache'
      });
    });

    it('should handle connectivity check failure', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      const isConnected = await errorHandler.checkConnectivity();

      expect(isConnected).toBe(false);
    });
  });

  describe('Error Log Management', () => {
    it('should maintain error log', () => {
      const testError: AppError = {
        type: ErrorTypeValues.API_ERROR,
        message: 'API error',
        retryable: false,
        severity: ErrorSeverityValues.MEDIUM,
        timestamp: new Date()
      };

      errorHandler.handleError(testError, undefined, false);

      const errorLog = errorHandler.getErrorLog();
      expect(errorLog).toHaveLength(1);
      expect(errorLog[0]).toBe(testError);
    });

    it('should clear error log', () => {
      const testError: AppError = {
        type: ErrorTypeValues.API_ERROR,
        message: 'API error',
        retryable: false,
        severity: ErrorSeverityValues.MEDIUM,
        timestamp: new Date()
      };

      errorHandler.handleError(testError, undefined, false);
      expect(errorHandler.getErrorLog()).toHaveLength(1);

      errorHandler.clearErrorLog();
      expect(errorHandler.getErrorLog()).toHaveLength(0);
    });
  });
});

describe('ErrorMessageParser', () => {
  it('should parse network errors to user-friendly messages', () => {
    const networkError: NetworkError = {
      type: ErrorTypeValues.NETWORK_ERROR,
      message: 'Failed to fetch',
      retryable: true,
      severity: ErrorSeverityValues.HIGH,
      timestamp: new Date()
    };

    const userMessage = ErrorMessageParser.parseError(networkError);
    expect(userMessage).toBe('Unable to connect to the server. Please check your internet connection.');
  });

  it('should parse upload errors to user-friendly messages', () => {
    const uploadError: UploadError = {
      type: ErrorTypeValues.UPLOAD_ERROR,
      message: 'File too large',
      filename: 'test.pdf',
      reason: 'file_too_large',
      retryable: false,
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date()
    };

    const userMessage = ErrorMessageParser.parseError(uploadError);
    expect(userMessage).toBe('The selected file is too large. Please choose a smaller file.');
  });

  it('should parse search errors to user-friendly messages', () => {
    const searchError: SearchError = {
      type: ErrorTypeValues.SEARCH_ERROR,
      message: 'No results found',
      query: 'test query',
      reason: 'no_results',
      retryable: false,
      severity: ErrorSeverityValues.LOW,
      timestamp: new Date()
    };

    const userMessage = ErrorMessageParser.parseError(searchError);
    expect(userMessage).toBe('No relevant information found. Try rephrasing your question.');
  });

  it('should fallback to original message for unknown errors', () => {
    const unknownError: AppError = {
      type: ErrorTypeValues.UNKNOWN_ERROR,
      message: 'Custom error message',
      retryable: false,
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date()
    };

    const userMessage = ErrorMessageParser.parseError(unknownError);
    expect(userMessage).toBe('Custom error message');
  });
});

describe('ErrorRecoveryManager', () => {
  it('should provide retry options for retryable errors', () => {
    const retryableError: AppError = {
      type: ErrorTypeValues.NETWORK_ERROR,
      message: 'Connection failed',
      retryable: true,
      severity: ErrorSeverityValues.HIGH,
      timestamp: new Date()
    };

    const retryAction = vi.fn();
    const recoveryOptions = ErrorRecoveryManager.getRecoveryOptions(retryableError, retryAction);

    expect(recoveryOptions).toHaveLength(3); // retry, refresh, dismiss
    expect(recoveryOptions[0].action).toBe('retry');
    expect(recoveryOptions[0].label).toBe('Retry Connection');
  });

  it('should provide contact support for critical errors', () => {
    const criticalError: AppError = {
      type: ErrorTypeValues.UNKNOWN_ERROR,
      message: 'Critical system error',
      retryable: false,
      severity: ErrorSeverityValues.CRITICAL,
      timestamp: new Date()
    };

    const recoveryOptions = ErrorRecoveryManager.getRecoveryOptions(criticalError);

    const contactSupportOption = recoveryOptions.find(option => option.action === 'contact_support');
    expect(contactSupportOption).toBeDefined();
    expect(contactSupportOption?.label).toBe('Contact Support');
  });

  it('should always provide dismiss option', () => {
    const testError: AppError = {
      type: ErrorTypeValues.API_ERROR,
      message: 'API error',
      retryable: false,
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date()
    };

    const recoveryOptions = ErrorRecoveryManager.getRecoveryOptions(testError);

    const dismissOption = recoveryOptions.find(option => option.action === 'dismiss');
    expect(dismissOption).toBeDefined();
    expect(dismissOption?.label).toBe('Dismiss');
  });

  it('should provide appropriate options for upload errors', () => {
    const uploadError: UploadError = {
      type: ErrorTypeValues.UPLOAD_ERROR,
      message: 'Upload failed',
      filename: 'test.pdf',
      reason: 'upload_failed',
      retryable: true,
      severity: ErrorSeverityValues.MEDIUM,
      timestamp: new Date()
    };

    const retryAction = vi.fn();
    const recoveryOptions = ErrorRecoveryManager.getRecoveryOptions(uploadError, retryAction);

    expect(recoveryOptions).toHaveLength(3); // retry, navigate_home, dismiss
    expect(recoveryOptions[0].action).toBe('retry');
    expect(recoveryOptions[0].label).toBe('Try Upload Again');
  });
});