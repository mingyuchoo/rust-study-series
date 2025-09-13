<script lang="ts">
  import { onMount } from 'svelte';
  import ValidatedFileUpload from './ValidatedFileUpload.svelte';
  import SearchForm from './SearchForm.svelte';
  import SearchConfig from './SearchConfig.svelte';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationError } from '../types/state.js';

  // Demo state
  let selectedFile: File | null = null;
  let searchQuery = '';
  let showAdvanced = false;
  let searchConfig: QueryConfig = {};
  let validationResults: {
    fileErrors: ValidationError[];
    searchErrors: ValidationError[];
    searchWarnings: ValidationError[];
    configErrors: ValidationError[];
    configWarnings: ValidationError[];
  } = {
    fileErrors: [],
    searchErrors: [],
    searchWarnings: [],
    configErrors: [],
    configWarnings: []
  };

  // Handle file upload events
  function handleFileSelect(event: CustomEvent<{ file: File; sanitizedFilename: string }>) {
    selectedFile = event.detail.file;
    console.log('File selected:', event.detail.file.name, 'Sanitized:', event.detail.sanitizedFilename);
  }

  function handleFileRemove() {
    selectedFile = null;
    validationResults.fileErrors = [];
    console.log('File removed');
  }

  function handleFileValidationError(event: CustomEvent<{ errors: ValidationError[] }>) {
    validationResults.fileErrors = event.detail.errors;
    console.log('File validation errors:', event.detail.errors);
  }

  function handleFileValidationSuccess(event: CustomEvent<{ file: File }>) {
    validationResults.fileErrors = [];
    console.log('File validation success:', event.detail.file.name);
  }

  // Handle search form events
  function handleSearchSubmit(event: CustomEvent<{ query: string; config?: QueryConfig }>) {
    console.log('Search submitted:', event.detail.query, event.detail.config);
  }

  function handleSearchValidationChange(event: CustomEvent<{ isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] }>) {
    validationResults.searchErrors = event.detail.errors;
    validationResults.searchWarnings = event.detail.warnings;
    console.log('Search validation changed:', event.detail);
  }

  function handleQueryChange(event: CustomEvent<string>) {
    searchQuery = event.detail;
  }

  function handleToggleAdvanced(event: CustomEvent<boolean>) {
    showAdvanced = event.detail;
  }

  // Handle config events
  function handleConfigChange(event: CustomEvent<QueryConfig>) {
    searchConfig = event.detail;
    console.log('Config changed:', event.detail);
  }

  function handleConfigValidationChange(event: CustomEvent<{ isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] }>) {
    validationResults.configErrors = event.detail.errors;
    validationResults.configWarnings = event.detail.warnings;
    console.log('Config validation changed:', event.detail);
  }

  function handleConfigReset() {
    console.log('Config reset to defaults');
  }
</script>

<div class="validation-demo">
  <div class="demo-header">
    <h1>Client-Side Validation Demo</h1>
    <p>This demo showcases the comprehensive client-side validation implementation for file uploads, search queries, and configuration parameters.</p>
  </div>

  <div class="demo-sections">
    <!-- File Upload Validation Demo -->
    <section class="demo-section">
      <h2>File Upload Validation</h2>
      <p>Try uploading different file types, sizes, and names to see validation in action:</p>
      
      <ValidatedFileUpload
        bind:selectedFile
        on:fileSelect={handleFileSelect}
        on:fileRemove={handleFileRemove}
        on:validationError={handleFileValidationError}
        on:validationSuccess={handleFileValidationSuccess}
      />

      {#if validationResults.fileErrors.length > 0}
        <div class="validation-summary error">
          <h3>File Validation Errors:</h3>
          <ul>
            {#each validationResults.fileErrors as error}
              <li>{error.message} (Code: {error.code})</li>
            {/each}
          </ul>
        </div>
      {/if}
    </section>

    <!-- Search Query Validation Demo -->
    <section class="demo-section">
      <h2>Search Query Validation</h2>
      <p>Type different queries to see real-time validation, character counting, and input sanitization:</p>
      
      <SearchForm
        bind:query={searchQuery}
        bind:showAdvanced
        bind:config={searchConfig}
        on:submit={handleSearchSubmit}
        on:query-change={handleQueryChange}
        on:toggle-advanced={handleToggleAdvanced}
        on:validation-change={handleSearchValidationChange}
      />

      <div class="validation-summary-grid">
        {#if validationResults.searchErrors.length > 0}
          <div class="validation-summary error">
            <h3>Search Validation Errors:</h3>
            <ul>
              {#each validationResults.searchErrors as error}
                <li>{error.message} (Code: {error.code})</li>
              {/each}
            </ul>
          </div>
        {/if}

        {#if validationResults.searchWarnings.length > 0}
          <div class="validation-summary warning">
            <h3>Search Validation Warnings:</h3>
            <ul>
              {#each validationResults.searchWarnings as warning}
                <li>{warning.message} (Code: {warning.code})</li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </section>

    <!-- Configuration Validation Demo -->
    <section class="demo-section">
      <h2>Configuration Parameter Validation</h2>
      <p>Adjust the configuration parameters to see validation feedback and warnings:</p>
      
      <SearchConfig
        bind:config={searchConfig}
        bind:visible={showAdvanced}
        on:config-change={handleConfigChange}
        on:validation-change={handleConfigValidationChange}
        on:reset={handleConfigReset}
      />

      <div class="validation-summary-grid">
        {#if validationResults.configErrors.length > 0}
          <div class="validation-summary error">
            <h3>Config Validation Errors:</h3>
            <ul>
              {#each validationResults.configErrors as error}
                <li>{error.message} (Code: {error.code})</li>
              {/each}
            </ul>
          </div>
        {/if}

        {#if validationResults.configWarnings.length > 0}
          <div class="validation-summary warning">
            <h3>Config Validation Warnings:</h3>
            <ul>
              {#each validationResults.configWarnings as warning}
                <li>{warning.message} (Code: {warning.code})</li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </section>

    <!-- Current State Display -->
    <section class="demo-section">
      <h2>Current State</h2>
      <div class="state-display">
        <div class="state-item">
          <h3>Selected File:</h3>
          <pre>{selectedFile ? JSON.stringify({
            name: selectedFile.name,
            size: selectedFile.size,
            type: selectedFile.type
          }, null, 2) : 'None'}</pre>
        </div>
        
        <div class="state-item">
          <h3>Search Query:</h3>
          <pre>{JSON.stringify(searchQuery, null, 2)}</pre>
        </div>
        
        <div class="state-item">
          <h3>Search Config:</h3>
          <pre>{JSON.stringify(searchConfig, null, 2)}</pre>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .validation-demo {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    font-family: system-ui, -apple-system, sans-serif;
  }

  .demo-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .demo-header h1 {
    font-size: 2.5rem;
    color: var(--color-surface-900, #1a1a1a);
    margin-bottom: 1rem;
  }

  .demo-header p {
    font-size: 1.125rem;
    color: var(--color-surface-600, #666);
    max-width: 600px;
    margin: 0 auto;
  }

  .demo-sections {
    display: flex;
    flex-direction: column;
    gap: 3rem;
  }

  .demo-section {
    background: var(--color-surface-50, #f9f9f9);
    border-radius: 0.75rem;
    padding: 2rem;
    border: 1px solid var(--color-surface-200, #e5e5e5);
  }

  .demo-section h2 {
    font-size: 1.75rem;
    color: var(--color-surface-900, #1a1a1a);
    margin-bottom: 0.5rem;
  }

  .demo-section p {
    color: var(--color-surface-600, #666);
    margin-bottom: 1.5rem;
  }

  .validation-summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .validation-summary {
    padding: 1rem;
    border-radius: 0.5rem;
    border: 1px solid;
  }

  .validation-summary.error {
    background-color: var(--color-error-50, #fef2f2);
    border-color: var(--color-error-200, #fecaca);
    color: var(--color-error-800, #991b1b);
  }

  .validation-summary.warning {
    background-color: var(--color-warning-50, #fffbeb);
    border-color: var(--color-warning-200, #fed7aa);
    color: var(--color-warning-800, #92400e);
  }

  .validation-summary h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .validation-summary ul {
    margin: 0;
    padding-left: 1.25rem;
  }

  .validation-summary li {
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
  }

  .state-display {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .state-item {
    background: white;
    border-radius: 0.5rem;
    padding: 1rem;
    border: 1px solid var(--color-surface-200, #e5e5e5);
  }

  .state-item h3 {
    margin: 0 0 0.75rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-surface-900, #1a1a1a);
  }

  .state-item pre {
    background: var(--color-surface-100, #f5f5f5);
    border-radius: 0.375rem;
    padding: 0.75rem;
    font-size: 0.8125rem;
    overflow-x: auto;
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .demo-section {
      background: var(--color-surface-800, #2a2a2a);
      border-color: var(--color-surface-700, #404040);
    }

    .demo-header h1,
    .demo-section h2 {
      color: var(--color-surface-100, #f5f5f5);
    }

    .demo-header p,
    .demo-section p {
      color: var(--color-surface-400, #a0a0a0);
    }

    .state-item {
      background: var(--color-surface-700, #404040);
      border-color: var(--color-surface-600, #525252);
    }

    .state-item h3 {
      color: var(--color-surface-100, #f5f5f5);
    }

    .state-item pre {
      background: var(--color-surface-800, #2a2a2a);
      color: var(--color-surface-200, #e5e5e5);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .validation-demo {
      padding: 1rem;
    }

    .demo-header h1 {
      font-size: 2rem;
    }

    .demo-section {
      padding: 1.5rem;
    }

    .validation-summary-grid,
    .state-display {
      grid-template-columns: 1fr;
    }
  }
</style>