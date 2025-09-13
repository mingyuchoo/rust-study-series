<script lang="ts">
  import { onMount } from 'svelte';
  import { CheckCircle, AlertCircle, Loader, Clock } from 'lucide-svelte';

  // Props
  export let progress: number = 0;
  export let status: 'idle' | 'loading' | 'success' | 'error' = 'idle';
  export let message: string = '';
  export let showPercentage: boolean = true;
  export let showTimeRemaining: boolean = false;
  export let estimatedTimeRemaining: number | null = null;
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let variant: 'linear' | 'circular' = 'linear';
  export let animated: boolean = true;
  export let showIcon: boolean = true;

  // Internal state
  let mounted = false;
  let animationPhase = 0;

  // Reactive computations
  $: progressPercentage = Math.min(100, Math.max(0, progress));
  $: isActive = status === 'loading';
  $: isComplete = status === 'success';
  $: hasError = status === 'error';

  // Size mappings
  $: containerHeight = size === 'sm' ? '4px' : size === 'md' ? '8px' : '12px';
  $: iconSize = size === 'sm' ? 16 : size === 'md' ? 20 : 24;
  $: textSize = size === 'sm' ? 'text-xs' : size === 'md' ? 'text-sm' : 'text-base';

  // Get status icon
  function getStatusIcon() {
    switch (status) {
      case 'success':
        return CheckCircle;
      case 'error':
        return AlertCircle;
      case 'loading':
        return Loader;
      default:
        return Clock;
    }
  }

  // Get status color
  function getStatusColor(): string {
    switch (status) {
      case 'success':
        return 'var(--color-success-500)';
      case 'error':
        return 'var(--color-error-500)';
      case 'loading':
        return 'var(--color-primary-500)';
      default:
        return 'var(--color-surface-400)';
    }
  }

  // Format time remaining
  function formatTimeRemaining(seconds: number): string {
    if (seconds < 60) {
      return `${Math.round(seconds)}s remaining`;
    } else if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = Math.round(seconds % 60);
      return `${minutes}m ${remainingSeconds}s remaining`;
    } else {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return `${hours}h ${minutes}m remaining`;
    }
  }

  // Animate progress bar stripes
  onMount(() => {
    mounted = true;
    if (animated && isActive) {
      const interval = setInterval(() => {
        animationPhase = (animationPhase + 1) % 100;
      }, 50);

      return () => clearInterval(interval);
    }
  });

  // Calculate circular progress
  $: circumference = 2 * Math.PI * 45; // radius = 45
  $: strokeDasharray = circumference;
  $: strokeDashoffset = circumference - (progressPercentage / 100) * circumference;
</script>

<div 
  class="progress-indicator {size}" 
  class:active={isActive}
  class:complete={isComplete}
  class:error={hasError}
  role="progressbar" 
  aria-valuenow={progressPercentage} 
  aria-valuemin="0" 
  aria-valuemax="100"
  aria-label={message || `Progress: ${progressPercentage}%`}
>
  {#if variant === 'circular'}
    <!-- Circular Progress -->
    <div class="circular-progress">
      <svg class="progress-ring" width="100" height="100" viewBox="0 0 100 100">
        <!-- Background circle -->
        <circle
          class="progress-ring-background"
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke="var(--color-surface-200)"
          stroke-width="6"
        />
        <!-- Progress circle -->
        <circle
          class="progress-ring-progress"
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke={getStatusColor()}
          stroke-width="6"
          stroke-linecap="round"
          stroke-dasharray={strokeDasharray}
          stroke-dashoffset={strokeDashoffset}
          transform="rotate(-90 50 50)"
          class:animated={animated && isActive}
        />
      </svg>
      
      <!-- Center content -->
      <div class="progress-center">
        {#if showIcon}
          <svelte:component 
            this={getStatusIcon()} 
            size={iconSize} 
            color={getStatusColor()}
            class={isActive ? 'spinning' : ''}
          />
        {/if}
        {#if showPercentage}
          <span class="progress-percentage {textSize}">
            {Math.round(progressPercentage)}%
          </span>
        {/if}
      </div>
    </div>
  {:else}
    <!-- Linear Progress -->
    <div class="linear-progress">
      <!-- Progress header -->
      {#if showIcon || message || showPercentage}
        <div class="progress-header">
          <div class="progress-info">
            {#if showIcon}
              <svelte:component 
                this={getStatusIcon()} 
                size={iconSize} 
                color={getStatusColor()}
                class={isActive ? 'spinning' : ''}
              />
            {/if}
            {#if message}
              <span class="progress-message {textSize}">
                {message}
              </span>
            {/if}
          </div>
          {#if showPercentage}
            <span class="progress-percentage {textSize}">
              {Math.round(progressPercentage)}%
            </span>
          {/if}
        </div>
      {/if}

      <!-- Progress bar -->
      <div class="progress-track" style="height: {containerHeight}">
        <div 
          class="progress-fill"
          style="width: {progressPercentage}%; background-color: {getStatusColor()}"
          class:animated={animated && isActive}
          class:pulsing={isActive && progressPercentage < 5}
        ></div>
      </div>

      <!-- Progress footer -->
      {#if showTimeRemaining && estimatedTimeRemaining}
        <div class="progress-footer">
          <span class="time-remaining {textSize}">
            {formatTimeRemaining(estimatedTimeRemaining)}
          </span>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .progress-indicator {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .progress-indicator.sm {
    font-size: var(--font-size-xs);
  }

  .progress-indicator.md {
    font-size: var(--font-size-sm);
  }

  .progress-indicator.lg {
    font-size: var(--font-size-base);
  }

  /* Circular Progress Styles */
  .circular-progress {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 100px;
    margin: 0 auto;
  }

  .progress-ring {
    transform: rotate(-90deg);
  }

  .progress-ring-progress {
    transition: stroke-dashoffset 0.3s ease;
  }

  .progress-ring-progress.animated {
    animation: progress-pulse 2s ease-in-out infinite;
  }

  .progress-center {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-xs);
  }

  .progress-percentage {
    font-weight: 600;
    color: var(--color-surface-700);
    line-height: 1;
  }

  /* Linear Progress Styles */
  .linear-progress {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--spacing-sm);
  }

  .progress-info {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    flex: 1;
    min-width: 0;
  }

  .progress-message {
    color: var(--color-surface-600);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .progress-track {
    width: 100%;
    background-color: var(--color-surface-200);
    border-radius: 9999px;
    overflow: hidden;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    border-radius: 9999px;
    transition: width 0.3s ease;
    position: relative;
    overflow: hidden;
  }

  .progress-fill.animated {
    background-image: linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.2) 25%,
      transparent 25%,
      transparent 50%,
      rgba(255, 255, 255, 0.2) 50%,
      rgba(255, 255, 255, 0.2) 75%,
      transparent 75%,
      transparent
    );
    background-size: 1rem 1rem;
    animation: progress-stripes 1s linear infinite;
  }

  .progress-fill.pulsing {
    animation: progress-pulse 1.5s ease-in-out infinite;
  }

  .progress-footer {
    display: flex;
    justify-content: flex-end;
  }

  .time-remaining {
    color: var(--color-surface-500);
    font-size: 0.75em;
  }

  /* Status-specific styles */
  .progress-indicator.complete .progress-message {
    color: var(--color-success-600);
  }

  .progress-indicator.error .progress-message {
    color: var(--color-error-600);
  }

  .progress-indicator.active .progress-message {
    color: var(--color-primary-600);
  }

  /* Icon animations */
  :global(.spinning) {
    animation: spin 1s linear infinite;
  }

  /* Animations */
  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes progress-stripes {
    from {
      background-position: 1rem 0;
    }
    to {
      background-position: 0 0;
    }
  }

  @keyframes progress-pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.7;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .progress-track {
      background-color: var(--color-surface-700);
    }

    .progress-percentage {
      color: var(--color-surface-200);
    }

    .progress-message {
      color: var(--color-surface-300);
    }

    .time-remaining {
      color: var(--color-surface-400);
    }

    .progress-ring-background {
      stroke: var(--color-surface-700);
    }

    .progress-indicator.complete .progress-message {
      color: var(--color-success-400);
    }

    .progress-indicator.error .progress-message {
      color: var(--color-error-400);
    }

    .progress-indicator.active .progress-message {
      color: var(--color-primary-400);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .progress-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--spacing-xs);
    }

    .progress-info {
      width: 100%;
    }

    .progress-percentage {
      align-self: flex-end;
    }

    .circular-progress {
      width: 80px;
      height: 80px;
    }
  }

  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    .progress-ring-progress,
    .progress-fill {
      transition: none;
    }

    .progress-fill.animated,
    .progress-fill.pulsing,
    .progress-ring-progress.animated,
    :global(.spinning) {
      animation: none;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .progress-track {
      border: 2px solid var(--color-surface-900);
    }

    .progress-ring-background {
      stroke-width: 8;
    }

    .progress-ring-progress {
      stroke-width: 8;
    }
  }
</style>