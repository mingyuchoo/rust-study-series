<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { currentError, appActions } from '../stores/app.store.js';
  import { AlertTriangle, RefreshCw, Home } from 'lucide-svelte';

  export let fallback: any = null;
  export let onError: ((error: Error) => void) | null = null;

  let hasError = false;
  let errorDetails: Error | null = null;
  let errorId = '';

  // Generate unique error ID for tracking
  function generateErrorId(): string {
    return `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  // Handle errors
  function handleError(error: Error) {
    hasError = true;
    errorDetails = error;
    errorId = generateErrorId();
    
    // Log error for debugging
    console.error('ErrorBoundary caught error:', error);
    
    // Call custom error handler if provided
    if (onError) {
      onError(error);
    }

    // Update global error state
    appActions.setError(`An unexpected error occurred (${errorId})`);
  }

  // Retry functionality
  function retry() {
    hasError = false;
    errorDetails = null;
    errorId = '';
    appActions.clearError();
    
    // Force component re-render by updating a reactive variable
    window.location.reload();
  }

  // Go to home page
  function goHome() {
    hasError = false;
    errorDetails = null;
    errorId = '';
    appActions.clearError();
    appActions.setCurrentPage('upload');
  }

  // Report error (placeholder for future error reporting service)
  function reportError(): void {
    if (errorDetails) {
      const errorReport = {
        id: errorId,
        message: errorDetails.message,
        stack: errorDetails.stack,
        timestamp: new Date().toISOString(),
        userAgent: navigator.userAgent,
        url: window.location.href
      };
      
      console.log('Error report:', errorReport);
      // TODO: Send to error reporting service
      // eslint-disable-next-line no-alert
      window.alert('Error report logged to console. In production, this would be sent to an error tracking service.');
    }
  }

  // Global error handler
  function globalErrorHandler(event: Event): void {
    const errorEvent = event as ErrorEvent;
    handleError(new Error(errorEvent.message));
  }

  // Unhandled promise rejection handler
  function unhandledRejectionHandler(event: Event): void {
    const rejectionEvent = event as PromiseRejectionEvent;
    const error = rejectionEvent.reason instanceof Error 
      ? rejectionEvent.reason 
      : new Error(String(rejectionEvent.reason));
    handleError(error);
  }

  onMount(() => {
    // Listen for global errors
    window.addEventListener('error', globalErrorHandler);
    window.addEventListener('unhandledrejection', unhandledRejectionHandler);
  });

  onDestroy(() => {
    // Clean up event listeners
    window.removeEventListener('error', globalErrorHandler);
    window.removeEventListener('unhandledrejection', unhandledRejectionHandler);
  });
</script>

{#if hasError}
  <div class="error-boundary">
    <div class="error-container">
      <div class="error-icon">
        <AlertTriangle size={48} />
      </div>
      
      <div class="error-content">
        <h1 class="error-title">Something went wrong</h1>
        <p class="error-message">
          We're sorry, but an unexpected error occurred. This has been logged and we'll look into it.
        </p>
        
        {#if errorDetails}
          <details class="error-details">
            <summary>Technical Details</summary>
            <div class="error-info">
              <p><strong>Error ID:</strong> {errorId}</p>
              <p><strong>Message:</strong> {errorDetails.message}</p>
              {#if errorDetails.stack}
                <pre class="error-stack">{errorDetails.stack}</pre>
              {/if}
            </div>
          </details>
        {/if}
        
        <div class="error-actions">
          <button class="btn btn-primary" on:click={retry}>
            <RefreshCw size={16} />
            Try Again
          </button>
          
          <button class="btn btn-secondary" on:click={goHome}>
            <Home size={16} />
            Go Home
          </button>
          
          <button class="btn btn-outline" on:click={reportError}>
            Report Issue
          </button>
        </div>
      </div>
    </div>
  </div>
{:else if fallback && $currentError}
  <svelte:component this={fallback} error={$currentError} />
{:else}
  <slot />
{/if}

<style>
  .error-boundary {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    background-color: var(--color-surface-50);
  }

  .error-container {
    max-width: 600px;
    width: 100%;
    text-align: center;
    background-color: white;
    border-radius: 0.75rem;
    padding: 3rem 2rem;
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    border: 1px solid var(--color-surface-200);
  }

  .error-icon {
    margin-bottom: 1.5rem;
    color: var(--color-error-500);
  }

  .error-title {
    margin: 0 0 1rem 0;
    font-size: 1.875rem;
    font-weight: 700;
    color: var(--color-surface-900);
  }

  .error-message {
    margin: 0 0 2rem 0;
    font-size: 1rem;
    color: var(--color-surface-600);
    line-height: 1.6;
  }

  .error-details {
    margin: 1.5rem 0;
    text-align: left;
    background-color: var(--color-surface-50);
    border-radius: 0.5rem;
    border: 1px solid var(--color-surface-200);
  }

  .error-details summary {
    padding: 1rem;
    cursor: pointer;
    font-weight: 500;
    color: var(--color-surface-700);
    border-radius: 0.5rem;
    transition: background-color 0.2s ease;
  }

  .error-details summary:hover {
    background-color: var(--color-surface-100);
  }

  .error-details[open] summary {
    border-bottom: 1px solid var(--color-surface-200);
    border-radius: 0.5rem 0.5rem 0 0;
  }

  .error-info {
    padding: 1rem;
  }

  .error-info p {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: var(--color-surface-600);
  }

  .error-stack {
    margin: 1rem 0 0 0;
    padding: 1rem;
    background-color: var(--color-surface-900);
    color: var(--color-surface-100);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    line-height: 1.4;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .error-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid transparent;
  }

  .btn-primary {
    background-color: var(--color-primary-600);
    color: white;
  }

  .btn-primary:hover {
    background-color: var(--color-primary-700);
  }

  .btn-secondary {
    background-color: var(--color-surface-600);
    color: white;
  }

  .btn-secondary:hover {
    background-color: var(--color-surface-700);
  }

  .btn-outline {
    background-color: transparent;
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .btn-outline:hover {
    background-color: var(--color-surface-50);
    border-color: var(--color-surface-400);
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .error-boundary {
      padding: 1rem;
    }

    .error-container {
      padding: 2rem 1.5rem;
    }

    .error-title {
      font-size: 1.5rem;
    }

    .error-actions {
      flex-direction: column;
      align-items: stretch;
    }

    .btn {
      justify-content: center;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .error-boundary {
      background-color: var(--color-surface-900);
    }

    .error-container {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .error-title {
      color: var(--color-surface-100);
    }

    .error-message {
      color: var(--color-surface-300);
    }

    .error-details {
      background-color: var(--color-surface-900);
      border-color: var(--color-surface-700);
    }

    .error-details summary {
      color: var(--color-surface-300);
    }

    .error-details summary:hover {
      background-color: var(--color-surface-800);
    }

    .error-details[open] summary {
      border-color: var(--color-surface-700);
    }

    .error-info p {
      color: var(--color-surface-400);
    }

    .btn-outline {
      color: var(--color-surface-300);
      border-color: var(--color-surface-600);
    }

    .btn-outline:hover {
      background-color: var(--color-surface-700);
      border-color: var(--color-surface-500);
    }
  }

  /* Focus styles for accessibility */
  .btn:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .error-details summary:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }
</style>