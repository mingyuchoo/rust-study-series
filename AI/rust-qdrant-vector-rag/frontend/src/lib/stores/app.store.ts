/**
 * Global Application State Store
 * Manages overall application state including loading, errors, and navigation
 */

import { writable, derived } from 'svelte/store';
import type { AppState } from '../types/state.js';

// Initial state
const initialAppState: AppState = {
  isLoading: false,
  error: null,
  currentPage: 'upload',
  isOnline: navigator.onLine
};

// Create the writable store
export const appStore = writable<AppState>(initialAppState);

// Derived stores for specific state slices
export const isLoading = derived(appStore, ($app) => $app.isLoading);
export const currentError = derived(appStore, ($app) => $app.error);
export const currentPage = derived(appStore, ($app) => $app.currentPage);
export const isOnline = derived(appStore, ($app) => $app.isOnline);

// Store actions
export const appActions = {
  setLoading: (loading: boolean) => {
    appStore.update(state => ({ ...state, isLoading: loading }));
  },

  setError: (error: string | null) => {
    appStore.update(state => ({ ...state, error }));
  },

  clearError: () => {
    appStore.update(state => ({ ...state, error: null }));
  },

  setCurrentPage: (page: string) => {
    appStore.update(state => ({ ...state, currentPage: page }));
  },

  setOnlineStatus: (isOnline: boolean) => {
    appStore.update(state => ({ ...state, isOnline }));
  },

  reset: () => {
    appStore.set(initialAppState);
  }
};

// Listen for online/offline events
if (typeof window !== 'undefined') {
  window.addEventListener('online', () => appActions.setOnlineStatus(true));
  window.addEventListener('offline', () => appActions.setOnlineStatus(false));
}