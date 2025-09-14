<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Upload, FileText, X } from 'lucide-svelte';
  import { env, formatFileSize } from '../config/env.js';
  import { FileUploadSchema } from '../schemas/validation.js';
  import { errorHandler } from '../services/error-handler.js';
  import type { ValidationError } from '../types/state.js';
  import { generateId, announceToScreenReader, KeyboardNavigation } from '../utils/accessibility.js';

  // Props
  export let disabled = false;
  export let dragActive = false;
  export let selectedFile: File | null = null;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    fileSelect: { file: File };
    fileRemove: void;
    validationError: { errors: ValidationError[] };
    dragEnter: void;
    dragLeave: void;
  }>();

  // Local state
  let fileInput: HTMLInputElement;
  let validationErrors: ValidationError[] = [];
  let isDragOver = false;
  
  // Generate unique IDs for accessibility
  const dropZoneId = generateId('drop-zone');
  const instructionsId = generateId('upload-instructions');
  const errorsId = generateId('upload-errors');

  // Reactive statements
  $: dragActive = isDragOver;

  // File validation function
  function validateFile(file: File): ValidationError[] {
    const errors: ValidationError[] = [];

    try {
      FileUploadSchema.parse({ file });
    } catch (error) {
      if (error instanceof Error && 'issues' in error) {
        const zodError = error as any;
        errors.push(...zodError.issues.map((issue: any) => ({
          field: 'file',
          message: issue.message,
          code: issue.code
        })));
      }
    }

    // Additional validation using error handler for consistent error types
    if (file.size > env.MAX_FILE_SIZE) {
      const uploadError = errorHandler.createUploadError(
        'File is too large',
        file.name,
        'file_too_large',
        { fileSize: file.size, fileType: file.type }
      );
      errors.push({
        field: 'file',
        message: uploadError.message,
        code: 'file_too_large'
      });
    }

    const lowerName = file.name.toLowerCase();
    const isMarkdown = lowerName.endsWith('.md') || lowerName.endsWith('.markdown');
    if (!isMarkdown) {
      const uploadError = errorHandler.createUploadError(
        'Invalid file type. Please select a Markdown (.md, .markdown) file.',
        file.name,
        'invalid_type',
        { fileType: file.type }
      );
      errors.push({
        field: 'file',
        message: uploadError.message,
        code: 'invalid_type'
      });
    }

    return errors;
  }

  // Handle file selection
  function handleFileSelect(file: File) {
    const errors = validateFile(file);
    
    if (errors.length > 0) {
      validationErrors = errors;
      dispatch('validationError', { errors });
      
      // Announce validation errors to screen readers
      const errorMessages = errors.map(e => e.message).join('. ');
      announceToScreenReader(`File validation failed: ${errorMessages}`, 'assertive');
      return;
    }

    validationErrors = [];
    selectedFile = file;
    dispatch('fileSelect', { file });
    
    // Announce successful file selection
    announceToScreenReader(`File selected: ${file.name}, ${formatFileSize(file.size)}`, 'polite');
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
    if (fileInput) {
      fileInput.value = '';
    }
    dispatch('fileRemove');
    
    // Announce file removal to screen readers
    if (fileName) {
      announceToScreenReader(`File removed: ${fileName}`, 'polite');
    }
  }

  // Handle drag events
  function handleDragEnter(event: DragEvent) {
    event.preventDefault();
    event.stopPropagation();
    isDragOver = true;
    dispatch('dragEnter');
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
      // 드롭된 첫 번째 파일을 안전하게 가져옵니다. (TS: File | undefined 방지)
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
</script>

<div class="file-upload-container">
  <input
    bind:this={fileInput}
    type="file"
    accept=".md,.markdown"
    on:change={handleInputChange}
    style="display: none;"
    {disabled}
    aria-describedby={instructionsId}
    aria-invalid={validationErrors.length > 0}
  />

  <div
    id={dropZoneId}
    class="drop-zone min-touch-target focus-visible"
    class:drag-over={isDragOver}
    class:has-file={!!selectedFile}
    class:disabled
    on:click={handleClick}
    on:keydown={handleKeydown}
    on:dragenter={handleDragEnter}
    on:dragleave={handleDragLeave}
    on:dragover={handleDragOver}
    on:drop={handleDrop}
    role="button"
    tabindex={disabled ? -1 : 0}
    aria-label={selectedFile ? `Selected file: ${selectedFile.name}. Click to change file or remove.` : 'Upload PDF file. Click to browse or drag and drop.'}
    aria-describedby={`${instructionsId} ${validationErrors.length > 0 ? errorsId : ''}`}
    aria-disabled={disabled}
  >
    {#if selectedFile}
      <!-- Selected file display -->
      <div class="file-display">
        <FileText size={48} color="var(--color-primary-600)" />
        <div class="file-info">
          <h3 class="file-name" id={generateId('file-name')}>{selectedFile.name}</h3>
          <p class="file-size" id={generateId('file-size')}>{formatFileSize(selectedFile.size)}</p>
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
            aria-label="Choose a different PDF file"
          >
            <span>Choose Different File</span>
          </button>
        </div>
      </div>
    {:else}
      <!-- Upload prompt -->
      <div class="upload-prompt">
        <Upload 
          size={48} 
          color={isDragOver ? "var(--color-primary-600)" : "var(--color-surface-500)"} 
          aria-hidden="true"
        />
        <div class="upload-text">
          <h3 class="upload-title">
            {isDragOver ? 'Drop your Markdown file here' : 'Upload Markdown Document'}
          </h3>
          <p class="upload-description" id={instructionsId}>
            Drag and drop a Markdown file here, or click to browse. Only .md or .markdown files are accepted.
          </p>
          <p class="upload-limit">
            Maximum file size: {formatFileSize(env.MAX_FILE_SIZE)}
          </p>
        </div>
        <span class="btn btn-primary min-touch-target" aria-hidden="true">
          <Upload size={20} aria-hidden="true" />
          <span>Choose File</span>
        </span>
      </div>
    {/if}
  </div>

  <!-- Validation errors -->
  {#if validationErrors.length > 0}
    <div 
      id={errorsId}
      class="validation-errors" 
      role="alert" 
      aria-live="assertive"
      aria-atomic="true"
    >
      <h4 class="sr-only">File validation errors</h4>
      {#each validationErrors as error, index}
        <p class="error-message" id={generateId(`error-${index}`)}>
          <span class="sr-only">Error:</span>
          {error.message}
        </p>
      {/each}
    </div>
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
  }

  .file-info {
    text-align: center;
  }

  .file-name {
    margin: 0 0 0.25rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .file-size {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-surface-600);
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

  .validation-errors {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background-color: var(--color-error-50);
    border: 1px solid var(--color-error-200);
    border-radius: 0.375rem;
  }

  .error-message {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-error-700);
  }

  .error-message:not(:last-child) {
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

    .error-message {
      color: var(--color-error-400);
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
</style>