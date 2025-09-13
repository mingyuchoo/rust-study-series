<script lang="ts">
  import { onMount } from 'svelte';
  import { ArrowRight, Search, Upload } from 'lucide-svelte';
  import FileUpload from '../components/FileUpload.svelte';
  import UploadProgress from '../components/UploadProgress.svelte';
  import UploadResult from '../components/UploadResult.svelte';
  import { InteractiveButton, ProgressIndicator, SuccessNotification, LoadingOverlay } from '../components/index.js';
  import { uploadActions, isUploading, uploadProgress, uploadResult, selectedFile } from '../stores/upload.store.js';
  import { toastActions } from '../stores/toast.store.js';
  import { apiService } from '../services/api.js';
  import { errorHandler } from '../services/error-handler.js';
  import type { ValidationError } from '../types/state.js';
  import type { AppError } from '../types/errors.js';

  // Local state
  let uploadStartTime: number | null = null;
  let estimatedTimeRemaining: number | null = null;
  let showSuccessNotification = false;
  let showLoadingOverlay = false;
  let successNotificationRef: SuccessNotification;

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
      const uploadError = errorHandler.createUploadError(
        'No file selected for upload',
        'unknown',
        'upload_failed'
      );
      uploadActions.failUpload(uploadError);
      return;
    }

    // Check if we're online before attempting upload
    if (!errorHandler.isOnline()) {
      const networkError = errorHandler.createNetworkError(
        'Cannot upload while offline. Please check your connection.',
        { url: '/upload', method: 'POST' }
      );
      uploadActions.failUpload(networkError, () => startUpload());
      return;
    }

    let progressInterval: number | null = null;

    try {
      uploadActions.startUpload();
      uploadStartTime = Date.now();
      estimatedTimeRemaining = null;

      // Simulate progress updates (since we don't have real progress from the API)
      progressInterval = window.setInterval(() => {
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
          if (progressInterval) {
            clearInterval(progressInterval);
            progressInterval = null;
          }
        }
      }, 500);

      // Perform the actual upload
      const result = await apiService.uploadDocument(file);
      
      if (progressInterval) {
        clearInterval(progressInterval);
        progressInterval = null;
      }
      estimatedTimeRemaining = null;
      
      uploadActions.completeUpload(result);
      
      // Show enhanced success notification
      showSuccessNotification = true;
      toastActions.success(`Successfully uploaded ${file.name}`);

    } catch (error) {
      if (progressInterval) {
        clearInterval(progressInterval);
        progressInterval = null;
      }
      estimatedTimeRemaining = null;
      
      // Handle error with comprehensive error handling and retry capability
      const appError = error as AppError;
      uploadActions.failUpload(appError, () => startUpload());
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
  <div class="upload-container container-responsive">
    <!-- Page header -->
    <header class="page-header">
      <h1 class="page-title text-responsive-xl" id="upload-page-title">Upload Documents</h1>
      <p class="page-description text-responsive-base" aria-describedby="upload-page-title">
        Upload PDF documents to make them searchable with AI-powered queries
      </p>
    </header>

    <!-- Upload section -->
    <main class="upload-section" aria-labelledby="upload-page-title">
      {#if !$uploadResult}
        <!-- File upload component -->
        <section class="upload-area" aria-labelledby="upload-area-title">
          <h2 id="upload-area-title" class="sr-only">File Upload Area</h2>
          <FileUpload
            bind:selectedFile={$selectedFile}
            disabled={$isUploading}
            on:fileSelect={handleFileSelect}
            on:fileRemove={handleFileRemove}
            on:validationError={handleValidationError}
            on:dragEnter={handleDragEnter}
            on:dragLeave={handleDragLeave}
          />
        </section>

        <!-- Upload button -->
        {#if $selectedFile && !$isUploading}
          <div class="upload-actions">
            <InteractiveButton
              variant="primary"
              size="lg"
              fullWidth={true}
              icon={Upload}
              loading={$isUploading}
              loadingText="Uploading..."
              rippleEffect={true}
              hoverEffect={true}
              focusEffect={true}
              ariaLabel="Upload {$selectedFile.name} to make it searchable"
              on:click={startUpload}
            >
              Upload Document
            </InteractiveButton>
          </div>
        {/if}

        <!-- Upload progress -->
        {#if $isUploading || $uploadProgress > 0}
          <section class="progress-section" aria-labelledby="progress-title" aria-live="polite">
            <h2 id="progress-title" class="sr-only">Upload Progress</h2>
            <div class="progress-container">
              <ProgressIndicator
                progress={$uploadProgress}
                status={$isUploading ? 'loading' : ($uploadResult?.status === 'success' ? 'success' : 'idle')}
                message={$isUploading ? 'Processing your document...' : ''}
                showPercentage={true}
                showTimeRemaining={true}
                estimatedTimeRemaining={estimatedTimeRemaining}
                size="md"
                variant="linear"
                animated={true}
                showIcon={true}
              />
              
              <!-- Enhanced upload progress with file details -->
              <div class="upload-details">
                <p class="file-name">
                  <strong>File:</strong> {$selectedFile?.name || 'Unknown file'}
                </p>
                <p class="file-size">
                  <strong>Size:</strong> {$selectedFile ? Math.round($selectedFile.size / 1024) : 0} KB
                </p>
              </div>
            </div>
          </section>
        {/if}
      {:else}
        <!-- Upload result -->
        <section class="result-section" aria-labelledby="result-title" role="status">
          <h2 id="result-title" class="sr-only">Upload Result</h2>
          <UploadResult
            result={$uploadResult}
            on:retry={handleRetry}
            on:newUpload={handleNewUpload}
            on:dismiss={handleResultDismiss}
          />
        </section>

        <!-- Success actions -->
        {#if $uploadResult.status === 'success'}
          <section class="success-actions" aria-labelledby="success-title">
            <h2 id="success-title" class="sr-only">Next Steps</h2>
            <p class="success-message text-responsive-base">
              Your document has been processed and is ready for searching
            </p>
            <div class="success-buttons flex-responsive">
              <InteractiveButton
                variant="primary"
                size="md"
                icon={Search}
                rippleEffect={true}
                hoverEffect={true}
                focusEffect={true}
                ariaLabel="Go to search page to query your uploaded documents"
                on:click={navigateToSearch}
              >
                Search Documents
              </InteractiveButton>
              <InteractiveButton
                variant="secondary"
                size="md"
                icon={Upload}
                rippleEffect={true}
                hoverEffect={true}
                focusEffect={true}
                ariaLabel="Upload another document"
                on:click={handleNewUpload}
              >
                Upload Another
              </InteractiveButton>
            </div>
          </section>
        {/if}
      {/if}
    </main>

    <!-- Help text -->
    <aside class="help-section" aria-labelledby="help-title">
      <h2 id="help-title" class="help-title text-responsive-base">Upload Guidelines</h2>
      <ul class="help-list" role="list">
        <li class="help-text text-responsive-sm">
          <strong>Supported formats:</strong> PDF files up to 10MB
        </li>
        <li class="help-text text-responsive-sm">
          <strong>Processing:</strong> Your documents will be processed and split into searchable chunks for AI-powered queries
        </li>
        <li class="help-text text-responsive-sm">
          <strong>Privacy:</strong> Your documents are processed securely and stored temporarily for search purposes
        </li>
      </ul>
    </aside>
  </div>
</div>

<!-- Success notification -->
<SuccessNotification
  bind:this={successNotificationRef}
  bind:visible={showSuccessNotification}
  title="Upload Successful!"
  message="Your document has been processed and is ready for searching"
  variant="celebration"
  duration={6000}
  actions={[
    { label: 'Search Now', action: 'search', icon: Search, variant: 'primary' },
    { label: 'Upload Another', action: 'upload', variant: 'secondary' }
  ]}
  on:action={(e) => {
    if (e.detail.action === 'search') {
      navigateToSearch();
    } else if (e.detail.action === 'upload') {
      handleNewUpload();
    }
    showSuccessNotification = false;
  }}
  on:dismiss={() => showSuccessNotification = false}
/>

<!-- Loading overlay for intensive operations -->
<LoadingOverlay
  bind:visible={showLoadingOverlay}
  message="Processing your document..."
  variant="upload"
  showProgress={true}
  progress={$uploadProgress}
  estimatedTime={estimatedTimeRemaining}
  backdrop="blur"
  size="md"
  allowDismiss={false}
/>

<style>
  .upload-page {
    min-height: 100vh;
    padding: var(--spacing-xl) 0;
    background-color: var(--color-surface-50);
    scroll-behavior: smooth;
  }

  .upload-container {
    max-width: 800px;
    margin: 0 auto;
  }

  .page-header {
    text-align: center;
    margin-bottom: var(--spacing-3xl);
  }

  .page-title {
    margin: 0 0 var(--spacing-lg) 0;
    font-weight: 700;
    color: var(--color-surface-900);
    line-height: var(--line-height-tight);
  }

  .page-description {
    margin: 0 auto;
    color: var(--color-surface-600);
    max-width: 600px;
    line-height: var(--line-height-relaxed);
  }

  .upload-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
    max-width: 600px;
    margin: 0 auto;
  }

  .upload-area {
    position: relative;
  }

  .upload-actions {
    display: flex;
    justify-content: center;
  }

  .progress-section,
  .result-section {
    position: relative;
  }

  .progress-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    background-color: var(--color-surface-100);
    border-radius: 0.75rem;
    padding: var(--spacing-lg);
    border: 2px solid var(--color-surface-200);
  }

  .upload-details {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: var(--font-size-sm);
    color: var(--color-surface-600);
  }

  .file-name,
  .file-size {
    margin: 0;
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md) var(--spacing-xl);
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-size: var(--font-size-base);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--duration-fast) ease;
    text-decoration: none;
    background: none;
    white-space: nowrap;
    position: relative;
    line-height: var(--line-height-tight);
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    pointer-events: none;
  }

  .btn-lg {
    padding: var(--spacing-lg) var(--spacing-2xl);
    font-size: var(--font-size-lg);
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
    transform: translateY(-2px);
    box-shadow: var(--shadow-lg);
  }

  .btn-primary:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: var(--shadow-md);
  }

  .btn-secondary {
    background-color: var(--color-surface-100);
    color: var(--color-surface-700);
    border-color: var(--color-surface-300);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .btn-secondary:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }

  .success-actions {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
    align-items: center;
    text-align: center;
    padding: var(--spacing-2xl);
    background-color: var(--color-success-50);
    border-radius: 0.75rem;
    border: 2px solid var(--color-success-200);
    box-shadow: var(--shadow-sm);
  }

  .success-message {
    margin: 0;
    color: var(--color-surface-600);
    line-height: var(--line-height-relaxed);
    max-width: 500px;
  }

  .success-buttons {
    display: flex;
    gap: var(--spacing-lg);
    flex-wrap: wrap;
    justify-content: center;
    width: 100%;
  }

  .help-section {
    text-align: center;
    margin-top: var(--spacing-3xl);
    padding-top: var(--spacing-2xl);
    border-top: 2px solid var(--color-surface-200);
  }

  .help-title {
    margin: 0 0 var(--spacing-lg) 0;
    font-weight: 600;
    color: var(--color-surface-700);
  }

  .help-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    max-width: 600px;
    margin: 0 auto;
  }

  .help-text {
    margin: 0;
    color: var(--color-surface-500);
    line-height: var(--line-height-relaxed);
    text-align: left;
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: var(--color-surface-100);
    border-radius: 0.5rem;
    border-left: 4px solid var(--color-primary-300);
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

    .help-title {
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
      background-color: var(--color-surface-800);
      border-left-color: var(--color-primary-400);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .btn {
      border-width: 3px;
    }

    .success-actions {
      border-width: 3px;
    }

    .help-text {
      border-left-width: 6px;
    }

    .help-section {
      border-top-width: 3px;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .btn {
      transition: none;
    }

    .btn:hover:not(:disabled),
    .btn:active:not(:disabled) {
      transform: none;
    }

    .upload-page {
      scroll-behavior: auto;
    }
  }

  /* Print styles */
  @media print {
    .upload-page {
      background: white;
      color: black;
    }

    .btn {
      border: 2px solid black;
      background: white;
      color: black;
    }

    .success-actions {
      border: 2px solid black;
      background: white;
    }

    .help-text {
      background: white;
      border-left: 4px solid black;
    }
  }

  /* Focus management for keyboard users */
  .upload-section:focus-within {
    outline: 2px solid transparent;
  }

  /* Enhanced touch targets for mobile */
  @media (max-width: 767px) {
    .btn {
      min-height: 48px;
      min-width: 48px;
      font-size: 16px; /* Prevent zoom on iOS */
    }
  }

  /* Mobile styles (default) */
  @media (max-width: 767px) {
    .upload-page {
      padding: var(--spacing-lg) 0;
    }

    .page-header {
      margin-bottom: var(--spacing-2xl);
    }

    .upload-section {
      max-width: 100%;
      gap: var(--spacing-lg);
    }

    .success-actions {
      padding: var(--spacing-xl);
    }

    .success-buttons {
      flex-direction: column;
      width: 100%;
      gap: var(--spacing-md);
    }

    .btn {
      width: 100%;
      justify-content: center;
    }

    .help-section {
      margin-top: var(--spacing-2xl);
      padding-top: var(--spacing-xl);
    }

    .help-list {
      gap: var(--spacing-sm);
    }

    .help-text {
      text-align: center;
      padding: var(--spacing-md);
    }
  }

  /* Tablet styles */
  @media (min-width: 768px) and (max-width: 1023px) {
    .upload-page {
      padding: var(--spacing-2xl) 0;
    }

    .page-header {
      margin-bottom: var(--spacing-3xl);
    }

    .success-buttons {
      flex-direction: row;
      justify-content: center;
    }

    .btn {
      min-width: 200px;
    }
  }

  /* Desktop styles */
  @media (min-width: 1024px) {
    .upload-page {
      padding: var(--spacing-3xl) 0;
    }

    .page-header {
      margin-bottom: var(--spacing-4xl);
    }

    .upload-section {
      gap: var(--spacing-2xl);
    }

    .success-actions {
      padding: var(--spacing-3xl);
    }

    .btn:hover:not(:disabled) {
      transform: translateY(-2px);
    }

    .btn:active:not(:disabled) {
      transform: translateY(0);
    }
  }

  /* Large desktop styles */
  @media (min-width: 1440px) {
    .upload-container {
      max-width: 900px;
    }

    .upload-section {
      max-width: 700px;
    }
  }
</style>