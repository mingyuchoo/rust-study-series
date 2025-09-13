<script lang="ts">
  import { currentPage } from '../stores/app.store.js';
  import { fade } from 'svelte/transition';
  import LoadingSpinner from './LoadingSpinner.svelte';
  
  type PageKey = 'upload' | 'search' | 'dashboard';

  // Lazy-loaded page components with code splitting
  const routes: Record<PageKey, () => Promise<any>> = {
    upload: () => import('../pages/UploadPage.svelte'),
    search: () => import('../pages/SearchPage.svelte'),
    dashboard: () => import('../pages/DashboardPage.svelte')
  };

  // Component loading state
  let currentComponent: any = null;
  let isLoading = false;
  let loadError: string | null = null;

  // Load component dynamically
  async function loadComponent(page: PageKey) {
    if (isLoading) return;
    
    isLoading = true;
    loadError = null;
    
    try {
      const componentModule = await routes[page]();
      currentComponent = componentModule.default;
    } catch (error) {
      console.error(`Failed to load page component: ${page}`, error);
      loadError = `Failed to load ${page} page. Please try again.`;
      currentComponent = null;
    } finally {
      isLoading = false;
    }
  }

  // React to page changes
  $: if ($currentPage) {
    loadComponent($currentPage as PageKey);
  }

  $: pageTitle = getPageTitle($currentPage);

  function getPageTitle(page: string): string {
    const titles: Record<PageKey, string> = {
      upload: 'Upload Documents',
      search: 'Search Documents',
      dashboard: 'System Dashboard'
    };
    return titles[page as PageKey] || 'Upload Documents';
  }
</script>

<svelte:head>
  <title>{pageTitle} - RAG Search</title>
</svelte:head>

<div class="router-container">
  <div class="page-header">
    <h1 class="page-title">{pageTitle}</h1>
    <p class="page-description">
      {#if $currentPage === 'upload'}
        Upload PDF documents to make them searchable with AI-powered queries.
      {:else if $currentPage === 'search'}
        Ask questions about your uploaded documents and get intelligent answers.
      {:else if $currentPage === 'dashboard'}
        Monitor system health and performance metrics.
      {/if}
    </p>
  </div>

  <div class="page-content">
    {#if isLoading}
      <div class="loading-container" in:fade={{ duration: 150 }}>
        <LoadingSpinner size="lg" message="Loading page..." />
      </div>
    {:else if loadError}
      <div class="error-container" in:fade={{ duration: 150 }}>
        <div class="error-content">
          <h2>Page Load Error</h2>
          <p>{loadError}</p>
          <button 
            class="retry-button"
            on:click={() => loadComponent($currentPage as PageKey)}
          >
            Retry
          </button>
        </div>
      </div>
    {:else if currentComponent}
      {#key $currentPage}
        <div class="page-wrapper" in:fade={{ duration: 200, delay: 100 }} out:fade={{ duration: 100 }}>
          <svelte:component this={currentComponent} />
        </div>
      {/key}
    {/if}
  </div>
</div>

<style>
  .router-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    max-width: 1200px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--color-surface-300);
  }

  .page-title {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-surface-900);
    line-height: 1.2;
  }

  .page-description {
    margin: 0;
    font-size: 1rem;
    color: var(--color-surface-600);
    line-height: 1.5;
  }

  .page-content {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .page-wrapper {
    height: 100%;
    overflow-y: auto;
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .router-container {
      padding: 0;
    }

    .page-header {
      margin-bottom: 1.5rem;
      padding-bottom: 0.75rem;
    }

    .page-title {
      font-size: 1.75rem;
    }

    .page-description {
      font-size: 0.875rem;
    }
  }

  /* Tablet responsive */
  @media (min-width: 768px) and (max-width: 1023px) {
    .page-title {
      font-size: 1.875rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .page-header {
      border-color: var(--color-surface-700);
    }

    .page-title {
      color: var(--color-surface-100);
    }

    .page-description {
      color: var(--color-surface-400);
    }
  }

  /* Accessibility improvements */
  .page-wrapper {
    scroll-behavior: smooth;
  }

  /* Focus management for screen readers */
  .page-wrapper:focus {
    outline: none;
  }

  .loading-container,
  .error-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 400px;
  }

  .error-content {
    text-align: center;
    padding: 2rem;
    background-color: var(--color-surface-100);
    border-radius: 0.75rem;
    border: 2px solid var(--color-error-200);
    max-width: 400px;
  }

  .error-content h2 {
    margin: 0 0 1rem 0;
    color: var(--color-error-700);
    font-size: 1.25rem;
  }

  .error-content p {
    margin: 0 0 1.5rem 0;
    color: var(--color-surface-600);
    line-height: 1.5;
  }

  .retry-button {
    background-color: var(--color-primary-600);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .retry-button:hover {
    background-color: var(--color-primary-700);
  }

  .retry-button:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  /* Dark mode support for error states */
  @media (prefers-color-scheme: dark) {
    .error-content {
      background-color: var(--color-surface-800);
      border-color: var(--color-error-700);
    }

    .error-content h2 {
      color: var(--color-error-400);
    }

    .error-content p {
      color: var(--color-surface-300);
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .page-wrapper {
      transition: none;
    }
  }
</style>