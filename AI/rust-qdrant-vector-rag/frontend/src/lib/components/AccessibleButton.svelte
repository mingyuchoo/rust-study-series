<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { generateId, KeyboardNavigation } from '../utils/accessibility.js';

  // Props
  export let variant: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let disabled = false;
  export let loading = false;
  export let fullWidth = false;
  export let ariaLabel: string | undefined = undefined;
  export let ariaDescribedby: string | undefined = undefined;
  export let ariaExpanded: boolean | undefined = undefined;
  export let ariaHaspopup: boolean | 'true' | 'false' | 'menu' | 'listbox' | 'tree' | 'grid' | 'dialog' = undefined;
  export let ariaPressed: boolean | undefined = undefined;
  export let role: string | undefined = undefined;
  export let id: string | undefined = undefined;
  export let className = '';
  export let href: string | undefined = undefined;
  export let target: string | undefined = undefined;
  export let download: string | boolean | undefined = undefined;

  // Generate unique ID if not provided
  const buttonId = id || generateId('button');

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    keydown: KeyboardEvent;
    keyup: KeyboardEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  // Internal state
  let buttonElement: HTMLButtonElement | HTMLAnchorElement;

  // Computed classes
  $: buttonClasses = [
    'accessible-button',
    `variant-${variant}`,
    `size-${size}`,
    className,
    fullWidth ? 'full-width' : '',
    loading ? 'loading' : '',
    disabled ? 'disabled' : ''
  ].filter(Boolean).join(' ');

  // Event handlers
  function handleClick(event: MouseEvent) {
    if (disabled || loading) {
      event.preventDefault();
      return;
    }
    dispatch('click', event);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled || loading) return;
    
    // Handle activation keys for button-like elements
    if (KeyboardNavigation.isActivationKey(event.key)) {
      if (!href) {
        event.preventDefault();
        handleClick(event as any);
      }
    }
    
    dispatch('keydown', event);
  }

  function handleKeyup(event: KeyboardEvent) {
    dispatch('keyup', event);
  }

  function handleFocus(event: FocusEvent) {
    dispatch('focus', event);
  }

  function handleBlur(event: FocusEvent) {
    dispatch('blur', event);
  }

  // Public methods
  export function focus() {
    buttonElement?.focus();
  }

  export function blur() {
    buttonElement?.blur();
  }

  export function click() {
    if (!disabled && !loading) {
      buttonElement?.click();
    }
  }
</script>

{#if href}
  <!-- Render as link -->
  <a
    bind:this={buttonElement}
    id={buttonId}
    {href}
    {target}
    {download}
    class={buttonClasses}
    class:disabled
    aria-label={ariaLabel}
    aria-describedby={ariaDescribedby}
    aria-expanded={ariaExpanded}
    aria-haspopup={ariaHaspopup}
    aria-pressed={ariaPressed}
    {role}
    tabindex={disabled ? -1 : 0}
    on:click={handleClick}
    on:keydown={handleKeydown}
    on:keyup={handleKeyup}
    on:focus={handleFocus}
    on:blur={handleBlur}
  >
    {#if loading}
      <span class="loading-spinner" aria-hidden="true"></span>
      <span class="sr-only">Loading...</span>
    {/if}
    
    <span class="button-content" class:loading>
      <slot />
    </span>
  </a>
{:else}
  <!-- Render as button -->
  <button
    bind:this={buttonElement}
    id={buttonId}
    {type}
    {disabled}
    class={buttonClasses}
    aria-label={ariaLabel}
    aria-describedby={ariaDescribedby}
    aria-expanded={ariaExpanded}
    aria-haspopup={ariaHaspopup}
    aria-pressed={ariaPressed}
    {role}
    on:click={handleClick}
    on:keydown={handleKeydown}
    on:keyup={handleKeyup}
    on:focus={handleFocus}
    on:blur={handleBlur}
  >
    {#if loading}
      <span class="loading-spinner" aria-hidden="true"></span>
      <span class="sr-only">Loading...</span>
    {/if}
    
    <span class="button-content" class:loading>
      <slot />
    </span>
  </button>
{/if}

<style>
  .accessible-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-xs);
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-family: inherit;
    font-weight: 600;
    line-height: var(--line-height-tight);
    cursor: pointer;
    transition: all var(--duration-fast) ease;
    text-decoration: none;
    position: relative;
    box-sizing: border-box;
    white-space: nowrap;
    user-select: none;
    vertical-align: middle;
  }

  /* Size variants */
  .accessible-button.size-sm {
    padding: var(--spacing-xs) var(--spacing-sm);
    font-size: var(--font-size-sm);
    min-height: 36px;
    min-width: 36px;
  }

  .accessible-button.size-md {
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-base);
    min-height: var(--min-touch-target);
    min-width: var(--min-touch-target);
  }

  .accessible-button.size-lg {
    padding: var(--spacing-md) var(--spacing-lg);
    font-size: var(--font-size-lg);
    min-height: 52px;
    min-width: 52px;
  }

  /* Color variants */
  .accessible-button.variant-primary {
    background-color: var(--color-primary-500);
    color: white;
    border-color: var(--color-primary-500);
  }

  .accessible-button.variant-primary:hover:not(:disabled) {
    background-color: var(--color-primary-600);
    border-color: var(--color-primary-600);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .accessible-button.variant-primary:active:not(:disabled) {
    background-color: var(--color-primary-700);
    border-color: var(--color-primary-700);
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .accessible-button.variant-secondary {
    background-color: var(--color-secondary-500);
    color: white;
    border-color: var(--color-secondary-500);
  }

  .accessible-button.variant-secondary:hover:not(:disabled) {
    background-color: var(--color-secondary-600);
    border-color: var(--color-secondary-600);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .accessible-button.variant-secondary:active:not(:disabled) {
    background-color: var(--color-secondary-700);
    border-color: var(--color-secondary-700);
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .accessible-button.variant-outline {
    background-color: transparent;
    color: var(--color-primary-600);
    border-color: var(--color-primary-500);
  }

  .accessible-button.variant-outline:hover:not(:disabled) {
    background-color: var(--color-primary-50);
    color: var(--color-primary-700);
    border-color: var(--color-primary-600);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .accessible-button.variant-outline:active:not(:disabled) {
    background-color: var(--color-primary-100);
    color: var(--color-primary-800);
    border-color: var(--color-primary-700);
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .accessible-button.variant-ghost {
    background-color: transparent;
    color: var(--color-surface-700);
    border-color: transparent;
  }

  .accessible-button.variant-ghost:hover:not(:disabled) {
    background-color: var(--color-surface-100);
    color: var(--color-surface-900);
    border-color: var(--color-surface-200);
    transform: translateY(-1px);
    box-shadow: var(--shadow-sm);
  }

  .accessible-button.variant-ghost:active:not(:disabled) {
    background-color: var(--color-surface-200);
    color: var(--color-surface-900);
    border-color: var(--color-surface-300);
    transform: translateY(0);
  }

  .accessible-button.variant-danger {
    background-color: var(--color-error-500);
    color: white;
    border-color: var(--color-error-500);
  }

  .accessible-button.variant-danger:hover:not(:disabled) {
    background-color: var(--color-error-600);
    border-color: var(--color-error-600);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .accessible-button.variant-danger:active:not(:disabled) {
    background-color: var(--color-error-700);
    border-color: var(--color-error-700);
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  /* Full width */
  .accessible-button.full-width {
    width: 100%;
  }

  /* Disabled state */
  .accessible-button:disabled,
  .accessible-button.disabled {
    opacity: 0.6;
    cursor: not-allowed;
    pointer-events: none;
    transform: none !important;
    box-shadow: none !important;
  }

  /* Loading state */
  .accessible-button.loading {
    cursor: wait;
    pointer-events: none;
  }

  .button-content.loading {
    opacity: 0.7;
  }

  /* Loading spinner */
  .loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin var(--duration-normal) linear infinite;
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
  }

  @keyframes spin {
    from { transform: translate(-50%, -50%) rotate(0deg); }
    to { transform: translate(-50%, -50%) rotate(360deg); }
  }

  /* Focus styles */
  .accessible-button:focus-visible {
    outline: 3px solid var(--color-primary-500);
    outline-offset: 2px;
    border-radius: 0.5rem;
  }

  .accessible-button.variant-danger:focus-visible {
    outline-color: var(--color-error-500);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .accessible-button.variant-primary {
      background-color: var(--color-primary-600);
      border-color: var(--color-primary-600);
    }

    .accessible-button.variant-primary:hover:not(:disabled) {
      background-color: var(--color-primary-500);
      border-color: var(--color-primary-500);
    }

    .accessible-button.variant-primary:active:not(:disabled) {
      background-color: var(--color-primary-400);
      border-color: var(--color-primary-400);
    }

    .accessible-button.variant-outline {
      color: var(--color-primary-400);
      border-color: var(--color-primary-400);
    }

    .accessible-button.variant-outline:hover:not(:disabled) {
      background-color: var(--color-primary-900);
      color: var(--color-primary-300);
      border-color: var(--color-primary-300);
    }

    .accessible-button.variant-outline:active:not(:disabled) {
      background-color: var(--color-primary-800);
      color: var(--color-primary-200);
      border-color: var(--color-primary-200);
    }

    .accessible-button.variant-ghost {
      color: var(--color-surface-300);
    }

    .accessible-button.variant-ghost:hover:not(:disabled) {
      background-color: var(--color-surface-800);
      color: var(--color-surface-100);
      border-color: var(--color-surface-700);
    }

    .accessible-button.variant-ghost:active:not(:disabled) {
      background-color: var(--color-surface-700);
      color: var(--color-surface-100);
      border-color: var(--color-surface-600);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .accessible-button {
      border-width: 3px;
    }

    .accessible-button:focus-visible {
      outline-width: 4px;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .accessible-button {
      transition: none;
    }

    .accessible-button:hover:not(:disabled) {
      transform: none;
    }

    .accessible-button:active:not(:disabled) {
      transform: none;
    }

    .loading-spinner {
      animation: none;
    }
  }

  /* Mobile responsive adjustments */
  @media (max-width: 767px) {
    .accessible-button.size-sm {
      min-height: 40px;
      min-width: 40px;
      padding: var(--spacing-sm) var(--spacing-md);
    }

    .accessible-button.size-md {
      min-height: var(--min-touch-target);
      min-width: var(--min-touch-target);
      padding: var(--spacing-md) var(--spacing-lg);
    }

    .accessible-button.size-lg {
      min-height: 56px;
      min-width: 56px;
      padding: var(--spacing-lg) var(--spacing-xl);
    }
  }

  /* Print styles */
  @media print {
    .accessible-button {
      border: 2px solid black !important;
      background: white !important;
      color: black !important;
      box-shadow: none !important;
      transform: none !important;
    }

    .loading-spinner {
      display: none;
    }
  }
</style>