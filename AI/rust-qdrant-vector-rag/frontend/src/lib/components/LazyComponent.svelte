<script lang="ts">
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';
  import LoadingSpinner from './LoadingSpinner.svelte';

  // Props
  export let loader: () => Promise<any>;
  export let fallback: any = null;
  export let errorComponent: any = null;
  export let props: Record<string, any> = {};
  export let threshold: number = 0.1;
  export let rootMargin: string = '50px';
  export let immediate: boolean = false;

  // State
  let component: any = null;
  let isLoading = false;
  let error: Error | null = null;
  let isVisible = immediate;
  let element: HTMLElement;
  let observer: IntersectionObserver;

  const dispatch = createEventDispatcher<{
    load: { component: any };
    error: { error: Error };
    visible: void;
  }>();

  // Initialize intersection observer for lazy loading
  onMount(() => {
    if (immediate) {
      loadComponent();
      return;
    }

    if (!element) return;

    observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && !isVisible) {
            isVisible = true;
            dispatch('visible');
            loadComponent();
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

  // Load the component dynamically
  async function loadComponent() {
    if (isLoading || component) return;

    isLoading = true;
    error = null;

    try {
      const loadedModule = await loader();
      component = loadedModule.default || loadedModule;
      dispatch('load', { component });
    } catch (err) {
      error = err as Error;
      console.error('Failed to load component:', err);
      dispatch('error', { error: err as Error });
    } finally {
      isLoading = false;
    }
  }

  // Retry loading
  function retry() {
    error = null;
    component = null;
    loadComponent();
  }
</script>

<div 
  bind:this={element}
  class="lazy-component-container"
  class:loading={isLoading}
  class:error={!!error}
  class:loaded={!!component}
>
  {#if !isVisible && !immediate}
    <!-- Placeholder before intersection -->
    <div class="placeholder">
      {#if fallback}
        <svelte:component this={fallback} {...props} />
      {:else}
        <div class="default-placeholder">
          <div class="placeholder-content">
            <div class="placeholder-icon">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14,2 14,8 20,8"/>
                <line x1="16" y1="13" x2="8" y2="13"/>
                <line x1="16" y1="17" x2="8" y2="17"/>
                <polyline points="10,9 9,9 8,9"/>
              </svg>
            </div>
            <p>Loading component...</p>
          </div>
        </div>
      {/if}
    </div>
  {:else if error}
    <!-- Error state -->
    <div class="error-state">
      {#if errorComponent}
        <svelte:component this={errorComponent} {error} {retry} />
      {:else}
        <div class="default-error">
          <div class="error-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="15" y1="9" x2="9" y2="15"/>
              <line x1="9" y1="9" x2="15" y2="15"/>
            </svg>
          </div>
          <h3>Failed to load component</h3>
          <p>{error.message}</p>
          <button class="retry-button" on:click={retry}>
            Try Again
          </button>
        </div>
      {/if}
    </div>
  {:else if isLoading}
    <!-- Loading state -->
    <div class="loading-state">
      <LoadingSpinner size="md" message="Loading component..." />
    </div>
  {:else if component}
    <!-- Loaded component -->
    <div class="loaded-component">
      <svelte:component this={component} {...props} />
    </div>
  {/if}
</div>

<style>
  .lazy-component-container {
    position: relative;
    width: 100%;
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .default-placeholder {
    width: 100%;
    height: 100%;
    background-color: var(--color-surface-100);
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px dashed var(--color-surface-300);
  }

  .placeholder-content {
    text-align: center;
    color: var(--color-surface-500);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .placeholder-icon {
    opacity: 0.6;
  }

  .placeholder-content p {
    margin: 0;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .loading-state {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-surface-50);
    border-radius: 0.5rem;
  }

  .error-state {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .default-error {
    text-align: center;
    padding: 2rem;
    background-color: var(--color-error-50);
    border-radius: 0.75rem;
    border: 2px solid var(--color-error-200);
    color: var(--color-error-700);
    max-width: 400px;
  }

  .error-icon {
    margin: 0 auto 1rem;
    opacity: 0.7;
    color: var(--color-error-600);
  }

  .default-error h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-error-800);
  }

  .default-error p {
    margin: 0 0 1.5rem 0;
    font-size: 0.875rem;
    color: var(--color-error-600);
    line-height: 1.5;
  }

  .retry-button {
    background-color: var(--color-error-600);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .retry-button:hover {
    background-color: var(--color-error-700);
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .retry-button:active {
    transform: translateY(0);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .retry-button:focus {
    outline: 2px solid var(--color-error-500);
    outline-offset: 2px;
  }

  .loaded-component {
    width: 100%;
    height: 100%;
    animation: fadeIn 0.3s ease-out;
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
    .default-placeholder {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .placeholder-content {
      color: var(--color-surface-400);
    }

    .loading-state {
      background-color: var(--color-surface-900);
    }

    .default-error {
      background-color: var(--color-error-900);
      border-color: var(--color-error-700);
      color: var(--color-error-300);
    }

    .default-error h3 {
      color: var(--color-error-200);
    }

    .default-error p {
      color: var(--color-error-400);
    }

    .error-icon {
      color: var(--color-error-400);
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .loaded-component {
      animation: none;
    }

    .retry-button:hover {
      transform: none;
    }

    .retry-button:active {
      transform: none;
    }
  }

  /* High contrast mode */
  @media (prefers-contrast: high) {
    .default-placeholder {
      border-width: 3px;
    }

    .default-error {
      border-width: 3px;
    }

    .retry-button {
      border: 2px solid var(--color-error-600);
    }
  }

  /* Loading state indicator */
  .lazy-component-container.loading {
    cursor: wait;
  }

  .lazy-component-container.error {
    /* Error state styling handled by .error-state */
  }

  .lazy-component-container.loaded {
    /* Loaded state styling handled by .loaded-component */
  }
</style>