<script lang="ts">
  import { Upload, CheckCircle, XCircle, Loader } from 'lucide-svelte';

  // Props
  export let progress: number = 0;
  export let isUploading: boolean = false;
  export let fileName: string = '';
  export let status: 'uploading' | 'success' | 'error' | 'idle' = 'idle';
  export let message: string = '';
  export let estimatedTimeRemaining: number | null = null;

  // Reactive computations
  $: progressPercentage = Math.min(100, Math.max(0, progress));
  $: isComplete = progressPercentage === 100 && !isUploading;
  $: hasError = status === 'error';
  $: isActive = isUploading || status === 'uploading';

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

  // Get progress color based on status
  function getProgressColor(): string {
    switch (status) {
      case 'success':
        return 'green';
      case 'error':
        return 'red';
      case 'uploading':
        return 'blue';
      default:
        return 'gray';
    }
  }

  // Get status icon
  function getStatusIcon() {
    switch (status) {
      case 'success':
        return CheckCircle;
      case 'error':
        return XCircle;
      case 'uploading':
        return Loader;
      default:
        return Upload;
    }
  }

  // Get status text
  function getStatusText(): string {
    switch (status) {
      case 'uploading':
        return 'Uploading...';
      case 'success':
        return 'Upload complete';
      case 'error':
        return 'Upload failed';
      default:
        return 'Ready to upload';
    }
  }
</script>

{#if isActive || isComplete || hasError}
  <div class="upload-progress">
    <div class="progress-header">
      <div class="progress-info">
        <svelte:component 
          this={getStatusIcon()} 
          size={20} 
          color={hasError ? 'var(--color-error-600)' : 
                 isComplete ? 'var(--color-success-600)' : 
                 'var(--color-primary-600)'} 
          class={isActive ? 'spinning' : ''}
        />
        <div class="file-details">
          <h4 class="file-name">
            {fileName || 'Uploading file...'}
          </h4>
          <p class="status-text">
            {getStatusText()}
          </p>
        </div>
      </div>
      
      <div class="progress-percentage">
        {progressPercentage.toFixed(0)}%
      </div>
    </div>

    <!-- Progress bar -->
    <div class="progress-bar-container">
      <div 
        class="progress-bar {getProgressColor()}" 
        style="width: {progressPercentage}%"
        class:animated={isActive}
      ></div>
    </div>

    <!-- Additional information -->
    <div class="progress-footer">
      <div class="message-container">
        {#if message}
          <p class="progress-message" class:error={hasError}>
            {message}
          </p>
        {/if}
      </div>
      
      <div class="time-remaining">
        {#if estimatedTimeRemaining && isActive}
          <p class="time-text">
            {formatTimeRemaining(estimatedTimeRemaining)}
          </p>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .upload-progress {
    width: 100%;
    background-color: var(--color-surface-50);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.5rem;
    padding: 1rem;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .progress-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
  }

  .file-details {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    margin: 0 0 0.25rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-900);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .status-text {
    margin: 0;
    font-size: 0.75rem;
    color: var(--color-surface-600);
  }

  .progress-percentage {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-700);
    margin-left: 1rem;
  }

  .progress-bar-container {
    width: 100%;
    height: 8px;
    background-color: var(--color-surface-200);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.75rem;
  }

  .progress-bar {
    height: 100%;
    transition: width 0.3s ease;
    border-radius: 4px;
  }

  .progress-bar.animated {
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

  .progress-bar.green {
    background-color: var(--color-success-500);
  }

  .progress-bar.red {
    background-color: var(--color-error-500);
  }

  .progress-bar.blue {
    background-color: var(--color-primary-500);
  }

  .progress-bar.gray {
    background-color: var(--color-surface-400);
  }

  .progress-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-height: 1rem;
  }

  .message-container {
    flex: 1;
  }

  .progress-message {
    margin: 0;
    font-size: 0.75rem;
    color: var(--color-surface-600);
  }

  .progress-message.error {
    color: var(--color-error-600);
  }

  .time-remaining {
    margin-left: 1rem;
  }

  .time-text {
    margin: 0;
    font-size: 0.75rem;
    color: var(--color-surface-500);
  }

  :global(.spinning) {
    animation: spin 1s linear infinite;
  }

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

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .upload-progress {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .file-name {
      color: var(--color-surface-100);
    }

    .status-text {
      color: var(--color-surface-300);
    }

    .progress-percentage {
      color: var(--color-surface-200);
    }

    .progress-bar-container {
      background-color: var(--color-surface-700);
    }

    .progress-message {
      color: var(--color-surface-300);
    }

    .progress-message.error {
      color: var(--color-error-400);
    }

    .time-text {
      color: var(--color-surface-400);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .upload-progress {
      padding: 0.75rem;
    }

    .progress-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .progress-info {
      width: 100%;
    }

    .progress-percentage {
      margin-left: 0;
      align-self: flex-end;
    }

    .progress-footer {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }

    .time-remaining {
      margin-left: 0;
      align-self: flex-end;
    }
  }
</style>