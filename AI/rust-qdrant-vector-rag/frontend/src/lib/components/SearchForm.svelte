<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Search, Settings } from 'lucide-svelte';
  import { SearchQuerySchema } from '../schemas/validation.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationErrorInput } from '../schemas/validation.js';

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

  // Character count and validation
  $: characterCount = query.length;
  $: isOverLimit = characterCount > 500;
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
    } catch (error) {
      console.error('Validation error on submit:', error);
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

  // Toggle advanced options
  function toggleAdvanced() {
    showAdvanced = !showAdvanced;
    dispatch('toggle-advanced', showAdvanced);
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="search-form">
  <div class="space-y-4">
    <!-- Main search input -->
    <div class="search-input-container">
      <textarea
        bind:value={query}
        on:input={handleQueryChange}
        on:keydown={handleKeydown}
        placeholder="Ask a question about your documents..."
        {disabled}
        rows="3"
        class="w-full px-4 py-3 border border-gray-300 rounded-lg resize-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-800 dark:border-gray-600 dark:text-white dark:placeholder-gray-400"
        class:border-red-500={validationErrors.find(e => e.field === 'question') || isOverLimit}
        class:border-orange-400={isUnderLimit && query.length > 0}
        aria-label="Search query input"
        aria-describedby="character-count search-help"
      ></textarea>
      
      <!-- Character count and validation feedback -->
      <div class="input-feedback">
        <span 
          id="character-count"
          class="text-sm"
          class:text-red-600={isOverLimit}
          class:text-orange-500={isUnderLimit && query.length > 0}
          class:text-gray-500={!isOverLimit && !isUnderLimit}
          aria-live="polite"
        >
          {characterCount}/500 characters
          {#if isUnderLimit && query.length > 0}
            (minimum 3 characters)
          {/if}
        </span>
        
        <span id="search-help" class="text-xs text-gray-500">
          Press Ctrl+Enter to search
        </span>
      </div>
    </div>

    <!-- Action buttons -->
    <div class="flex justify-between items-center">
      <button
        type="button"
        on:click={toggleAdvanced}
        class="inline-flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700"
        aria-expanded={showAdvanced}
        aria-controls="advanced-options"
        {disabled}
      >
        <Settings size={16} />
        {showAdvanced ? 'Hide' : 'Show'} Advanced Options
      </button>

      <button
        type="submit"
        class="inline-flex items-center gap-2 px-6 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={!isValid || disabled}
        aria-describedby="search-button-help"
      >
        {#if disabled}
          <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
        {:else}
          <Search size={16} />
        {/if}
        Search Documents
      </button>
    </div>

    <!-- Validation errors -->
    {#if validationErrors.length > 0}
      <div class="validation-errors" role="alert" aria-live="polite">
        {#each validationErrors as error}
          <p class="text-sm text-red-600 dark:text-red-400">
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