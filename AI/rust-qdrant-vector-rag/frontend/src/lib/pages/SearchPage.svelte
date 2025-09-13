<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertCircle, Search as SearchIcon } from 'lucide-svelte';
  import { 
    SearchForm, 
    SearchConfig, 
    LoadingSpinner, 
    AnswerDisplay, 
    SourceReferences 
  } from '../components/index.js';
  import { searchStore, searchActions, isSearching, searchResults, searchConfig } from '../stores/search.store.js';
  import { toastActions } from '../stores/toast.store.js';
  import { apiService } from '../services/api.js';
  import type { QueryConfig, RAGResponse } from '../types/api.js';
  import type { AppError } from '../types/errors.js';

  // Local state
  let query = '';
  let showAdvanced = false;
  let searchError: string | null = null;

  // Subscribe to stores
  let currentResults: RAGResponse | null = null;
  let currentConfig: QueryConfig = {};
  let searching = false;

  $: currentResults = $searchResults;
  $: currentConfig = $searchConfig;
  $: searching = $isSearching;

  // Handle search form submission
  async function handleSearch(event: CustomEvent<{ query: string; config?: QueryConfig }>) {
    const { query: searchQuery, config } = event.detail;
    
    if (!searchQuery.trim()) {
      toastActions.show({
        type: 'warning',
        message: 'Please enter a search query'
      });
      return;
    }

    searchError = null;
    searchActions.setQuery(searchQuery);
    searchActions.startSearch();

    try {
      const response = await apiService.queryDocuments({
        question: searchQuery,
        config: config || currentConfig
      });

      searchActions.setResults(response);
      
      toastActions.show({
        type: 'success',
        message: `Found ${response.sources.length} relevant sources`
      });

    } catch (error) {
      console.error('Search failed:', error);
      
      const appError = error as AppError;
      searchError = appError.message || 'Search failed. Please try again.';
      
      searchActions.failSearch(searchError);
      
      toastActions.show({
        type: 'error',
        message: searchError
      });
    }
  }

  // Handle query input changes
  function handleQueryChange(event: CustomEvent<string>) {
    query = event.detail;
    searchActions.setQuery(query);
  }

  // Handle advanced options toggle
  function handleAdvancedToggle(event: CustomEvent<boolean>) {
    showAdvanced = event.detail;
  }

  // Handle configuration changes
  function handleConfigChange(event: CustomEvent<QueryConfig>) {
    searchActions.updateConfig(event.detail);
  }

  // Handle configuration reset
  function handleConfigReset() {
    searchActions.resetConfig();
    toastActions.show({
      type: 'info',
      message: 'Search configuration reset to defaults'
    });
  }

  // Handle answer actions
  function handleAnswerCopy(event: CustomEvent<string>) {
    toastActions.show({
      type: 'success',
      message: 'Answer copied to clipboard'
    });
  }

  function handleAnswerShare(event: CustomEvent<RAGResponse>) {
    // Implement sharing functionality
    console.log('Share response:', event.detail);
    toastActions.show({
      type: 'info',
      message: 'Sharing functionality coming soon'
    });
  }

  function handleAnswerBookmark(event: CustomEvent<RAGResponse>) {
    // Implement bookmarking functionality
    console.log('Bookmark response:', event.detail);
    toastActions.show({
      type: 'info',
      message: 'Bookmarking functionality coming soon'
    });
  }

  // Handle source reference actions
  function handleSourceClick(event: CustomEvent<any>) {
    console.log('Source clicked:', event.detail);
    // Implement source navigation
  }

  function handleSnippetCopy(event: CustomEvent<{ source: any; snippet: string }>) {
    toastActions.show({
      type: 'success',
      message: 'Snippet copied to clipboard'
    });
  }

  function handleViewDocument(event: CustomEvent<string>) {
    console.log('View document:', event.detail);
    // Implement document viewing
    toastActions.show({
      type: 'info',
      message: 'Document viewing functionality coming soon'
    });
  }

  // Retry search function
  function retrySearch() {
    if (query.trim()) {
      handleSearch(new CustomEvent('submit', { 
        detail: { query, config: currentConfig } 
      }));
    }
  }

  // Initialize component
  onMount(() => {
    // Set initial query from store if available
    const storeState = $searchStore;
    if (storeState.query) {
      query = storeState.query;
    }
  });
</script>

<div class="max-w-4xl mx-auto px-4 py-8 search-page">
  <div class="space-y-8">
    <!-- Page header -->
    <div class="page-header text-center">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white flex items-center justify-center gap-2">
        <SearchIcon size={32} />
        Search Documents
      </h1>
      <p class="text-lg text-gray-600 dark:text-gray-400 mt-2">
        Ask questions about your uploaded documents and get AI-powered answers
      </p>
    </div>

    <!-- Search form -->
    <div class="search-form-container flex justify-center">
      <SearchForm
        bind:query
        bind:showAdvanced
        config={currentConfig}
        disabled={searching}
        on:submit={handleSearch}
        on:query-change={handleQueryChange}
        on:toggle-advanced={handleAdvancedToggle}
      />
    </div>

    <!-- Advanced configuration -->
    <SearchConfig
      bind:visible={showAdvanced}
      config={currentConfig}
      disabled={searching}
      on:config-change={handleConfigChange}
      on:reset={handleConfigReset}
    />

    <!-- Search error -->
    {#if searchError}
      <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4" role="alert">
        <div class="flex items-start gap-3">
          <AlertCircle size={20} class="text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
          <div class="flex-1">
            <h3 class="text-sm font-medium text-red-800 dark:text-red-400">Search Error</h3>
            <div class="mt-2 space-y-2">
              <p class="text-sm text-red-700 dark:text-red-300">{searchError}</p>
              <div>
                <button 
                  class="retry-button"
                  on:click={retrySearch}
                  disabled={searching}
                >
                  Try Again
                </button>
              </div>
            </div>
          </div>
          <button
            type="button"
            on:click={() => searchError = null}
            class="text-red-400 hover:text-red-600 dark:text-red-300 dark:hover:text-red-100"
          >
            <span class="sr-only">Close</span>
            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>
    {/if}

    <!-- Loading state -->
    {#if searching}
      <div class="loading-container flex justify-center py-8">
        <LoadingSpinner 
          size="lg" 
          variant="search"
          message="Searching through your documents..."
        />
      </div>
    {/if}

    <!-- Search results -->
    {#if currentResults && !searching}
      <div class="results-container space-y-6">
        <!-- AI Answer -->
        <AnswerDisplay
          response={currentResults}
          showMetadata={true}
          allowCopy={true}
          allowShare={false}
          on:copy={handleAnswerCopy}
          on:share={handleAnswerShare}
          on:bookmark={handleAnswerBookmark}
        />

        <!-- Source References -->
        <SourceReferences
          sources={currentResults.sources}
          maxVisible={3}
          allowExpansion={true}
          showSnippets={true}
          highlightQuery={query}
          on:source-click={handleSourceClick}
          on:snippet-copy={handleSnippetCopy}
          on:view-document={handleViewDocument}
        />
      </div>
    {/if}

    <!-- Empty state when no results and not searching -->
    {#if !currentResults && !searching && !searchError}
      <div class="empty-state">
        <div class="flex flex-col items-center space-y-4 text-center">
          <SearchIcon size={48} class="text-gray-400 dark:text-gray-500" />
          <p class="text-lg text-gray-600 dark:text-gray-400">
            Enter a question above to search your documents
          </p>
          <p class="text-sm text-gray-500 dark:text-gray-500">
            Try asking specific questions about the content in your uploaded PDFs
          </p>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .search-page {
    min-height: 100vh;
  }

  .empty-state {
    padding: 4rem 2rem;
    text-align: center;
    background: var(--color-surface-50);
    border-radius: 0.75rem;
    border: 2px dashed var(--color-surface-300);
  }

  .retry-button {
    background: #dc2626;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background-color 0.2s ease;
  }

  .retry-button:hover:not(:disabled) {
    background: #b91c1c;
  }

  .retry-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .empty-state {
      background: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .empty-state {
      padding: 2rem 1rem;
      margin: 0 1rem;
    }
  }

  /* Print styles */
  @media print {
    .search-form-container,
    .loading-container,
    .empty-state {
      display: none;
    }
  }

  /* Accessibility improvements */
  .search-page {
    scroll-behavior: smooth;
  }

  .results-container {
    scroll-margin-top: 2rem;
  }

  /* Focus styles */
  .retry-button:focus {
    outline: 2px solid #dc2626;
    outline-offset: 2px;
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .empty-state {
      border-width: 3px;
      border-color: #000;
    }

    .retry-button {
      border: 2px solid #fff;
    }
  }
</style>