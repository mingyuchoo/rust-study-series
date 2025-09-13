/**
 * API Request and Response Types
 * These interfaces define the structure of data exchanged with the backend API
 */

// Query Configuration Types
export interface QueryConfig {
  max_chunks?: number;
  similarity_threshold?: number;
  max_response_tokens?: number;
  temperature?: number;
  include_low_confidence?: boolean;
}

// API Request Types
export interface QueryRequest {
  question: string;
  config?: QueryConfig;
}

// API Response Types
export interface UploadResponse {
  document_id: string;
  filename: string;
  chunks_created: number;
  processing_time_ms: number;
  status: 'success' | 'failure';
  message: string;
  timestamp: string;
}

export interface SourceReference {
  document_id: string;
  chunk_id: string;
  relevance_score: number;
  snippet: string;
  source_file: string;
  chunk_index: number;
  headers: string[];
}

export interface RAGResponse {
  answer: string;
  sources: SourceReference[];
  confidence: number;
  query: string;
  response_time_ms: number;
  timestamp: string;
}

export interface HealthResponse {
  status: 'healthy' | 'unhealthy';
  timestamp: string;
  services: {
    qdrant: boolean;
    azure_openai: boolean;
  };
  uptime_seconds: number;
}

// API Error Response Type
export interface ApiErrorResponse {
  error: string;
  message: string;
  details?: unknown;
  timestamp: string;
}