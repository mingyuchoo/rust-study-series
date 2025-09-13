<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { Search, Settings, AlertCircle, CheckCircle } from 'lucide-svelte';
  import { SearchQuerySchema } from '../schemas/validation.js';
  import { errorHandler } from '../services/error-handler.js';
  import { env } from '../config/env.js';
  import { 
    RealTimeValidator, 
    InputSanitizer, 
    DebouncedValidation,
    debounce 
  } from '../utils/validation.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationError } from '../types/state.js';
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
    'validation-change': { isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] };
  }>();

  // Local state
  let validationErrors: ValidationError[] = [];
  let validationWarnings: ValidationError[] = [];
  let isValid = false;
  let characterCount = 0;
  let wordCount = 0;
  let sanitizedQuery = '';
  let isValidating = false;
  
  // Generate unique IDs for accessibility
  const formId = generateId('search-form');
  const textareaId = generateId('search-textarea');
  const characterCountId = generateId('character-count');
  const searchHelpId = generateId('search-help');
  const errorsId = generateId('search-errors');
  const warningsId = generateId('search-warnings');
  const advancedOptionsId = generateId('advanced-options');

  // Debounced validation function
  const debouncedValidation = debounce((queryValue: string) => {
    isValidating = true;
    DebouncedValidation.validateSearchQueryDebounced(queryValue, (result) => {
      validationErrors = result.errors;
      validationWarnings = result.warnings;
      characterCount = result.characterCount;
      wordCount = result.wordCount;
      isValid = result.isValid;
      sanitizedQuery = result.sanitizedQuery;
      isValidating = false;
      
      // Dispatch validation state change
      dispatch('validation-change', {
        isValid,
        errors: validationErrors,
        warnings: validationWarnings
      });
    });
  }, 300);

  // Real-time validation with debouncing
  $: {
    if (query !== undefined) {
      debouncedValidation(query);
    }
  }

  // Cleanup on destroy
  onDestroy(() => {
    debouncedValidation.cancel();
  });

  // Handle form submission
  function handleSubmit() {
    if (!isValid || disabled || isValidating) return;

    try {
      // Use sanitized query for submission
      const finalQuery = sanitizedQuery || query.trim();
      const validated = SearchQuerySchema.parse({ question: finalQuery, config });
      
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

  // Handle query input change with sanitization
  function handleQueryChange(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    const rawValue = target.value;
    
    // Apply basic sanitization for security
    const sanitized = InputSanitizer.sanitizeSearchQuery(rawValue);
    
    // If sanitization changed the input, update the textarea
    if (sanitized !== rawValue && sanitized.length > 0) {
      target.value = sanitized;
      query = sanitized;
      
      // Announce sanitization to screen readers
      announceToScreenReader('Input was automatically cleaned for security', 'polite');
    } else {
      query = rawValue;
    }
    
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
        class:border-red-500={validationErrors.length > 0}
        class:border-orange-400={validationWarnings.length > 0 && validationErrors.length === 0}
        class:border-green-500={isValid && query.length > 0}
        aria-describedby={`${characterCountId} ${searchHelpId} ${validationErrors.length > 0 ? errorsId : ''} ${validationWarnings.length > 0 ? warningsId : ''}`}
        aria-invalid={validationErrors.length > 0}
        aria-required="true"
        spellcheck="true"
        autocomplete="off"
      ></textarea>
      
      <!-- Enhanced character count and validation feedback -->
      <div class="input-feedback">
        <div class="feedback-left">
          <span 
            id={characterCountId}
            class="text-sm flex items-center gap-1"
            class:text-red-600={validationErrors.length > 0}
            class:text-orange-500={validationWarnings.length > 0 && validationErrors.length === 0}
            class:text-green-600={isValid && query.length > 0}
            class:text-gray-500={query.length === 0}
            aria-live="polite"
            aria-atomic="true"
            role="status"
          >
            {#if isValidating}
              <div class="animate-spin rounded-full h-3 w-3 border-b border-current" aria-hidden="true"></div>
            {:else if validationErrors.length > 0}
              <AlertCircle size={14} aria-hidden="true" />
            {:else if isValid && query.length > 0}
              <CheckCircle size={14} aria-hidden="true" />
            {/if}
            
            <span class="sr-only">Character count:</span>
            {characterCount} / {env.MAX_QUERY_LENGTH}
            
            {#if wordCount > 0}
              <span class="text-xs">({wordCount} words)</span>
            {/if}
            
            {#if validationErrors.length > 0}
              <span class="sr-only">. Has validation errors.</span>
            {:else if validationWarnings.length > 0}
              <span class="sr-only">. Has validation warnings.</span>
            {:else if isValid && query.length > 0}
              <span class="sr-only">. Valid input.</span>
            {/if}
          </span>
        </div>
        
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
        disabled={!isValid || disabled || isValidating}
        aria-describedby={searchHelpId}
        aria-label={disabled ? 'Searching documents...' : isValidating ? 'Validating query...' : 'Search documents with current query'}
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
        class="validation-errors bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3" 
        role="alert" 
        aria-live="assertive"
        aria-atomic="true"
      >
        <h4 class="sr-only">Search validation errors</h4>
        <div class="flex items-start gap-2">
          <AlertCircle size={16} class="text-red-600 dark:text-red-400 mt-0.5 flex-shrink-0" aria-hidden="true" />
          <div class="space-y-1">
            {#each validationErrors as error, index}
              <p class="text-sm text-red-700 dark:text-red-300" id={generateId(`search-error-${index}`)}>
                <span class="sr-only">Error:</span>
                {error.message}
              </p>
            {/each}
          </div>
        </div>
      </div>
    {/if}

    <!-- Validation warnings -->
    {#if validationWarnings.length > 0 && validationErrors.length === 0}
      <div 
        id={warningsId}
        class="validation-warnings bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 rounded-lg p-3" 
        role="status" 
        aria-live="polite"
        aria-atomic="true"
      >
        <h4 class="sr-only">Search validation warnings</h4>
        <div class="flex items-start gap-2">
          <AlertCircle size={16} class="text-orange-600 dark:text-orange-400 mt-0.5 flex-shrink-0" aria-hidden="true" />
          <div class="space-y-1">
            {#each validationWarnings as warning, index}
              <p class="text-sm text-orange-700 dark:text-orange-300" id={generateId(`search-warning-${index}`)}>
                <span class="sr-only">Warning:</span>
                {warning.message}
              </p>
            {/each}
          </div>
        </div>
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

  .feedback-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  /* Dark mode support - handled by Tailwind classes */

  /* Responsive design */
  @media (max-width: 768px) {
    .input-feedback {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }

    .feedback-left {
      width: 100%;
      justify-content: space-between;
    }
  }
</style>