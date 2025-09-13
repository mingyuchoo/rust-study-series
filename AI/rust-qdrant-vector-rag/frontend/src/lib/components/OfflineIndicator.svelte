<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { isOnline } from '../stores/app.store.js';
  import { errorHandler } from '../services/error-handler.js';
  import { WifiOff, Wifi, AlertTriangle } from 'lucide-svelte';

  let showOfflineMessage = false;
  let connectionStatus: 'online' | 'offline' | 'checking' = 'online';
  let lastOnlineTime: Date | null = null;
  let reconnectAttempts = 0;
  let maxReconnectAttempts = 5;
  let reconnectInterval: number | null = null;

  // Network status listener cleanup function
  let networkListenerCleanup: (() => void) | null = null;

  function formatLastOnlineTime(time: Date): string {
    const now = new Date();
    const diffMs = now.getTime() - time.getTime();
    const diffMinutes = Math.floor(diffMs / (1000 * 60));
    
    if (diffMinutes < 1) {
      return 'just now';
    } else if (diffMinutes < 60) {
      return `${diffMinutes} minute${diffMinutes === 1 ? '' : 's'} ago`;
    } else {
      const diffHours = Math.floor(diffMinutes / 60);
      return `${diffHours} hour${diffHours === 1 ? '' : 's'} ago`;
    }
  }

  async function checkConnection(): Promise<void> {
    connectionStatus = 'checking';
    
    try {
      const isConnected = await errorHandler.checkConnectivity();
      
      if (isConnected) {
        connectionStatus = 'online';
        showOfflineMessage = false;
        reconnectAttempts = 0;
        
        if (reconnectInterval) {
          clearInterval(reconnectInterval);
          reconnectInterval = null;
        }
      } else {
        connectionStatus = 'offline';
        showOfflineMessage = true;
        
        if (!lastOnlineTime) {
          lastOnlineTime = new Date();
        }
      }
    } catch (error) {
      connectionStatus = 'offline';
      showOfflineMessage = true;
      console.error('Connection check failed:', error);
    }
  }

  function startReconnectAttempts(): void {
    if (reconnectInterval || reconnectAttempts >= maxReconnectAttempts) {
      return;
    }

    reconnectInterval = window.setInterval(async () => {
      reconnectAttempts++;
      
      if (reconnectAttempts >= maxReconnectAttempts) {
        if (reconnectInterval) {
          clearInterval(reconnectInterval);
          reconnectInterval = null;
        }
        return;
      }

      await checkConnection();
    }, 5000); // Check every 5 seconds
  }

  function handleRetryConnection(): void {
    reconnectAttempts = 0;
    checkConnection();
    startReconnectAttempts();
  }

  onMount(() => {
    // Set initial connection status
    connectionStatus = $isOnline ? 'online' : 'offline';
    showOfflineMessage = !$isOnline;

    if (!$isOnline) {
      lastOnlineTime = new Date();
      startReconnectAttempts();
    }

    // Listen for network status changes
    networkListenerCleanup = errorHandler.addNetworkListener((online) => {
      if (online) {
        connectionStatus = 'online';
        showOfflineMessage = false;
        lastOnlineTime = null;
        reconnectAttempts = 0;
        
        if (reconnectInterval) {
          clearInterval(reconnectInterval);
          reconnectInterval = null;
        }
      } else {
        connectionStatus = 'offline';
        showOfflineMessage = true;
        lastOnlineTime = new Date();
        startReconnectAttempts();
      }
    });
  });

  onDestroy(() => {
    if (networkListenerCleanup) {
      networkListenerCleanup();
    }
    
    if (reconnectInterval) {
      clearInterval(reconnectInterval);
    }
  });
</script>

{#if showOfflineMessage}
  <div class="offline-indicator" class:checking={connectionStatus === 'checking'}>
    <div class="offline-content">
      <div class="offline-icon">
        {#if connectionStatus === 'checking'}
          <div class="spinner">
            <Wifi size={20} />
          </div>
        {:else}
          <WifiOff size={20} />
        {/if}
      </div>
      
      <div class="offline-message">
        {#if connectionStatus === 'checking'}
          <span class="status-text">Checking connection...</span>
        {:else}
          <span class="status-text">You're offline</span>
          {#if lastOnlineTime}
            <span class="last-online">Last online {formatLastOnlineTime(lastOnlineTime)}</span>
          {/if}
        {/if}
      </div>

      {#if connectionStatus === 'offline'}
        <div class="offline-actions">
          <button 
            class="retry-button" 
            on:click={handleRetryConnection}
            disabled={reconnectAttempts >= maxReconnectAttempts}
          >
            {#if reconnectAttempts >= maxReconnectAttempts}
              Max retries reached
            {:else}
              Try again
            {/if}
          </button>
          
          {#if reconnectAttempts > 0}
            <span class="retry-count">
              Attempt {reconnectAttempts}/{maxReconnectAttempts}
            </span>
          {/if}
        </div>
      {/if}
    </div>

    {#if connectionStatus === 'offline'}
      <div class="offline-warning">
        <AlertTriangle size={16} />
        <span>Some features may not work while offline</span>
      </div>
    {/if}
  </div>
{/if}

<style>
  .offline-indicator {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    background: linear-gradient(135deg, #ef4444, #dc2626);
    color: white;
    padding: 0.75rem 1rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    animation: slideDown 0.3s ease-out;
  }

  .offline-indicator.checking {
    background: linear-gradient(135deg, #f59e0b, #d97706);
  }

  .offline-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .offline-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  .offline-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .status-text {
    font-weight: 600;
    font-size: 0.875rem;
  }

  .last-online {
    font-size: 0.75rem;
    opacity: 0.9;
  }

  .offline-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .retry-button {
    background: rgba(255, 255, 255, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.3);
    color: white;
    padding: 0.375rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .retry-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.3);
    border-color: rgba(255, 255, 255, 0.5);
  }

  .retry-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .retry-count {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  .offline-warning {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid rgba(255, 255, 255, 0.2);
    font-size: 0.75rem;
    opacity: 0.9;
  }

  @keyframes slideDown {
    from {
      transform: translateY(-100%);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .offline-indicator {
      padding: 0.5rem;
    }

    .offline-content {
      flex-direction: column;
      gap: 0.5rem;
    }

    .offline-message {
      text-align: center;
    }

    .offline-actions {
      flex-direction: column;
      gap: 0.25rem;
    }

    .offline-warning {
      margin-top: 0.25rem;
      padding-top: 0.25rem;
    }
  }

  /* Focus styles for accessibility */
  .retry-button:focus {
    outline: 2px solid rgba(255, 255, 255, 0.5);
    outline-offset: 2px;
  }
</style>