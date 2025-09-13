/**
 * Component-specific Types and Interfaces
 * Type definitions for component props, events, and internal state
 */

import type { ComponentType } from 'svelte';
import type { QueryConfig, UploadResponse, RAGResponse, SourceReference } from './api.js';
import type { ToastMessage, ValidationError } from './state.js';
import type { AppError } from './errors.js';

// Base Component Props
export interface BaseComponentProps {
  class?: string;
  id?: string;
  'data-testid'?: string;
}

// File Upload Component Props
export interface FileUploadProps extends BaseComponentProps {
  onFileSelect: (_file: globalThis.File) => void;
  acceptedTypes: string[];
  maxSize: number;
  disabled?: boolean;
  multiple?: boolean;
  dragActive?: boolean;
  uploadProgress?: number;
  isUploading?: boolean;
}

// Upload Progress Component Props
export interface UploadProgressProps extends BaseComponentProps {
  progress: number;
  filename: string;
  isComplete: boolean;
  onCancel?: () => void;
  showCancel?: boolean;
}

// Upload Result Component Props
export interface UploadResultProps extends BaseComponentProps {
  result: UploadResponse;
  onReset: () => void;
  onUploadAnother: () => void;
}

// Search Form Component Props
export interface SearchFormProps extends BaseComponentProps {
  onSubmit: (_query: string, _config?: QueryConfig) => void;
  disabled?: boolean;
  showAdvanced?: boolean;
  initialQuery?: string;
  initialConfig?: QueryConfig;
  isSearching?: boolean;
}

// Search Configuration Component Props
export interface SearchConfigProps extends BaseComponentProps {
  config: QueryConfig;
  onChange: (_newConfig: QueryConfig) => void;
  onReset: () => void;
  disabled?: boolean;
}

// Loading Spinner Component Props
export interface LoadingSpinnerProps extends BaseComponentProps {
  size?: 'sm' | 'md' | 'lg';
  message?: string;
  showMessage?: boolean;
}

// Answer Display Component Props
export interface AnswerDisplayProps extends BaseComponentProps {
  answer: string;
  confidence: number;
  responseTime: number;
  onCopy?: () => void;
  showMetadata?: boolean;
}

// Source References Component Props
export interface SourceReferencesProps extends BaseComponentProps {
  sources: SourceReference[];
  onSourceClick: (_selectedSource: SourceReference) => void;
  maxVisible?: number;
  showExpanded?: boolean;
}

// Search Results Component Props
export interface SearchResultsProps extends BaseComponentProps {
  results: RAGResponse;
  onSourceClick: (_selectedSource: SourceReference) => void;
  onCopyAnswer: () => void;
  isLoading?: boolean;
}

// Health Status Component Props
export interface HealthStatusProps extends BaseComponentProps {
  status: 'healthy' | 'unhealthy' | 'checking';
  services: {
    qdrant: boolean;
    azure_openai: boolean;
  };
  lastChecked?: Date;
  onRefresh: () => void;
}

// System Metrics Component Props
export interface SystemMetricsProps extends BaseComponentProps {
  uptime: number;
  lastChecked: Date;
  responseTime?: number;
}

// Navigation Component Props
export interface NavigationProps extends BaseComponentProps {
  currentRoute: string;
  onNavigate: (_newRoute: string) => void;
  isCollapsed?: boolean;
  onToggleCollapse?: () => void;
}

// Error Boundary Component Props
export interface ErrorBoundaryProps extends BaseComponentProps {
  fallback?: ComponentType;
  onError?: (_caughtError: Error) => void;
  showDetails?: boolean;
}

// Toast Component Props
export interface ToastProps extends BaseComponentProps {
  messages: ToastMessage[];
  position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
  onDismiss: (_messageId: string) => void;
}

// Modal Component Props
export interface ModalProps extends BaseComponentProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  closable?: boolean;
  backdrop?: boolean;
}

// Form Field Component Props
export interface FormFieldProps extends BaseComponentProps {
  label: string;
  name: string;
  error?: ValidationError;
  required?: boolean;
  disabled?: boolean;
  helpText?: string;
}

// Button Component Props
export interface ButtonProps extends BaseComponentProps {
  variant?: 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  type?: 'button' | 'submit' | 'reset';
  onClick?: () => void;
}

// Input Component Props
export interface InputProps extends BaseComponentProps {
  type?: 'text' | 'email' | 'password' | 'number' | 'search';
  value: string;
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
  error?: ValidationError;
  onInput: (_inputValue: string) => void;
  onBlur?: () => void;
  onFocus?: () => void;
}

// Textarea Component Props
export interface TextareaProps extends BaseComponentProps {
  value: string;
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
  rows?: number;
  maxLength?: number;
  error?: ValidationError;
  onInput: (_inputValue: string) => void;
  onBlur?: () => void;
  onFocus?: () => void;
}

// Select Component Props
export interface SelectProps extends BaseComponentProps {
  value: string;
  options: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  error?: ValidationError;
  onChange: (_selectedValue: string) => void;
}

export interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

// Slider Component Props
export interface SliderProps extends BaseComponentProps {
  value: number;
  min: number;
  max: number;
  step?: number;
  disabled?: boolean;
  showValue?: boolean;
  onChange: (_sliderValue: number) => void;
}

// Checkbox Component Props
export interface CheckboxProps extends BaseComponentProps {
  checked: boolean;
  disabled?: boolean;
  label?: string;
  onChange: (_isChecked: boolean) => void;
}

// Component Event Types
export interface ComponentEvents {
  fileSelect: { file: globalThis.File };
  searchSubmit: { query: string; config?: QueryConfig };
  sourceClick: { source: SourceReference };
  configChange: { config: QueryConfig };
  navigate: { route: string };
  error: { error: AppError };
  toast: { message: ToastMessage };
}

// Accessibility Props
export interface A11yProps {
  'aria-label'?: string;
  'aria-describedby'?: string;
  'aria-expanded'?: boolean;
  'aria-hidden'?: boolean;
  'aria-live'?: 'polite' | 'assertive' | 'off';
  'aria-busy'?: boolean;
  role?: string;
  tabindex?: number;
}