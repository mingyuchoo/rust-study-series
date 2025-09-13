/**
 * Frontend State Management Types
 * These interfaces define the structure of application state
 */

import type { UploadResponse, RAGResponse, HealthResponse, QueryConfig } from './api.js';

// Global App State
export interface AppState {
  isLoading: boolean;
  error: string | null;
  currentPage: string;
  isOnline: boolean;
}

// Upload State Management
export interface UploadState {
  uploadProgress: number;
  isUploading: boolean;
  uploadResult: UploadResponse | null;
  selectedFile: globalThis.File | null;
  dragActive: boolean;
  currentStage?: string;
  estimatedTimeRemaining?: number | null;
  uploadSpeed?: number; // bytes per second
}

// Search State Management
export interface SearchState {
  query: string;
  results: RAGResponse | null;
  isSearching: boolean;
  searchConfig: QueryConfig;
  searchHistory: SearchHistoryItem[];
}

export interface SearchHistoryItem {
  id: string;
  query: string;
  timestamp: Date;
  resultCount: number;
}

// Health State Management
export interface HealthState {
  status: HealthResponse | null;
  lastChecked: Date | null;
  isChecking: boolean;
  checkInterval: number;
}

// UI State Types
export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
  dismissible?: boolean;
}

export interface ValidationError {
  field: string;
  message: string;
  code?: string;
}

// Navigation State
export interface NavigationState {
  currentRoute: string;
  previousRoute: string | null;
  isNavigating: boolean;
}

// Form State Types
export interface FormState<T = Record<string, unknown>> {
  data: T;
  errors: ValidationError[];
  isSubmitting: boolean;
  isDirty: boolean;
  isValid: boolean;
}

// File Upload Form State
export interface FileUploadFormState extends FormState<{ file: globalThis.File | null }> {
  uploadProgress: number;
  previewUrl?: string;
}

// Search Form State
export interface SearchFormState extends FormState<{ query: string; config: QueryConfig }> {
  characterCount: number;
  showAdvanced: boolean;
}