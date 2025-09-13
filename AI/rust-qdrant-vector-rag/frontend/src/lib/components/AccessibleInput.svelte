<script lang="ts">
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';
  import { createEventDispatcher } from 'svelte';

  // Props
  export let type: 'text' | 'email' | 'password' | 'search' | 'tel' | 'url' | 'number' = 'text';
  export let value = '';
  export let placeholder = '';
  export let label: string;
  export let description = '';
  export let error = '';
  export let required = false;
  export let disabled = false;
  export let readonly = false;
  export let autocomplete = '';
  export let maxlength: number | undefined = undefined;
  export let minlength: number | undefined = undefined;
  export let pattern: string | undefined = undefined;
  export let id: string | undefined = undefined;
  export let name: string | undefined = undefined;
  export let className = '';
  export let showCharacterCount = false;
  export let validateOnBlur = true;
  export let validateOnInput = false;

  // Generate unique IDs for accessibility
  const inputId = id || generateId('input');
  const labelId = generateId('label');
  const descriptionId = generateId('description');
  const errorId = generateId('error');

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    input: { value: string; validity: ValidityState };
    change: { value: string; validity: ValidityState };
    focus: { value: string };
    blur: { value: string; validity: ValidityState };
    keydown: KeyboardEvent;
    keyup: KeyboardEvent;
  }>();

  // Internal state
  let inputElement: HTMLInputElement;
  let hasBeenFocused = false;
  let internalError = '';

  // Reactive validation
  $: isValid = !error && !internalError;
  $: characterCount = value.length;
  $: remainingCharacters = maxlength ? maxlength - characterCount : null;
  $: showError = (error || internalError) && hasBeenFocused;

  // Accessibility attributes
  $: ariaDescribedBy = [
    description ? descriptionId : null,
    showError ? errorId : null,
    showCharacterCount && maxlength ? `${inputId}-count` : null
  ].filter(Boolean).join(' ');

  // Handle input events
  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;
    
    if (validateOnInput) {
      validateInput();
    }
    
    dispatch('input', { value, validity: target.validity });
  }

  function handleChange(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;
    validateInput();
    dispatch('change', { value, validity: target.validity });
  }

  function handleFocus(event: FocusEvent) {
    hasBeenFocused = true;
    dispatch('focus', { value });
  }

  function handleBlur(event: FocusEvent) {
    if (validateOnBlur) {
      validateInput();
    }
    
    const target = event.target as HTMLInputElement;
    dispatch('blur', { value, validity: target.validity });
  }

  function handleKeydown(event: KeyboardEvent) {
    dispatch('keydown', event);
  }

  function handleKeyup(event: KeyboardEvent) {
    dispatch('keyup', event);
  }

  // Validation function
  function validateInput() {
    if (!inputElement) return;
    
    internalError = '';
    
    // Custom validation logic
    if (required && !value.trim()) {
      internalError = 'This field is required';
    } else if (minlength && value.length < minlength) {
      internalError = `Minimum length is ${minlength} characters`;
    } else if (maxlength && value.length > maxlength) {
      internalError = `Maximum length is ${maxlength} characters`;
    } else if (pattern && !new RegExp(pattern).test(value)) {
      internalError = 'Please enter a valid value';
    }
    
    // Use browser validation if no custom errors
    if (!internalError && !inputElement.validity.valid) {
      internalError = inputElement.validationMessage;
    }
  }

  // Public methods
  export function focus() {
    inputElement?.focus();
  }

  export function blur() {
    inputElement?.blur();
  }

  export function select() {
    inputElement?.select();
  }

  export function validate() {
    validateInput();
    return isValid;
  }

  // Announce errors to screen readers
  $: if (showError && (error || internalError)) {
    announceToScreenReader(error || internalError, 'assertive');
  }
</script>

<div class="accessible-input-wrapper {className}">
  <!-- Label -->
  <label 
    id={labelId}
    for={inputId} 
    class="input-label"
    class:required
  >
    {label}
    {#if required}
      <span class="required-indicator" aria-label="required">*</span>
    {/if}
  </label>

  <!-- Description -->
  {#if description}
    <div 
      id={descriptionId} 
      class="input-description"
    >
      {description}
    </div>
  {/if}

  <!-- Input wrapper for styling -->
  <div class="input-container" class:error={showError} class:disabled>
    <input
      bind:this={inputElement}
      {id}
      {name}
      {type}
      {placeholder}
      {required}
      {disabled}
      {readonly}
      {autocomplete}
      {maxlength}
      {minlength}
      {pattern}
      bind:value
      class="input-field focus-visible min-touch-target"
      class:error={showError}
      aria-labelledby={labelId}
      aria-describedby={ariaDescribedBy || undefined}
      aria-invalid={showError}
      aria-required={required}
      on:input={handleInput}
      on:change={handleChange}
      on:focus={handleFocus}
      on:blur={handleBlur}
      on:keydown={handleKeydown}
      on:keyup={handleKeyup}
    />
  </div>

  <!-- Character count -->
  {#if showCharacterCount && maxlength}
    <div 
      id="{inputId}-count" 
      class="character-count"
      class:warning={remainingCharacters !== null && remainingCharacters < 20}
      class:error={remainingCharacters !== null && remainingCharacters < 0}
      aria-live="polite"
    >
      {characterCount}/{maxlength} characters
      {#if remainingCharacters !== null && remainingCharacters < 20}
        <span class="sr-only">
          {remainingCharacters < 0 ? 'Character limit exceeded' : `${remainingCharacters} characters remaining`}
        </span>
      {/if}
    </div>
  {/if}

  <!-- Error message -->
  {#if showError}
    <div 
      id={errorId} 
      class="error-message"
      role="alert"
      aria-live="assertive"
    >
      {error || internalError}
    </div>
  {/if}
</div>

<style>
  .accessible-input-wrapper {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    width: 100%;
  }

  .input-label {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-surface-700);
    line-height: var(--line-height-tight);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .input-label.required {
    /* Additional styling for required fields */
  }

  .required-indicator {
    color: var(--color-error-500);
    font-weight: 700;
    font-size: var(--font-size-base);
  }

  .input-description {
    font-size: var(--font-size-sm);
    color: var(--color-surface-600);
    line-height: var(--line-height-normal);
  }

  .input-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input-field {
    width: 100%;
    padding: var(--spacing-sm) var(--spacing-md);
    border: 2px solid var(--color-surface-300);
    border-radius: 0.5rem;
    font-size: var(--font-size-base);
    line-height: var(--line-height-normal);
    background-color: var(--color-surface-50);
    color: var(--color-surface-900);
    transition: all var(--duration-fast) ease;
    box-sizing: border-box;
  }

  .input-field:hover:not(:disabled) {
    border-color: var(--color-surface-400);
  }

  .input-field:focus {
    outline: none;
    border-color: var(--color-primary-500);
    box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
    background-color: var(--color-surface-50);
  }

  .input-field:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background-color: var(--color-surface-100);
    border-color: var(--color-surface-200);
  }

  .input-field:readonly {
    background-color: var(--color-surface-100);
    cursor: default;
  }

  .input-field::placeholder {
    color: var(--color-surface-500);
    opacity: 1;
  }

  .input-field.error {
    border-color: var(--color-error-500);
    background-color: var(--color-error-50);
  }

  .input-field.error:focus {
    border-color: var(--color-error-500);
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
  }

  .character-count {
    font-size: var(--font-size-xs);
    color: var(--color-surface-600);
    text-align: right;
    margin-top: var(--spacing-xs);
  }

  .character-count.warning {
    color: var(--color-warning-600);
    font-weight: 600;
  }

  .character-count.error {
    color: var(--color-error-600);
    font-weight: 600;
  }

  .error-message {
    font-size: var(--font-size-sm);
    color: var(--color-error-600);
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .error-message::before {
    content: '⚠';
    font-size: var(--font-size-base);
    flex-shrink: 0;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .input-label {
      color: var(--color-surface-300);
    }

    .input-description {
      color: var(--color-surface-400);
    }

    .input-field {
      background-color: var(--color-surface-800);
      color: var(--color-surface-100);
      border-color: var(--color-surface-600);
    }

    .input-field:hover:not(:disabled) {
      border-color: var(--color-surface-500);
    }

    .input-field:focus {
      border-color: var(--color-primary-400);
      box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.2);
      background-color: var(--color-surface-800);
    }

    .input-field:disabled {
      background-color: var(--color-surface-900);
      border-color: var(--color-surface-700);
    }

    .input-field:readonly {
      background-color: var(--color-surface-900);
    }

    .input-field::placeholder {
      color: var(--color-surface-400);
    }

    .input-field.error {
      border-color: var(--color-error-400);
      background-color: var(--color-error-900);
    }

    .input-field.error:focus {
      border-color: var(--color-error-400);
      box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.2);
    }

    .character-count {
      color: var(--color-surface-400);
    }

    .character-count.warning {
      color: var(--color-warning-400);
    }

    .character-count.error {
      color: var(--color-error-400);
    }

    .error-message {
      color: var(--color-error-400);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .input-field {
      border-width: 3px;
    }

    .input-field:focus {
      border-width: 4px;
    }

    .input-field.error {
      border-width: 3px;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .input-field {
      transition: none;
    }
  }

  /* Mobile responsive adjustments */
  @media (max-width: 767px) {
    .input-field {
      font-size: 16px; /* Prevent zoom on iOS */
      padding: var(--spacing-md);
    }
  }
</style>