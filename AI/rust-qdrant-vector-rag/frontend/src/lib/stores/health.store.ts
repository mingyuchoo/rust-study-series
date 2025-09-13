/**
 * Health State Store
 * Manages system health status and monitoring
 */

import { writable, derived } from 'svelte/store';
import type { HealthState } from '../types/state.js';
import type { HealthResponse } from '../types/api.js';

// Initial state
const initialHealthState: HealthState = {
  status: null,
  lastChecked: null,
  isChecking: false,
  checkInterval: 30000 // 30 seconds default
};

// Create the writable store
export const healthStore = writable<HealthState>(initialHealthState);

// Derived stores for specific state slices
export const healthStatus = derived(healthStore, ($health) => $health.status);
export const isHealthChecking = derived(healthStore, ($health) => $health.isChecking);
export const lastHealthCheck = derived(healthStore, ($health) => $health.lastChecked);
export const healthCheckInterval = derived(healthStore, ($health) => $health.checkInterval);

// Derived computed values
export const isSystemHealthy = derived(healthStore, ($health) => 
  $health.status?.status === 'healthy'
);

export const isSystemUnhealthy = derived(healthStore, ($health) => 
  $health.status?.status === 'unhealthy'
);

export const systemUptime = derived(healthStore, ($health) => 
  $health.status?.uptime_seconds || 0
);

export const serviceStatuses = derived(healthStore, ($health) => 
  $health.status?.services || { qdrant: false, azure_openai: false }
);

export const isQdrantHealthy = derived(healthStore, ($health) => 
  $health.status?.services?.qdrant === true
);

export const isOpenAIHealthy = derived(healthStore, ($health) => 
  $health.status?.services?.azure_openai === true
);

export const timeSinceLastCheck = derived(healthStore, ($health) => {
  if (!$health.lastChecked) return null;
  return Date.now() - $health.lastChecked.getTime();
});

export const isHealthDataStale = derived(timeSinceLastCheck, ($timeSince) => {
  if ($timeSince === null) return true;
  return $timeSince > 60000; // Consider stale after 1 minute
});

// Store actions
export const healthActions = {
  startCheck: () => {
    healthStore.update(state => ({ 
      ...state, 
      isChecking: true
    }));
  },

  setStatus: (status: HealthResponse) => {
    healthStore.update(state => ({ 
      ...state, 
      status,
      isChecking: false,
      lastChecked: new Date()
    }));
  },

  failCheck: (_error: string) => {
    // Create a failed health response
    const failedStatus: HealthResponse = {
      status: 'unhealthy',
      timestamp: new Date().toISOString(),
      services: {
        qdrant: false,
        azure_openai: false
      },
      uptime_seconds: 0
    };

    healthStore.update(state => ({ 
      ...state, 
      status: failedStatus,
      isChecking: false,
      lastChecked: new Date()
    }));
  },

  setCheckInterval: (interval: number) => {
    healthStore.update(state => ({ 
      ...state, 
      checkInterval: Math.max(5000, interval) // Minimum 5 seconds
    }));
  },

  clearStatus: () => {
    healthStore.update(state => ({ 
      ...state, 
      status: null,
      lastChecked: null
    }));
  },

  reset: () => {
    healthStore.set(initialHealthState);
  }
};

// Auto-refresh functionality
let healthCheckTimer: NodeJS.Timeout | null = null;

export const healthAutoRefresh = {
  start: (checkFunction: () => Promise<void>) => {
    healthAutoRefresh.stop(); // Clear any existing timer
    
    const runCheck = async () => {
      try {
        await checkFunction();
      } catch (error) {
        console.error('Health check failed:', error);
      }
    };

    // Initial check
    runCheck();

    // Set up recurring checks
    healthStore.subscribe(($health) => {
      if (healthCheckTimer) {
        clearInterval(healthCheckTimer);
      }
      
      healthCheckTimer = setInterval(runCheck, $health.checkInterval);
    });
  },

  stop: () => {
    if (healthCheckTimer) {
      clearInterval(healthCheckTimer);
      healthCheckTimer = null;
    }
  }
};