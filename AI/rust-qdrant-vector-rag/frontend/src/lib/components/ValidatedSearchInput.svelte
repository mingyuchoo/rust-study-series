<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { Search, AlertCircle, CheckCircle } from 'lucide-svelte';
  import { RealTimeValidator, InputSanitizer, debounce } from '../utils/validation.js';
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';
  import { env } from '../config/env.js';
  import type { ValidationError } from '../types/state.js';

  // Props
  export let value = '';
  export let placeholder = 'Ask a question about your documents...';
  export let disabled = false;
  export let maxLength = env.MAX_QUERY_LENGTH;
  export let minLength = 3;
  export let debounceMs = 300;
  export let showCharacterCount = true;
  export let showValidationIcon = true;
  export let autoFocus = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    input: { value: string; isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] };
    'validation-change': { isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] };
    submit: { value: string; sanitizedValue: string };
    focus: void;
    blur: void;
  }>();

  // Local state
  let textareaElement: HTMLTextAreaElement;
  let validationState = {
    errors: [] as ValidationError[],
    warnings: [] as ValidationError[],
    characterCount: 0,
    wordCount: 0,
    isValid: false,
    isValidating: false,
    sanitizedQuery: ''
  };
  let hasBeenFocused = false;
  let validationTimeout: NodeJS.Timeout;

  // Generate unique IDs for accessibility
  const inputId = generateId('search-input');
  const helpTextId = generateId('search-help');
  const errorsId = generateId('search-errors');
  const characterCountId = generateId('character-count');

  // Enhanced debounced validation function
  const debouncedValidate = debounce((query: string) => {
    validationState.isValidating = true;
    
    // Clear previous timeout
    if (validationTimeout) {
      clearTimeout(validationTimeout);
    }

    // Simulate async validation delay for better UX
    validationTimeout = setTimeout(() => {
      const result = RealTimeValidator.validateSearchQuery(query);
      validationState = {
        errors: result.errors,
        warnings: result.warnings,
        characterCount: result.characterCount,
        wordCount: result.wordCount,
        isValid: result.isValid,
        isValidating: false,
        sanitizedQuery: result.sanitizedQuery
      };

      dispatch('validation-change', {
        isValid: result.isValid,
        errors: result.errors,
        warnings: result.warnings
      });

      // Announce validation results to screen readers
      if (hasBeenFocused) {
        if (result.errors.length > 0) {
          const errorMessages = result.errors.map(e => e.message).join('. ');
          announceToScreenReader(`Validation errors: ${errorMessages}`, 'assertive');
        } else if (result.warnings.length > 0) {
          const warningMessages = result.warnings.map(w => w.message).join('. ');
          announceToScreenReader(`Validation warnings: ${warningMessages}`, 'polite');
        }
      }
    }, 100);
  }, debounceMs);

  // Reactive statements
  $: {
    // Update character count immediately for visual feedback
    validationState.characterCount = value.length;
    
    // Trigger debounced validation
    if (value !== undefined) {
      debouncedValidate(value);
    }
  }

  $: isOverLimit = validationState.characterCount > maxLength;
  $: isUnderLimit = validationState.characterCount > 0 && validationState.characterCount < minLength;
  $: hasErrors = validationState.errors.length > 0;
  $: hasWarnings = validationState.warnings.length > 0;

  // Handle input changes
  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    let newValue = target.value;

    // Sanitize input for security
    const sanitized = InputSanitizer.sanitizeSearchQuery(newValue);
    if (sanitized !== newValue) {
      newValue = sanitized;
      target.value = sanitized;
      
      // Announce sanitization to screen readers
      announceToScreenReader('Input has been sanitized for security', 'polite');
    }

    value = newValue;

    dispatch('input', {
      value: newValue,
      isValid: validationState.isValid,
      errors: validationState.errors,
      warnings: validationState.warnings
    });
  }

  // Handle keyboard shortcuts
  function handleKeydown(event: KeyboardEvent) {
    // Ctrl/Cmd + Enter to submit
    if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
      event.preventDefault();
      if (validationState.isValid && !disabled) {
        dispatch('submit', { 
          value, 
          sanitizedValue: validationState.sanitizedQuery 
        });
      }
    }

    // Escape to clear
    if (event.key === 'Escape') {
      event.preventDefault();
      value = '';
      textareaElement.blur();
    }
  }

  // Handle focus events
  function handleFocus() {
    hasBeenFocused = true;
    dispatch('focus');
  }

  function handleBlur() {
    dispatch('blur');
  }

  // Auto-resize textarea
  function autoResize() {
    if (textareaElement) {
      textareaElement.style.height = 'auto';
      textareaElement.style.height = textareaElement.scrollHeight + 'px';
    }
  }

  // Lifecycle
  onMount(() => {
    if (autoFocus && textareaElement) {
      textareaElement.focus();
    }
    
    // Initial validation
    if (value) {
      debouncedValidate(value);
    }
  });

  onDestroy(() => {
    if (validationTimeout) {
      clearTimeout(validationTimeout);
    }
  });

  // Watch for value changes to auto-resize
  $: if (textareaElement && value !== undefined) {
    autoResize();
  }
</script>

<div class="validated-search-input">
  <!-- Main input container -->
  <div class="input-container" class:has-errors={hasErrors} class:disabled>
    <div class="input-wrapper">
      <textarea
        bind:this={textareaElement}
        id={inputId}
        bind:value
        on:input={handleInput}
        on:keydown={handleKeydown}
        on:focus={handleFocus}
        on:blur={handleBlur}
        {placeholder}
        {disabled}
        rows="2"
        class="search-textarea"
        class:error={hasErrors}
        class:warning={isUnderLimit || isOverLimit}
        aria-label="Search query input"
        aria-describedby="{helpTextId} {characterCountId} {hasErrors ? errorsId : ''}"
        aria-invalid={hasErrors}
        aria-required="true"
        spellcheck="true"
        autocomplete="off"
        maxlength={maxLength}
      ></textarea>

      <!-- Validation icon -->
      {#if showValidationIcon && hasBeenFocused}
        <div class="validation-icon" aria-hidden="true">
          {#if validationState.isValidating}
            <div class="spinner" title="Validating..."></div>
          {:else if hasErrors}
            <AlertCircle size={20} class="error-icon" />
          {:else if validationState.isValid && value.length > 0}
            <CheckCircle size={20} class="success-icon" />
          {/if}
        </div>
      {/if}

      <!-- Search icon -->
      <div class="search-icon" aria-hidden="true">
        <Search size={20} />
      </div>
    </div>

    <!-- Character count and help text -->
    <div class="input-footer">
      {#if showCharacterCount}
        <span 
          id={characterCountId}
          class="character-count"
          class:error={isOverLimit}
          class:warning={isUnderLimit}
          aria-live="polite"
          role="status"
        >
          <span class="sr-only">Character count:</span>
          {validationState.characterCount} / {maxLength}
          {#if isOverLimit}
            <span class="sr-only">Exceeds maximum length</span>
          {:else if isUnderLimit}
            <span class="sr-only">Below minimum length</span>
          {/if}
        </span>
      {/if}

      <span id={helpTextId} class="help-text">
        Press Ctrl+Enter to search, Escape to clear
        {#if validationState.wordCount > 0}
          • {validationState.wordCount} word{validationState.wordCount !== 1 ? 's' : ''}
        {/if}
      </span>
    </div>
  </div>

  <!-- Validation errors -->
  {#if hasErrors && hasBeenFocused}
    <div 
      id={errorsId}
      class="validation-errors" 
      role="alert" 
      aria-live="assertive"
      aria-atomic="true"
    >
      <h4 class="sr-only">Input validation errors</h4>
      {#each validationState.errors as error, index}
        <div class="error-item" id={generateId(`error-${index}`)}>
          <AlertCircle size={16} aria-hidden="true" />
          <span>{error.message}</span>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Validation warnings -->
  {#if hasWarnings && hasBeenFocused && !hasErrors}
    <div 
      class="validation-warnings" 
      role="status" 
      aria-live="polite"
      aria-atomic="true"
    >
      <h4 class="sr-only">Input validation warnings</h4>
      {#each validationState.warnings as warning, index}
        <div class="warning-item" id={generateId(`warning-${index}`)}>
          <AlertCircle size={16} aria-hidden="true" />
          <span>{warning.message}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .validated-search-input {
    width: 100%;
    max-width: 800px;
  }

  .input-container {
    position: relative;
    border: 2px solid var(--color-surface-300);
    border-radius: 0.75rem;
    background-color: var(--color-surface-50);
    transition: all 0.2s ease;
  }

  .input-container:focus-within {
    border-color: var(--color-primary-500);
    background-color: white;
    box-shadow: 0 0 0 3px var(--color-primary-100);
  }

  .input-container.has-errors {
    border-color: var(--color-error-500);
  }

  .input-container.has-errors:focus-within {
    border-color: var(--color-error-500);
    box-shadow: 0 0 0 3px var(--color-error-100);
  }

  .input-container.disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background-color: var(--color-surface-100);
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: flex-start;
  }

  .search-textarea {
    flex: 1;
    padding: 1rem 3rem 1rem 1rem;
    border: none;
    background: transparent;
    font-size: 1rem;
    line-height: 1.5;
    resize: none;
    outline: none;
    min-height: 3rem;
    max-height: 8rem;
    overflow-y: auto;
    color: var(--color-surface-900);
  }

  .search-textarea::placeholder {
    color: var(--color-surface-500);
  }

  .search-textarea:disabled {
    cursor: not-allowed;
  }

  .validation-icon {
    position: absolute;
    right: 3rem;
    top: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .search-icon {
    position: absolute;
    right: 1rem;
    top: 1rem;
    color: var(--color-surface-500);
    pointer-events: none;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid var(--color-surface-300);
    border-top: 2px solid var(--color-primary-500);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .error-icon {
    color: var(--color-error-500);
  }

  .success-icon {
    color: var(--color-success-500);
  }

  .input-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    border-top: 1px solid var(--color-surface-200);
    background-color: var(--color-surface-25);
  }

  .character-count {
    font-size: 0.875rem;
    color: var(--color-surface-600);
    font-weight: 500;
  }

  .character-count.error {
    color: var(--color-error-600);
  }

  .character-count.warning {
    color: var(--color-warning-600);
  }

  .help-text {
    font-size: 0.75rem;
    color: var(--color-surface-500);
  }

  .validation-errors {
    margin-top: 0.5rem;
    padding: 0.75rem;
    background-color: var(--color-error-50);
    border: 1px solid var(--color-error-200);
    border-radius: 0.5rem;
  }

  .validation-warnings {
    margin-top: 0.5rem;
    padding: 0.75rem;
    background-color: var(--color-warning-50);
    border: 1px solid var(--color-warning-200);
    border-radius: 0.5rem;
  }

  .error-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--color-error-700);
    font-size: 0.875rem;
  }

  .warning-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--color-warning-700);
    font-size: 0.875rem;
  }

  .error-item:not(:last-child),
  .warning-item:not(:last-child) {
    margin-bottom: 0.5rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .input-container {
      border-color: var(--color-surface-600);
      background-color: var(--color-surface-800);
    }

    .input-container:focus-within {
      background-color: var(--color-surface-700);
    }

    .input-container.disabled {
      background-color: var(--color-surface-900);
    }

    .search-textarea {
      color: var(--color-surface-100);
    }

    .search-textarea::placeholder {
      color: var(--color-surface-400);
    }

    .input-footer {
      border-top-color: var(--color-surface-700);
      background-color: var(--color-surface-750);
    }

    .character-count {
      color: var(--color-surface-300);
    }

    .help-text {
      color: var(--color-surface-400);
    }

    .validation-errors {
      background-color: var(--color-surface-800);
      border-color: var(--color-error-700);
    }

    .validation-warnings {
      background-color: var(--color-surface-800);
      border-color: var(--color-warning-700);
    }

    .error-item {
      color: var(--color-error-400);
    }

    .warning-item {
      color: var(--color-warning-400);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .input-footer {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }

    .search-textarea {
      padding: 0.875rem 2.5rem 0.875rem 0.875rem;
    }

    .validation-icon {
      right: 2.5rem;
      top: 0.875rem;
    }

    .search-icon {
      right: 0.875rem;
      top: 0.875rem;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .input-container {
      border-width: 3px;
    }

    .validation-errors {
      border-width: 2px;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .input-container,
    .spinner {
      transition: none;
      animation: none;
    }
  }
</style>