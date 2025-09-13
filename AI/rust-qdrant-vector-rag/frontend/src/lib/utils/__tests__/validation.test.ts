/**
 * Comprehensive tests for enhanced validation utilities
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { 
  RealTimeValidator, 
  InputSanitizer, 
  DebouncedValidation,
  FormValidationManager,
  debounce 
} from '../validation.js';
import { afterEach } from 'node:test';
import { afterEach } from 'node:test';

describe('InputSanitizer', () => {
  describe('sanitizeSearchQuery', () => {
    it('should remove script tags', () => {
      const input = 'Hello <script>alert("xss")</script> world';
      const result = InputSanitizer.sanitizeSearchQuery(input);
      expect(result).toBe('Hello world');
    });

    it('should remove javascript protocols', () => {
      const input = 'javascript:alert("xss") test query';
      const result = InputSanitizer.sanitizeSearchQuery(input);
      expect(result).toBe('"xss") test query');
    });

    it('should remove SQL injection patterns', () => {
      const input = 'SELECT * FROM users WHERE name = "test"';
      const result = InputSanitizer.sanitizeSearchQuery(input);
      expect(result).toBe('FROM users WHERE name = "test"');
    });

    it('should normalize whitespace', () => {
      const input = 'test    query   with   spaces';
      const result = InputSanitizer.sanitizeSearchQuery(input);
      expect(result).toBe('test query with spaces');
    });

    it('should enforce length limits', () => {
      const input = 'a'.repeat(2000);
      const result = InputSanitizer.sanitizeSearchQuery(input);
      expect(result.length).toBeLessThanOrEqual(1000);
    });
  });

  describe('sanitizeFilename', () => {
    it('should replace unsafe characters', () => {
      const input = 'test<>file|name.pdf';
      const result = InputSanitizer.sanitizeFilename(input);
      expect(result).toBe('test_file_name.pdf');
    });

    it('should handle spaces', () => {
      const input = 'my test file.pdf';
      const result = InputSanitizer.sanitizeFilename(input);
      expect(result).toBe('my_test_file.pdf');
    });

    it('should remove leading/trailing special chars', () => {
      const input = '...test-file___.pdf';
      const result = InputSanitizer.sanitizeFilename(input);
      expect(result).toBe('test-file_.pdf');
    });
  });

  describe('sanitizeNumber', () => {
    it('should parse valid numbers', () => {
      expect(InputSanitizer.sanitizeNumber('123.45')).toBe(123.45);
      expect(InputSanitizer.sanitizeNumber('-67.89')).toBe(-67.89);
    });

    it('should handle invalid input', () => {
      expect(InputSanitizer.sanitizeNumber('abc')).toBeNull();
      expect(InputSanitizer.sanitizeNumber('')).toBeNull();
    });

    it('should enforce min/max bounds', () => {
      expect(InputSanitizer.sanitizeNumber('5', 10, 20)).toBe(10);
      expect(InputSanitizer.sanitizeNumber('25', 10, 20)).toBe(20);
      expect(InputSanitizer.sanitizeNumber('15', 10, 20)).toBe(15);
    });
  });
});

describe('RealTimeValidator', () => {
  describe('validateFile', () => {
    it('should validate PDF files correctly', () => {
      const validFile = new File(['content'], 'test.pdf', { type: 'application/pdf' });
      const result = RealTimeValidator.validateFile(validFile);
      expect(result).toHaveLength(0);
    });

    it('should reject non-PDF files', () => {
      const invalidFile = new File(['content'], 'test.txt', { type: 'text/plain' });
      const result = RealTimeValidator.validateFile(invalidFile);
      expect(result).toContainEqual(
        expect.objectContaining({
          code: 'invalid_file_type'
        })
      );
    });

    it('should reject empty files', () => {
      const emptyFile = new File([], 'test.pdf', { type: 'application/pdf' });
      const result = RealTimeValidator.validateFile(emptyFile);
      expect(result).toContainEqual(
        expect.objectContaining({
          code: 'empty_file'
        })
      );
    });

    it('should reject files that are too large', () => {
      const largeContent = new Array(11 * 1024 * 1024).fill('a').join(''); // 11MB
      const largeFile = new File([largeContent], 'large.pdf', { type: 'application/pdf' });
      const result = RealTimeValidator.validateFile(largeFile);
      expect(result).toContainEqual(
        expect.objectContaining({
          code: 'file_too_large'
        })
      );
    });

    it('should reject dangerous filenames', () => {
      const dangerousFile = new File(['content'], '../../../etc/passwd.pdf', { type: 'application/pdf' });
      const result = RealTimeValidator.validateFile(dangerousFile);
      expect(result).toContainEqual(
        expect.objectContaining({
          code: 'unsafe_filename'
        })
      );
    });

    it('should detect suspicious extensions', () => {
      const suspiciousFile = new File(['content'], 'test.pdf.exe', { type: 'application/pdf' });
      const result = RealTimeValidator.validateFile(suspiciousFile);
      expect(result).toContainEqual(
        expect.objectContaining({
          code: 'suspicious_extension'
        })
      );
    });
  });

  describe('validateSearchQuery', () => {
    it('should validate normal queries', () => {
      const result = RealTimeValidator.validateSearchQuery('What is machine learning?');
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
      expect(result.characterCount).toBe(25);
      expect(result.wordCount).toBe(4);
    });

    it('should reject queries that are too short', () => {
      const result = RealTimeValidator.validateSearchQuery('Hi');
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'query_too_short'
        })
      );
    });

    it('should reject queries that are too long', () => {
      const longQuery = 'a'.repeat(1001);
      const result = RealTimeValidator.validateSearchQuery(longQuery);
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'query_too_long'
        })
      );
    });

    it('should detect suspicious patterns', () => {
      const result = RealTimeValidator.validateSearchQuery('SELECT * FROM users; DROP TABLE users;');
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'query_suspicious'
        })
      );
    });

    it('should provide warnings for very long queries', () => {
      const longQuery = 'word '.repeat(51); // 51 words
      const result = RealTimeValidator.validateSearchQuery(longQuery);
      expect(result.warnings).toContainEqual(
        expect.objectContaining({
          code: 'query_very_long'
        })
      );
    });

    it('should sanitize input and warn about changes', () => {
      const result = RealTimeValidator.validateSearchQuery('test <script>alert("xss")</script> query');
      expect(result.warnings).toContainEqual(
        expect.objectContaining({
          code: 'query_sanitized'
        })
      );
      expect(result.sanitizedQuery).not.toContain('<script>');
    });
  });

  describe('validateConfigParameter', () => {
    it('should validate parameters within range', () => {
      const result = RealTimeValidator.validateConfigParameter('max_chunks', 10, 1, 20);
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should reject values below minimum', () => {
      const result = RealTimeValidator.validateConfigParameter('max_chunks', 0, 1, 20);
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'value_too_low'
        })
      );
      expect(result.sanitizedValue).toBe(1);
    });

    it('should reject values above maximum', () => {
      const result = RealTimeValidator.validateConfigParameter('max_chunks', 25, 1, 20);
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'value_too_high'
        })
      );
      expect(result.sanitizedValue).toBe(20);
    });

    it('should validate step increments', () => {
      const result = RealTimeValidator.validateConfigParameter('similarity_threshold', 0.73, 0, 1, 0.05);
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'invalid_step'
        })
      );
    });

    it('should provide performance warnings', () => {
      const result = RealTimeValidator.validateConfigParameter('max_chunks', 18, 1, 20);
      expect(result.warnings).toContainEqual(
        expect.objectContaining({
          code: 'performance_warning'
        })
      );
    });

    it('should handle invalid numbers', () => {
      const result = RealTimeValidator.validateConfigParameter('temperature', NaN, 0, 1);
      expect(result.isValid).toBe(false);
      expect(result.errors).toContainEqual(
        expect.objectContaining({
          code: 'invalid_number'
        })
      );
    });
  });
});

describe('debounce', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should debounce function calls', () => {
    const mockFn = vi.fn();
    const debouncedFn = debounce(mockFn, 100);

    debouncedFn('arg1');
    debouncedFn('arg2');
    debouncedFn('arg3');

    expect(mockFn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(100);

    expect(mockFn).toHaveBeenCalledTimes(1);
    expect(mockFn).toHaveBeenCalledWith('arg3');
  });

  it('should support cancellation', () => {
    const mockFn = vi.fn();
    const debouncedFn = debounce(mockFn, 100);

    debouncedFn('arg1');
    debouncedFn.cancel();

    vi.advanceTimersByTime(100);

    expect(mockFn).not.toHaveBeenCalled();
  });

  it('should support flushing', () => {
    const mockFn = vi.fn();
    const debouncedFn = debounce(mockFn, 100);

    debouncedFn('arg1');
    debouncedFn.flush();

    expect(mockFn).toHaveBeenCalledTimes(1);
    expect(mockFn).toHaveBeenCalledWith('arg1');
  });
});

describe('FormValidationManager', () => {
  it('should manage form validation state', () => {
    const initialData = { name: '', email: '' };
    const manager = new FormValidationManager(initialData);

    // Add validators
    manager.addValidator('name', (value) => 
      value.length < 2 ? [{ field: 'name', message: 'Name too short', code: 'too_short' }] : []
    );
    manager.addValidator('email', (value) => 
      !value.includes('@') ? [{ field: 'email', message: 'Invalid email', code: 'invalid' }] : []
    );

    // Test validation
    manager.updateField('name', 'A');
    expect(manager.getFieldErrors('name')).toHaveLength(1);
    expect(manager.isValid()).toBe(false);

    manager.updateField('name', 'Alice');
    manager.updateField('email', 'alice@example.com');
    expect(manager.isValid()).toBe(true);
  });
});

describe('DebouncedValidation', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should debounce search query validation', () => {
    const mockCallback = vi.fn();
    
    DebouncedValidation.validateSearchQueryDebounced('test query', mockCallback);
    DebouncedValidation.validateSearchQueryDebounced('test query updated', mockCallback);

    expect(mockCallback).not.toHaveBeenCalled();

    vi.advanceTimersByTime(300);

    expect(mockCallback).toHaveBeenCalledTimes(1);
    expect(mockCallback).toHaveBeenCalledWith(
      expect.objectContaining({
        sanitizedQuery: 'test query updated'
      })
    );
  });

  it('should handle validation errors gracefully', () => {
    const mockCallback = vi.fn();
    
    // Mock RealTimeValidator to throw an error
    const originalValidate = RealTimeValidator.validateSearchQuery;
    RealTimeValidator.validateSearchQuery = vi.fn().mockImplementation(() => {
      throw new Error('Validation error');
    });

    DebouncedValidation.validateSearchQueryDebounced('test', mockCallback);
    vi.advanceTimersByTime(300);

    expect(mockCallback).toHaveBeenCalledWith(
      expect.objectContaining({
        errors: expect.arrayContaining([
          expect.objectContaining({
            code: 'validation_error'
          })
        ])
      })
    );

    // Restore original function
    RealTimeValidator.validateSearchQuery = originalValidate;
  });
});