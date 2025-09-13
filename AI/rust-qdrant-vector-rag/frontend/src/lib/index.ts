/**
 * Library Index
 * Central export point for all library modules
 */

// Export all types
export * from './types/index.js';

// Export services (with specific exports to avoid conflicts)
export { apiService } from './services/index.js';
export type { RequestConfig } from './services/index.js';

// Export stores
export * from './stores/index.js';

// Export components
export * from './components/index.js';

// Export configuration
export * from './config/env.js';
export * from './config/theme.js';

// Export utilities
export * from './utils/accessibility.js';
export * from './utils/responsive.js';
export * from './utils/accessibility-testing.js';

// Export validation schemas
export * from './schemas/validation.js';