<script lang="ts">
  import { toastStore, toastActions } from '../stores/toast.store.js';
  import { CheckCircle, AlertCircle, AlertTriangle, Info, X } from 'lucide-svelte';
  import { fly, fade } from 'svelte/transition';
  import { flip } from 'svelte/animate';

  export let position: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left' = 'top-right';
  export let maxToasts = 5;

  // Limit the number of toasts displayed
  $: displayedToasts = $toastStore.slice(-maxToasts);

  // Get icon for toast type
  function getIcon(type: string) {
    switch (type) {
      case 'success':
        return CheckCircle;
      case 'error':
        return AlertCircle;
      case 'warning':
        return AlertTriangle;
      case 'info':
      default:
        return Info;
    }
  }

  // Get color classes for toast type
  function getColorClasses(type: string) {
    switch (type) {
      case 'success':
        return 'toast-success';
      case 'error':
        return 'toast-error';
      case 'warning':
        return 'toast-warning';
      case 'info':
      default:
        return 'toast-info';
    }
  }

  // Handle toast dismissal
  function dismissToast(id: string) {
    toastActions.remove(id);
  }

  // Handle keyboard dismissal
  function handleKeydown(event: KeyboardEvent, id: string): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      dismissToast(id);
    }
  }

  // Get transition direction based on position
  function getTransition(position: string) {
    switch (position) {
      case 'top-right':
      case 'bottom-right':
        return { x: 300, y: 0 };
      case 'top-left':
      case 'bottom-left':
        return { x: -300, y: 0 };
      default:
        return { x: 300, y: 0 };
    }
  }

  $: transitionParams = getTransition(position);
</script>

{#if displayedToasts.length > 0}
  <div class="toast-container {position}" role="region" aria-label="Notifications">
    {#each displayedToasts as toast (toast.id)}
      <div
        class="toast {getColorClasses(toast.type)}"
        role="alert"
        aria-live="polite"
        in:fly={{ ...transitionParams, duration: 300 }}
        out:fade={{ duration: 200 }}
        animate:flip={{ duration: 200 }}
      >
        <div class="toast-content">
          <div class="toast-icon">
            <svelte:component this={getIcon(toast.type)} size={20} />
          </div>
          
          <div class="toast-message">
            {toast.message}
          </div>
          
          {#if toast.dismissible}
            <button
              class="toast-dismiss"
              on:click={() => dismissToast(toast.id)}
              on:keydown={(e) => handleKeydown(e, toast.id)}
              aria-label="Dismiss notification"
              title="Dismiss"
            >
              <X size={16} />
            </button>
          {/if}
        </div>
        
        {#if toast.duration && toast.duration > 0}
          <div class="toast-progress">
            <div 
              class="toast-progress-bar"
              style="animation-duration: {toast.duration}ms"
            ></div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 400px;
    width: 100%;
    padding: 1rem;
    pointer-events: none;
  }

  .toast-container.top-right {
    top: 0;
    right: 0;
  }

  .toast-container.top-left {
    top: 0;
    left: 0;
  }

  .toast-container.bottom-right {
    bottom: 0;
    right: 0;
    flex-direction: column-reverse;
  }

  .toast-container.bottom-left {
    bottom: 0;
    left: 0;
    flex-direction: column-reverse;
  }

  .toast {
    background-color: white;
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    border: 1px solid var(--color-surface-200);
    overflow: hidden;
    pointer-events: auto;
    position: relative;
    min-height: 60px;
  }

  .toast-content {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
  }

  .toast-icon {
    flex-shrink: 0;
    margin-top: 0.125rem;
  }

  .toast-message {
    flex: 1;
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--color-surface-900);
    word-wrap: break-word;
  }

  .toast-dismiss {
    flex-shrink: 0;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 0.25rem;
    color: var(--color-surface-500);
    transition: all 0.2s ease;
    margin-top: -0.125rem;
  }

  .toast-dismiss:hover {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
  }

  .toast-dismiss:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .toast-progress {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3px;
    background-color: rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .toast-progress-bar {
    height: 100%;
    background-color: currentColor;
    animation: toast-progress linear forwards;
    transform-origin: left;
  }

  @keyframes toast-progress {
    from {
      transform: scaleX(1);
    }
    to {
      transform: scaleX(0);
    }
  }

  /* Toast type styles */
  .toast-success {
    border-left: 4px solid var(--color-success-500);
  }

  .toast-success .toast-icon {
    color: var(--color-success-600);
  }

  .toast-success .toast-progress-bar {
    background-color: var(--color-success-500);
  }

  .toast-error {
    border-left: 4px solid var(--color-error-500);
  }

  .toast-error .toast-icon {
    color: var(--color-error-600);
  }

  .toast-error .toast-progress-bar {
    background-color: var(--color-error-500);
  }

  .toast-warning {
    border-left: 4px solid var(--color-warning-500);
  }

  .toast-warning .toast-icon {
    color: var(--color-warning-600);
  }

  .toast-warning .toast-progress-bar {
    background-color: var(--color-warning-500);
  }

  .toast-info {
    border-left: 4px solid var(--color-info-500);
  }

  .toast-info .toast-icon {
    color: var(--color-info-600);
  }

  .toast-info .toast-progress-bar {
    background-color: var(--color-info-500);
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .toast-container {
      max-width: none;
      left: 1rem !important;
      right: 1rem !important;
      padding: 0;
    }

    .toast-container.top-left,
    .toast-container.top-right {
      top: 1rem;
    }

    .toast-container.bottom-left,
    .toast-container.bottom-right {
      bottom: 1rem;
    }

    .toast {
      margin: 0;
    }

    .toast-content {
      padding: 0.875rem;
    }

    .toast-message {
      font-size: 0.8125rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .toast {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .toast-message {
      color: var(--color-surface-100);
    }

    .toast-dismiss {
      color: var(--color-surface-400);
    }

    .toast-dismiss:hover {
      background-color: var(--color-surface-700);
      color: var(--color-surface-200);
    }

    .toast-progress {
      background-color: rgba(255, 255, 255, 0.1);
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .toast {
      transition: none;
    }

    .toast-progress-bar {
      animation: none;
      transform: scaleX(0);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .toast {
      border-width: 2px;
    }

    .toast-dismiss:focus {
      outline-width: 3px;
    }
  }
</style>