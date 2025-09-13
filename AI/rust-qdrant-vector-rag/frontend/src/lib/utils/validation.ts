/**
 * Client-side validation utilities
 * Provides real-time validation, input sanitization, and debouncing
 */

import { env } from '../config/env.js';
import type { ValidationError } from '../types/state.js';

// Enhanced debounce utility with cancellation support
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): {
  (...args: Parameters<T>): void;
  cancel: () => void;
  flush: (...args: Parameters<T>) => void;
} {
  let timeout: NodeJS.Timeout | null = null;
  let lastArgs: Parameters<T> | null = null;

  const debounced = (...args: Parameters<T>) => {
    lastArgs = args;
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => {
      timeout = null;
      func(...args);
    }, wait);
  };

  debounced.cancel = () => {
    if (timeout) {
      clearTimeout(timeout);
      timeout = null;
      lastArgs = null;
    }
  };

  debounced.flush = (...args: Parameters<T>) => {
    if (timeout) {
      clearTimeout(timeout);
      timeout = null;
    }
    const argsToUse = args.length > 0 ? args : lastArgs;
    if (argsToUse) {
      func(...argsToUse);
    }
  };

  return debounced;
}

// Input sanitization utilities
export class InputSanitizer {
  // Remove potentially dangerous characters and scripts
  static sanitizeText(input: string): string {
    return input
      .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '') // Remove script tags
      .replace(/<iframe\b[^<]*(?:(?!<\/iframe>)<[^<]*)*<\/iframe>/gi, '') // Remove iframe tags
      .replace(/<object\b[^<]*(?:(?!<\/object>)<[^<]*)*<\/object>/gi, '') // Remove object tags
      .replace(/<embed\b[^<]*(?:(?!<\/embed>)<[^<]*)*<\/embed>/gi, '') // Remove embed tags
      .replace(/javascript:/gi, '') // Remove javascript: protocol
      .replace(/data:/gi, '') // Remove data: protocol
      .replace(/vbscript:/gi, '') // Remove vbscript: protocol
      .replace(/on\w+\s*=/gi, '') // Remove event handlers
      .replace(/[<>]/g, '') // Remove angle brackets
      .replace(/[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]/g, '') // Remove control characters
      .trim();
  }

  // Sanitize search query with enhanced security
  static sanitizeSearchQuery(query: string): string {
    let sanitized = this.sanitizeText(query);
    
    // Remove SQL injection patterns
    sanitized = sanitized.replace(/(\b(SELECT|INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|EXEC|UNION)\b)/gi, '');
    
    // Remove XSS patterns
    sanitized = sanitized.replace(/(\b(alert|confirm|prompt|eval)\s*\()/gi, '');
    
    // Allow only safe characters for search queries
    sanitized = sanitized.replace(/[^\w\s\-.,?!'"():;@#$%&+=]/g, '');
    
    // Normalize whitespace
    sanitized = sanitized.replace(/\s+/g, ' ').trim();
    
    // Enforce length limit
    return sanitized.substring(0, env.MAX_QUERY_LENGTH);
  }

  // Sanitize filename with enhanced security
  static sanitizeFilename(filename: string): string {
    return filename
      .replace(/[^a-zA-Z0-9._\-\s]/g, '_') // Replace unsafe characters with underscore
      .replace(/_{2,}/g, '_') // Replace multiple underscores with single
      .replace(/\s+/g, '_') // Replace spaces with underscores
      .replace(/^[._\-]+|[._\-]+$/g, '') // Remove leading/trailing dots, underscores, hyphens
      .substring(0, 255); // Enforce filename length limit
  }

  // Sanitize numeric input with validation
  static sanitizeNumber(input: string, min?: number, max?: number): number | null {
    // Remove all non-numeric characters except decimal point and minus sign
    const cleaned = input.replace(/[^\d.-]/g, '');
    
    // Ensure only one decimal point and minus sign at the beginning
    const parts = cleaned.split('.');
    let sanitized = parts[0] || '';
    if (parts.length > 1) {
      sanitized += '.' + parts.slice(1).join('');
    }
    
    // Handle negative numbers
    const isNegative = sanitized.startsWith('-');
    if (isNegative) {
      sanitized = '-' + sanitized.replace(/-/g, '');
    } else {
      sanitized = sanitized.replace(/-/g, '');
    }
    
    const num = parseFloat(sanitized);
    if (isNaN(num)) return null;
    
    if (min !== undefined && num < min) return min;
    if (max !== undefined && num > max) return max;
    
    return num;
  }

  // Sanitize configuration values
  static sanitizeConfigValue(value: any, type: 'number' | 'boolean' | 'string'): any {
    switch (type) {
      case 'number':
        return typeof value === 'number' ? value : this.sanitizeNumber(String(value));
      case 'boolean':
        return Boolean(value);
      case 'string':
        return this.sanitizeText(String(value));
      default:
        return value;
    }
  }
}

// Real-time validation utilities
export class RealTimeValidator {
  private static validationCache = new Map<string, { result: ValidationError[]; timestamp: number }>();
  private static readonly CACHE_TTL = 5000; // 5 seconds

  // File validation with real-time feedback
  static validateFile(file: File): ValidationError[] {
    const errors: ValidationError[] = [];

    // File type validation - enhanced with MIME type checking
    const isValidPDF = file.type === 'application/pdf' || 
                      file.name.toLowerCase().endsWith('.pdf');
    
    if (!isValidPDF) {
      errors.push({
        field: 'file',
        message: 'Only PDF files are allowed. Please select a valid PDF document.',
        code: 'invalid_file_type'
      });
    }

    // File size validation with detailed feedback
    if (file.size === 0) {
      errors.push({
        field: 'file',
        message: 'File appears to be empty. Please select a valid PDF file.',
        code: 'empty_file'
      });
    } else if (file.size > env.MAX_FILE_SIZE) {
      errors.push({
        field: 'file',
        message: `File size (${this.formatFileSize(file.size)}) exceeds the maximum allowed size of ${this.formatFileSize(env.MAX_FILE_SIZE)}`,
        code: 'file_too_large'
      });
    }

    // File name validation - enhanced
    if (file.name.length === 0) {
      errors.push({
        field: 'file',
        message: 'File must have a valid name',
        code: 'invalid_filename'
      });
    } else if (file.name.length > 255) {
      errors.push({
        field: 'file',
        message: 'Filename is too long. Maximum 255 characters allowed.',
        code: 'filename_too_long'
      });
    }

    // Check for potentially dangerous filenames
    if (this.isDangerousFilename(file.name)) {
      errors.push({
        field: 'file',
        message: 'Filename contains potentially unsafe characters. Please rename the file.',
        code: 'unsafe_filename'
      });
    }

    // Check for suspicious file extensions (double extensions)
    if (this.hasSuspiciousExtension(file.name)) {
      errors.push({
        field: 'file',
        message: 'File has suspicious extension pattern. Please ensure it is a valid PDF.',
        code: 'suspicious_extension'
      });
    }

    return errors;
  }

  // Search query validation with character count and enhanced security
  static validateSearchQuery(query: string): {
    errors: ValidationError[];
    warnings: ValidationError[];
    characterCount: number;
    wordCount: number;
    isValid: boolean;
    sanitizedQuery: string;
  } {
    const cacheKey = `search_${query}`;
    const cached = this.validationCache.get(cacheKey);
    
    if (cached && Date.now() - cached.timestamp < this.CACHE_TTL) {
      const sanitizedQuery = InputSanitizer.sanitizeSearchQuery(query);
      const wordCount = sanitizedQuery.trim().split(/\s+/).filter(word => word.length > 0).length;
      
      return {
        errors: cached.result,
        warnings: [],
        characterCount: query.length,
        wordCount,
        isValid: cached.result.length === 0 && query.trim().length >= 3,
        sanitizedQuery
      };
    }

    const errors: ValidationError[] = [];
    const warnings: ValidationError[] = [];
    const sanitizedQuery = InputSanitizer.sanitizeSearchQuery(query);
    const wordCount = sanitizedQuery.trim().split(/\s+/).filter(word => word.length > 0).length;
    
    // Length validation
    if (query.length < 3 && query.length > 0) {
      errors.push({
        field: 'query',
        message: 'Search query must be at least 3 characters long for effective results',
        code: 'query_too_short'
      });
    }

    if (query.length > env.MAX_QUERY_LENGTH) {
      errors.push({
        field: 'query',
        message: `Search query cannot exceed ${env.MAX_QUERY_LENGTH} characters (currently ${query.length})`,
        code: 'query_too_long'
      });
    }

    // Content validation
    if (query.trim().length === 0 && query.length > 0) {
      errors.push({
        field: 'query',
        message: 'Search query cannot be empty or contain only whitespace',
        code: 'query_empty'
      });
    }

    // Word count validation
    if (wordCount > 50) {
      warnings.push({
        field: 'query',
        message: 'Very long queries may not produce optimal results. Consider shortening your question.',
        code: 'query_very_long'
      });
    }

    // Check for potentially malicious content
    if (sanitizedQuery !== query && query.length > 0) {
      const removedChars = query.length - sanitizedQuery.length;
      warnings.push({
        field: 'query',
        message: `${removedChars} potentially unsafe character(s) were removed from your query`,
        code: 'query_sanitized'
      });
    }

    // Check for suspicious patterns
    if (this.hasSuspiciousQueryPatterns(query)) {
      errors.push({
        field: 'query',
        message: 'Query contains patterns that are not allowed for security reasons',
        code: 'query_suspicious'
      });
    }

    // Check for very short words (potential typos)
    const shortWords = sanitizedQuery.split(/\s+/).filter(word => word.length === 1 && !/[aI]/.test(word));
    if (shortWords.length > 2) {
      warnings.push({
        field: 'query',
        message: 'Your query contains several single-letter words. Check for typos.',
        code: 'potential_typos'
      });
    }

    // Cache the result
    this.validationCache.set(cacheKey, {
      result: errors,
      timestamp: Date.now()
    });

    return {
      errors,
      warnings,
      characterCount: query.length,
      wordCount,
      isValid: errors.length === 0 && query.trim().length >= 3,
      sanitizedQuery
    };
  }

  // Configuration parameter validation with enhanced feedback
  static validateConfigParameter(
    field: string,
    value: number,
    min: number,
    max: number,
    step?: number
  ): {
    errors: ValidationError[];
    warnings: ValidationError[];
    isValid: boolean;
    sanitizedValue: number;
  } {
    const errors: ValidationError[] = [];
    const warnings: ValidationError[] = [];
    let sanitizedValue = value;

    // Type validation
    if (isNaN(value) || !isFinite(value)) {
      errors.push({
        field,
        message: `${this.formatFieldName(field)} must be a valid number`,
        code: 'invalid_number'
      });
      sanitizedValue = min; // Default to minimum value
      return { errors, warnings, isValid: false, sanitizedValue };
    }

    // Range validation
    if (value < min) {
      errors.push({
        field,
        message: `${this.formatFieldName(field)} must be at least ${min} (currently ${value})`,
        code: 'value_too_low'
      });
      sanitizedValue = min;
    }

    if (value > max) {
      errors.push({
        field,
        message: `${this.formatFieldName(field)} cannot exceed ${max} (currently ${value})`,
        code: 'value_too_high'
      });
      sanitizedValue = max;
    }

    // Step validation
    if (step && Math.abs((value * 100) % (step * 100)) > 0.01) {
      errors.push({
        field,
        message: `${this.formatFieldName(field)} must be in increments of ${step}`,
        code: 'invalid_step'
      });
      // Round to nearest valid step
      sanitizedValue = Math.round(value / step) * step;
    }

    // Performance warnings
    if (field === 'max_chunks' && value > 15) {
      warnings.push({
        field,
        message: 'High chunk count may slow down search performance',
        code: 'performance_warning'
      });
    }

    if (field === 'max_response_tokens' && value > 2000) {
      warnings.push({
        field,
        message: 'Very long responses may take more time to generate',
        code: 'response_length_warning'
      });
    }

    if (field === 'temperature' && value > 0.8) {
      warnings.push({
        field,
        message: 'High temperature may produce less focused responses',
        code: 'temperature_warning'
      });
    }

    if (field === 'similarity_threshold' && value < 0.3) {
      warnings.push({
        field,
        message: 'Low similarity threshold may include irrelevant results',
        code: 'relevance_warning'
      });
    }

    return {
      errors,
      warnings,
      isValid: errors.length === 0,
      sanitizedValue: Math.round(sanitizedValue * 100) / 100 // Round to 2 decimal places
    };
  }

  // Format field names for user-friendly display
  private static formatFieldName(field: string): string {
    const fieldNames: Record<string, string> = {
      'max_chunks': 'Maximum chunks',
      'similarity_threshold': 'Similarity threshold',
      'max_response_tokens': 'Maximum response tokens',
      'temperature': 'Temperature',
      'include_low_confidence': 'Include low confidence'
    };
    
    return fieldNames[field] || field.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  }

  // Helper methods
  private static formatFileSize(bytes: number): string {
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 Bytes';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + ' ' + sizes[i];
  }

  private static isDangerousFilename(filename: string): boolean {
    const dangerousPatterns = [
      /\.\./,           // Directory traversal
      /[<>:"|?*]/,      // Windows reserved characters
      /^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])$/i, // Windows reserved names
      /^\./,            // Hidden files
      /\s+$/,           // Trailing whitespace
      /[\x00-\x1f\x7f-\x9f]/,  // Control characters
      /[\\\/]/,         // Path separators
    ];

    return dangerousPatterns.some(pattern => pattern.test(filename));
  }

  private static hasSuspiciousExtension(filename: string): boolean {
    // Check for double extensions like .pdf.exe, .pdf.js, etc.
    const suspiciousPatterns = [
      /\.pdf\.(exe|bat|cmd|scr|com|pif|js|jar|vbs|ps1)$/i,
      /\.(exe|bat|cmd|scr|com|pif|js|jar|vbs|ps1)\.pdf$/i,
      /\.pdf\..*\.pdf$/i, // Multiple .pdf extensions
    ];

    return suspiciousPatterns.some(pattern => pattern.test(filename));
  }

  private static hasSuspiciousQueryPatterns(query: string): boolean {
    const suspiciousPatterns = [
      // SQL injection patterns
      /(\b(SELECT|INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|EXEC|UNION)\b.*\b(FROM|INTO|SET|WHERE|VALUES)\b)/gi,
      // Script injection patterns
      /(<script|javascript:|data:text\/html|vbscript:)/gi,
      // Command injection patterns
      /(;|\||&|`|\$\(|\${)/g,
      // Path traversal patterns
      /(\.\.\/|\.\.\\)/g,
      // Excessive special characters (potential obfuscation)
      /[!@#$%^&*()_+=\[\]{}|\\:";'<>?,./]{10,}/g,
    ];

    return suspiciousPatterns.some(pattern => pattern.test(query));
  }

  // Clear validation cache
  static clearCache(): void {
    this.validationCache.clear();
  }

  // Clean expired cache entries
  static cleanExpiredCache(): void {
    const now = Date.now();
    for (const [key, value] of this.validationCache.entries()) {
      if (now - value.timestamp > this.CACHE_TTL) {
        this.validationCache.delete(key);
      }
    }
  }
}

// Enhanced debounced validation functions with better performance
export class DebouncedValidation {
  private static searchValidationDebounced = debounce(
    (query: string, callback: (result: ReturnType<typeof RealTimeValidator.validateSearchQuery>) => void) => {
      try {
        const result = RealTimeValidator.validateSearchQuery(query);
        callback(result);
      } catch (error) {
        console.error('Search validation error:', error);
        callback({
          errors: [{ field: 'query', message: 'Validation error occurred', code: 'validation_error' }],
          warnings: [],
          characterCount: query.length,
          wordCount: 0,
          isValid: false,
          sanitizedQuery: query
        });
      }
    },
    300
  );

  private static configValidationDebounced = debounce(
    (field: string, value: number, min: number, max: number, step: number | undefined, callback: (result: ReturnType<typeof RealTimeValidator.validateConfigParameter>) => void) => {
      try {
        const result = RealTimeValidator.validateConfigParameter(field, value, min, max, step);
        callback(result);
      } catch (error) {
        console.error('Config validation error:', error);
        callback({
          errors: [{ field, message: 'Validation error occurred', code: 'validation_error' }],
          warnings: [],
          isValid: false,
          sanitizedValue: value
        });
      }
    },
    200
  );

  private static fileValidationDebounced = debounce(
    (file: File, callback: (errors: ValidationError[]) => void) => {
      try {
        const errors = RealTimeValidator.validateFile(file);
        callback(errors);
      } catch (error) {
        console.error('File validation error:', error);
        callback([{ field: 'file', message: 'File validation error occurred', code: 'validation_error' }]);
      }
    },
    100
  );

  static validateSearchQueryDebounced(
    query: string,
    callback: (result: ReturnType<typeof RealTimeValidator.validateSearchQuery>) => void
  ): void {
    this.searchValidationDebounced(query, callback);
  }

  static validateConfigParameterDebounced(
    field: string,
    value: number,
    min: number,
    max: number,
    step: number | undefined,
    callback: (result: ReturnType<typeof RealTimeValidator.validateConfigParameter>) => void
  ): void {
    this.configValidationDebounced(field, value, min, max, step, callback);
  }

  static validateFileDebounced(
    file: File,
    callback: (errors: ValidationError[]) => void
  ): void {
    this.fileValidationDebounced(file, callback);
  }

  // Utility to cancel all pending validations
  static cancelAllValidations(): void {
    // Note: This would require modifying the debounce function to return a cancellable function
    // For now, we'll clear the validation cache
    RealTimeValidator.clearCache();
  }
}

// Form validation state manager
export class FormValidationManager<T = Record<string, any>> {
  private data: T;
  private errors: Map<string, ValidationError[]> = new Map();
  private isDirty = false;
  private validators: Map<string, (value: any) => ValidationError[]> = new Map();

  constructor(initialData: T) {
    this.data = { ...initialData };
  }

  // Register field validator
  addValidator(field: string, validator: (value: any) => ValidationError[]): void {
    this.validators.set(field, validator);
  }

  // Update field value and validate
  updateField(field: string, value: any): void {
    (this.data as any)[field] = value;
    this.isDirty = true;
    this.validateField(field);
  }

  // Validate specific field
  validateField(field: string): ValidationError[] {
    const validator = this.validators.get(field);
    if (!validator) return [];

    const fieldValue = (this.data as any)[field];
    const errors = validator(fieldValue);
    
    if (errors.length > 0) {
      this.errors.set(field, errors);
    } else {
      this.errors.delete(field);
    }

    return errors;
  }

  // Validate all fields
  validateAll(): ValidationError[] {
    const allErrors: ValidationError[] = [];
    
    for (const field of this.validators.keys()) {
      const errors = this.validateField(field);
      allErrors.push(...errors);
    }

    return allErrors;
  }

  // Get field errors
  getFieldErrors(field: string): ValidationError[] {
    return this.errors.get(field) || [];
  }

  // Get all errors
  getAllErrors(): ValidationError[] {
    const allErrors: ValidationError[] = [];
    for (const errors of this.errors.values()) {
      allErrors.push(...errors);
    }
    return allErrors;
  }

  // Check if form is valid
  isValid(): boolean {
    return this.errors.size === 0;
  }

  // Check if form is dirty
  isDirtyForm(): boolean {
    return this.isDirty;
  }

  // Get current data
  getData(): T {
    return { ...this.data };
  }

  // Reset form
  reset(newData?: T): void {
    this.data = newData ? { ...newData } : { ...this.data };
    this.errors.clear();
    this.isDirty = false;
  }
}

// Export validation schemas for consistency
export { 
  FileUploadSchema,
  SearchQuerySchema,
  QueryConfigSchema 
} from '../schemas/validation.js';