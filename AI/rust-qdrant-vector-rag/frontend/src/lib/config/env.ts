/**
 * Environment configuration utility
 * Provides type-safe access to environment variables
 */

export const env = {
  // API 설정
  // 개발 모드에서는 Vite 프록시(`/api` → `http://localhost:8080`)를 사용해 CORS를 우회합니다.
  // 프로덕션에서는 VITE_API_BASE_URL을 설정하여 실제 백엔드 주소를 사용하세요.
  API_BASE_URL: (import.meta.env.DEV ? '/api' : (import.meta.env.VITE_API_BASE_URL || '/api')),
  API_TIMEOUT: parseInt(import.meta.env.VITE_API_TIMEOUT || '30000', 10),

  // Application Configuration
  APP_NAME: import.meta.env.VITE_APP_NAME || 'RAG Document Search',
  APP_VERSION: import.meta.env.VITE_APP_VERSION || '1.0.0',

  // File Upload Configuration
  MAX_FILE_SIZE: parseInt(import.meta.env.VITE_MAX_FILE_SIZE || '10485760', 10), // 10MB
  SUPPORTED_FILE_TYPES: import.meta.env.VITE_SUPPORTED_FILE_TYPES || '.md,.markdown',
  MAX_FILES_PER_UPLOAD: parseInt(import.meta.env.VITE_MAX_FILES_PER_UPLOAD || '1', 10),

  // Search Configuration
  DEFAULT_MAX_CHUNKS: parseInt(import.meta.env.VITE_DEFAULT_MAX_CHUNKS || '10', 10),
  DEFAULT_SIMILARITY_THRESHOLD: parseFloat(
    import.meta.env.VITE_DEFAULT_SIMILARITY_THRESHOLD || '0.7'
  ),
  DEFAULT_TEMPERATURE: parseFloat(import.meta.env.VITE_DEFAULT_TEMPERATURE || '0.3'),
  MAX_QUERY_LENGTH: parseInt(import.meta.env.VITE_MAX_QUERY_LENGTH || '1000', 10),

  // UI Configuration
  THEME: import.meta.env.VITE_THEME || 'light',
  ENABLE_ANIMATIONS: import.meta.env.VITE_ENABLE_ANIMATIONS === 'true',

  // 개발 플래그
  DEV: import.meta.env.DEV,
  PROD: import.meta.env.PROD,
} as const;

// Type for environment configuration
export type EnvConfig = typeof env;

// Validation function to ensure required environment variables are set
export function validateEnv(): void {
  // 프로덕션에서만 API_BASE_URL 유효성 경고를 표시합니다.
  if (import.meta.env.PROD && !import.meta.env.VITE_API_BASE_URL) {
    console.warn('VITE_API_BASE_URL 값이 설정되지 않았습니다. 프로덕션에서는 필수입니다.');
  }
}

// File size formatting utility
export function formatFileSize(bytes: number): string {
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  if (bytes === 0) return '0 Bytes';
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + ' ' + sizes[i];
}

// File type validation
export function isValidFileType(fileName: string): boolean {
  const supportedTypes = env.SUPPORTED_FILE_TYPES.split(',').map(type => type.trim());
  const fileExtension = '.' + fileName.split('.').pop()?.toLowerCase();
  return supportedTypes.includes(fileExtension);
}

// File size validation
export function isValidFileSize(fileSize: number): boolean {
  return fileSize <= env.MAX_FILE_SIZE;
}
