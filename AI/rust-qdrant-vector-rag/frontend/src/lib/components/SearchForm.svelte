<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Search, Settings } from 'lucide-svelte';
  import { SearchQuerySchema } from '../schemas/validation.js';
  import { errorHandler } from '../services/error-handler.js';
  import { env } from '../config/env.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationErrorInput } from '../schemas/validation.js';
  import { generateId, announceToScreenReader, KeyboardNavigation } from '../utils/accessibility.js';

  // Props
  export let query = '';
  export let disabled = false;
  export let showAdvanced = false;
  export let config: QueryConfig = {};

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    submit: { query: string; config?: QueryConfig };
    'toggle-advanced': boolean;
    'query-change': string;
  }>();

  // Local state
  let validationErrors: ValidationErrorInput[] = [];
  let isValid = false;
  
  // Generate unique IDs for accessibility
  const formId = generateId('search-form');
  const textareaId = generateId('search-textarea');
  const characterCountId = generateId('character-count');
  const searchHelpId = generateId('search-help');
  const errorsId = generateId('search-errors');
  const advancedOptionsId = generateId('advanced-options');

  // Character count and validation
  $: characterCount = query.length;
  $: isOverLimit = characterCount > env.MAX_QUERY_LENGTH;
  $: isUnderLimit = characterCount < 3;
  $: isValid = !isOverLimit && !isUnderLimit && query.trim().length > 0;

  // Validate query in real-time
  $: {
    validationErrors = [];
    if (query.length > 0) {
      try {
        SearchQuerySchema.parse({ question: query, config });
      } catch (error) {
        if (error instanceof Error && 'issues' in error) {
          const zodError = error as any;
          validationErrors = zodError.issues.map((issue: any) => ({
            field: issue.path.join('.'),
            message: issue.message
          }));
        }
      }

      // Additional validation using error handler for consistent error types
      if (query.length > env.MAX_QUERY_LENGTH) {
        const searchError = errorHandler.createSearchError(
          'Search query is too long',
          query,
          'query_too_long'
        );
        validationErrors.push({
          field: 'question',
          message: searchError.message
        });
      } else if (query.trim().length > 0 && query.trim().length < 3) {
        const searchError = errorHandler.createSearchError(
          'Please enter a longer search query',
          query,
          'query_too_short'
        );
        validationErrors.push({
          field: 'question',
          message: searchError.message
        });
      }
    }
  }

  // Handle form submission
  function handleSubmit() {
    if (!isValid || disabled) return;

    try {
      const validated = SearchQuerySchema.parse({ question: query.trim(), config });
      dispatch('submit', { 
        query: validated.question, 
        config: validated.config 
      });
      
      // Announce search initiation to screen readers
      announceToScreenReader(`Searching for: ${validated.question}`, 'polite');
    } catch (error) {
      console.error('Validation error on submit:', error);
      announceToScreenReader('Search validation failed. Please check your input.', 'assertive');
    }
  }

  // Handle query input change
  function handleQueryChange(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    query = target.value;
    dispatch('query-change', query);
  }

  // Handle Enter key (Ctrl+Enter to submit)
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      handleSubmit();
    }
  }

  // Handle advanced options toggle with keyboard
  function handleAdvancedToggleKeydown(event: KeyboardEvent) {
    if (KeyboardNavigation.isActivationKey(event.key)) {
      event.preventDefault();
      toggleAdvanced();
    }
  }

  // Toggle advanced options
  function toggleAdvanced() {
    showAdvanced = !showAdvanced;
    dispatch('toggle-advanced', showAdvanced);
    
    // Announce state change to screen readers
    announceToScreenReader(
      `Advanced search options ${showAdvanced ? 'expanded' : 'collapsed'}`, 
      'polite'
    );
  }
</script>

<form 
  id={formId}
  on:submit|preventDefault={handleSubmit} 
  class="search-form"
  role="search"
  aria-label="Document search form"
>
  <div class="space-y-4">
    <!-- Main search input -->
    <div class="search-input-container">
      <label for={textareaId} class="sr-only">
        Search query - Ask a question about your uploaded documents
      </label>
      <textarea
        id={textareaId}
        bind:value={query}
        on:input={handleQueryChange}
        on:keydown={handleKeydown}
        placeholder="Ask a question about your documents..."
        {disabled}
        rows="3"
        class="w-full px-4 py-3 border border-gray-300 rounded-lg resize-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-800 dark:border-gray-600 dark:text-white dark:placeholder-gray-400 min-touch-target focus-visible"
        class:border-red-500={validationErrors.find(e => e.field === 'question') || isOverLimit}
        class:border-orange-400={isUnderLimit && query.length > 0}
        aria-describedby={`${characterCountId} ${searchHelpId} ${validationErrors.length > 0 ? errorsId : ''}`}
        aria-invalid={validationErrors.length > 0 || isOverLimit}
        aria-required="true"
        spellcheck="true"
        autocomplete="off"
      ></textarea>
      
      <!-- Character count and validation feedback -->
      <div class="input-feedback">
        <span 
          id={characterCountId}
          class="text-sm"
          class:text-red-600={isOverLimit}
          class:text-orange-500={isUnderLimit && query.length > 0}
          class:text-gray-500={!isOverLimit && !isUnderLimit}
          aria-live="polite"
          aria-atomic="true"
          role="status"
        >
          <span class="sr-only">Character count:</span>
          {characterCount} of 500 characters used
          {#if isOverLimit}
            <span class="sr-only">. Exceeds maximum length.</span>
          {:else if isUnderLimit && query.length > 0}
            <span class="sr-only">. Minimum 3 characters required.</span>
            (minimum 3 characters)
          {/if}
        </span>
        
        <span id={searchHelpId} class="text-xs text-gray-500">
          <span class="sr-only">Keyboard shortcut:</span>
          Press Ctrl+Enter to search
        </span>
      </div>
    </div>

    <!-- Action buttons -->
    <div class="flex justify-between items-center gap-4">
      <button
        type="button"
        on:click={toggleAdvanced}
        on:keydown={handleAdvancedToggleKeydown}
        class="inline-flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700 min-touch-target focus-visible"
        aria-expanded={showAdvanced}
        aria-controls={advancedOptionsId}
        aria-label={`${showAdvanced ? 'Hide' : 'Show'} advanced search options`}
        {disabled}
      >
        <Settings size={16} aria-hidden="true" />
        <span>{showAdvanced ? 'Hide' : 'Show'} Advanced Options</span>
      </button>

      <button
        type="submit"
        class="inline-flex items-center gap-2 px-6 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed min-touch-target focus-visible"
        disabled={!isValid || disabled}
        aria-describedby={searchHelpId}
        aria-label={disabled ? 'Searching documents...' : 'Search documents with current query'}
      >
        {#if disabled}
          <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white" aria-hidden="true"></div>
          <span class="sr-only">Searching...</span>
        {:else}
          <Search size={16} aria-hidden="true" />
        {/if}
        <span>Search Documents</span>
      </button>
    </div>

    <!-- Validation errors -->
    {#if validationErrors.length > 0}
      <div 
        id={errorsId}
        class="validation-errors" 
        role="alert" 
        aria-live="assertive"
        aria-atomic="true"
      >
        <h4 class="sr-only">Search validation errors</h4>
        {#each validationErrors as error, index}
          <p class="text-sm text-red-600 dark:text-red-400" id={generateId(`search-error-${index}`)}>
            <span class="sr-only">Error:</span>
            {error.message}
          </p>
        {/each}
      </div>
    {/if}
  </div>
</form>

<style>
  .search-form {
    width: 100%;
    max-width: 800px;
  }

  .search-input-container {
    position: relative;
  }

  .input-feedback {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 0.5rem;
    padding: 0 0.25rem;
  }

  .validation-errors {
    padding: 0.75rem;
    background-color: var(--color-surface-50);
    border: 1px solid #fca5a5;
    border-radius: 0.5rem;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .validation-errors {
      background-color: var(--color-surface-800);
      border-color: #dc2626;
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .input-feedback {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>