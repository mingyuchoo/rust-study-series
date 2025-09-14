/**
 * API Service Layer
 * Provides a centralized interface for all backend API communications
 * with error handling, retry logic, and request/response interceptors
 */

import { env } from '../config/env.js';
import { errorHandler } from './error-handler.js';
import type {
  QueryRequest,
  UploadResponse,
  RAGResponse,
  HealthResponse,
  ApiErrorResponse
} from '../types/api.js';
import type {
  AppError,
  ApiError,
  RetryConfig
} from '../types/errors.js';

import { ErrorTypeValues, ErrorSeverityValues } from '../types/errors.js';

// Request configuration interface
interface RequestConfig {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  url: string;
  data?: unknown;
  headers?: Record<string, string>;
  timeout?: number;
  retryable?: boolean;
}

// Response wrapper interface
interface ApiResponse<T = unknown> {
  data: T;
  status: number;
  statusText: string;
  headers: Headers;
}

// Retry configuration
const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxAttempts: 3,
  baseDelay: 1000,
  maxDelay: 10000,
  backoffMultiplier: 2,
  retryableErrors: [
    ErrorTypeValues.NETWORK_ERROR,
    ErrorTypeValues.TIMEOUT_ERROR
  ]
};

/**
 * Base API Client Class
 * Handles all HTTP communications with retry logic and error handling
 */
class ApiClient {
  private baseURL: string;
  private timeout: number;
  private retryConfig: RetryConfig;

  constructor() {
    this.baseURL = env.API_BASE_URL;
    this.timeout = env.API_TIMEOUT;
    this.retryConfig = DEFAULT_RETRY_CONFIG;
  }

  /**
   * Create an AbortController for request timeout
   */
  private createTimeoutController(timeout: number): AbortController {
    const controller = new AbortController();
    setTimeout(() => controller.abort(), timeout);
    return controller;
  }

  /**
   * Sleep utility for retry delays
   */
  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * Calculate retry delay with exponential backoff
   */
  private calculateRetryDelay(attempt: number): number {
    const delay = this.retryConfig.baseDelay * Math.pow(this.retryConfig.backoffMultiplier, attempt - 1);
    return Math.min(delay, this.retryConfig.maxDelay);
  }

  /**
   * Check if error is retryable
   */
  private isRetryableError(error: AppError): boolean {
    return this.retryConfig.retryableErrors.includes(error.type) && error.retryable;
  }

  /**
   * Create AppError from fetch error
   */
  private createErrorFromFetch(error: unknown, config: RequestConfig): AppError {
    // Check if we're offline
    if (!errorHandler.isOnline()) {
      return errorHandler.createNetworkError(
        'You are currently offline. Please check your internet connection.',
        { url: config.url, method: config.method }
      );
    }

    if (error instanceof TypeError && error.message.includes('fetch')) {
      return errorHandler.createNetworkError(
        'Network connection failed',
        { url: config.url, method: config.method }
      );
    }

    if (error instanceof DOMException && error.name === 'AbortError') {
      return {
        type: ErrorTypeValues.TIMEOUT_ERROR,
        message: 'Request timed out',
        details: { timeout: this.timeout, url: config.url, method: config.method },
        retryable: true,
        severity: ErrorSeverityValues.MEDIUM,
        timestamp: new Date()
      };
    }

    return {
      type: ErrorTypeValues.UNKNOWN_ERROR,
      message: 'An unexpected error occurred',
      details: error,
      retryable: false,
      severity: ErrorSeverityValues.HIGH,
      timestamp: new Date()
    };
  }

  /**
   * Create AppError from HTTP response
   */
  private async createErrorFromResponse(response: Response, config: RequestConfig): Promise<ApiError> {
    let errorData: ApiErrorResponse | null = null;
    
    try {
      const contentType = response.headers.get('content-type');
      if (contentType && contentType.includes('application/json')) {
        errorData = await response.json();
      }
    } catch {
      // Ignore JSON parsing errors
    }

    return {
      type: ErrorTypeValues.API_ERROR,
      message: errorData?.message || response.statusText || 'API request failed',
      details: {
        statusCode: response.status,
        statusText: response.statusText,
        url: config.url,
        method: config.method,
        errorData
      },
      retryable: response.status >= 500 && response.status < 600,
      severity: response.status >= 500 ? ErrorSeverityValues.HIGH : ErrorSeverityValues.MEDIUM,
      timestamp: new Date(),
      statusCode: response.status,
      endpoint: config.url,
      method: config.method
    };
  }

  /**
   * Request interceptor - modify request before sending
   */
  private requestInterceptor(config: RequestConfig): RequestConfig {
    // Add default headers
    const defaultHeaders = {
      'Content-Type': 'application/json',
      'X-Requested-With': 'XMLHttpRequest'
    };

    return {
      ...config,
      headers: {
        ...defaultHeaders,
        ...config.headers
      }
    };
  }

  /**
   * Response interceptor - process response after receiving
   */
  private async responseInterceptor<T>(response: Response): Promise<ApiResponse<T>> {
    const data = await response.json();
    
    return {
      data,
      status: response.status,
      statusText: response.statusText,
      headers: response.headers
    };
  }

  /**
   * Make HTTP request with retry logic
   */
  private async makeRequest<T>(config: RequestConfig): Promise<ApiResponse<T>> {
    const processedConfig = this.requestInterceptor(config);
    let lastError: AppError | null = null;

    for (let attempt = 1; attempt <= this.retryConfig.maxAttempts; attempt++) {
      try {
        const controller = this.createTimeoutController(processedConfig.timeout || this.timeout);
        const url = processedConfig.url.startsWith('http') 
          ? processedConfig.url 
          : `${this.baseURL}${processedConfig.url}`;

        const fetchOptions: RequestInit = {
          method: processedConfig.method,
          headers: processedConfig.headers || {},
          signal: controller.signal
        };

        // Add body for non-GET requests
        if (processedConfig.data && processedConfig.method !== 'GET') {
          if (processedConfig.data instanceof FormData) {
            fetchOptions.body = processedConfig.data;
            // Remove Content-Type header for FormData (browser will set it with boundary)
            delete (fetchOptions.headers as Record<string, string>)['Content-Type'];
          } else {
            fetchOptions.body = JSON.stringify(processedConfig.data);
          }
        }

        const response = await fetch(url, fetchOptions);

        if (!response.ok) {
          const error = await this.createErrorFromResponse(response, processedConfig);
          
          if (attempt < this.retryConfig.maxAttempts && this.isRetryableError(error)) {
            lastError = error;
            const delay = this.calculateRetryDelay(attempt);
            console.log(`API request failed (attempt ${attempt}/${this.retryConfig.maxAttempts}), retrying in ${delay}ms...`);
            await this.sleep(delay);
            continue;
          }
          
          // Handle the error through the error handler service
          errorHandler.handleError(error, undefined, false); // Don't show toast here, let the calling code handle it
          throw error;
        }

        return await this.responseInterceptor<T>(response);

      } catch (error) {
        const appError = error instanceof Error && 'type' in error && 'retryable' in error && 'severity' in error && 'timestamp' in error
          ? error as AppError
          : this.createErrorFromFetch(error, processedConfig);

        if (attempt < this.retryConfig.maxAttempts && this.isRetryableError(appError)) {
          lastError = appError;
          const delay = this.calculateRetryDelay(attempt);
          console.log(`API request failed (attempt ${attempt}/${this.retryConfig.maxAttempts}), retrying in ${delay}ms...`);
          await this.sleep(delay);
          continue;
        }

        // Handle the error through the error handler service
        errorHandler.handleError(appError, undefined, false); // Don't show toast here, let the calling code handle it
        throw appError;
      }
    }

    // This should never be reached, but TypeScript requires it
    throw lastError || new Error('Maximum retry attempts exceeded');
  }

  /**
   * GET request
   */
  async get<T>(url: string, config?: Partial<RequestConfig>): Promise<ApiResponse<T>> {
    return this.makeRequest<T>({
      method: 'GET',
      url,
      ...config
    });
  }

  /**
   * POST request
   */
  async post<T>(url: string, data?: unknown, config?: Partial<RequestConfig>): Promise<ApiResponse<T>> {
    return this.makeRequest<T>({
      method: 'POST',
      url,
      data,
      ...config
    });
  }

  /**
   * PUT request
   */
  async put<T>(url: string, data?: unknown, config?: Partial<RequestConfig>): Promise<ApiResponse<T>> {
    return this.makeRequest<T>({
      method: 'PUT',
      url,
      data,
      ...config
    });
  }

  /**
   * DELETE request
   */
  async delete<T>(url: string, config?: Partial<RequestConfig>): Promise<ApiResponse<T>> {
    return this.makeRequest<T>({
      method: 'DELETE',
      url,
      ...config
    });
  }
}

/**
 * API Service Class
 * High-level API methods for specific endpoints
 */
class ApiService {
  private client: ApiClient;

  constructor() {
    this.client = new ApiClient();
  }

  /**
   * Upload document to the backend
   */
  async uploadDocument(file: File): Promise<UploadResponse> {
    try {
      // Validate file before upload
      if (!file) {
        throw errorHandler.createUploadError(
          'No file selected',
          'unknown',
          'upload_failed'
        );
      }

      if (file.size > env.MAX_FILE_SIZE) {
        throw errorHandler.createUploadError(
          'File is too large',
          file.name,
          'file_too_large',
          { fileSize: file.size, fileType: file.type }
        );
      }

      const lowerName = file.name.toLowerCase();
      const isMarkdown = lowerName.endsWith('.md') || lowerName.endsWith('.markdown');
      if (!isMarkdown) {
        throw errorHandler.createUploadError(
          'Invalid file type. Please select a Markdown (.md, .markdown) file.',
          file.name,
          'invalid_type',
          { fileType: file.type }
        );
      }

      const formData = new FormData();
      formData.append('file', file);

      const response = await this.client.post<UploadResponse>('/upload', formData, {
        timeout: 60000, // 60 seconds for file upload
        retryable: false // Don't retry file uploads
      });

      return response.data;
    } catch (error) {
      if (error instanceof Error && 'type' in error) {
        // Re-throw AppError as-is
        throw error;
      }
      
      // Convert unknown errors to upload errors
      throw errorHandler.createUploadError(
        'Upload failed due to an unexpected error',
        file?.name || 'unknown',
        'upload_failed'
      );
    }
  }

  /**
   * Query documents using RAG
   */
  async queryDocuments(request: QueryRequest): Promise<RAGResponse> {
    try {
      // Validate query before sending
      if (!request.question || request.question.trim().length === 0) {
        throw errorHandler.createSearchError(
          'Please enter a search query',
          request.question || '',
          'query_too_short'
        );
      }

      if (request.question.length > env.MAX_QUERY_LENGTH) {
        throw errorHandler.createSearchError(
          'Search query is too long',
          request.question,
          'query_too_long'
        );
      }

      const response = await this.client.post<RAGResponse>('/query', request, {
        timeout: 45000, // 45 seconds for AI processing
        retryable: true
      });

      // Check if response indicates no results
      if (response.data.sources.length === 0) {
        throw errorHandler.createSearchError(
          'No relevant information found for your query',
          request.question,
          'no_results'
        );
      }

      return response.data;
    } catch (error) {
      if (error instanceof Error && 'type' in error) {
        // Re-throw AppError as-is
        throw error;
      }
      
      // Convert unknown errors to search errors
      throw errorHandler.createSearchError(
        'Search failed due to an unexpected error',
        request.question || '',
        'service_unavailable'
      );
    }
  }

  /**
   * Get system health status
   */
  async getHealth(): Promise<HealthResponse> {
    try {
      const response = await this.client.get<HealthResponse>('/health', {
        timeout: 10000, // 10 seconds for health check
        retryable: true
      });

      return response.data;
    } catch (error) {
      if (error instanceof Error && 'type' in error) {
        // Re-throw AppError as-is
        throw error;
      }
      
      // Convert unknown errors to API errors
      throw {
        type: ErrorTypeValues.API_ERROR,
        message: 'Health check failed',
        details: error,
        retryable: true,
        severity: ErrorSeverityValues.MEDIUM,
        timestamp: new Date(),
        statusCode: 0,
        endpoint: '/health',
        method: 'GET'
      } as ApiError;
    }
  }
}

// Export singleton instance
export const apiService = new ApiService();

// Export types for external use
export type { RequestConfig, ApiResponse };