import { vi } from 'vitest';
import '@testing-library/jest-dom/vitest';

// Mock environment variables
vi.mock('./lib/config/env.js', () => ({
  env: {
    API_BASE_URL: 'http://localhost:8080',
    API_TIMEOUT: 30000,
    APP_NAME: 'RAG Document Search',
    APP_VERSION: '1.0.0',
    MAX_FILE_SIZE: 10485760,
    SUPPORTED_FILE_TYPES: '.pdf',
    MAX_FILES_PER_UPLOAD: 1,
    DEFAULT_MAX_CHUNKS: 10,
    DEFAULT_SIMILARITY_THRESHOLD: 0.7,
    DEFAULT_TEMPERATURE: 0.3,
    MAX_QUERY_LENGTH: 1000,
    THEME: 'light',
    ENABLE_ANIMATIONS: false,
    DEV: true,
    PROD: false
  }
}));