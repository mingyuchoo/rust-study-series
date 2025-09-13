<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { CheckCircle, XCircle, RefreshCw, FileText, Clock, Hash } from 'lucide-svelte';
  import type { UploadResponse } from '../types/api.js';

  // Props
  export let result: UploadResponse;
  export let showRetryButton: boolean = true;
  export let showNewUploadButton: boolean = true;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    retry: void;
    newUpload: void;
    dismiss: void;
  }>();

  // Reactive computations
  $: isSuccess = result.status === 'success';
  $: isFailure = result.status === 'failure';

  // Format timestamp
  function formatTimestamp(timestamp: string): string {
    try {
      const date = new Date(timestamp);
      return date.toLocaleString();
    } catch {
      return timestamp;
    }
  }

  // Format processing time
  function formatProcessingTime(ms: number): string {
    if (ms < 1000) {
      return `${ms}ms`;
    } else if (ms < 60000) {
      return `${(ms / 1000).toFixed(1)}s`;
    } else {
      const minutes = Math.floor(ms / 60000);
      const seconds = Math.floor((ms % 60000) / 1000);
      return `${minutes}m ${seconds}s`;
    }
  }

  // Handle retry
  function handleRetry() {
    dispatch('retry');
  }

  // Handle new upload
  function handleNewUpload() {
    dispatch('newUpload');
  }

  // Handle dismiss
  function handleDismiss() {
    dispatch('dismiss');
  }
</script>

<div class="upload-result {isSuccess ? 'success' : 'error'}">
  <!-- Header with status -->
  <div class="result-header">
    <div class="status-info">
      {#if isSuccess}
        <CheckCircle size={24} color="var(--color-success-600)" />
        <h3 class="status-title success">Upload Successful</h3>
      {:else}
        <XCircle size={24} color="var(--color-error-600)" />
        <h3 class="status-title error">Upload Failed</h3>
      {/if}
    </div>
    
    <div class="status-badge {isSuccess ? 'success' : 'error'}">
      {result.status}
    </div>
  </div>

  <!-- Message -->
  <p class="result-message" class:error={!isSuccess}>
    {result.message}
  </p>

  {#if isSuccess}
    <div class="divider"></div>
    
    <!-- Success details -->
    <div class="details-section">
      <div class="section-header">
        <FileText size={16} color="var(--color-surface-600)" />
        <h4 class="section-title">File Details</h4>
      </div>
      
      <div class="detail-grid">
        <div class="detail-item">
          <span class="detail-label">Filename</span>
          <span class="detail-value">{result.filename}</span>
        </div>
        
        <div class="detail-item">
          <span class="detail-label">Document ID</span>
          <span class="detail-value monospace">{result.document_id}</span>
        </div>
      </div>
    </div>

    <div class="details-section">
      <div class="section-header">
        <Hash size={16} color="var(--color-surface-600)" />
        <h4 class="section-title">Processing Results</h4>
      </div>
      
      <div class="detail-grid">
        <div class="detail-item">
          <span class="detail-label">Chunks Created</span>
          <span class="detail-value">{result.chunks_created.toLocaleString()}</span>
        </div>
        
        <div class="detail-item">
          <span class="detail-label">Processing Time</span>
          <span class="detail-value">{formatProcessingTime(result.processing_time_ms)}</span>
        </div>
      </div>
    </div>

    <div class="details-section">
      <div class="section-header">
        <Clock size={16} color="var(--color-surface-600)" />
        <h4 class="section-title">Upload Time</h4>
      </div>
      
      <p class="timestamp">
        {formatTimestamp(result.timestamp)}
      </p>
    </div>
  {/if}

  <!-- Action buttons -->
  <div class="action-buttons">
    {#if isFailure && showRetryButton}
      <button
        class="btn btn-primary btn-sm"
        on:click={handleRetry}
      >
        <RefreshCw size={16} />
        Retry Upload
      </button>
    {/if}
    
    {#if showNewUploadButton}
      <button
        class="btn {isSuccess ? 'btn-success' : 'btn-secondary'} btn-sm"
        on:click={handleNewUpload}
      >
        Upload Another File
      </button>
    {/if}
    
    <button
      class="btn btn-ghost btn-sm"
      on:click={handleDismiss}
    >
      Dismiss
    </button>
  </div>
</div>

<style>
  .upload-result {
    width: 100%;
    background-color: var(--color-surface-50);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.5rem;
    padding: 1.5rem;
  }

  .upload-result.success {
    border-color: var(--color-success-300);
    background-color: var(--color-success-50);
  }

  .upload-result.error {
    border-color: var(--color-error-300);
    background-color: var(--color-error-50);
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .status-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .status-title {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .status-title.success {
    color: var(--color-success-700);
  }

  .status-title.error {
    color: var(--color-error-700);
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
  }

  .status-badge.success {
    background-color: var(--color-success-100);
    color: var(--color-success-800);
  }

  .status-badge.error {
    background-color: var(--color-error-100);
    color: var(--color-error-800);
  }

  .result-message {
    margin: 0 0 1rem 0;
    font-size: 0.875rem;
    color: var(--color-surface-600);
  }

  .result-message.error {
    color: var(--color-error-600);
  }

  .divider {
    height: 1px;
    background-color: var(--color-surface-200);
    margin: 1rem 0;
  }

  .details-section {
    margin-bottom: 1.5rem;
  }

  .details-section:last-of-type {
    margin-bottom: 1rem;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .section-title {
    margin: 0;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-surface-700);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
    margin-left: 1.5rem;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .detail-label {
    font-size: 0.75rem;
    color: var(--color-surface-500);
    font-weight: 500;
  }

  .detail-value {
    font-size: 0.875rem;
    color: var(--color-surface-900);
    word-break: break-all;
  }

  .detail-value.monospace {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.8125rem;
  }

  .timestamp {
    margin: 0;
    margin-left: 1.5rem;
    font-size: 0.875rem;
    color: var(--color-surface-600);
  }

  .action-buttons {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    flex-wrap: wrap;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border: 1px solid transparent;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    text-decoration: none;
    background: none;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
  }

  .btn-primary {
    background-color: var(--color-primary-600);
    color: white;
    border-color: var(--color-primary-600);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-primary-700);
    border-color: var(--color-primary-700);
  }

  .btn-success {
    background-color: var(--color-success-600);
    color: white;
    border-color: var(--color-success-600);
  }

  .btn-success:hover:not(:disabled) {
    background-color: var(--color-success-700);
    border-color: var(--color-success-700);
  }

  .btn-secondary {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
  }

  .btn-ghost {
    background-color: transparent;
    color: var(--color-surface-600);
    border-color: transparent;
  }

  .btn-ghost:hover:not(:disabled) {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
  }

  .btn:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .upload-result {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .upload-result.success {
      border-color: var(--color-success-700);
      background-color: var(--color-surface-800);
    }

    .upload-result.error {
      border-color: var(--color-error-700);
      background-color: var(--color-surface-800);
    }

    .status-title.success {
      color: var(--color-success-400);
    }

    .status-title.error {
      color: var(--color-error-400);
    }

    .status-badge.success {
      background-color: var(--color-success-900);
      color: var(--color-success-200);
    }

    .status-badge.error {
      background-color: var(--color-error-900);
      color: var(--color-error-200);
    }

    .result-message {
      color: var(--color-surface-300);
    }

    .result-message.error {
      color: var(--color-error-400);
    }

    .divider {
      background-color: var(--color-surface-600);
    }

    .section-title {
      color: var(--color-surface-200);
    }

    .detail-label {
      color: var(--color-surface-400);
    }

    .detail-value {
      color: var(--color-surface-100);
    }

    .timestamp {
      color: var(--color-surface-300);
    }

    .btn-secondary {
      background-color: var(--color-surface-700);
      color: var(--color-surface-200);
      border-color: var(--color-surface-600);
    }

    .btn-secondary:hover:not(:disabled) {
      background-color: var(--color-surface-600);
      border-color: var(--color-surface-500);
    }

    .btn-ghost {
      color: var(--color-surface-400);
    }

    .btn-ghost:hover:not(:disabled) {
      background-color: var(--color-surface-700);
      color: var(--color-surface-200);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .upload-result {
      padding: 1rem;
    }

    .result-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.75rem;
    }

    .detail-grid {
      grid-template-columns: 1fr;
      gap: 0.5rem;
      margin-left: 1rem;
    }

    .action-buttons {
      justify-content: stretch;
      flex-direction: column;
    }

    .btn {
      justify-content: center;
    }
  }
</style>