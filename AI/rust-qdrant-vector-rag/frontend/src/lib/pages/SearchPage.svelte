<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { AlertCircle, Search as SearchIcon, Sparkles, HelpCircle, Clock } from 'lucide-svelte';
  import { 
    SearchForm, 
    SearchConfig, 
    LoadingSpinner, 
    AnswerDisplay, 
    SourceReferences,
    InteractiveButton,
    SuccessNotification,
    LoadingOverlay
  } from '../components/index.js';
  import SearchHistory from '../components/SearchHistory.svelte';
  import KeyboardShortcutsHelp from '../components/KeyboardShortcutsHelp.svelte';
  import { searchStore, searchActions, isSearching, searchResults, searchConfig } from '../stores/search.store.js';
  import { toastActions } from '../stores/toast.store.js';
  import { apiService } from '../services/api.js';
  import { errorHandler } from '../services/error-handler.js';
  import { KeyboardShortcutManager, DEFAULT_SHORTCUTS } from '../utils/keyboard-shortcuts.js';
  import { LocalStorageManager, STORAGE_KEYS } from '../utils/local-storage.js';
  import type { QueryConfig, RAGResponse } from '../types/api.js';
  import type { AppError } from '../types/errors.js';

  // Local state
  let query = '';
  let showAdvanced = false;
  let searchError: string | null = null;
  let showSuccessNotification = false;
  let showLoadingOverlay = false;
  let searchStartTime: number | null = null;
  let showSearchHistory = false;
  let showKeyboardHelp = false;
  let searchHistoryComponent: SearchHistory;
  let keyboardManager: KeyboardShortcutManager;
  let searchInputElement: HTMLInputElement;

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
      const searchError = errorHandler.createSearchError(
        'Please enter a search query',
        searchQuery,
        'query_too_short'
      );
      searchActions.failSearch(searchError);
      return;
    }

    // Check if we're online before attempting search
    if (!errorHandler.isOnline()) {
      const networkError = errorHandler.createNetworkError(
        'Cannot search while offline. Please check your connection.',
        { url: '/query', method: 'POST' }
      );
      searchActions.failSearch(networkError, () => handleSearch(event));
      return;
    }

    searchError = null;
    searchActions.setQuery(searchQuery);
    searchActions.startSearch();
    searchStartTime = Date.now();

    try {
      const response = await apiService.queryDocuments({
        question: searchQuery,
        config: config || currentConfig
      });

      searchActions.setResults(response);
      
      // Add to search history
      if (searchHistoryComponent) {
        searchHistoryComponent.addToHistory(searchQuery, response.sources.length);
      }
      
      // Calculate search time
      const searchTime = searchStartTime ? (Date.now() - searchStartTime) / 1000 : 0;
      
      // Show enhanced success notification
      showSuccessNotification = true;
      toastActions.success(`Found ${response.sources.length} relevant sources in ${searchTime.toFixed(1)}s`);

    } catch (error) {
      console.error('Search failed:', error);
      
      // Handle error with comprehensive error handling and retry capability
      const appError = error as AppError;
      searchActions.failSearch(appError, () => handleSearch(event));
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

  // Handle search history actions
  function handleSelectQuery(event: CustomEvent<string>) {
    query = event.detail;
    searchActions.setQuery(query);
    showSearchHistory = false;
    
    // Focus search input after selection
    if (searchInputElement) {
      searchInputElement.focus();
    }
  }

  function handleClearHistory() {
    toastActions.show({
      type: 'info',
      message: 'Search history cleared'
    });
  }

  function handleRemoveHistoryItem() {
    // History item removed - no additional action needed
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

  // Keyboard shortcuts setup
  function setupKeyboardShortcuts() {
    keyboardManager = new KeyboardShortcutManager();
    
    // Focus search input
    keyboardManager.register({
      key: '/',
      description: 'Focus search input',
      action: () => {
        if (searchInputElement) {
          searchInputElement.focus();
        }
      }
    });

    // New search
    keyboardManager.register({
      key: 'n',
      ctrlKey: true,
      description: 'Start new search',
      action: () => {
        query = '';
        searchActions.clearResults();
        if (searchInputElement) {
          searchInputElement.focus();
        }
      }
    });

    // Toggle advanced options
    keyboardManager.register({
      key: 'a',
      ctrlKey: true,
      description: 'Toggle advanced options',
      action: () => {
        showAdvanced = !showAdvanced;
      }
    });

    // Copy answer (when results are visible)
    keyboardManager.register({
      key: 'c',
      ctrlKey: true,
      shiftKey: true,
      description: 'Copy AI answer',
      action: () => {
        if (currentResults) {
          navigator.clipboard.writeText(currentResults.answer.replace(/<[^>]*>/g, ''));
          toastActions.success('Answer copied to clipboard');
        }
      }
    });

    // Clear search results
    keyboardManager.register({
      key: 'Escape',
      description: 'Clear search results',
      action: () => {
        if (currentResults) {
          searchActions.clearResults();
        }
      }
    });

    // Show keyboard shortcuts help
    keyboardManager.register({
      key: '?',
      description: 'Show keyboard shortcuts help',
      action: () => {
        showKeyboardHelp = true;
      }
    });

    // Toggle search history
    keyboardManager.register({
      key: 'h',
      ctrlKey: true,
      description: 'Toggle search history',
      action: () => {
        showSearchHistory = !showSearchHistory;
      }
    });
  }

  // Initialize component
  onMount(() => {
    // Set initial query from store if available
    const storeState = $searchStore;
    if (storeState.query) {
      query = storeState.query;
    }

    // Setup keyboard shortcuts
    setupKeyboardShortcuts();
  });

  // Cleanup
  onDestroy(() => {
    if (keyboardManager) {
      keyboardManager.destroy();
    }
  });
</script>

<div class="search-page container-responsive">
  <div class="search-layout">
    <!-- Page header -->
    <header class="page-header">
      <div class="header-content">
        <div class="title-section">
          <h1 class="page-title text-responsive-xl" id="search-page-title">
            <SearchIcon size={32} aria-hidden="true" />
            Search Documents
          </h1>
          <p class="page-description text-responsive-base" aria-describedby="search-page-title">
            Ask questions about your uploaded documents and get AI-powered answers
          </p>
        </div>
        
        <div class="header-actions">
          <button
            type="button"
            class="header-action-button"
            on:click={() => showSearchHistory = !showSearchHistory}
            aria-label="Toggle search history"
            title="Search history (Ctrl+H)"
          >
            <Clock size={20} />
          </button>
          
          <button
            type="button"
            class="header-action-button"
            on:click={() => showKeyboardHelp = true}
            aria-label="Show keyboard shortcuts"
            title="Keyboard shortcuts (?)"
          >
            <HelpCircle size={20} />
          </button>
        </div>
      </div>
    </header>

    <!-- Search form -->
    <main class="search-main" aria-labelledby="search-page-title">
      <!-- Search history -->
      {#if showSearchHistory}
        <section class="search-history-container" aria-labelledby="search-history-title">
          <h2 id="search-history-title" class="sr-only">Search History</h2>
          <SearchHistory
            bind:this={searchHistoryComponent}
            visible={showSearchHistory}
            maxItems={10}
            allowClear={true}
            on:select-query={handleSelectQuery}
            on:clear-history={handleClearHistory}
            on:remove-item={handleRemoveHistoryItem}
          />
        </section>
      {/if}

      <section class="search-form-container" aria-labelledby="search-form-title">
        <h2 id="search-form-title" class="sr-only">Search Form</h2>
        <SearchForm
          bind:query
          bind:showAdvanced
          config={currentConfig}
          disabled={searching}
          on:submit={handleSearch}
          on:query-change={handleQueryChange}
          on:toggle-advanced={handleAdvancedToggle}
        />
      </section>

      <!-- Advanced configuration -->
      <section class="search-config-container" aria-labelledby="search-config-title">
        <h2 id="search-config-title" class="sr-only">Advanced Search Configuration</h2>
        <SearchConfig
          bind:visible={showAdvanced}
          config={currentConfig}
          disabled={searching}
          initiallyExpanded={false}
          on:config-change={handleConfigChange}
          on:reset={handleConfigReset}
        />
      </section>

      <!-- Search error -->
      {#if searchError}
        <section class="search-error" role="alert" aria-labelledby="error-title" aria-live="assertive">
          <div class="error-content">
            <span class="error-icon" aria-hidden="true">
              <AlertCircle size={20} aria-hidden="true" />
            </span>
            <div class="error-details">
              <h3 id="error-title" class="error-title">Search Error</h3>
              <p class="error-message">{searchError}</p>
              <div class="error-actions">
                <InteractiveButton
                  variant="error"
                  size="sm"
                  loading={searching}
                  loadingText="Retrying..."
                  rippleEffect={true}
                  hoverEffect={true}
                  focusEffect={true}
                  ariaLabel="Retry the search with the same query"
                  on:click={retrySearch}
                >
                  Try Again
                </InteractiveButton>
              </div>
            </div>
            <button
              type="button"
              class="close-button focus-ring-enhanced"
              on:click={() => searchError = null}
              aria-label="Close error message"
            >
              <svg class="close-icon" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>
        </section>
      {/if}

      <!-- Loading state -->
      {#if searching}
        <section class="loading-container" role="status" aria-live="polite" aria-labelledby="loading-title">
          <h2 id="loading-title" class="sr-only">Searching</h2>
          <div class="enhanced-loading">
            <LoadingSpinner 
              size="lg" 
              variant="search"
              message="AI is analyzing your question..."
            />
            <div class="loading-details">
              <p class="loading-step">
                <Sparkles size={16} />
                Processing natural language query
              </p>
              <p class="loading-step">
                <SearchIcon size={16} />
                Searching vector database
              </p>
              <p class="loading-step">
                <Sparkles size={16} />
                Generating AI response
              </p>
            </div>
          </div>
        </section>
      {/if}

      <!-- Search results -->
      {#if currentResults && !searching}
        <section class="results-container" aria-labelledby="results-title">
          <h2 id="results-title" class="sr-only">Search Results</h2>
          
          <!-- AI Answer -->
          <div class="answer-section" aria-labelledby="answer-title">
            <h3 id="answer-title" class="sr-only">AI Generated Answer</h3>
            <AnswerDisplay
              response={currentResults}
              showMetadata={true}
              allowCopy={true}
              allowShare={false}
              on:copy={handleAnswerCopy}
              on:share={handleAnswerShare}
              on:bookmark={handleAnswerBookmark}
            />
          </div>

          <!-- Source References -->
          <div class="sources-section" aria-labelledby="sources-title">
            <h3 id="sources-title" class="sr-only">Source References</h3>
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
        </section>
      {/if}

      <!-- Empty state when no results and not searching -->
      {#if !currentResults && !searching && !searchError}
        <section class="empty-state" role="status" aria-labelledby="empty-state-title">
          <div class="empty-state-content">
            <span class="empty-state-icon" aria-hidden="true">
              <SearchIcon size={48} aria-hidden="true" />
            </span>
            <h2 id="empty-state-title" class="empty-state-title text-responsive-lg">
              Enter a question above to search your documents
            </h2>
            <p class="empty-state-description text-responsive-sm">
              Try asking specific questions about the content in your uploaded PDFs
            </p>
          </div>
        </section>
      {/if}
    </main>
  </div>
</div>

<!-- Success notification for search results -->
<SuccessNotification
  bind:visible={showSuccessNotification}
  title="Search Complete!"
  message="Found relevant information in your documents"
  variant="success"
  duration={4000}
  actions={[
    { label: 'New Search', action: 'new_search', variant: 'secondary' }
  ]}
  on:action={(e) => {
    if (e.detail.action === 'new_search') {
      query = '';
      searchActions.clearResults();
    }
    showSuccessNotification = false;
  }}
  on:dismiss={() => showSuccessNotification = false}
/>

<!-- Loading overlay for intensive search operations -->
<LoadingOverlay
  bind:visible={showLoadingOverlay}
  message="AI is processing your query..."
  variant="search"
  showProgress={false}
  backdrop="blur"
  size="md"
  allowDismiss={false}
/>

<!-- Keyboard shortcuts help -->
<KeyboardShortcutsHelp
  bind:visible={showKeyboardHelp}
  shortcuts={keyboardManager ? keyboardManager.getShortcuts() : []}
  on:close={() => showKeyboardHelp = false}
/>

<style>
  .search-page {
    min-height: 100vh;
    padding: var(--spacing-xl) 0;
    background-color: var(--color-surface-50);
    scroll-behavior: smooth;
  }

  .search-layout {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-2xl);
    max-width: 1200px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: var(--spacing-xl);
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--spacing-lg);
  }

  .title-section {
    text-align: center;
    flex: 1;
  }

  .header-actions {
    display: flex;
    gap: var(--spacing-sm);
  }

  .header-action-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-sm);
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s ease;
    min-height: 44px;
    min-width: 44px;
  }

  .header-action-button:hover {
    background: #f9fafb;
    color: #374151;
    border-color: #d1d5db;
  }

  .header-action-button:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  .page-title {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    margin: 0 0 var(--spacing-lg) 0;
    font-weight: 700;
    color: var(--color-surface-900);
    line-height: var(--line-height-tight);
  }

  .page-description {
    margin: 0 auto;
    color: var(--color-surface-600);
    max-width: 600px;
    line-height: var(--line-height-relaxed);
  }

  .search-main {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .search-form-container {
    display: flex;
    justify-content: center;
  }

  .search-config-container {
    display: flex;
    justify-content: center;
  }

  .search-history-container {
    display: flex;
    justify-content: center;
    margin-bottom: var(--spacing-lg);
  }

  /* Error styles */
  .search-error {
    background-color: var(--color-error-50);
    border: 2px solid var(--color-error-200);
    border-radius: 0.75rem;
    padding: var(--spacing-lg);
    margin: var(--spacing-lg) 0;
  }

  .error-content {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-md);
  }

  .error-icon {
    color: var(--color-error-600);
    flex-shrink: 0;
    margin-top: 2px;
  }

  .error-details {
    flex: 1;
  }

  .error-title {
    margin: 0 0 var(--spacing-sm) 0;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--color-error-800);
  }

  .error-message {
    margin: 0 0 var(--spacing-md) 0;
    color: var(--color-error-700);
    line-height: var(--line-height-normal);
  }

  .error-actions {
    display: flex;
    gap: var(--spacing-sm);
  }



  .close-button {
    background: none;
    border: none;
    color: var(--color-error-400);
    cursor: pointer;
    padding: var(--spacing-xs);
    border-radius: 0.375rem;
    transition: color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .close-button:hover {
    color: var(--color-error-600);
  }

  .close-icon {
    width: 20px;
    height: 20px;
  }

  /* Loading styles */
  .loading-container {
    display: flex;
    justify-content: center;
    padding: var(--spacing-2xl) 0;
  }

  .enhanced-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xl);
    max-width: 400px;
    text-align: center;
  }

  .loading-details {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    opacity: 0.8;
  }

  .loading-step {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-xs);
    font-size: var(--font-size-sm);
    color: var(--color-surface-600);
    margin: 0;
    animation: fade-in-sequence 0.5s ease-out forwards;
  }

  .loading-step:nth-child(1) {
    animation-delay: 0.5s;
    opacity: 0;
  }

  .loading-step:nth-child(2) {
    animation-delay: 1s;
    opacity: 0;
  }

  .loading-step:nth-child(3) {
    animation-delay: 1.5s;
    opacity: 0;
  }

  @keyframes fade-in-sequence {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 0.8;
      transform: translateY(0);
    }
  }

  /* Results styles */
  .results-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .answer-section,
  .sources-section {
    position: relative;
  }

  /* Empty state styles */
  .empty-state {
    padding: var(--spacing-4xl) var(--spacing-2xl);
    text-align: center;
    background: var(--color-surface-100);
    border-radius: 1rem;
    border: 2px dashed var(--color-surface-300);
    margin: var(--spacing-2xl) 0;
  }

  .empty-state-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-lg);
    max-width: 500px;
    margin: 0 auto;
  }

  .empty-state-icon {
    color: var(--color-surface-400);
  }

  .empty-state-title {
    margin: 0;
    color: var(--color-surface-600);
    font-weight: 600;
  }

  .empty-state-description {
    margin: 0;
    color: var(--color-surface-500);
    line-height: var(--line-height-relaxed);
  }

  /* Mobile styles (default) */
  @media (max-width: 767px) {
    .search-page {
      padding: var(--spacing-lg) 0;
    }

    .search-layout {
      gap: var(--spacing-xl);
    }

    .page-header {
      margin-bottom: var(--spacing-lg);
    }

    .header-content {
      flex-direction: column;
      gap: var(--spacing-md);
    }

    .page-title {
      flex-direction: column;
      gap: var(--spacing-sm);
    }

    .header-actions {
      justify-content: center;
    }

    .search-main {
      gap: var(--spacing-lg);
    }

    .empty-state {
      padding: var(--spacing-2xl) var(--spacing-lg);
      margin: var(--spacing-lg) 0;
    }

    .empty-state-content {
      gap: var(--spacing-md);
    }

    .error-content {
      flex-direction: column;
      align-items: stretch;
      gap: var(--spacing-sm);
    }

    .error-actions {
      justify-content: center;
    }


  }

  /* Tablet styles */
  @media (min-width: 768px) and (max-width: 1023px) {
    .search-page {
      padding: var(--spacing-2xl) 0;
    }

    .search-layout {
      gap: var(--spacing-2xl);
    }

    .page-title {
      gap: var(--spacing-md);
    }

    .empty-state {
      padding: var(--spacing-3xl) var(--spacing-2xl);
    }
  }

  /* Desktop styles */
  @media (min-width: 1024px) {
    .search-page {
      padding: var(--spacing-3xl) 0;
    }

    .search-layout {
      gap: var(--spacing-3xl);
    }

    .page-header {
      margin-bottom: var(--spacing-2xl);
    }

    .search-main {
      gap: var(--spacing-2xl);
    }

    .empty-state {
      padding: var(--spacing-4xl) var(--spacing-3xl);
    }


  }

  /* Large desktop styles */
  @media (min-width: 1440px) {
    .search-layout {
      max-width: 1400px;
    }

    .empty-state {
      padding: var(--spacing-5xl) var(--spacing-4xl);
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .search-page {
      background-color: var(--color-surface-900);
    }

    .page-title {
      color: var(--color-surface-100);
    }

    .page-description {
      color: var(--color-surface-300);
    }

    .search-error {
      background-color: var(--color-error-900);
      border-color: var(--color-error-700);
    }

    .error-title {
      color: var(--color-error-300);
    }

    .error-message {
      color: var(--color-error-400);
    }

    .error-icon {
      color: var(--color-error-400);
    }

    .close-button {
      color: var(--color-error-300);
    }

    .close-button:hover {
      color: var(--color-error-100);
    }

    .empty-state {
      background: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .empty-state-icon {
      color: var(--color-surface-500);
    }

    .empty-state-title {
      color: var(--color-surface-400);
    }

    .empty-state-description {
      color: var(--color-surface-500);
    }

    .header-action-button {
      background: #1f2937;
      border-color: #374151;
      color: #9ca3af;
    }

    .header-action-button:hover {
      background: #374151;
      color: #f3f4f6;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .search-error {
      border-width: 3px;
    }

    .empty-state {
      border-width: 3px;
      border-color: #000;
    }



    .close-button {
      border: 2px solid;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .search-page {
      scroll-behavior: auto;
    }



    .close-button {
      transition: none;
    }
  }

  /* Print styles */
  @media print {
    .search-form-container,
    .search-config-container,
    .loading-container,
    .empty-state,
    .search-error {
      display: none;
    }

    .search-page {
      background: white;
      color: black;
    }

    .results-container {
      break-inside: avoid;
    }
  }

  /* Focus management for keyboard users */
  .search-main:focus-within {
    outline: 2px solid transparent;
  }

  .results-container {
    scroll-margin-top: var(--spacing-2xl);
  }

  /* Enhanced touch targets for mobile */
  @media (max-width: 767px) {
    .close-button {
      min-height: 44px;
      min-width: 44px;
    }
  }
</style>