/**
 * Zod Validation Schemas
 * Runtime validation schemas for API requests, form data, and user inputs
 */

import { z } from 'zod';

// File Upload Validation
export const FileUploadSchema = z.object({
  file: z
    .instanceof(globalThis.File)
    .refine((file) => file.type === 'application/pdf', {
      message: 'Only PDF files are allowed'
    })
    .refine((file) => file.size <= 10 * 1024 * 1024, {
      message: 'File size must be less than 10MB'
    })
    .refine((file) => file.name.length > 0, {
      message: 'File must have a valid name'
    })
});

// Query Configuration Validation
export const QueryConfigSchema = z.object({
  max_chunks: z
    .number()
    .int()
    .min(1, 'Max chunks must be at least 1')
    .max(20, 'Max chunks cannot exceed 20')
    .optional(),
  similarity_threshold: z
    .number()
    .min(0.0, 'Similarity threshold must be at least 0.0')
    .max(1.0, 'Similarity threshold cannot exceed 1.0')
    .optional(),
  max_response_tokens: z
    .number()
    .int()
    .min(50, 'Max response tokens must be at least 50')
    .max(4000, 'Max response tokens cannot exceed 4000')
    .optional(),
  temperature: z
    .number()
    .min(0.0, 'Temperature must be at least 0.0')
    .max(1.0, 'Temperature cannot exceed 1.0')
    .optional(),
  include_low_confidence: z.boolean().optional()
});

// Search Query Validation
export const SearchQuerySchema = z.object({
  question: z
    .string()
    .min(3, 'Question must be at least 3 characters long')
    .max(500, 'Question cannot exceed 500 characters')
    .trim()
    .refine((val) => val.length > 0, {
      message: 'Question cannot be empty'
    }),
  config: QueryConfigSchema.optional()
});

// API Response Validation Schemas
export const UploadResponseSchema = z.object({
  document_id: z.string().uuid('Invalid document ID format'),
  filename: z.string().min(1, 'Filename cannot be empty'),
  chunks_created: z.number().int().min(0, 'Chunks created must be non-negative'),
  processing_time_ms: z.number().min(0, 'Processing time must be non-negative'),
  status: z.enum(['success', 'failure']),
  message: z.string(),
  timestamp: z.string().datetime('Invalid timestamp format')
});

export const SourceReferenceSchema = z.object({
  document_id: z.string().uuid('Invalid document ID format'),
  chunk_id: z.string().min(1, 'Chunk ID cannot be empty'),
  relevance_score: z.number().min(0).max(1, 'Relevance score must be between 0 and 1'),
  snippet: z.string().min(1, 'Snippet cannot be empty'),
  source_file: z.string().min(1, 'Source file cannot be empty'),
  chunk_index: z.number().int().min(0, 'Chunk index must be non-negative'),
  headers: z.array(z.string())
});

export const RAGResponseSchema = z.object({
  answer: z.string().min(1, 'Answer cannot be empty'),
  sources: z.array(SourceReferenceSchema),
  confidence: z.number().min(0).max(1, 'Confidence must be between 0 and 1'),
  query: z.string().min(1, 'Query cannot be empty'),
  response_time_ms: z.number().min(0, 'Response time must be non-negative'),
  timestamp: z.string().datetime('Invalid timestamp format')
});

export const HealthResponseSchema = z.object({
  status: z.enum(['healthy', 'unhealthy']),
  timestamp: z.string().datetime('Invalid timestamp format'),
  services: z.object({
    qdrant: z.boolean(),
    azure_openai: z.boolean()
  }),
  uptime_seconds: z.number().min(0, 'Uptime must be non-negative')
});

// Form Validation Schemas
export const SearchFormSchema = z.object({
  query: z
    .string()
    .min(3, 'Search query must be at least 3 characters')
    .max(500, 'Search query cannot exceed 500 characters')
    .trim(),
  showAdvanced: z.boolean().default(false),
  config: QueryConfigSchema.default({})
});

export const UploadFormSchema = z.object({
  file: z
    .instanceof(globalThis.File)
    .refine((file) => file.type === 'application/pdf', {
      message: 'Only PDF files are supported'
    })
    .refine((file) => file.size <= 10 * 1024 * 1024, {
      message: 'File size must be less than 10MB'
    })
});

// Environment Configuration Validation
export const EnvConfigSchema = z.object({
  VITE_API_BASE_URL: z.string().url('Invalid API base URL'),
  VITE_APP_NAME: z.string().min(1, 'App name cannot be empty'),
  VITE_MAX_FILE_SIZE: z.string().transform((val) => parseInt(val, 10)),
  VITE_SUPPORTED_FILE_TYPES: z.string().min(1, 'Supported file types cannot be empty')
});

// Toast Message Validation
export const ToastMessageSchema = z.object({
  id: z.string().min(1, 'Toast ID cannot be empty'),
  type: z.enum(['success', 'error', 'warning', 'info']),
  message: z.string().min(1, 'Toast message cannot be empty'),
  duration: z.number().positive().optional(),
  dismissible: z.boolean().default(true)
});

// Validation Error Schema
export const ValidationErrorSchema = z.object({
  field: z.string().min(1, 'Field name cannot be empty'),
  message: z.string().min(1, 'Error message cannot be empty'),
  code: z.string().optional()
});

// Type exports for use in components
export type FileUploadInput = z.infer<typeof FileUploadSchema>;
export type QueryConfigInput = z.infer<typeof QueryConfigSchema>;
export type SearchQueryInput = z.infer<typeof SearchQuerySchema>;
export type SearchFormInput = z.infer<typeof SearchFormSchema>;
export type UploadFormInput = z.infer<typeof UploadFormSchema>;
export type ToastMessageInput = z.infer<typeof ToastMessageSchema>;
export type ValidationErrorInput = z.infer<typeof ValidationErrorSchema>;
export type EnvConfigInput = z.infer<typeof EnvConfigSchema>;