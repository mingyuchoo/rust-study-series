<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { CheckCircle, X, ExternalLink, Copy, Share } from 'lucide-svelte';
  import { fly, fade } from 'svelte/transition';

  // Props
  export let title: string = 'Success!';
  export let message: string = '';
  export let duration: number = 5000; // 0 = no auto-dismiss
  export let dismissible: boolean = true;
  export let showIcon: boolean = true;
  export let variant: 'success' | 'info' | 'celebration' = 'success';
  export let actions: Array<{
    label: string;
    action: string;
    icon?: any;
    variant?: 'primary' | 'secondary';
  }> = [];
  export let autoShow: boolean = true;
  export let position: 'top' | 'center' | 'bottom' = 'top';

  // Internal state
  let visible = autoShow;
  let timeoutId: number | null = null;
  let progressWidth = 100;
  let progressInterval: number | null = null;

  const dispatch = createEventDispatcher();

  // Show notification
  export function show() {
    visible = true;
    startAutoHide();
  }

  // Hide notification
  export function hide() {
    visible = false;
    clearTimers();
    dispatch('dismiss');
  }

  // Start auto-hide timer
  function startAutoHide() {
    if (duration > 0) {
      // Start progress bar animation
      progressWidth = 100;
      const startTime = Date.now();
      
      progressInterval = window.setInterval(() => {
        const elapsed = Date.now() - startTime;
        const remaining = Math.max(0, duration - elapsed);
        progressWidth = (remaining / duration) * 100;
        
        if (remaining <= 0) {
          hide();
        }
      }, 50);

      // Set timeout for auto-hide
      timeoutId = window.setTimeout(() => {
        hide();
      }, duration);
    }
  }

  // Clear all timers
  function clearTimers() {
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = null;
    }
  }

  // Handle action click
  function handleAction(action: string) {
    dispatch('action', { action });
  }

  // Handle mouse enter (pause auto-hide)
  function handleMouseEnter() {
    clearTimers();
  }

  // Handle mouse leave (resume auto-hide)
  function handleMouseLeave() {
    if (duration > 0 && visible) {
      startAutoHide();
    }
  }

  // Handle keyboard events
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && dismissible) {
      hide();
    }
  }

  // Initialize
  onMount(() => {
    if (autoShow) {
      startAutoHide();
    }

    return () => {
      clearTimers();
    };
  });

  // Get icon based on variant
  function getIcon() {
    switch (variant) {
      case 'celebration':
        return '🎉';
      case 'info':
        return 'ℹ️';
      default:
        return CheckCircle;
    }
  }

  // Get transition parameters based on position
  function getTransitionParams() {
    switch (position) {
      case 'center':
        return { y: 0, x: 0 };
      case 'bottom':
        return { y: 100, x: 0 };
      default:
        return { y: -100, x: 0 };
    }
  }
</script>

{#if visible}
  <div
    class="success-notification {variant} {position}"
    role="alert"
    aria-live="polite"
    in:fly={{ ...getTransitionParams(), duration: 300 }}
    out:fade={{ duration: 200 }}
    on:mouseenter={handleMouseEnter}
    on:mouseleave={handleMouseLeave}
    tabindex="-1"
  >
    <div class="notification-content">
      <!-- Icon -->
      {#if showIcon}
        <div class="notification-icon">
          {#if variant === 'celebration'}
            <span class="celebration-emoji" role="img" aria-label="Celebration">
              🎉
            </span>
          {:else}
            <svelte:component this={getIcon()} size={24} />
          {/if}
        </div>
      {/if}

      <!-- Content -->
      <div class="notification-body">
        <h4 class="notification-title">
          {title}
        </h4>
        {#if message}
          <p class="notification-message">
            {message}
          </p>
        {/if}

        <!-- Actions -->
        {#if actions.length > 0}
          <div class="notification-actions">
            {#each actions as action}
              <button
                class="action-button {action.variant || 'secondary'}"
                on:click={() => handleAction(action.action)}
                aria-label={action.label}
              >
                {#if action.icon}
                  <svelte:component this={action.icon} size={16} />
                {/if}
                {action.label}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Dismiss button -->
      {#if dismissible}
        <button
          class="dismiss-button"
          on:click={hide}
          aria-label="Dismiss notification"
          title="Dismiss"
        >
          <X size={20} />
        </button>
      {/if}
    </div>

    <!-- Progress bar -->
    {#if duration > 0}
      <div class="progress-bar">
        <div 
          class="progress-fill"
          style="width: {progressWidth}%"
        ></div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .success-notification {
    position: fixed;
    z-index: var(--z-toast);
    max-width: 400px;
    width: calc(100% - 2rem);
    background: white;
    border-radius: 0.75rem;
    box-shadow: var(--shadow-xl);
    border: 1px solid var(--color-surface-200);
    overflow: hidden;
    pointer-events: auto;
  }

  .success-notification.top {
    top: 1rem;
    right: 1rem;
  }

  .success-notification.center {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: auto;
    min-width: 300px;
  }

  .success-notification.bottom {
    bottom: 1rem;
    right: 1rem;
  }

  .notification-content {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
  }

  .notification-icon {
    flex-shrink: 0;
    margin-top: 0.125rem;
  }

  .celebration-emoji {
    font-size: 1.5rem;
    animation: celebration-bounce 0.6s ease-out;
  }

  .notification-body {
    flex: 1;
    min-width: 0;
  }

  .notification-title {
    margin: 0 0 var(--spacing-xs) 0;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--color-surface-900);
    line-height: var(--line-height-tight);
  }

  .notification-message {
    margin: 0 0 var(--spacing-md) 0;
    font-size: var(--font-size-sm);
    color: var(--color-surface-600);
    line-height: var(--line-height-normal);
    word-wrap: break-word;
  }

  .notification-actions {
    display: flex;
    gap: var(--spacing-sm);
    flex-wrap: wrap;
  }

  .action-button {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-xs) var(--spacing-sm);
    border: 1px solid transparent;
    border-radius: 0.375rem;
    font-size: var(--font-size-xs);
    font-weight: 500;
    cursor: pointer;
    transition: all var(--duration-fast) ease;
    text-decoration: none;
    background: none;
  }

  .action-button.primary {
    background-color: var(--color-primary-600);
    color: white;
    border-color: var(--color-primary-600);
  }

  .action-button.primary:hover {
    background-color: var(--color-primary-700);
    border-color: var(--color-primary-700);
  }

  .action-button.secondary {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .action-button.secondary:hover {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
  }

  .action-button:focus-visible {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .dismiss-button {
    flex-shrink: 0;
    background: none;
    border: none;
    color: var(--color-surface-400);
    cursor: pointer;
    padding: var(--spacing-xs);
    border-radius: 0.375rem;
    transition: all var(--duration-fast) ease;
    margin-top: -0.125rem;
  }

  .dismiss-button:hover {
    background-color: var(--color-surface-100);
    color: var(--color-surface-600);
  }

  .dismiss-button:focus-visible {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .progress-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3px;
    background-color: rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: currentColor;
    transition: width 0.05s linear;
    transform-origin: left;
  }

  /* Variant styles */
  .success {
    border-left: 4px solid var(--color-success-500);
  }

  .success .notification-icon {
    color: var(--color-success-600);
  }

  .success .progress-fill {
    background-color: var(--color-success-500);
  }

  .info {
    border-left: 4px solid var(--color-info-500);
  }

  .info .notification-icon {
    color: var(--color-info-600);
  }

  .info .progress-fill {
    background-color: var(--color-info-500);
  }

  .celebration {
    border-left: 4px solid var(--color-warning-500);
    background: linear-gradient(135deg, #fff 0%, #fef3c7 100%);
  }

  .celebration .progress-fill {
    background-color: var(--color-warning-500);
  }

  /* Animations */
  @keyframes celebration-bounce {
    0%, 20%, 50%, 80%, 100% {
      transform: translateY(0);
    }
    40% {
      transform: translateY(-10px);
    }
    60% {
      transform: translateY(-5px);
    }
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .success-notification {
      left: 1rem;
      right: 1rem;
      width: auto;
      max-width: none;
    }

    .success-notification.center {
      left: 1rem;
      right: 1rem;
      top: 50%;
      transform: translateY(-50%);
      width: auto;
      min-width: auto;
    }

    .notification-content {
      padding: var(--spacing-md);
    }

    .notification-actions {
      flex-direction: column;
    }

    .action-button {
      justify-content: center;
      min-height: 44px;
      font-size: 16px; /* Prevent zoom on iOS */
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .success-notification {
      background: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .notification-title {
      color: var(--color-surface-100);
    }

    .notification-message {
      color: var(--color-surface-300);
    }

    .action-button.secondary {
      background-color: var(--color-surface-700);
      color: var(--color-surface-200);
      border-color: var(--color-surface-600);
    }

    .action-button.secondary:hover {
      background-color: var(--color-surface-600);
      border-color: var(--color-surface-500);
    }

    .dismiss-button {
      color: var(--color-surface-500);
    }

    .dismiss-button:hover {
      background-color: var(--color-surface-700);
      color: var(--color-surface-300);
    }

    .progress-bar {
      background-color: rgba(255, 255, 255, 0.1);
    }

    .celebration {
      background: linear-gradient(135deg, var(--color-surface-800) 0%, var(--color-warning-900) 100%);
    }
  }

  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    .celebration-emoji {
      animation: none;
    }

    .progress-fill {
      transition: none;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .success-notification {
      border-width: 3px;
      border-color: #000;
    }

    .action-button {
      border-width: 2px;
    }

    .dismiss-button:focus-visible,
    .action-button:focus-visible {
      outline-width: 3px;
      outline-color: #000;
    }
  }

  /* Print styles */
  @media print {
    .success-notification {
      position: static;
      box-shadow: none;
      border: 2px solid black;
      background: white;
      color: black;
      page-break-inside: avoid;
    }

    .progress-bar {
      display: none;
    }

    .action-button {
      border: 1px solid black;
      background: white;
      color: black;
    }
  }
</style>