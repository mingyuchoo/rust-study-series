<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { HealthStatus, SystemMetrics } from '../components/index.js';
  import { healthActions, healthAutoRefresh } from '../stores/health.store.js';
  import { apiService } from '../services/api.js';
  import { toastActions } from '../stores/toast.store.js';

  // Dashboard configuration
  const HEALTH_CHECK_INTERVAL = 30000; // 30 seconds
  const AUTO_REFRESH_ENABLED = true;

  // Health check function for the dashboard
  const performHealthCheck = async () => {
    try {
      healthActions.startCheck();
      const healthData = await apiService.getHealth();
      healthActions.setStatus(healthData);
    } catch (error) {
      console.error('Dashboard health check failed:', error);
      healthActions.failCheck(error instanceof Error ? error.message : 'Unknown error');
      
      toastActions.error('Failed to fetch system health status', {
        duration: 5000
      });
    }
  };

  // Initialize dashboard
  onMount(() => {
    // Set up health monitoring
    if (AUTO_REFRESH_ENABLED) {
      healthActions.setCheckInterval(HEALTH_CHECK_INTERVAL);
      healthAutoRefresh.start(performHealthCheck);
    } else {
      // Perform initial health check
      performHealthCheck();
    }
  });

  // Cleanup on destroy
  onDestroy(() => {
    if (AUTO_REFRESH_ENABLED) {
      healthAutoRefresh.stop();
    }
  });
</script>

<div class="dashboard-page">
  <div class="dashboard-header">
    <div class="header-content">
      <h1>System Dashboard</h1>
      <p class="header-description">
        Monitor system health, service status, and performance metrics in real-time.
      </p>
    </div>
  </div>

  <div class="dashboard-content">
    <div class="dashboard-grid">
      <!-- System Health Status -->
      <div class="dashboard-section health-section">
        <HealthStatus 
          showDetails={true} 
          autoRefresh={AUTO_REFRESH_ENABLED}
          refreshInterval={HEALTH_CHECK_INTERVAL}
        />
      </div>

      <!-- System Metrics -->
      <div class="dashboard-section metrics-section">
        <SystemMetrics showDetailedMetrics={true} />
      </div>
    </div>
  </div>
</div>

<style>
  .dashboard-page {
    min-height: 100%;
    background: var(--color-surface-25);
    padding: 2rem;
  }

  .dashboard-header {
    margin-bottom: 2rem;
  }

  .header-content {
    max-width: 1200px;
    margin: 0 auto;
  }

  .dashboard-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-surface-900);
    line-height: 1.2;
  }

  .header-description {
    margin: 0;
    font-size: 1rem;
    color: var(--color-surface-600);
    line-height: 1.5;
  }

  .dashboard-content {
    max-width: 1200px;
    margin: 0 auto;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 2rem;
  }

  .dashboard-section {
    width: 100%;
  }

  .health-section {
    /* Health status gets priority positioning */
    order: 1;
  }

  .metrics-section {
    /* Metrics section comes second */
    order: 2;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .dashboard-page {
      background: var(--color-surface-900);
    }

    .dashboard-header h1 {
      color: var(--color-surface-100);
    }

    .header-description {
      color: var(--color-surface-400);
    }
  }

  /* Responsive design */
  @media (min-width: 1024px) {
    .dashboard-grid {
      grid-template-columns: 1fr;
      gap: 2.5rem;
    }
  }

  @media (max-width: 768px) {
    .dashboard-page {
      padding: 1rem;
    }

    .dashboard-header {
      margin-bottom: 1.5rem;
    }

    .dashboard-header h1 {
      font-size: 1.75rem;
    }

    .header-description {
      font-size: 0.875rem;
    }

    .dashboard-grid {
      gap: 1.5rem;
    }
  }

  @media (max-width: 480px) {
    .dashboard-page {
      padding: 0.75rem;
    }

    .dashboard-header h1 {
      font-size: 1.5rem;
    }

    .dashboard-grid {
      gap: 1rem;
    }
  }

  /* Print styles */
  @media print {
    .dashboard-page {
      background: white;
      padding: 1rem;
    }

    .dashboard-header h1 {
      color: black;
    }

    .header-description {
      color: #666;
    }
  }
</style>