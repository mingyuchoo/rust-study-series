<script lang="ts">
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';
  import LoadingSpinner from './LoadingSpinner.svelte';

  // Props
  export let src: string = '';
  export let alt: string = '';
  export let placeholder: string = '';
  export let threshold: number = 0.1;
  export let rootMargin: string = '50px';
  export let fadeInDuration: number = 300;
  export let showSpinner: boolean = true;
  export let spinnerSize: 'sm' | 'md' | 'lg' = 'md';
  export let component: any = null; // For lazy loading components
  export let componentProps: Record<string, any> = {};

  // State
  let isIntersecting = false;
  let isLoaded = false;
  let isError = false;
  let element: HTMLElement;
  let observer: IntersectionObserver;

  const dispatch = createEventDispatcher<{
    load: void;
    error: Error;
    visible: void;
  }>();

  // Initialize intersection observer
  onMount(() => {
    if (!element) return;

    observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && !isIntersecting) {
            isIntersecting = true;
            dispatch('visible');
            
            // Start loading the content
            if (src || component) {
              loadContent();
            }
          }
        });
      },
      {
        threshold,
        rootMargin
      }
    );

    observer.observe(element);

    return () => {
      if (observer) {
        observer.disconnect();
      }
    };
  });

  // Load content (image or component)
  async function loadContent() {
    if (isLoaded) return;

    try {
      if (component) {
        // For component lazy loading, we just mark as loaded
        // The actual component loading is handled by the parent
        isLoaded = true;
        dispatch('load');
      } else if (src) {
        // For image lazy loading
        const img = new Image();
        img.onload = () => {
          isLoaded = true;
          dispatch('load');
        };
        img.onerror = (error) => {
          isError = true;
          dispatch('error', new Error('Failed to load image'));
        };
        img.src = src;
      }
    } catch (error) {
      isError = true;
      dispatch('error', error as Error);
    }
  }

  // Handle retry
  function handleRetry() {
    isError = false;
    isLoaded = false;
    loadContent();
  }
</script>

<div 
  bind:this={element}
  class="lazy-load-container"
  class:loaded={isLoaded}
  class:error={isError}
>
  {#if !isIntersecting}
    <!-- Placeholder before intersection -->
    <div class="placeholder">
      {#if placeholder}
        <img src={placeholder} {alt} class="placeholder-image" />
      {:else}
        <div class="placeholder-content">
          <div class="placeholder-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <circle cx="8.5" cy="8.5" r="1.5"/>
              <polyline points="21,15 16,10 5,21"/>
            </svg>
          </div>
        </div>
      {/if}
    </div>
  {:else if isError}
    <!-- Error state -->
    <div class="error-state">
      <div class="error-content">
        <div class="error-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="15" y1="9" x2="9" y2="15"/>
            <line x1="9" y1="9" x2="15" y2="15"/>
          </svg>
        </div>
        <p class="error-message">Failed to load content</p>
        <button class="retry-button" on:click={handleRetry}>
          Retry
        </button>
      </div>
    </div>
  {:else if !isLoaded && showSpinner}
    <!-- Loading state -->
    <div class="loading-state">
      <LoadingSpinner size={spinnerSize} message="Loading..." />
    </div>
  {:else if isLoaded}
    <!-- Loaded content -->
    <div 
      class="loaded-content"
      style="animation-duration: {fadeInDuration}ms"
    >
      {#if component}
        <svelte:component this={component} {...componentProps} />
      {:else if src}
        <img {src} {alt} class="lazy-image" />
      {/if}
    </div>
  {/if}
</div>

<style>
  .lazy-load-container {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-surface-100);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-surface-200);
  }

  .placeholder-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    opacity: 0.7;
  }

  .placeholder-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-surface-400);
    gap: 0.5rem;
  }

  .placeholder-icon {
    opacity: 0.5;
  }

  .loading-state {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .error-state {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-error-50);
  }

  .error-content {
    text-align: center;
    color: var(--color-error-600);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .error-icon {
    opacity: 0.7;
  }

  .error-message {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-error-700);
  }

  .retry-button {
    background-color: var(--color-error-600);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .retry-button:hover {
    background-color: var(--color-error-700);
  }

  .retry-button:focus {
    outline: 2px solid var(--color-error-500);
    outline-offset: 2px;
  }

  .loaded-content {
    width: 100%;
    height: 100%;
    animation: fadeIn var(--animation-duration, 300ms) ease-out;
  }

  .lazy-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .lazy-load-container {
      background-color: var(--color-surface-800);
    }

    .placeholder {
      background-color: var(--color-surface-700);
    }

    .placeholder-content {
      color: var(--color-surface-500);
    }

    .error-state {
      background-color: var(--color-error-900);
    }

    .error-content {
      color: var(--color-error-400);
    }

    .error-message {
      color: var(--color-error-300);
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .loaded-content {
      animation: none;
    }
  }

  /* High contrast mode */
  @media (prefers-contrast: high) {
    .lazy-load-container {
      border: 2px solid;
    }

    .retry-button {
      border: 2px solid var(--color-error-600);
    }
  }
</style>