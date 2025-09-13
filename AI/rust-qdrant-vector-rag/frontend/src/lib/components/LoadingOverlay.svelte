<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import LoadingSpinner from './LoadingSpinner.svelte';
  import ProgressIndicator from './ProgressIndicator.svelte';

  // Props
  export let visible: boolean = false;
  export let message: string = 'Loading...';
  export let variant: 'search' | 'upload' | 'processing' = 'processing';
  export let showProgress: boolean = false;
  export let progress: number = 0;
  export let estimatedTime: number | null = null;
  export let backdrop: 'light' | 'dark' | 'blur' = 'light';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let allowDismiss: boolean = false;
  export let zIndex: number = 1040;

  // Internal state
  let overlayElement: HTMLElement;
  let mounted = false;

  // Handle escape key
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && allowDismiss) {
      visible = false;
    }
  }

  // Handle backdrop click
  function handleBackdropClick(event: MouseEvent) {
    if (allowDismiss && event.target === overlayElement) {
      visible = false;
    }
  }

  // Prevent body scroll when overlay is visible
  $: if (typeof document !== 'undefined') {
    if (visible) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
  }

  onMount(() => {
    mounted = true;
    
    return () => {
      if (typeof document !== 'undefined') {
        document.body.style.overflow = '';
      }
    };
  });
</script>

{#if visible && mounted}
  <div
    class="loading-overlay {backdrop}"
    style="z-index: {zIndex}"
    role="dialog"
    aria-modal="true"
    aria-labelledby="loading-title"
    aria-describedby="loading-description"
    bind:this={overlayElement}
    on:click={handleBackdropClick}
    on:keydown={handleKeydown}
    tabindex="-1"
    transition:fade={{ duration: 200 }}
  >
    <div class="loading-content {size}" role="presentation" on:click|stopPropagation on:keydown|stopPropagation>
      <div class="loading-container">
        {#if showProgress}
          <ProgressIndicator
            {progress}
            status="loading"
            {message}
            showPercentage={true}
            showTimeRemaining={estimatedTime !== null}
            estimatedTimeRemaining={estimatedTime}
            {size}
            variant="circular"
            animated={true}
          />
        {:else}
          <LoadingSpinner
            {size}
            {message}
            {variant}
            showProgress={false}
          />
        {/if}

        <div class="loading-text">
          <h2 id="loading-title" class="loading-title">
            {showProgress ? 'Processing...' : 'Loading'}
          </h2>
          <p id="loading-description" class="loading-description">
            {message}
          </p>
          
          {#if estimatedTime && showProgress}
            <p class="loading-time">
              Estimated time: {Math.round(estimatedTime)}s
            </p>
          {/if}
        </div>

        {#if allowDismiss}
          <p class="dismiss-hint">
            Press Escape or click outside to dismiss
          </p>
        {/if}
      </div>
    </div>
  </div>
{/if}

<svelte:window on:keydown={handleKeydown} />

<style>
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-lg);
  }

  .loading-overlay.light {
    background-color: rgba(255, 255, 255, 0.9);
  }

  .loading-overlay.dark {
    background-color: rgba(0, 0, 0, 0.7);
  }

  .loading-overlay.blur {
    background-color: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
  }

  .loading-content {
    background: white;
    border-radius: 1rem;
    box-shadow: var(--shadow-xl);
    border: 1px solid var(--color-surface-200);
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
  }

  .loading-content.sm {
    padding: var(--spacing-lg);
    min-width: 280px;
  }

  .loading-content.md {
    padding: var(--spacing-xl);
    min-width: 320px;
  }

  .loading-content.lg {
    padding: var(--spacing-2xl);
    min-width: 400px;
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-lg);
    text-align: center;
  }

  .loading-text {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    max-width: 300px;
  }

  .loading-title {
    margin: 0;
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-surface-900);
    line-height: var(--line-height-tight);
  }

  .loading-description {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--color-surface-600);
    line-height: var(--line-height-normal);
  }

  .loading-time {
    margin: 0;
    font-size: var(--font-size-xs);
    color: var(--color-surface-500);
    font-style: italic;
  }

  .dismiss-hint {
    margin: 0;
    font-size: var(--font-size-xs);
    color: var(--color-surface-400);
    text-align: center;
    margin-top: var(--spacing-md);
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .loading-overlay {
      padding: var(--spacing-md);
    }

    .loading-content {
      min-width: auto;
      width: 100%;
      max-width: 350px;
    }

    .loading-content.sm,
    .loading-content.md,
    .loading-content.lg {
      padding: var(--spacing-lg);
    }

    .loading-text {
      max-width: none;
    }

    .loading-title {
      font-size: var(--font-size-base);
    }

    .loading-description {
      font-size: var(--font-size-xs);
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .loading-overlay.light {
      background-color: rgba(0, 0, 0, 0.8);
    }

    .loading-overlay.blur {
      background-color: rgba(0, 0, 0, 0.7);
    }

    .loading-content {
      background: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .loading-title {
      color: var(--color-surface-100);
    }

    .loading-description {
      color: var(--color-surface-300);
    }

    .loading-time {
      color: var(--color-surface-400);
    }

    .dismiss-hint {
      color: var(--color-surface-500);
    }
  }

  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    .loading-overlay {
      transition: none;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .loading-content {
      border-width: 3px;
      border-color: #000;
    }

    .loading-overlay.light {
      background-color: rgba(255, 255, 255, 0.95);
    }

    .loading-overlay.dark {
      background-color: rgba(0, 0, 0, 0.9);
    }
  }

  /* Print styles */
  @media print {
    .loading-overlay {
      display: none;
    }
  }

  /* Focus management */
  .loading-overlay:focus {
    outline: none;
  }

  /* Prevent text selection */
  .loading-overlay {
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
  }

  /* Smooth backdrop filter support */
  @supports (backdrop-filter: blur(8px)) {
    .loading-overlay.blur {
      background-color: rgba(255, 255, 255, 0.6);
    }
  }

  @supports not (backdrop-filter: blur(8px)) {
    .loading-overlay.blur {
      background-color: rgba(255, 255, 255, 0.9);
    }
  }
</style>