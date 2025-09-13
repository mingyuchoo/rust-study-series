/**
 * Type Usage Examples
 * This file demonstrates how to use the defined types and schemas
 * This file is for documentation purposes and can be removed in production
 */

import {
  type QueryConfig,
  type SearchQueryInput,
  type UploadResponse,
  type AppError,
  type ToastMessage,
  SearchQuerySchema,
  FileUploadSchema,
  DEFAULT_QUERY_CONFIG,
  ErrorTypeValues,
  ErrorSeverityValues
} from './index.js';

// Example: Using API types
const exampleQueryConfig: QueryConfig = {
  max_chunks: 10,
  similarity_threshold: 0.8,
  temperature: 0.5,
  include_low_confidence: false
};

// Example: Using validation schemas
function validateSearchQuery(input: unknown): SearchQueryInput | null {
  try {
    return SearchQuerySchema.parse(input);
  } catch (error) {
    console.error('Validation failed:', error);
    return null;
  }
}

// Example: Using file validation
function validateFile(file: globalThis.File): boolean {
  try {
    FileUploadSchema.parse({ file });
    return true;
  } catch (error) {
    console.error('File validation failed:', error);
    return false;
  }
}

// Example: Creating error objects
function createNetworkError(message: string): AppError {
  return {
    type: ErrorTypeValues.NETWORK_ERROR,
    message,
    details: null,
    retryable: true,
    severity: ErrorSeverityValues.MEDIUM,
    timestamp: new Date(),
    code: 'NETWORK_001'
  };
}

// Example: Using default configurations
function getDefaultSearchConfig(): QueryConfig {
  return { ...DEFAULT_QUERY_CONFIG };
}

// Example: Creating toast messages
function createSuccessToast(message: string): ToastMessage {
  return {
    id: globalThis.crypto.randomUUID(),
    type: 'success',
    message,
    duration: 3000,
    dismissible: true
  };
}

// Example: Type-safe API response handling
function handleUploadResponse(response: UploadResponse): void {
  if (response.status === 'success') {
    console.log(`Upload successful: ${response.filename}`);
    console.log(`Document ID: ${response.document_id}`);
    console.log(`Chunks created: ${response.chunks_created}`);
  } else {
    console.error(`Upload failed: ${response.message}`);
  }
}

// Example: Using utility types
type PartialQueryConfig = Partial<QueryConfig>;
type RequiredQueryConfig = Required<QueryConfig>;

// Example: Component props usage
interface ExampleComponentProps {
  config: QueryConfig;
  onConfigChange: (_newConfig: QueryConfig) => void;
  disabled?: boolean;
}

// Export examples for documentation
export {
  exampleQueryConfig,
  validateSearchQuery,
  validateFile,
  createNetworkError,
  getDefaultSearchConfig,
  createSuccessToast,
  handleUploadResponse
};

export type {
  PartialQueryConfig,
  RequiredQueryConfig,
  ExampleComponentProps
};