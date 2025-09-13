/**
 * Store Tests
 * Basic tests to verify store functionality
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import {
  appStore,
  appActions,
  uploadStore,
  uploadActions,
  searchStore,
  searchActions,
  healthStore,
  healthActions,
  toastStore,
  toastActions
} from './index.js';

describe('Store Tests', () => {
  beforeEach(() => {
    // Reset all stores before each test
    appActions.reset();
    uploadActions.reset();
    searchActions.reset();
    healthActions.reset();
    toastActions.clear();
  });

  describe('App Store', () => {
    it('should initialize with default state', () => {
      const state = get(appStore);
      expect(state.isLoading).toBe(false);
      expect(state.error).toBe(null);
      expect(state.currentPage).toBe('upload');
      expect(state.isOnline).toBe(true);
    });

    it('should update loading state', () => {
      appActions.setLoading(true);
      const state = get(appStore);
      expect(state.isLoading).toBe(true);
    });

    it('should set and clear errors', () => {
      appActions.setError('Test error');
      expect(get(appStore).error).toBe('Test error');
      
      appActions.clearError();
      expect(get(appStore).error).toBe(null);
    });
  });

  describe('Upload Store', () => {
    it('should initialize with default state', () => {
      const state = get(uploadStore);
      expect(state.uploadProgress).toBe(0);
      expect(state.isUploading).toBe(false);
      expect(state.uploadResult).toBe(null);
      expect(state.selectedFile).toBe(null);
      expect(state.dragActive).toBe(false);
    });

    it('should handle upload progress', () => {
      uploadActions.startUpload();
      expect(get(uploadStore).isUploading).toBe(true);
      
      uploadActions.updateProgress(50);
      expect(get(uploadStore).uploadProgress).toBe(50);
    });

    it('should handle file selection', () => {
      const mockFile = new File(['test'], 'test.pdf', { type: 'application/pdf' });
      uploadActions.setSelectedFile(mockFile);
      expect(get(uploadStore).selectedFile).toBe(mockFile);
    });
  });

  describe('Search Store', () => {
    it('should initialize with default state', () => {
      const state = get(searchStore);
      expect(state.query).toBe('');
      expect(state.results).toBe(null);
      expect(state.isSearching).toBe(false);
      expect(state.searchHistory).toEqual([]);
    });

    it('should update query', () => {
      searchActions.setQuery('test query');
      expect(get(searchStore).query).toBe('test query');
    });

    it('should handle search configuration', () => {
      searchActions.updateConfig({ max_chunks: 10 });
      const state = get(searchStore);
      expect(state.searchConfig.max_chunks).toBe(10);
    });
  });

  describe('Health Store', () => {
    it('should initialize with default state', () => {
      const state = get(healthStore);
      expect(state.status).toBe(null);
      expect(state.lastChecked).toBe(null);
      expect(state.isChecking).toBe(false);
      expect(state.checkInterval).toBe(30000);
    });

    it('should handle health check start', () => {
      healthActions.startCheck();
      expect(get(healthStore).isChecking).toBe(true);
    });
  });

  describe('Toast Store', () => {
    it('should initialize with empty state', () => {
      const toasts = get(toastStore);
      expect(toasts).toEqual([]);
    });

    it('should add and remove toasts', () => {
      const id = toastActions.success('Test message');
      expect(get(toastStore)).toHaveLength(1);
      
      toastActions.remove(id);
      expect(get(toastStore)).toHaveLength(0);
    });

    it('should add different types of toasts', () => {
      toastActions.success('Success message');
      toastActions.error('Error message');
      toastActions.warning('Warning message');
      toastActions.info('Info message');
      
      const toasts = get(toastStore);
      expect(toasts).toHaveLength(4);
      expect(toasts.map(t => t.type)).toEqual(['success', 'error', 'warning', 'info']);
    });
  });
});