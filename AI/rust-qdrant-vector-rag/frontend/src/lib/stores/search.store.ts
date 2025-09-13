/**
 * Search State Store
 * Manages search queries, results, configuration, and history
 */

import { writable, derived } from 'svelte/store';
import { errorHandler } from '../services/error-handler.js';
import { toastActions } from './toast.store.js';
import type { SearchState, SearchHistoryItem } from '../types/state.js';
import type { RAGResponse, QueryConfig } from '../types/api.js';
import type { AppError, SearchError } from '../types/errors.js';

// Default search configuration
const defaultSearchConfig: QueryConfig = {
  max_chunks: 5,
  similarity_threshold: 0.7,
  max_response_tokens: 500,
  temperature: 0.3,
  include_low_confidence: false
};

// Initial state
const initialSearchState: SearchState = {
  query: '',
  results: null,
  isSearching: false,
  searchConfig: { ...defaultSearchConfig },
  searchHistory: []
};

// Create the writable store
export const searchStore = writable<SearchState>(initialSearchState);

// Derived stores for specific state slices
export const searchQuery = derived(searchStore, ($search) => $search.query);
export const searchResults = derived(searchStore, ($search) => $search.results);
export const isSearching = derived(searchStore, ($search) => $search.isSearching);
export const searchConfig = derived(searchStore, ($search) => $search.searchConfig);
export const searchHistory = derived(searchStore, ($search) => $search.searchHistory);

// Derived computed values
export const hasResults = derived(searchStore, ($search) => 
  $search.results !== null
);

export const hasSearchError = derived(searchStore, ($search) => 
  $search.results === null && !$search.isSearching && $search.query.length > 0
);

export const queryCharacterCount = derived(searchStore, ($search) => 
  $search.query.length
);

export const isQueryValid = derived(searchStore, ($search) => 
  $search.query.trim().length >= 3 && $search.query.length <= 1000
);

// Store actions
export const searchActions = {
  setQuery: (query: string) => {
    searchStore.update(state => ({ 
      ...state, 
      query,
      results: null // Clear results when query changes
    }));
  },

  updateConfig: (config: Partial<QueryConfig>) => {
    searchStore.update(state => ({ 
      ...state, 
      searchConfig: { ...state.searchConfig, ...config }
    }));
  },

  resetConfig: () => {
    searchStore.update(state => ({ 
      ...state, 
      searchConfig: { ...defaultSearchConfig }
    }));
  },

  startSearch: () => {
    searchStore.update(state => ({ 
      ...state, 
      isSearching: true,
      results: null
    }));
  },

  setResults: (results: RAGResponse) => {
    searchStore.update(state => {
      // Add to search history
      const historyItem: SearchHistoryItem = {
        id: crypto.randomUUID(),
        query: results.query,
        timestamp: new Date(),
        resultCount: results.sources.length
      };

      const updatedHistory = [historyItem, ...state.searchHistory.slice(0, 9)]; // Keep last 10

      return {
        ...state,
        results,
        isSearching: false,
        searchHistory: updatedHistory
      };
    });
  },

  failSearch: (error: string | AppError, retryAction?: () => Promise<void>) => {
    if (typeof error === 'string') {
      toastActions.error(error);
    } else {
      // Handle AppError with comprehensive error handling
      const errorContext = errorHandler.handleError(error, retryAction, false);
      
      // Show toast with recovery options for search errors
      if (error.type === 'search_error') {
        const searchError = error as SearchError;
        if (searchError.retryable && retryAction) {
          toastActions.error(errorContext.userMessage, {
            duration: 0, // Don't auto-dismiss
            dismissible: true
          });
        } else {
          // For non-retryable search errors (like no results), show as info
          if (searchError.reason === 'no_results') {
            toastActions.info(errorContext.userMessage);
          } else {
            toastActions.warning(errorContext.userMessage);
          }
        }
      } else {
        toastActions.error(errorContext.userMessage);
      }
    }

    searchStore.update(state => ({ 
      ...state, 
      results: null,
      isSearching: false
    }));
  },

  clearResults: () => {
    searchStore.update(state => ({ 
      ...state, 
      results: null
    }));
  },

  clearHistory: () => {
    searchStore.update(state => ({ 
      ...state, 
      searchHistory: []
    }));
  },

  removeFromHistory: (id: string) => {
    searchStore.update(state => ({ 
      ...state, 
      searchHistory: state.searchHistory.filter(item => item.id !== id)
    }));
  },

  loadFromHistory: (historyItem: SearchHistoryItem) => {
    searchStore.update(state => ({ 
      ...state, 
      query: historyItem.query,
      results: null
    }));
  },

  reset: () => {
    searchStore.set(initialSearchState);
  }
};