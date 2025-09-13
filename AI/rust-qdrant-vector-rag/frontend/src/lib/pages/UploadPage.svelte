<script lang="ts">
  import { onMount } from 'svelte';
  import { ArrowRight, Search } from 'lucide-svelte';
  import FileUpload from '../components/FileUpload.svelte';
  import UploadProgress from '../components/UploadProgress.svelte';
  import UploadResult from '../components/UploadResult.svelte';
  import { uploadActions, isUploading, uploadProgress, uploadResult, selectedFile } from '../stores/upload.store.js';
  import { toastActions } from '../stores/toast.store.js';
  import { apiService } from '../services/api.js';
  import type { ValidationError } from '../types/state.js';

  // Local state
  let uploadStartTime: number | null = null;
  let estimatedTimeRemaining: number | null = null;

  // Handle file selection
  function handleFileSelect(event: CustomEvent<{ file: File }>) {
    const { file } = event.detail;
    uploadActions.setSelectedFile(file);
    
    toastActions.add({
      type: 'info',
      message: `Selected file: ${file.name}`,
      duration: 3000
    });
  }

  // Handle file removal
  function handleFileRemove() {
    uploadActions.setSelectedFile(null);
    uploadActions.clearResult();
  }

  // Handle validation errors
  function handleValidationError(event: CustomEvent<{ errors: ValidationError[] }>) {
    const { errors } = event.detail;
    const errorMessage = errors.map(e => e.message).join(', ');
    
    toastActions.add({
      type: 'error',
      message: `File validation failed: ${errorMessage}`,
      duration: 5000
    });
  }

  // Handle drag events
  function handleDragEnter() {
    uploadActions.setDragActive(true);
  }

  function handleDragLeave() {
    uploadActions.setDragActive(false);
  }

  // Start upload process
  async function startUpload() {
    const file = $selectedFile;
    if (!file) {
      toastActions.add({
        type: 'error',
        message: 'No file selected for upload',
        duration: 3000
      });
      return;
    }

    try {
      uploadActions.startUpload();
      uploadStartTime = Date.now();
      estimatedTimeRemaining = null;

      // Simulate progress updates (since we don't have real progress from the API)
      const progressInterval = setInterval(() => {
        const currentProgress = $uploadProgress;
        if (currentProgress < 90 && $isUploading) {
          const increment = Math.random() * 10 + 5; // Random increment between 5-15%
          uploadActions.updateProgress(currentProgress + increment);
          
          // Calculate estimated time remaining
          if (uploadStartTime && currentProgress > 10) {
            const elapsed = (Date.now() - uploadStartTime) / 1000;
            const rate = currentProgress / elapsed;
            const remaining = (100 - currentProgress) / rate;
            estimatedTimeRemaining = remaining;
          }
        } else if (!$isUploading) {
          clearInterval(progressInterval);
        }
      }, 500);

      // Perform the actual upload
      const result = await apiService.uploadDocument(file);
      
      clearInterval(progressInterval);
      estimatedTimeRemaining = null;
      
      uploadActions.completeUpload(result);
      
      toastActions.add({
        type: 'success',
        message: `Successfully uploaded ${file.name}`,
        duration: 5000
      });

    } catch (error) {
      estimatedTimeRemaining = null;
      
      let errorMessage = 'Upload failed';
      if (error && typeof error === 'object' && 'message' in error) {
        errorMessage = error.message as string;
      }
      
      uploadActions.failUpload(errorMessage);
      
      toastActions.add({
        type: 'error',
        message: `Upload failed: ${errorMessage}`,
        duration: 5000
      });
    }
  }

  // Handle retry upload
  function handleRetry() {
    uploadActions.clearResult();
    startUpload();
  }

  // Handle new upload
  function handleNewUpload() {
    uploadActions.reset();
  }

  // Handle result dismiss
  function handleResultDismiss() {
    uploadActions.clearResult();
  }

  // Navigate to search page
  function navigateToSearch() {
    // This would typically use a router
    // For now, we'll just show a toast
    toastActions.add({
      type: 'info',
      message: 'Navigate to search page to query your uploaded documents',
      duration: 3000
    });
  }

  // Cleanup on unmount
  onMount(() => {
    return () => {
      uploadActions.reset();
    };
  });
</script>

<div class="upload-page">
  <div class="upload-container">
    <!-- Page header -->
    <div class="page-header">
      <h1 class="page-title">Upload Documents</h1>
      <p class="page-description">
        Upload PDF documents to make them searchable with AI-powered queries
      </p>
    </div>

    <!-- Upload section -->
    <div class="upload-section">
      {#if !$uploadResult}
        <!-- File upload component -->
        <FileUpload
          bind:selectedFile={$selectedFile}
          disabled={$isUploading}
          on:fileSelect={handleFileSelect}
          on:fileRemove={handleFileRemove}
          on:validationError={handleValidationError}
          on:dragEnter={handleDragEnter}
          on:dragLeave={handleDragLeave}
        />

        <!-- Upload button -->
        {#if $selectedFile && !$isUploading}
          <div class="upload-actions">
            <button
              class="btn btn-primary btn-lg"
              on:click={startUpload}
            >
              <ArrowRight size={20} />
              Upload Document
            </button>
          </div>
        {/if}

        <!-- Upload progress -->
        {#if $isUploading || $uploadProgress > 0}
          <UploadProgress
            progress={$uploadProgress}
            isUploading={$isUploading}
            fileName={$selectedFile?.name || ''}
            status={$isUploading ? 'uploading' : 'idle'}
            message={$isUploading ? 'Processing your document...' : ''}
            {estimatedTimeRemaining}
          />
        {/if}
      {:else}
        <!-- Upload result -->
        <UploadResult
          result={$uploadResult}
          on:retry={handleRetry}
          on:newUpload={handleNewUpload}
          on:dismiss={handleResultDismiss}
        />

        <!-- Success actions -->
        {#if $uploadResult.status === 'success'}
          <div class="success-actions">
            <p class="success-message">
              Your document has been processed and is ready for searching
            </p>
            <div class="success-buttons">
              <button
                class="btn btn-primary"
                on:click={navigateToSearch}
              >
                <Search size={20} />
                Search Documents
              </button>
              <button
                class="btn btn-secondary"
                on:click={handleNewUpload}
              >
                Upload Another
              </button>
            </div>
          </div>
        {/if}
      {/if}
    </div>

    <!-- Help text -->
    <div class="help-section">
      <p class="help-text">
        Supported formats: PDF files up to 10MB
      </p>
      <p class="help-text">
        Your documents will be processed and split into searchable chunks for AI-powered queries
      </p>
    </div>
  </div>
</div>

<style>
  .upload-page {
    min-height: 100vh;
    padding: 2rem 0;
    background-color: var(--color-surface-50);
  }

  .upload-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .page-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .page-title {
    margin: 0 0 1rem 0;
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--color-surface-900);
  }

  .page-description {
    margin: 0;
    font-size: 1.125rem;
    color: var(--color-surface-600);
    max-width: 600px;
    margin: 0 auto;
    line-height: 1.6;
  }

  .upload-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 600px;
    margin: 0 auto;
  }

  .upload-actions {
    display: flex;
    justify-content: center;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    border: 1px solid transparent;
    border-radius: 0.5rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    text-decoration: none;
    background: none;
    white-space: nowrap;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-lg {
    padding: 1rem 2rem;
    font-size: 1.125rem;
    width: 100%;
    justify-content: center;
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

  .btn-secondary {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
  }

  .btn:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .success-actions {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    align-items: center;
    text-align: center;
    padding: 2rem;
    background-color: var(--color-success-50);
    border-radius: 0.75rem;
    border: 1px solid var(--color-success-200);
  }

  .success-message {
    margin: 0;
    font-size: 1rem;
    color: var(--color-surface-600);
    line-height: 1.5;
  }

  .success-buttons {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    justify-content: center;
  }

  .help-section {
    text-align: center;
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid var(--color-surface-200);
  }

  .help-text {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: var(--color-surface-500);
    line-height: 1.5;
  }

  .help-text:last-child {
    margin-bottom: 0;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .upload-page {
      background-color: var(--color-surface-900);
    }

    .page-title {
      color: var(--color-surface-100);
    }

    .page-description {
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

    .success-actions {
      background-color: var(--color-surface-800);
      border-color: var(--color-success-700);
    }

    .success-message {
      color: var(--color-surface-300);
    }

    .help-section {
      border-color: var(--color-surface-700);
    }

    .help-text {
      color: var(--color-surface-400);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .upload-page {
      padding: 1rem 0;
    }

    .upload-container {
      padding: 0 1rem;
    }

    .page-header {
      margin-bottom: 2rem;
    }

    .page-title {
      font-size: 2rem;
    }

    .page-description {
      font-size: 1rem;
    }

    .upload-section {
      max-width: 100%;
    }

    .success-actions {
      padding: 1.5rem;
    }

    .success-buttons {
      flex-direction: column;
      width: 100%;
    }

    .btn {
      justify-content: center;
    }

    .help-section {
      margin-top: 2rem;
      padding-top: 1.5rem;
    }
  }
</style>