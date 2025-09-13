<script lang="ts">
  import { onMount } from 'svelte';
  import Navigation from './lib/components/Navigation.svelte';
  import Router from './lib/components/Router.svelte';
  import ErrorBoundary from './lib/components/ErrorBoundary.svelte';
  import Toast from './lib/components/Toast.svelte';
  import { appActions } from './lib/stores/app.store.js';

  // Initialize app on mount
  onMount(() => {
    // Set up online/offline listeners
    const handleOnline = () => appActions.setOnlineStatus(true);
    const handleOffline = () => appActions.setOnlineStatus(false);
    
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  });
</script>

<div class="app-container">
  <ErrorBoundary>
    <div class="app-layout">
      <Navigation />
      <main class="main-content">
        <Router />
      </main>
    </div>
    <Toast />
  </ErrorBoundary>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
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
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  /* Responsive design */
  @media (min-width: 768px) {
    .app-layout {
      flex-direction: row;
    }

    .main-content {
      padding: 2rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .app-container {
      background-color: var(--color-surface-900);
      color: var(--color-surface-50);
    }
  }
</style>
