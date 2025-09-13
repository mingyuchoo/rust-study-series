<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { ErrorContext, ErrorRecovery as ErrorRecoveryType } from '../types/errors.js';
  import { 
    RefreshCw, 
    Home, 
    Mail, 
    X, 
    AlertTriangle, 
    Info, 
    ExternalLink,
    Clock
  } from 'lucide-svelte';

  export let errorContext: ErrorContext;
  export let showTechnicalDetails = false;
  export let compact = false;

  const dispatch = createEventDispatcher<{
    recover: { action: string };
    dismiss: void;
  }>();

  function getIconForAction(action: string) {
    switch (action) {
      case 'retry':
        return RefreshCw;
      case 'refresh':
        return RefreshCw;
      case 'navigate_home':
        return Home;
      case 'contact_support':
        return Mail;
      case 'dismiss':
        return X;
      default:
        return Info;
    }
  }

  function getActionButtonClass(action: string): string {
    switch (action) {
      case 'retry':
        return 'btn-primary';
      case 'refresh':
        return 'btn-primary';
      case 'navigate_home':
        return 'btn-secondary';
      case 'contact_support':
        return 'btn-outline';
      case 'dismiss':
        return 'btn-ghost';
      default:
        return 'btn-outline';
    }
  }

  async function handleRecoveryAction(recovery: ErrorRecoveryType): Promise<void> {
    try {
      await recovery.handler();
      dispatch('recover', { action: recovery.action });
    } catch (error) {
      console.error('Recovery action failed:', error);
      // Could show a toast here about recovery failure
    }
  }

  function handleDismiss(): void {
    dispatch('dismiss');
  }

  function formatTimestamp(timestamp: Date): string {
    return timestamp.toLocaleString();
  }

  function getSeverityClass(severity: string): string {
    switch (severity) {
      case 'critical':
        return 'severity-critical';
      case 'high':
        return 'severity-high';
      case 'medium':
        return 'severity-medium';
      case 'low':
        return 'severity-low';
      default:
        return 'severity-medium';
    }
  }

  function getSeverityIcon(severity: string) {
    switch (severity) {
      case 'critical':
      case 'high':
        return AlertTriangle;
      default:
        return Info;
    }
  }
</script>

<div class="error-recovery" class:compact>
  <div class="error-header">
    <div class="error-icon {getSeverityClass(errorContext.error.severity)}">
      <svelte:component this={getSeverityIcon(errorContext.error.severity)} size={compact ? 20 : 24} />
    </div>
    
    <div class="error-info">
      <h3 class="error-title">
        {#if errorContext.error.severity === 'critical'}
          Critical Error
        {:else if errorContext.error.severity === 'high'}
          Error Occurred
        {:else}
          Something went wrong
        {/if}
      </h3>
      
      <p class="error-message">
        {errorContext.userMessage}
      </p>
      
      {#if !compact}
        <div class="error-meta">
          <span class="error-type">{errorContext.error.type.replace('_', ' ')}</span>
          <span class="error-time">
            <Clock size={12} />
            {formatTimestamp(errorContext.error.timestamp)}
          </span>
        </div>
      {/if}
    </div>

    {#if !compact}
      <button class="dismiss-button" on:click={handleDismiss} aria-label="Dismiss error">
        <X size={16} />
      </button>
    {/if}
  </div>

  {#if errorContext.recoveryOptions.length > 0}
    <div class="recovery-actions">
      {#each errorContext.recoveryOptions as recovery}
        <button
          class="recovery-button {getActionButtonClass(recovery.action)}"
          on:click={() => handleRecoveryAction(recovery)}
        >
          <svelte:component this={getIconForAction(recovery.action)} size={16} />
          {recovery.label}
          {#if recovery.action === 'contact_support'}
            <ExternalLink size={12} />
          {/if}
        </button>
      {/each}
    </div>
  {/if}

  {#if showTechnicalDetails && errorContext.technicalMessage}
    <details class="technical-details">
      <summary>Technical Details</summary>
      <div class="technical-content">
        <div class="detail-item">
          <strong>Error Type:</strong> {errorContext.error.type}
        </div>
        <div class="detail-item">
          <strong>Severity:</strong> {errorContext.error.severity}
        </div>
        <div class="detail-item">
          <strong>Retryable:</strong> {errorContext.error.retryable ? 'Yes' : 'No'}
        </div>
        <div class="detail-item">
          <strong>Timestamp:</strong> {errorContext.error.timestamp.toISOString()}
        </div>
        {#if errorContext.technicalMessage !== errorContext.userMessage}
          <div class="detail-item">
            <strong>Technical Message:</strong> {errorContext.technicalMessage}
          </div>
        {/if}
        {#if errorContext.error.details}
          <div class="detail-item">
            <strong>Details:</strong>
            <pre class="error-details-json">{JSON.stringify(errorContext.error.details, null, 2)}</pre>
          </div>
        {/if}
      </div>
    </details>
  {/if}
</div>

<style>
  .error-recovery {
    background: white;
    border-radius: 0.75rem;
    border: 1px solid var(--color-surface-200);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    overflow: hidden;
  }

  .error-recovery.compact {
    border-radius: 0.5rem;
    box-shadow: 0 2px 4px -1px rgba(0, 0, 0, 0.1);
  }

  .error-header {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.5rem;
  }

  .error-recovery.compact .error-header {
    padding: 1rem;
    gap: 0.75rem;
  }

  .error-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
  }

  .error-recovery.compact .error-icon {
    width: 2rem;
    height: 2rem;
  }

  .severity-critical {
    background-color: var(--color-error-100);
    color: var(--color-error-600);
  }

  .severity-high {
    background-color: var(--color-error-100);
    color: var(--color-error-600);
  }

  .severity-medium {
    background-color: var(--color-warning-100);
    color: var(--color-warning-600);
  }

  .severity-low {
    background-color: var(--color-info-100);
    color: var(--color-info-600);
  }

  .error-info {
    flex: 1;
    min-width: 0;
  }

  .error-title {
    margin: 0 0 0.5rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .error-recovery.compact .error-title {
    font-size: 1rem;
    margin-bottom: 0.25rem;
  }

  .error-message {
    margin: 0 0 0.75rem 0;
    color: var(--color-surface-700);
    line-height: 1.5;
  }

  .error-recovery.compact .error-message {
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
  }

  .error-meta {
    display: flex;
    align-items: center;
    gap: 1rem;
    font-size: 0.75rem;
    color: var(--color-surface-500);
  }

  .error-type {
    text-transform: capitalize;
    background: var(--color-surface-100);
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-weight: 500;
  }

  .error-time {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .dismiss-button {
    flex-shrink: 0;
    background: none;
    border: none;
    color: var(--color-surface-400);
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
  }

  .dismiss-button:hover {
    background: var(--color-surface-100);
    color: var(--color-surface-600);
  }

  .recovery-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    padding: 0 1.5rem 1.5rem 1.5rem;
    border-top: 1px solid var(--color-surface-100);
    margin-top: 0;
    padding-top: 1rem;
  }

  .error-recovery.compact .recovery-actions {
    padding: 0 1rem 1rem 1rem;
    gap: 0.5rem;
  }

  .recovery-button {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid transparent;
    text-decoration: none;
  }

  .error-recovery.compact .recovery-button {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
  }

  .btn-primary {
    background: var(--color-primary-600);
    color: white;
  }

  .btn-primary:hover {
    background: var(--color-primary-700);
  }

  .btn-secondary {
    background: var(--color-surface-600);
    color: white;
  }

  .btn-secondary:hover {
    background: var(--color-surface-700);
  }

  .btn-outline {
    background: transparent;
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .btn-outline:hover {
    background: var(--color-surface-50);
    border-color: var(--color-surface-400);
  }

  .btn-ghost {
    background: transparent;
    color: var(--color-surface-500);
  }

  .btn-ghost:hover {
    background: var(--color-surface-100);
    color: var(--color-surface-700);
  }

  .technical-details {
    border-top: 1px solid var(--color-surface-200);
    background: var(--color-surface-50);
  }

  .technical-details summary {
    padding: 1rem 1.5rem;
    cursor: pointer;
    font-weight: 500;
    color: var(--color-surface-700);
    transition: background-color 0.2s ease;
  }

  .technical-details summary:hover {
    background: var(--color-surface-100);
  }

  .technical-details[open] summary {
    border-bottom: 1px solid var(--color-surface-200);
  }

  .technical-content {
    padding: 1rem 1.5rem;
  }

  .detail-item {
    margin-bottom: 0.75rem;
    font-size: 0.875rem;
  }

  .detail-item:last-child {
    margin-bottom: 0;
  }

  .detail-item strong {
    color: var(--color-surface-900);
    display: inline-block;
    min-width: 120px;
  }

  .error-details-json {
    margin-top: 0.5rem;
    padding: 0.75rem;
    background: var(--color-surface-900);
    color: var(--color-surface-100);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    line-height: 1.4;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-all;
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .error-header {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }

    .error-recovery.compact .error-header {
      flex-direction: row;
      align-items: flex-start;
    }

    .recovery-actions {
      flex-direction: column;
      align-items: stretch;
    }

    .recovery-button {
      justify-content: center;
    }

    .error-meta {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .technical-content {
      padding: 1rem;
    }

    .detail-item strong {
      display: block;
      margin-bottom: 0.25rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .error-recovery {
      background: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .error-title {
      color: var(--color-surface-100);
    }

    .error-message {
      color: var(--color-surface-300);
    }

    .error-type {
      background: var(--color-surface-700);
      color: var(--color-surface-300);
    }

    .error-time {
      color: var(--color-surface-400);
    }

    .dismiss-button {
      color: var(--color-surface-500);
    }

    .dismiss-button:hover {
      background: var(--color-surface-700);
      color: var(--color-surface-300);
    }

    .recovery-actions {
      border-color: var(--color-surface-700);
    }

    .btn-outline {
      color: var(--color-surface-300);
      border-color: var(--color-surface-600);
    }

    .btn-outline:hover {
      background: var(--color-surface-700);
      border-color: var(--color-surface-500);
    }

    .btn-ghost {
      color: var(--color-surface-400);
    }

    .btn-ghost:hover {
      background: var(--color-surface-700);
      color: var(--color-surface-200);
    }

    .technical-details {
      background: var(--color-surface-900);
      border-color: var(--color-surface-700);
    }

    .technical-details summary {
      color: var(--color-surface-300);
    }

    .technical-details summary:hover {
      background: var(--color-surface-800);
    }

    .technical-details[open] summary {
      border-color: var(--color-surface-700);
    }

    .detail-item strong {
      color: var(--color-surface-100);
    }
  }

  /* Focus styles for accessibility */
  .recovery-button:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .dismiss-button:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .technical-details summary:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }
</style>