<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { Clock, Search, X, Trash2, ChevronDown, ChevronUp } from 'lucide-svelte';
  import { LocalStorageManager, STORAGE_KEYS } from '../utils/local-storage.js';
  import { announceToScreenReader } from '../utils/accessibility.js';
  import type { SearchHistoryItem } from '../types/state.js';

  // Props
  export let visible = false;
  export let maxItems = 10;
  export let allowClear = true;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    'select-query': string;
    'clear-history': void;
    'remove-item': string;
  }>();

  // Local state
  let searchHistory: SearchHistoryItem[] = [];
  let isExpanded = false;
  let visibleItems = 5;

  // Load search history from localStorage
  function loadSearchHistory(): void {
    if (LocalStorageManager.isAvailable()) {
      searchHistory = LocalStorageManager.get(STORAGE_KEYS.SEARCH_HISTORY, []);
    }
  }

  // Save search history to localStorage
  function saveSearchHistory(): void {
    if (LocalStorageManager.isAvailable()) {
      LocalStorageManager.set(STORAGE_KEYS.SEARCH_HISTORY, searchHistory);
    }
  }

  // Add item to search history
  export function addToHistory(query: string, resultCount: number = 0): void {
    const newItem: SearchHistoryItem = {
      id: crypto.randomUUID(),
      query: query.trim(),
      timestamp: new Date(),
      resultCount
    };

    // Remove duplicate queries
    searchHistory = searchHistory.filter(item => item.query !== newItem.query);
    
    // Add to beginning and limit to maxItems
    searchHistory = [newItem, ...searchHistory].slice(0, maxItems);
    
    saveSearchHistory();
    announceToScreenReader(`Added "${query}" to search history`, 'polite');
  }

  // Select a query from history
  function selectQuery(query: string): void {
    dispatch('select-query', query);
    announceToScreenReader(`Selected query: ${query}`, 'polite');
  }

  // Remove item from history
  function removeItem(id: string, event: Event): void {
    event.stopPropagation();
    const item = searchHistory.find(h => h.id === id);
    searchHistory = searchHistory.filter(h => h.id !== id);
    saveSearchHistory();
    dispatch('remove-item', id);
    
    if (item) {
      announceToScreenReader(`Removed "${item.query}" from search history`, 'polite');
    }
  }

  // Clear all history
  function clearHistory(): void {
    searchHistory = [];
    saveSearchHistory();
    dispatch('clear-history');
    announceToScreenReader('Search history cleared', 'polite');
  }

  // Toggle expanded view
  function toggleExpanded(): void {
    isExpanded = !isExpanded;
    announceToScreenReader(
      isExpanded ? 'Search history expanded' : 'Search history collapsed',
      'polite'
    );
  }

  // Format timestamp
  function formatTimestamp(timestamp: Date): string {
    const now = new Date();
    const diff = now.getTime() - timestamp.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
    return timestamp.toLocaleDateString();
  }

  // Get visible history items
  $: displayedItems = isExpanded ? searchHistory : searchHistory.slice(0, visibleItems);
  $: hasMoreItems = searchHistory.length > visibleItems;

  // Handle keyboard navigation
  function handleKeyDown(event: KeyboardEvent, query: string): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      selectQuery(query);
    }
  }

  // Initialize component
  onMount(() => {
    loadSearchHistory();
  });
</script>

{#if visible && searchHistory.length > 0}
  <div class="search-history" role="region" aria-labelledby="search-history-title">
    <div class="search-history-header">
      <h3 id="search-history-title" class="search-history-title">
        <Clock size={16} aria-hidden="true" />
        Recent Searches
      </h3>
      
      <div class="search-history-actions">
        {#if hasMoreItems}
          <button
            type="button"
            class="expand-button"
            on:click={toggleExpanded}
            aria-expanded={isExpanded}
            aria-label={isExpanded ? 'Show fewer searches' : 'Show all searches'}
          >
            {#if isExpanded}
              <ChevronUp size={16} />
            {:else}
              <ChevronDown size={16} />
            {/if}
          </button>
        {/if}
        
        {#if allowClear}
          <button
            type="button"
            class="clear-button"
            on:click={clearHistory}
            aria-label="Clear search history"
            title="Clear all search history"
          >
            <Trash2 size={16} />
          </button>
        {/if}
      </div>
    </div>

    <div class="search-history-list" role="list">
      {#each displayedItems as item (item.id)}
        <div class="search-history-item" role="listitem">
          <button
            class="history-item-button"
            on:click={() => selectQuery(item.query)}
            on:keydown={(e) => handleKeyDown(e, item.query)}
            aria-label={`Search for "${item.query}" from ${formatTimestamp(item.timestamp)}`}
          >
            <div class="history-item-content">
              <div class="history-item-main">
                <Search size={14} class="history-item-icon" aria-hidden="true" />
                <span class="history-item-query">{item.query}</span>
              </div>
              
              <div class="history-item-meta">
                <span class="history-item-time" aria-label={`Searched ${formatTimestamp(item.timestamp)}`}>
                  {formatTimestamp(item.timestamp)}
                </span>
                {#if item.resultCount > 0}
                  <span class="history-item-results" aria-label={`${item.resultCount} results found`}>
                    {item.resultCount} results
                  </span>
                {/if}
              </div>
            </div>
          </button>

          <button
            type="button"
            class="remove-item-button"
            on:click={(e) => removeItem(item.id, e)}
            aria-label={`Remove "${item.query}" from history`}
            title="Remove from history"
          >
            <X size={14} />
          </button>
        </div>
      {/each}
    </div>

    {#if hasMoreItems && !isExpanded}
      <div class="search-history-footer">
        <button
          type="button"
          class="show-more-button"
          on:click={toggleExpanded}
          aria-label={`Show ${searchHistory.length - visibleItems} more searches`}
        >
          Show {searchHistory.length - visibleItems} more
        </button>
      </div>
    {/if}
  </div>
{/if}

<style>
  .search-history {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .search-history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: #f9fafb;
    border-bottom: 1px solid #e5e7eb;
  }

  .search-history-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
  }

  .search-history-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .expand-button,
  .clear-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background: none;
    border: none;
    border-radius: 0.25rem;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .expand-button:hover,
  .clear-button:hover {
    background: #e5e7eb;
    color: #374151;
  }

  .expand-button:focus,
  .clear-button:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  .search-history-list {
    max-height: 300px;
    overflow-y: auto;
  }

  .search-history-item {
    display: flex;
    align-items: center;
    border-bottom: 1px solid #f3f4f6;
    position: relative;
  }

  .search-history-item:last-child {
    border-bottom: none;
  }

  .search-history-item:hover {
    background: #f9fafb;
  }

  .history-item-button {
    flex: 1;
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s ease;
    text-align: left;
    width: 100%;
  }

  .history-item-button:focus {
    outline: 2px solid #3b82f6;
    outline-offset: -2px;
    background: #eff6ff;
  }

  .history-item-content {
    flex: 1;
    min-width: 0;
  }

  .history-item-main {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.25rem;
  }

  .history-item-icon {
    color: #6b7280;
    flex-shrink: 0;
  }

  .history-item-query {
    font-size: 0.875rem;
    color: #111827;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .history-item-meta {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-left: 1.25rem;
  }

  .history-item-time {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .history-item-results {
    font-size: 0.75rem;
    color: #059669;
    font-weight: 500;
  }

  .remove-item-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background: none;
    border: none;
    border-radius: 0.25rem;
    color: #9ca3af;
    cursor: pointer;
    transition: all 0.2s ease;
    opacity: 0;
    margin-left: 0.5rem;
  }

  .search-history-item:hover .remove-item-button,
  .search-history-item:focus-within .remove-item-button {
    opacity: 1;
  }

  .remove-item-button:hover {
    background: #fee2e2;
    color: #dc2626;
  }

  .remove-item-button:focus {
    outline: 2px solid #dc2626;
    outline-offset: 2px;
    opacity: 1;
  }

  .search-history-footer {
    padding: 0.5rem 1rem;
    background: #f9fafb;
    border-top: 1px solid #e5e7eb;
    text-align: center;
  }

  .show-more-button {
    font-size: 0.75rem;
    color: #3b82f6;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
  }

  .show-more-button:hover {
    background: #dbeafe;
    color: #1d4ed8;
  }

  .show-more-button:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .search-history {
      background: #1f2937;
      border-color: #374151;
    }

    .search-history-header {
      background: #111827;
      border-bottom-color: #374151;
    }

    .search-history-title {
      color: #f3f4f6;
    }

    .expand-button,
    .clear-button {
      color: #9ca3af;
    }

    .expand-button:hover,
    .clear-button:hover {
      background: #374151;
      color: #f3f4f6;
    }

    .search-history-item {
      border-bottom-color: #374151;
    }

    .search-history-item:hover {
      background: #111827;
    }

    .history-item-button:focus {
      background: #1e3a8a;
    }

    .history-item-query {
      color: #f9fafb;
    }

    .history-item-time {
      color: #9ca3af;
    }

    .history-item-results {
      color: #10b981;
    }

    .history-item-icon {
      color: #9ca3af;
    }

    .remove-item-button {
      color: #6b7280;
    }

    .remove-item-button:hover {
      background: #7f1d1d;
      color: #f87171;
    }

    .search-history-footer {
      background: #111827;
      border-top-color: #374151;
    }

    .show-more-button {
      color: #60a5fa;
    }

    .show-more-button:hover {
      background: #1e3a8a;
      color: #93c5fd;
    }
  }

  /* Mobile responsive */
  @media (max-width: 768px) {
    .search-history-item {
      padding: 1rem;
    }

    .history-item-meta {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }

    .history-item-query {
      font-size: 1rem;
    }

    .remove-item-button {
      opacity: 1;
      min-height: 44px;
      min-width: 44px;
    }
  }

  /* High contrast mode */
  @media (prefers-contrast: high) {
    .search-history {
      border-width: 2px;
      border-color: #000;
    }

    .search-history-item:focus {
      outline-width: 3px;
    }

    .expand-button:focus,
    .clear-button:focus,
    .remove-item-button:focus,
    .show-more-button:focus {
      outline-width: 3px;
    }
  }

  /* Reduced motion */
  @media (prefers-reduced-motion: reduce) {
    .search-history-item,
    .expand-button,
    .clear-button,
    .remove-item-button,
    .show-more-button {
      transition: none;
    }
  }
</style>