<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Loader } from 'lucide-svelte';

  // Props
  export let variant: 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'ghost' = 'primary';
  export let size: 'xs' | 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let disabled: boolean = false;
  export let loading: boolean = false;
  export let loadingText: string = 'Loading...';
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let fullWidth: boolean = false;
  export let icon: any = null;
  export let iconPosition: 'left' | 'right' = 'left';
  export let href: string | null = null;
  export let target: string | null = null;
  export let ariaLabel: string | null = null;
  export let ariaDescribedBy: string | null = null;
  export let rippleEffect: boolean = true;
  export let hoverEffect: boolean = true;
  export let focusEffect: boolean = true;

  // Internal state
  let buttonElement: HTMLElement;
  let isPressed = false;
  let ripples: Array<{ id: string; x: number; y: number; size: number }> = [];

  const dispatch = createEventDispatcher();

  // Reactive computations
  $: isDisabled = disabled || loading;
  $: showIcon = icon && !loading;
  $: showLoadingIcon = loading;
  $: buttonText = loading ? loadingText : $$slots.default;

  // Size mappings
  $: sizeClasses = {
    xs: 'px-2 py-1 text-xs min-h-[32px]',
    sm: 'px-3 py-1.5 text-sm min-h-[36px]',
    md: 'px-4 py-2 text-sm min-h-[40px]',
    lg: 'px-6 py-3 text-base min-h-[44px]',
    xl: 'px-8 py-4 text-lg min-h-[48px]'
  };

  $: iconSizes = {
    xs: 14,
    sm: 16,
    md: 18,
    lg: 20,
    xl: 22
  };

  // Handle click with ripple effect
  function handleClick(event: MouseEvent) {
    if (isDisabled) {
      event.preventDefault();
      return;
    }

    // Create ripple effect
    if (rippleEffect && buttonElement) {
      const rect = buttonElement.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;
      const size = Math.max(rect.width, rect.height) * 2;
      
      const ripple = {
        id: `ripple-${Date.now()}-${Math.random()}`,
        x: x - size / 2,
        y: y - size / 2,
        size
      };

      ripples = [...ripples, ripple];

      // Remove ripple after animation
      setTimeout(() => {
        ripples = ripples.filter(r => r.id !== ripple.id);
      }, 600);
    }

    dispatch('click', event);
  }

  // Handle mouse events for press effect
  function handleMouseDown() {
    if (!isDisabled) {
      isPressed = true;
    }
  }

  function handleMouseUp() {
    isPressed = false;
  }

  function handleMouseLeave() {
    isPressed = false;
  }

  // Handle keyboard events
  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      if (!isDisabled) {
        isPressed = true;
      }
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      isPressed = false;
      if (!isDisabled) {
        dispatch('click', event);
      }
    }
  }
</script>

{#if href && !isDisabled}
  <a
    {href}
    {target}
    class="interactive-button {variant} {sizeClasses[size]} {fullWidth ? 'w-full' : ''}"
    class:disabled={isDisabled}
    class:loading
    class:pressed={isPressed}
    class:hover-effect={hoverEffect}
    class:focus-effect={focusEffect}
    aria-label={ariaLabel}
    aria-describedby={ariaDescribedBy}
    role="button"
    tabindex={isDisabled ? -1 : 0}
    bind:this={buttonElement}
    on:click={handleClick}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mouseleave={handleMouseLeave}
    on:keydown={handleKeyDown}
    on:keyup={handleKeyUp}
  >
    <span class="button-content">
      {#if showLoadingIcon}
        <Loader size={iconSizes[size]} class="loading-icon" />
      {:else if showIcon && iconPosition === 'left'}
        <svelte:component this={icon} size={iconSizes[size]} class="button-icon" />
      {/if}
      
      <span class="button-text">
        <slot>{buttonText}</slot>
      </span>
      
      {#if showIcon && iconPosition === 'right'}
        <svelte:component this={icon} size={iconSizes[size]} class="button-icon" />
      {/if}
    </span>

    <!-- Ripple effects -->
    {#if rippleEffect}
      <span class="ripple-container">
        {#each ripples as ripple (ripple.id)}
          <span
            class="ripple"
            style="left: {ripple.x}px; top: {ripple.y}px; width: {ripple.size}px; height: {ripple.size}px;"
          ></span>
        {/each}
      </span>
    {/if}
  </a>
{:else}
  <button
    {type}
    class="interactive-button {variant} {sizeClasses[size]} {fullWidth ? 'w-full' : ''}"
    class:disabled={isDisabled}
    class:loading
    class:pressed={isPressed}
    class:hover-effect={hoverEffect}
    class:focus-effect={focusEffect}
    disabled={isDisabled}
    aria-label={ariaLabel}
    aria-describedby={ariaDescribedBy}
    bind:this={buttonElement}
    on:click={handleClick}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mouseleave={handleMouseLeave}
    on:keydown={handleKeyDown}
    on:keyup={handleKeyUp}
  >
    <span class="button-content">
      {#if showLoadingIcon}
        <Loader size={iconSizes[size]} class="loading-icon" />
      {:else if showIcon && iconPosition === 'left'}
        <svelte:component this={icon} size={iconSizes[size]} class="button-icon" />
      {/if}
      
      <span class="button-text">
        <slot>{buttonText}</slot>
      </span>
      
      {#if showIcon && iconPosition === 'right'}
        <svelte:component this={icon} size={iconSizes[size]} class="button-icon" />
      {/if}
    </span>

    <!-- Ripple effects -->
    {#if rippleEffect}
      <span class="ripple-container">
        {#each ripples as ripple (ripple.id)}
          <span
            class="ripple"
            style="left: {ripple.x}px; top: {ripple.y}px; width: {ripple.size}px; height: {ripple.size}px;"
          ></span>
        {/each}
      </span>
    {/if}
  </button>
{/if}

<style>
  .interactive-button {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.5rem;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    transition: all var(--duration-fast) cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
    border: 2px solid transparent;
    outline: none;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
  }

  .button-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-xs);
    position: relative;
    z-index: 1;
  }

  .button-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .button-icon {
    flex-shrink: 0;
  }

  .loading-icon {
    animation: spin 1s linear infinite;
    flex-shrink: 0;
  }

  /* Ripple effect */
  .ripple-container {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;
    border-radius: inherit;
    pointer-events: none;
  }

  .ripple {
    position: absolute;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.3);
    transform: scale(0);
    animation: ripple-animation 0.6s ease-out;
    pointer-events: none;
  }

  @keyframes ripple-animation {
    to {
      transform: scale(1);
      opacity: 0;
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Variant styles */
  .primary {
    background-color: var(--color-primary-600);
    color: white;
    border-color: var(--color-primary-600);
  }

  .primary.hover-effect:hover:not(.disabled) {
    background-color: var(--color-primary-700);
    border-color: var(--color-primary-700);
    transform: translateY(-1px);
    box-shadow: var(--shadow-lg);
  }

  .primary.pressed {
    background-color: var(--color-primary-800);
    transform: translateY(0);
    box-shadow: var(--shadow-md);
  }

  .secondary {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .secondary.hover-effect:hover:not(.disabled) {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .secondary.pressed {
    background-color: var(--color-surface-300);
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .success {
    background-color: var(--color-success-600);
    color: white;
    border-color: var(--color-success-600);
  }

  .success.hover-effect:hover:not(.disabled) {
    background-color: var(--color-success-700);
    border-color: var(--color-success-700);
    transform: translateY(-1px);
    box-shadow: var(--shadow-lg);
  }

  .success.pressed {
    background-color: var(--color-success-800);
    transform: translateY(0);
    box-shadow: var(--shadow-md);
  }

  .warning {
    background-color: var(--color-warning-600);
    color: white;
    border-color: var(--color-warning-600);
  }

  .warning.hover-effect:hover:not(.disabled) {
    background-color: var(--color-warning-700);
    border-color: var(--color-warning-700);
    transform: translateY(-1px);
    box-shadow: var(--shadow-lg);
  }

  .warning.pressed {
    background-color: var(--color-warning-800);
    transform: translateY(0);
    box-shadow: var(--shadow-md);
  }

  .error {
    background-color: var(--color-error-600);
    color: white;
    border-color: var(--color-error-600);
  }

  .error.hover-effect:hover:not(.disabled) {
    background-color: var(--color-error-700);
    border-color: var(--color-error-700);
    transform: translateY(-1px);
    box-shadow: var(--shadow-lg);
  }

  .error.pressed {
    background-color: var(--color-error-800);
    transform: translateY(0);
    box-shadow: var(--shadow-md);
  }

  .ghost {
    background-color: transparent;
    color: var(--color-primary-600);
    border-color: transparent;
  }

  .ghost.hover-effect:hover:not(.disabled) {
    background-color: var(--color-primary-50);
    color: var(--color-primary-700);
    border-color: var(--color-primary-200);
  }

  .ghost.pressed {
    background-color: var(--color-primary-100);
    color: var(--color-primary-800);
  }

  /* Focus styles */
  .focus-effect:focus-visible {
    outline: 3px solid var(--color-primary-500);
    outline-offset: 2px;
    box-shadow: 0 0 0 6px rgba(33, 150, 243, 0.1);
  }

  /* Disabled state */
  .disabled {
    opacity: 0.6;
    cursor: not-allowed;
    pointer-events: none;
    transform: none !important;
    box-shadow: none !important;
  }

  /* Loading state */
  .loading {
    cursor: wait;
  }

  .loading .button-text {
    opacity: 0.8;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .secondary {
      background-color: var(--color-surface-700);
      color: var(--color-surface-200);
      border-color: var(--color-surface-600);
    }

    .secondary.hover-effect:hover:not(.disabled) {
      background-color: var(--color-surface-600);
      border-color: var(--color-surface-500);
    }

    .secondary.pressed {
      background-color: var(--color-surface-500);
    }

    .ghost {
      color: var(--color-primary-400);
    }

    .ghost.hover-effect:hover:not(.disabled) {
      background-color: var(--color-primary-900);
      color: var(--color-primary-300);
      border-color: var(--color-primary-700);
    }

    .ghost.pressed {
      background-color: var(--color-primary-800);
      color: var(--color-primary-200);
    }

    .ripple {
      background-color: rgba(255, 255, 255, 0.2);
    }
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .interactive-button {
      min-height: 48px;
      font-size: 16px; /* Prevent zoom on iOS */
    }

    .xs {
      min-height: 44px;
    }

    .sm {
      min-height: 46px;
    }
  }

  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    .interactive-button {
      transition: none;
    }

    .interactive-button.hover-effect:hover:not(.disabled),
    .interactive-button.pressed {
      transform: none;
    }

    .loading-icon {
      animation: none;
    }

    .ripple {
      animation: none;
      display: none;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .interactive-button {
      border-width: 3px;
    }

    .focus-effect:focus-visible {
      outline-width: 4px;
      outline-color: #000;
    }

    .primary,
    .success,
    .warning,
    .error {
      border-color: #000;
    }

    .secondary {
      border-color: #000;
      background-color: #fff;
      color: #000;
    }

    .ghost {
      border-color: #000;
    }
  }

  /* Print styles */
  @media print {
    .interactive-button {
      background: white !important;
      color: black !important;
      border: 2px solid black !important;
      box-shadow: none !important;
      transform: none !important;
    }

    .ripple-container {
      display: none;
    }
  }
</style>