<script lang="ts">
  import { 
    healthStatus, 
    systemUptime, 
    lastHealthCheck,
    isHealthDataStale 
  } from '../stores/health.store.js';

  // Props
  export let showDetailedMetrics = true;

  // Format uptime into human readable format
  const formatUptime = (seconds: number): { value: string; unit: string; detailed: string } => {
    if (seconds < 60) {
      return {
        value: seconds.toString(),
        unit: 'seconds',
        detailed: `${seconds} seconds`
      };
    }
    
    if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = seconds % 60;
      return {
        value: minutes.toString(),
        unit: 'minutes',
        detailed: `${minutes}m ${remainingSeconds}s`
      };
    }
    
    if (seconds < 86400) {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return {
        value: hours.toString(),
        unit: 'hours',
        detailed: `${hours}h ${minutes}m`
      };
    }
    
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    return {
      value: days.toString(),
      unit: 'days',
      detailed: `${days}d ${hours}h`
    };
  };

  // Calculate uptime percentage (assuming 99.9% is excellent)
  const calculateUptimePercentage = (uptimeSeconds: number): number => {
    // For demo purposes, we'll calculate based on a 30-day period
    const thirtyDaysInSeconds = 30 * 24 * 60 * 60;
    const percentage = Math.min((uptimeSeconds / thirtyDaysInSeconds) * 100, 99.99);
    return Math.round(percentage * 100) / 100;
  };

  // Get uptime status color
  const getUptimeStatusColor = (percentage: number): 'excellent' | 'good' | 'warning' | 'critical' => {
    if (percentage >= 99.9) return 'excellent';
    if (percentage >= 99.0) return 'good';
    if (percentage >= 95.0) return 'warning';
    return 'critical';
  };

  // Format timestamp
  const formatTimestamp = (timestamp: string): string => {
    return new Date(timestamp).toLocaleString();
  };

  // Calculate time since last check
  const getTimeSinceLastCheck = (lastCheck: Date | null): string => {
    if (!lastCheck) return 'Never';
    
    const now = new Date();
    const diff = now.getTime() - lastCheck.getTime();
    
    if (diff < 1000) return 'Just now';
    if (diff < 60000) return `${Math.floor(diff / 1000)}s ago`;
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return lastCheck.toLocaleDateString();
  };

  // Reactive calculations
  $: uptimeFormatted = $systemUptime ? formatUptime($systemUptime) : null;
  $: uptimePercentage = $systemUptime ? calculateUptimePercentage($systemUptime) : 0;
  $: uptimeStatus = getUptimeStatusColor(uptimePercentage);
  $: lastCheckFormatted = getTimeSinceLastCheck($lastHealthCheck);
</script>

<div class="system-metrics">
  <div class="metrics-header">
    <h3>System Metrics</h3>
    {#if $isHealthDataStale}
      <span class="stale-indicator" title="Health data may be outdated">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="8" x2="12" y2="12"/>
          <line x1="12" y1="16" x2="12.01" y2="16"/>
        </svg>
        Data may be stale
      </span>
    {/if}
  </div>

  {#if $healthStatus}
    <div class="metrics-grid">
      <!-- Primary Uptime Metric -->
      <div class="metric-card primary" class:excellent={uptimeStatus === 'excellent'} 
           class:good={uptimeStatus === 'good'} class:warning={uptimeStatus === 'warning'} 
           class:critical={uptimeStatus === 'critical'}>
        <div class="metric-header">
          <div class="metric-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="22,12 18,12 15,21 9,3 6,12 2,12"/>
            </svg>
          </div>
          <div class="metric-info">
            <h4>System Uptime</h4>
            <div class="metric-value">
              {#if uptimeFormatted}
                <span class="value">{uptimeFormatted.value}</span>
                <span class="unit">{uptimeFormatted.unit}</span>
              {:else}
                <span class="value">--</span>
              {/if}
            </div>
          </div>
        </div>
        
        {#if showDetailedMetrics && uptimeFormatted}
          <div class="metric-details">
            <div class="detail-item">
              <span class="detail-label">Detailed:</span>
              <span class="detail-value">{uptimeFormatted.detailed}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Availability:</span>
              <span class="detail-value availability" class:excellent={uptimeStatus === 'excellent'} 
                    class:good={uptimeStatus === 'good'} class:warning={uptimeStatus === 'warning'} 
                    class:critical={uptimeStatus === 'critical'}>
                {uptimePercentage}%
              </span>
            </div>
          </div>
        {/if}
      </div>

      <!-- Last Health Check -->
      <div class="metric-card">
        <div class="metric-header">
          <div class="metric-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <polyline points="12,6 12,12 16,14"/>
            </svg>
          </div>
          <div class="metric-info">
            <h4>Last Health Check</h4>
            <div class="metric-value">
              <span class="value small">{lastCheckFormatted}</span>
            </div>
          </div>
        </div>
        
        {#if showDetailedMetrics}
          <div class="metric-details">
            <div class="detail-item">
              <span class="detail-label">Timestamp:</span>
              <span class="detail-value">{formatTimestamp($healthStatus.timestamp)}</span>
            </div>
          </div>
        {/if}
      </div>

      <!-- Service Status Summary -->
      <div class="metric-card">
        <div class="metric-header">
          <div class="metric-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/>
              <path d="M2 17l10 5 10-5"/>
              <path d="M2 12l10 5 10-5"/>
            </svg>
          </div>
          <div class="metric-info">
            <h4>Services</h4>
            <div class="metric-value">
              <span class="value">
                {Object.values($healthStatus.services).filter(Boolean).length}
              </span>
              <span class="unit">of {Object.keys($healthStatus.services).length} online</span>
            </div>
          </div>
        </div>
        
        {#if showDetailedMetrics}
          <div class="metric-details">
            <div class="services-status">
              {#each Object.entries($healthStatus.services) as [serviceName, isHealthy]}
                <div class="service-status-item">
                  <div class="service-dot" class:healthy={isHealthy} class:unhealthy={!isHealthy}></div>
                  <span class="service-name">{serviceName.replace('_', ' ')}</span>
                  <span class="service-state" class:healthy={isHealthy} class:unhealthy={!isHealthy}>
                    {isHealthy ? 'Online' : 'Offline'}
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Overall System Status -->
      <div class="metric-card status-card" class:healthy={$healthStatus.status === 'healthy'} 
           class:unhealthy={$healthStatus.status === 'unhealthy'}>
        <div class="metric-header">
          <div class="metric-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              {#if $healthStatus.status === 'healthy'}
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                <polyline points="22,4 12,14.01 9,11.01"/>
              {:else}
                <circle cx="12" cy="12" r="10"/>
                <line x1="15" y1="9" x2="9" y2="15"/>
                <line x1="9" y1="9" x2="15" y2="15"/>
              {/if}
            </svg>
          </div>
          <div class="metric-info">
            <h4>Overall Status</h4>
            <div class="metric-value">
              <span class="value status-text" class:healthy={$healthStatus.status === 'healthy'} 
                    class:unhealthy={$healthStatus.status === 'unhealthy'}>
                {$healthStatus.status === 'healthy' ? 'Healthy' : 'Unhealthy'}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="no-data">
      <div class="no-data-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 6v6l4 2"/>
        </svg>
      </div>
      <p>No system metrics available</p>
      <span class="no-data-subtitle">Health check data is required to display metrics</span>
    </div>
  {/if}
</div>

<style>
  .system-metrics {
    background: var(--color-surface-50);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .metrics-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .metrics-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .stale-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
    color: var(--color-warning-600);
    background: var(--color-warning-100);
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    border: 1px solid var(--color-warning-300);
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1rem;
  }

  .metric-card {
    background: var(--color-surface-100);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.5rem;
    padding: 1.25rem;
    transition: all 0.2s ease;
  }

  .metric-card.primary {
    grid-column: span 2;
  }

  .metric-card.excellent {
    border-color: var(--color-success-300);
    background: var(--color-success-50);
  }

  .metric-card.good {
    border-color: var(--color-success-400);
    background: var(--color-success-100);
  }

  .metric-card.warning {
    border-color: var(--color-warning-400);
    background: var(--color-warning-50);
  }

  .metric-card.critical {
    border-color: var(--color-error-400);
    background: var(--color-error-50);
  }

  .metric-card.healthy {
    border-color: var(--color-success-300);
    background: var(--color-success-50);
  }

  .metric-card.unhealthy {
    border-color: var(--color-error-300);
    background: var(--color-error-50);
  }

  .metric-header {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .metric-icon {
    flex-shrink: 0;
    color: var(--color-surface-600);
  }

  .metric-card.excellent .metric-icon,
  .metric-card.healthy .metric-icon {
    color: var(--color-success-600);
  }

  .metric-card.warning .metric-icon {
    color: var(--color-warning-600);
  }

  .metric-card.critical .metric-icon,
  .metric-card.unhealthy .metric-icon {
    color: var(--color-error-600);
  }

  .metric-info {
    flex: 1;
    min-width: 0;
  }

  .metric-info h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-700);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .metric-value {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
  }

  .value {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-surface-900);
    line-height: 1;
  }

  .value.small {
    font-size: 1.25rem;
  }

  .unit {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-surface-600);
  }

  .status-text.healthy {
    color: var(--color-success-700);
  }

  .status-text.unhealthy {
    color: var(--color-error-700);
  }

  .metric-details {
    border-top: 1px solid var(--color-surface-200);
    padding-top: 1rem;
    margin-top: 1rem;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .detail-item:last-child {
    margin-bottom: 0;
  }

  .detail-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-surface-600);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .detail-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .availability.excellent {
    color: var(--color-success-700);
  }

  .availability.good {
    color: var(--color-success-600);
  }

  .availability.warning {
    color: var(--color-warning-600);
  }

  .availability.critical {
    color: var(--color-error-600);
  }

  .services-status {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .service-status-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .service-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--color-surface-400);
  }

  .service-dot.healthy {
    background: var(--color-success-500);
  }

  .service-dot.unhealthy {
    background: var(--color-error-500);
  }

  .service-name {
    flex: 1;
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-surface-700);
    text-transform: capitalize;
  }

  .service-state {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .service-state.healthy {
    color: var(--color-success-600);
  }

  .service-state.unhealthy {
    color: var(--color-error-600);
  }

  .no-data {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 1rem;
    text-align: center;
  }

  .no-data-icon {
    margin-bottom: 1rem;
    color: var(--color-surface-400);
  }

  .no-data p {
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-surface-700);
  }

  .no-data-subtitle {
    font-size: 0.875rem;
    color: var(--color-surface-500);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .system-metrics {
      background: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .metrics-header h3 {
      color: var(--color-surface-100);
    }

    .stale-indicator {
      background: var(--color-warning-900);
      border-color: var(--color-warning-700);
      color: var(--color-warning-300);
    }

    .metric-card {
      background: var(--color-surface-700);
      border-color: var(--color-surface-600);
    }

    .metric-card.excellent {
      background: var(--color-success-900);
      border-color: var(--color-success-700);
    }

    .metric-card.good {
      background: var(--color-success-800);
      border-color: var(--color-success-600);
    }

    .metric-card.warning {
      background: var(--color-warning-900);
      border-color: var(--color-warning-700);
    }

    .metric-card.critical {
      background: var(--color-error-900);
      border-color: var(--color-error-700);
    }

    .metric-card.healthy {
      background: var(--color-success-900);
      border-color: var(--color-success-700);
    }

    .metric-card.unhealthy {
      background: var(--color-error-900);
      border-color: var(--color-error-700);
    }

    .metric-info h4 {
      color: var(--color-surface-300);
    }

    .value {
      color: var(--color-surface-100);
    }

    .unit {
      color: var(--color-surface-400);
    }

    .metric-details {
      border-color: var(--color-surface-600);
    }

    .detail-label {
      color: var(--color-surface-400);
    }

    .detail-value {
      color: var(--color-surface-100);
    }

    .service-name {
      color: var(--color-surface-300);
    }

    .no-data-icon {
      color: var(--color-surface-600);
    }

    .no-data p {
      color: var(--color-surface-300);
    }

    .no-data-subtitle {
      color: var(--color-surface-500);
    }
  }

  /* Responsive design */
  @media (max-width: 1024px) {
    .metric-card.primary {
      grid-column: span 1;
    }
  }

  @media (max-width: 768px) {
    .metrics-grid {
      grid-template-columns: 1fr;
    }

    .metrics-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .value {
      font-size: 1.5rem;
    }
  }
</style>