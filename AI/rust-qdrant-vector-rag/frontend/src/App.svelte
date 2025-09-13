<script lang="ts">
  import { onMount } from 'svelte';
  import Navigation from './lib/components/Navigation.svelte';
  import Router from './lib/components/Router.svelte';
  import ErrorBoundary from './lib/components/ErrorBoundary.svelte';
  import Toast from './lib/components/Toast.svelte';
  import OfflineIndicator from './lib/components/OfflineIndicator.svelte';
  import AccessibilityTester from './lib/components/AccessibilityTester.svelte';
  import { appActions } from './lib/stores/app.store.js';
  import { errorHandler } from './lib/services/error-handler.js';

  // Initialize app on mount
  onMount(() => {
    // The error handler service will manage online/offline status
    // No need to set up additional listeners here since ErrorHandlerService handles it
    
    // Initialize error handler (this sets up global error listeners)
    console.log('Error handler initialized');
    
    return () => {
      // Cleanup is handled by the error handler service
    };
  });
</script>

<div class="app-container">
  <ErrorBoundary>
    <!-- Offline indicator at the top -->
    <OfflineIndicator />
    
    <div class="app-layout">
      <Navigation />
      <main id="main-content" class="main-content" tabindex="-1">
        <Router />
      </main>
    </div>
    
    <!-- Toast notifications with retry functionality -->
    <Toast enableRetryActions={true} />
    
    <!-- Accessibility tester for development -->
    <AccessibilityTester autoRun={false} />
  </ErrorBoundary>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: var(--font-family-base);
    scroll-behavior: smooth;
  }

  :global(#app) {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .app-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: var(--color-surface-50);
    overflow: hidden;
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0; /* Allow flex children to shrink */
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--spacing-md);
    scroll-behavior: smooth;
    position: relative;
  }

  /* Focus styles for main content */
  .main-content:focus {
    outline: none;
  }

  /* Mobile styles (default) */
  @media (max-width: 767px) {
    .main-content {
      padding: var(--spacing-sm);
    }
  }

  /* Tablet styles */
  @media (min-width: 768px) and (max-width: 1023px) {
    .app-layout {
      flex-direction: row;
    }

    .main-content {
      padding: var(--spacing-lg);
      max-width: calc(100vw - 320px);
    }
  }

  /* Desktop styles */
  @media (min-width: 1024px) {
    .app-layout {
      flex-direction: row;
    }

    .main-content {
      padding: var(--spacing-xl);
      max-width: calc(100vw - 320px);
    }
  }

  /* Large desktop styles */
  @media (min-width: 1440px) {
    .main-content {
      padding: var(--spacing-2xl);
      max-width: calc(100vw - 360px);
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .app-container {
      background-color: var(--color-surface-900);
      color: var(--color-surface-50);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .app-container {
      border: 2px solid;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    :global(html) {
      scroll-behavior: auto;
    }
    
    .main-content {
      scroll-behavior: auto;
    }
  }

  /* Print styles */
  @media print {
    .app-container {
      height: auto;
      background: white;
      color: black;
    }
    
    .app-layout {
      flex-direction: column;
    }
    
    .main-content {
      padding: 1rem;
      overflow: visible;
    }
  }
</style>
