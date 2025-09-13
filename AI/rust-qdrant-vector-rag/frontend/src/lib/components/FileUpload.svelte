<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Upload, FileText, X } from 'lucide-svelte';
  import { env, formatFileSize } from '../config/env.js';
  import { FileUploadSchema } from '../schemas/validation.js';
  import type { ValidationError } from '../types/state.js';

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

    return errors;
  }

  // Handle file selection
  function handleFileSelect(file: File) {
    const errors = validateFile(file);
    
    if (errors.length > 0) {
      validationErrors = errors;
      dispatch('validationError', { errors });
      return;
    }

    validationErrors = [];
    selectedFile = file;
    dispatch('fileSelect', { file });
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
    selectedFile = null;
    validationErrors = [];
    if (fileInput) {
      fileInput.value = '';
    }
    dispatch('fileRemove');
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
</script>

<div class="file-upload-container">
  <input
    bind:this={fileInput}
    type="file"
    accept=".pdf"
    on:change={handleInputChange}
    style="display: none;"
    {disabled}
  />

  <div
    class="drop-zone {isDragOver ? 'drag-over' : ''} {selectedFile ? 'has-file' : ''}"
    on:click={handleClick}
    on:keydown={(e) => e.key === 'Enter' || e.key === ' ' ? handleClick() : null}
    on:dragenter={handleDragEnter}
    on:dragleave={handleDragLeave}
    on:dragover={handleDragOver}
    on:drop={handleDrop}
    role="button"
    tabindex={disabled ? -1 : 0}
    aria-label="Upload PDF file"
    aria-describedby="upload-instructions"
  >
    {#if selectedFile}
      <!-- Selected file display -->
      <div class="file-display">
        <FileText size={48} color="var(--color-primary-600)" />
        <div class="file-info">
          <h3 class="file-name">{selectedFile.name}</h3>
          <p class="file-size">{formatFileSize(selectedFile.size)}</p>
        </div>
        <div class="file-actions">
          <button
            class="btn btn-danger btn-sm"
            on:click|stopPropagation={handleFileRemove}
            {disabled}
          >
            <X size={16} />
            Remove
          </button>
          <button
            class="btn btn-secondary btn-sm"
            on:click|stopPropagation={handleClick}
            {disabled}
          >
            Choose Different File
          </button>
        </div>
      </div>
    {:else}
      <!-- Upload prompt -->
      <div class="upload-prompt">
        <Upload 
          size={48} 
          color={isDragOver ? "var(--color-primary-600)" : "var(--color-surface-500)"} 
        />
        <div class="upload-text">
          <h3 class="upload-title">
            {isDragOver ? 'Drop your PDF file here' : 'Upload PDF Document'}
          </h3>
          <p class="upload-description" id="upload-instructions">
            Drag and drop a PDF file here, or click to browse
          </p>
          <p class="upload-limit">
            Maximum file size: {formatFileSize(env.MAX_FILE_SIZE)}
          </p>
        </div>
        <button
          class="btn btn-primary"
          {disabled}
        >
          <Upload size={20} />
          Choose File
        </button>
      </div>
    {/if}
  </div>

  <!-- Validation errors -->
  {#if validationErrors.length > 0}
    <div class="validation-errors">
      {#each validationErrors as error}
        <p class="error-message">{error.message}</p>
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