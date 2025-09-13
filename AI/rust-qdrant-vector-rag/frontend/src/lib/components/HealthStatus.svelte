<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { 
    healthStatus, 
    isSystemHealthy, 
    isSystemUnhealthy, 
    serviceStatuses, 
    isQdrantHealthy, 
    isOpenAIHealthy,
    lastHealthCheck,
    isHealthDataStale,
    healthActions,
    healthAutoRefresh
  } from '../stores/health.store.js';
  import { apiService } from '../services/api.js';
  import { toastActions } from '../stores/toast.store.js';
  import LoadingSpinner from './LoadingSpinner.svelte';

  // Props
  export let showDetails = true;
  export let autoRefresh = true;
  export let refreshInterval = 30000; // 30 seconds

  // Local state
  let mounted = false;

  // Health check function
  const performHealthCheck = async () => {
    try {
      healthActions.startCheck();
      const healthData = await apiService.getHealth();
      healthActions.setStatus(healthData);
    } catch (error) {
      console.error('Health check failed:', error);
      healthActions.failCheck(error instanceof Error ? error.message : 'Unknown error');
      
      if (mounted) {
        toastActions.error('Health check failed. System status may be outdated.', {
          duration: 5000
        });
      }
    }
  };

  // Manual refresh handler
  const handleRefresh = () => {
    performHealthCheck();
  };

  // Format uptime display
  const formatUptime = (seconds: number): string => {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
    if (seconds < 86400) {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return `${hours}h ${minutes}m`;
    }
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    return `${days}d ${hours}h`;
  };

  // Format last checked time
  const formatLastChecked = (date: Date | null): string => {
    if (!date) return 'Never';
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    
    if (diff < 60000) return 'Just now';
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return date.toLocaleDateString();
  };

  onMount(() => {
    mounted = true;
    
    if (autoRefresh) {
      healthActions.setCheckInterval(refreshInterval);
      healthAutoRefresh.start(performHealthCheck);
    } else {
      // Perform initial check if not auto-refreshing
      performHealthCheck();
    }
  });

  onDestroy(() => {
    mounted = false;
    if (autoRefresh) {
      healthAutoRefresh.stop();
    }
  });
</script>

<div class="health-status" class:healthy={$isSystemHealthy} class:unhealthy={$isSystemUnhealthy}>
  <div class="health-header">
    <div class="status-indicator">
      <div class="status-dot" class:healthy={$isSystemHealthy} class:unhealthy={$isSystemUnhealthy}></div>
      <h3 class="status-title">
        System Status: 
        <span class="status-text" class:healthy={$isSystemHealthy} class:unhealthy={$isSystemUnhealthy}>
          {$isSystemHealthy ? 'Healthy' : $isSystemUnhealthy ? 'Unhealthy' : 'Unknown'}
        </span>
      </h3>
    </div>
    
    <div class="health-actions">
      <button 
        class="refresh-btn" 
        on:click={handleRefresh}
        disabled={$healthStatus === null && !$isSystemHealthy && !$isSystemUnhealthy}
        title="Refresh health status"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
          <path d="M21 3v5h-5"/>
          <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/>
          <path d="M3 21v-5h5"/>
        </svg>
        Refresh
      </button>
    </div>
  </div>

  {#if $healthStatus === null}
    <div class="loading-state">
      <LoadingSpinner size="small" />
      <span>Checking system health...</span>
    </div>
  {:else}
    <div class="health-content">
      {#if showDetails}
        <div class="services-grid">
          <div class="service-item" class:healthy={$isQdrantHealthy} class:unhealthy={!$isQdrantHealthy}>
            <div class="service-indicator">
              <div class="service-dot" class:healthy={$isQdrantHealthy} class:unhealthy={!$isQdrantHealthy}></div>
            </div>
            <div class="service-info">
              <h4>Qdrant Vector Database</h4>
              <span class="service-status" class:healthy={$isQdrantHealthy} class:unhealthy={!$isQdrantHealthy}>
                {$isQdrantHealthy ? 'Connected' : 'Disconnected'}
              </span>
            </div>
          </div>

          <div class="service-item" class:healthy={$isOpenAIHealthy} class:unhealthy={!$isOpenAIHealthy}>
            <div class="service-indicator">
              <div class="service-dot" class:healthy={$isOpenAIHealthy} class:unhealthy={!$isOpenAIHealthy}></div>
            </div>
            <div class="service-info">
              <h4>Azure OpenAI</h4>
              <span class="service-status" class:healthy={$isOpenAIHealthy} class:unhealthy={!$isOpenAIHealthy}>
                {$isOpenAIHealthy ? 'Connected' : 'Disconnected'}
              </span>
            </div>
          </div>
        </div>

        <div class="health-metadata">
          <div class="metadata-item">
            <span class="metadata-label">System Uptime:</span>
            <span class="metadata-value">{formatUptime($healthStatus.uptime_seconds)}</span>
          </div>
          <div class="metadata-item">
            <span class="metadata-label">Last Checked:</span>
            <span class="metadata-value" class:stale={$isHealthDataStale}>
              {formatLastChecked($lastHealthCheck)}
            </span>
          </div>
          <div class="metadata-item">
            <span class="metadata-label">Response Time:</span>
            <span class="metadata-value">
              {new Date($healthStatus.timestamp).toLocaleTimeString()}
            </span>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .health-status {
    background: var(--color-surface-50);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.75rem;
    padding: 1.5rem;
    transition: all 0.2s ease;
  }

  .health-status.healthy {
    border-color: var(--color-success-300);
    background: var(--color-success-50);
  }

  .health-status.unhealthy {
    border-color: var(--color-error-300);
    background: var(--color-error-50);
  }

  .health-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .status-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--color-surface-400);
    transition: background-color 0.2s ease;
  }

  .status-dot.healthy {
    background: var(--color-success-500);
    box-shadow: 0 0 0 2px var(--color-success-200);
  }

  .status-dot.unhealthy {
    background: var(--color-error-500);
    box-shadow: 0 0 0 2px var(--color-error-200);
  }

  .status-title {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .status-text.healthy {
    color: var(--color-success-700);
  }

  .status-text.unhealthy {
    color: var(--color-error-700);
  }

  .health-actions {
    display: flex;
    gap: 0.5rem;
  }

  .refresh-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--color-surface-100);
    border: 1px solid var(--color-surface-300);
    border-radius: 0.5rem;
    color: var(--color-surface-700);
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .refresh-btn:hover:not(:disabled) {
    background: var(--color-surface-200);
    border-color: var(--color-surface-400);
  }

  .refresh-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-state {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 0;
    color: var(--color-surface-600);
  }

  .health-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .services-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .service-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--color-surface-100);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.5rem;
    transition: all 0.2s ease;
  }

  .service-item.healthy {
    background: var(--color-success-100);
    border-color: var(--color-success-300);
  }

  .service-item.unhealthy {
    background: var(--color-error-100);
    border-color: var(--color-error-300);
  }

  .service-indicator {
    flex-shrink: 0;
  }

  .service-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--color-surface-400);
    transition: background-color 0.2s ease;
  }

  .service-dot.healthy {
    background: var(--color-success-500);
  }

  .service-dot.unhealthy {
    background: var(--color-error-500);
  }

  .service-info {
    flex: 1;
    min-width: 0;
  }

  .service-info h4 {
    margin: 0 0 0.25rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .service-status {
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .service-status.healthy {
    color: var(--color-success-700);
  }

  .service-status.unhealthy {
    color: var(--color-error-700);
  }

  .health-metadata {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    padding: 1rem;
    background: var(--color-surface-100);
    border-radius: 0.5rem;
  }

  .metadata-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .metadata-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-surface-600);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .metadata-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .metadata-value.stale {
    color: var(--color-warning-600);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .health-status {
      background: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .health-status.healthy {
      background: var(--color-success-900);
      border-color: var(--color-success-700);
    }

    .health-status.unhealthy {
      background: var(--color-error-900);
      border-color: var(--color-error-700);
    }

    .status-title {
      color: var(--color-surface-100);
    }

    .refresh-btn {
      background: var(--color-surface-700);
      border-color: var(--color-surface-600);
      color: var(--color-surface-200);
    }

    .refresh-btn:hover:not(:disabled) {
      background: var(--color-surface-600);
      border-color: var(--color-surface-500);
    }

    .loading-state {
      color: var(--color-surface-400);
    }

    .service-item {
      background: var(--color-surface-700);
      border-color: var(--color-surface-600);
    }

    .service-item.healthy {
      background: var(--color-success-800);
      border-color: var(--color-success-600);
    }

    .service-item.unhealthy {
      background: var(--color-error-800);
      border-color: var(--color-error-600);
    }

    .service-info h4 {
      color: var(--color-surface-100);
    }

    .health-metadata {
      background: var(--color-surface-700);
    }

    .metadata-label {
      color: var(--color-surface-400);
    }

    .metadata-value {
      color: var(--color-surface-100);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .health-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .services-grid {
      grid-template-columns: 1fr;
    }

    .health-metadata {
      grid-template-columns: 1fr;
    }
  }
</style>