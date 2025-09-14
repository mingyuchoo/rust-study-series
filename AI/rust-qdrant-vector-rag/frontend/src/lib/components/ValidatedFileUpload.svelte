<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { Upload, FileText, X, AlertCircle, CheckCircle, Shield } from 'lucide-svelte';
  import { 
    InputSanitizer, 
    DebouncedValidation,
    debounce 
  } from '../utils/validation.js';
  import { env, formatFileSize } from '../config/env.js';
  import { generateId, announceToScreenReader, KeyboardNavigation } from '../utils/accessibility.js';
  import type { ValidationError } from '../types/state.js';

  // Props
  export let disabled = false;
  export let selectedFile: File | null = null;
  export let showValidationFeedback = true;
  export let allowedTypes = ['.md', '.markdown'];
  export let maxFileSize = env.MAX_FILE_SIZE;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    fileSelect: { file: File; sanitizedFilename: string };
    fileRemove: void;
    validationError: { errors: ValidationError[] };
    validationSuccess: { file: File };
    dragEnter: void;
    dragLeave: void;
  }>();

  // Local state
  let fileInput: HTMLInputElement;
  let validationErrors: ValidationError[] = [];
  let validationWarnings: ValidationError[] = [];
  let isDragOver = false;
  let isValidating = false;
  let sanitizedFilename = '';
  
  // Generate unique IDs for accessibility
  const dropZoneId = generateId('drop-zone');
  const instructionsId = generateId('upload-instructions');
  const errorsId = generateId('upload-errors');
  const warningsId = generateId('upload-warnings');

  // Reactive statements
  $: hasErrors = validationErrors.length > 0;
  $: hasWarnings = validationWarnings.length > 0;
  $: isValid = !hasErrors && selectedFile !== null;

  // Debounced file validation
  const debouncedFileValidation = debounce((file: File) => {
    isValidating = true;
    DebouncedValidation.validateFileDebounced(file, (errors) => {
      validationErrors = errors;
      isValidating = false;
      
      if (errors.length === 0) {
        // File is valid, sanitize filename
        const originalName = file.name;
        sanitizedFilename = InputSanitizer.sanitizeFilename(originalName);
        
        if (sanitizedFilename !== originalName) {
          validationWarnings = [{
            field: 'filename',
            message: `Filename was cleaned for security: "${originalName}" → "${sanitizedFilename}"`,
            code: 'filename_sanitized'
          }];
          announceToScreenReader(`Filename was automatically cleaned for security`, 'polite');
        } else {
          validationWarnings = [];
        }
        
        dispatch('validationSuccess', { file });
        dispatch('fileSelect', { file, sanitizedFilename });
      } else {
        validationWarnings = [];
        dispatch('validationError', { errors });
      }
    });
  }, 100);

  // Validation is handled via DebouncedValidation and RealTimeValidator

  // Handle file selection with enhanced validation
  function handleFileSelect(file: File) {
    selectedFile = file;
    debouncedFileValidation(file);
    
    // Announce file selection attempt to screen readers
    announceToScreenReader(`Validating file: ${file.name}`, 'polite');
  }

  // Handle file input change
  function handleInputChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    
    if (file) {
      handleFileSelect(file);
    }
  }

  // Handle file removal
  function handleFileRemove() {
    const fileName = selectedFile?.name;
    selectedFile = null;
    validationErrors = [];
    validationWarnings = [];
    sanitizedFilename = '';
    
    if (fileInput) {
      fileInput.value = '';
    }
    
    dispatch('fileRemove');
    
    // Announce file removal to screen readers
    if (fileName) {
      announceToScreenReader(`File removed: ${fileName}`, 'polite');
    }
  }

  // Handle drag events with enhanced feedback
  function handleDragEnter(event: DragEvent) {
    event.preventDefault();
    event.stopPropagation();
    isDragOver = true;
    dispatch('dragEnter');
    
    // Announce drag state to screen readers
    announceToScreenReader('File dragged over upload area', 'polite');
  }

  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    // Only set dragOver to false if we're leaving the drop zone entirely
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    const x = event.clientX;
    const y = event.clientY;
    
    if (x < rect.left || x > rect.right || y < rect.top || y > rect.bottom) {
      isDragOver = false;
      dispatch('dragLeave');
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    event.stopPropagation();
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    event.stopPropagation();
    isDragOver = false;

    const files = event.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files.item(0);
      if (file) {
        handleFileSelect(file);
      }
    }
  }

  // Handle click to open file dialog
  function handleClick() {
    if (!disabled && fileInput) {
      fileInput.click();
    }
  }

  // Handle keyboard navigation
  function handleKeydown(event: KeyboardEvent) {
    if (disabled) return;
    
    if (KeyboardNavigation.isActivationKey(event.key)) {
      event.preventDefault();
      handleClick();
    }
  }

  // Cleanup on destroy
  onDestroy(() => {
    debouncedFileValidation.cancel();
  });
</script>

<div class="file-upload-container">
  <input
    bind:this={fileInput}
    type="file"
    accept={allowedTypes.join(',')}
    on:change={handleInputChange}
    style="display: none;"
    {disabled}
    aria-describedby={instructionsId}
    aria-invalid={hasErrors}
  />

  <div
    id={dropZoneId}
    class="drop-zone min-touch-target focus-visible"
    class:drag-over={isDragOver}
    class:has-file={!!selectedFile && isValid}
    class:has-errors={hasErrors}
    class:has-warnings={hasWarnings && !hasErrors}
    class:validating={isValidating}
    class:disabled
    on:click={handleClick}
    on:keydown={handleKeydown}
    on:dragenter={handleDragEnter}
    on:dragleave={handleDragLeave}
    on:dragover={handleDragOver}
    on:drop={handleDrop}
    role="button"
    tabindex={disabled ? -1 : 0}
    aria-label={selectedFile ? `Selected file: ${selectedFile.name}. ${isValid ? 'Valid file.' : 'File has validation errors.'} Click to change file or remove.` : 'Upload Markdown file. Click to browse or drag and drop.'}
    aria-describedby={`${instructionsId} ${hasErrors ? errorsId : ''} ${hasWarnings ? warningsId : ''}`}
    aria-disabled={disabled}
  >
    {#if selectedFile}
      <!-- Selected file display with validation status -->
      <div class="file-display">
        <div class="file-icon-container">
          <FileText size={48} color="var(--color-primary-600)" />
          {#if isValidating}
            <div class="validation-spinner">
              <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-primary-600" aria-hidden="true"></div>
            </div>
          {:else if hasErrors}
            <div class="validation-status error">
              <AlertCircle size={16} aria-hidden="true" />
            </div>
          {:else if hasWarnings}
            <div class="validation-status warning">
              <AlertCircle size={16} aria-hidden="true" />
            </div>
          {:else if isValid}
            <div class="validation-status success">
              <CheckCircle size={16} aria-hidden="true" />
            </div>
          {/if}
        </div>
        
        <div class="file-info">
          <h3 class="file-name" id={generateId('file-name')}>
            {sanitizedFilename || selectedFile.name}
            {#if sanitizedFilename && sanitizedFilename !== selectedFile.name}
              <Shield size={14} class="inline ml-1 text-orange-500" aria-label="Filename was sanitized" />
            {/if}
          </h3>
          <p class="file-size" id={generateId('file-size')}>{formatFileSize(selectedFile.size)}</p>
          
          {#if isValidating}
            <p class="validation-status-text validating">Validating file...</p>
          {:else if hasErrors}
            <p class="validation-status-text error">File has validation errors</p>
          {:else if hasWarnings}
            <p class="validation-status-text warning">File validated with warnings</p>
          {:else if isValid}
            <p class="validation-status-text success">File validated successfully</p>
          {/if}
        </div>
        
        <div class="file-actions">
          <button
            class="btn btn-danger btn-sm min-touch-target focus-visible"
            on:click|stopPropagation={handleFileRemove}
            {disabled}
            aria-label={`Remove file ${selectedFile.name}`}
          >
            <X size={16} aria-hidden="true" />
            <span>Remove</span>
          </button>
          <button
            class="btn btn-secondary btn-sm min-touch-target focus-visible"
            on:click|stopPropagation={handleClick}
            {disabled}
            aria-label="Choose a different Markdown file"
          >
            <span>Choose Different File</span>
          </button>
        </div>
      </div>
    {:else}
      <!-- Upload prompt -->
      <div class="upload-prompt">
        <div class="upload-icon">
          <Upload 
            size={48} 
            color={isDragOver ? "var(--color-primary-600)" : "var(--color-surface-500)"} 
            aria-hidden="true"
          />
          {#if isValidating}
            <div class="validation-spinner">
              <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-primary-600" aria-hidden="true"></div>
            </div>
          {/if}
        </div>
        
        <div class="upload-text">
          <h3 class="upload-title">
            {isDragOver ? 'Drop your Markdown file here' : 'Upload Markdown Document'}
          </h3>
          <p class="upload-description" id={instructionsId}>
            Drag and drop a Markdown file here, or click to browse. Only .md or .markdown files are accepted.
          </p>
          <p class="upload-limit">
            Maximum file size: {formatFileSize(maxFileSize)}
          </p>
        </div>
        
        <span class="btn btn-primary min-touch-target" aria-hidden="true">
          <Upload size={20} aria-hidden="true" />
          <span>Choose File</span>
        </span>
      </div>
    {/if}
  </div>

  <!-- Validation feedback -->
  {#if showValidationFeedback}
    <!-- Validation errors -->
    {#if hasErrors}
      <div 
        id={errorsId}
        class="validation-feedback validation-errors" 
        role="alert" 
        aria-live="assertive"
        aria-atomic="true"
      >
        <h4 class="sr-only">File validation errors</h4>
        <div class="flex items-start gap-2">
          <AlertCircle size={16} class="text-red-600 dark:text-red-400 mt-0.5 flex-shrink-0" aria-hidden="true" />
          <div class="space-y-1">
            {#each validationErrors as error, index}
              <p class="error-message" id={generateId(`error-${index}`)}>
                <span class="sr-only">Error:</span>
                {error.message}
              </p>
            {/each}
          </div>
        </div>
      </div>
    {/if}

    <!-- Validation warnings -->
    {#if hasWarnings && !hasErrors}
      <div 
        id={warningsId}
        class="validation-feedback validation-warnings" 
        role="status" 
        aria-live="polite"
        aria-atomic="true"
      >
        <h4 class="sr-only">File validation warnings</h4>
        <div class="flex items-start gap-2">
          <AlertCircle size={16} class="text-orange-600 dark:text-orange-400 mt-0.5 flex-shrink-0" aria-hidden="true" />
          <div class="space-y-1">
            {#each validationWarnings as warning, index}
              <p class="warning-message" id={generateId(`warning-${index}`)}>
                <span class="sr-only">Warning:</span>
                {warning.message}
              </p>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .file-upload-container {
    width: 100%;
  }

  .drop-zone {
    cursor: pointer;
    transition: all 0.2s ease;
    border: 2px dashed var(--color-surface-300);
    background-color: var(--color-surface-50);
    border-radius: 0.5rem;
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    position: relative;
  }

  .drop-zone:hover:not([disabled]) {
    border-color: var(--color-primary-400);
    background-color: var(--color-primary-50);
  }

  .drop-zone.drag-over {
    border-color: var(--color-primary-600);
    background-color: var(--color-primary-100);
    transform: scale(1.02);
  }

  .drop-zone.has-file {
    border-color: var(--color-success-400);
    background-color: var(--color-success-50);
  }

  .drop-zone.has-errors {
    border-color: var(--color-error-400);
    background-color: var(--color-error-50);
  }

  .drop-zone.has-warnings {
    border-color: var(--color-warning-400);
    background-color: var(--color-warning-50);
  }

  .drop-zone.validating {
    border-color: var(--color-primary-400);
    background-color: var(--color-primary-50);
  }

  .drop-zone:disabled {
    cursor: not-allowed;
    opacity: 0.6;
    background-color: var(--color-surface-100);
  }

  .file-display,
  .upload-prompt {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    text-align: center;
    width: 100%;
  }

  .file-icon-container,
  .upload-icon {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .validation-spinner {
    position: absolute;
    bottom: -2px;
    right: -2px;
    background: white;
    border-radius: 50%;
    padding: 2px;
  }

  .validation-status {
    position: absolute;
    bottom: -2px;
    right: -2px;
    border-radius: 50%;
    padding: 2px;
  }

  .validation-status.success {
    background: var(--color-success-100);
    color: var(--color-success-600);
  }

  .validation-status.warning {
    background: var(--color-warning-100);
    color: var(--color-warning-600);
  }

  .validation-status.error {
    background: var(--color-error-100);
    color: var(--color-error-600);
  }

  .file-info {
    text-align: center;
    flex: 1;
  }

  .file-name {
    margin: 0 0 0.25rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-surface-900);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
  }

  .file-size {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: var(--color-surface-600);
  }

  .validation-status-text {
    margin: 0;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .validation-status-text.validating {
    color: var(--color-primary-600);
  }

  .validation-status-text.success {
    color: var(--color-success-600);
  }

  .validation-status-text.warning {
    color: var(--color-warning-600);
  }

  .validation-status-text.error {
    color: var(--color-error-600);
  }

  .file-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: center;
  }

  .upload-text {
    text-align: center;
    max-width: 300px;
  }

  .upload-title {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .upload-description {
    margin: 0 0 0.25rem 0;
    font-size: 0.875rem;
    color: var(--color-surface-600);
  }

  .upload-limit {
    margin: 0;
    font-size: 0.75rem;
    color: var(--color-surface-500);
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

  .btn-danger {
    background-color: var(--color-error-600);
    color: white;
    border-color: var(--color-error-600);
  }

  .btn-danger:hover:not(:disabled) {
    background-color: var(--color-error-700);
    border-color: var(--color-error-700);
  }

  .btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
  }

  .validation-feedback {
    margin-top: 0.75rem;
    padding: 0.75rem;
    border-radius: 0.375rem;
  }

  .validation-errors {
    background-color: var(--color-error-50);
    border: 1px solid var(--color-error-200);
  }

  .validation-warnings {
    background-color: var(--color-warning-50);
    border: 1px solid var(--color-warning-200);
  }

  .error-message {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-error-700);
  }

  .warning-message {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-warning-700);
  }

  .error-message:not(:last-child),
  .warning-message:not(:last-child) {
    margin-bottom: 0.5rem;
  }

  /* Focus styles for accessibility */
  .drop-zone:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .btn:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .drop-zone {
      border-color: var(--color-surface-600);
      background-color: var(--color-surface-800);
    }

    .drop-zone:hover:not([disabled]) {
      border-color: var(--color-primary-400);
      background-color: var(--color-surface-700);
    }

    .drop-zone.drag-over {
      background-color: var(--color-surface-700);
    }

    .drop-zone.has-file {
      border-color: var(--color-success-400);
      background-color: var(--color-surface-700);
    }

    .drop-zone.has-errors {
      border-color: var(--color-error-400);
      background-color: var(--color-surface-800);
    }

    .drop-zone.has-warnings {
      border-color: var(--color-warning-400);
      background-color: var(--color-surface-800);
    }

    .drop-zone.validating {
      background-color: var(--color-surface-700);
    }

    .drop-zone:disabled {
      background-color: var(--color-surface-900);
    }

    .file-name,
    .upload-title {
      color: var(--color-surface-100);
    }

    .file-size,
    .upload-description {
      color: var(--color-surface-300);
    }

    .upload-limit {
      color: var(--color-surface-400);
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

    .validation-errors {
      background-color: var(--color-surface-800);
      border-color: var(--color-error-700);
    }

    .validation-warnings {
      background-color: var(--color-surface-800);
      border-color: var(--color-warning-700);
    }

    .error-message {
      color: var(--color-error-400);
    }

    .warning-message {
      color: var(--color-warning-400);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .drop-zone {
      min-height: 150px;
      padding: 1.5rem 1rem;
    }

    .upload-text {
      max-width: 250px;
    }

    .file-actions {
      flex-direction: column;
      width: 100%;
    }

    .btn {
      justify-content: center;
    }
  }

  /* Animation for validation spinner */
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>