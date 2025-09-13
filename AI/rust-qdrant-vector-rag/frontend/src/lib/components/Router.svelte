<script lang="ts">
  import { currentPage } from '../stores/app.store.js';
  import UploadPage from '../pages/UploadPage.svelte';
  import SearchPage from '../pages/SearchPage.svelte';
  import DashboardPage from '../pages/DashboardPage.svelte';
  import { fade } from 'svelte/transition';
  
  type PageKey = 'upload' | 'search' | 'dashboard';

  // Route configuration
  const routes: Record<PageKey, any> = {
    upload: UploadPage,
    search: SearchPage,
    dashboard: DashboardPage
  };

  // Get current component based on route
  $: currentComponent = routes[($currentPage as PageKey)] || routes.upload;
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
    {#key $currentPage}
      <div class="page-wrapper" in:fade={{ duration: 200, delay: 100 }} out:fade={{ duration: 100 }}>
        <svelte:component this={currentComponent} />
      </div>
    {/key}
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

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .page-wrapper {
      transition: none;
    }
  }
</style>