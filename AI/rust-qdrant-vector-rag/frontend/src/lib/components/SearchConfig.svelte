<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { RotateCcw, Info, AlertCircle, CheckCircle, ChevronDown, ChevronUp } from 'lucide-svelte';
  import { QueryConfigSchema } from '../schemas/validation.js';
  import { 
    RealTimeValidator, 
    DebouncedValidation,
    debounce 
  } from '../utils/validation.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationError } from '../types/state.js';
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';

  // Props
  export let config: QueryConfig = {};
  export let visible = false;
  export let disabled = false;

  export let initiallyExpanded = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    'config-change': QueryConfig;
    'validation-change': { isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] };
    reset: void;
  }>();

  // Default values
  const defaultConfig: Required<QueryConfig> = {
    max_chunks: 5,
    similarity_threshold: 0.7,
    max_response_tokens: 500,
    temperature: 0.3,
    include_low_confidence: false
  };

  // Local state with defaults
  let localConfig: Required<QueryConfig> = {
    max_chunks: config.max_chunks ?? defaultConfig.max_chunks,
    similarity_threshold: config.similarity_threshold ?? defaultConfig.similarity_threshold,
    max_response_tokens: config.max_response_tokens ?? defaultConfig.max_response_tokens,
    temperature: config.temperature ?? defaultConfig.temperature,
    include_low_confidence: config.include_low_confidence ?? defaultConfig.include_low_confidence
  };

  let validationErrors: ValidationError[] = [];
  let validationWarnings: ValidationError[] = [];
  let fieldValidationState: Record<string, { isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] }> = {};
  let isExpanded = initiallyExpanded;
  
  // Generate unique IDs for accessibility
  const configId = generateId('search-config');
  const errorsId = generateId('config-errors');

  // Sync with external config changes
  $: {
    localConfig = {
      max_chunks: config.max_chunks ?? defaultConfig.max_chunks,
      similarity_threshold: config.similarity_threshold ?? defaultConfig.similarity_threshold,
      max_response_tokens: config.max_response_tokens ?? defaultConfig.max_response_tokens,
      temperature: config.temperature ?? defaultConfig.temperature,
      include_low_confidence: config.include_low_confidence ?? defaultConfig.include_low_confidence
    };
  }

  // Debounced validation for each field
  const debouncedFieldValidation = debounce((field: string, value: number, min: number, max: number, step?: number) => {
    DebouncedValidation.validateConfigParameterDebounced(field, value, min, max, step, (result) => {
      fieldValidationState[field] = result;
      fieldValidationState = { ...fieldValidationState };
      
      // Aggregate all field validation results
      updateOverallValidation();
    });
  }, 200);

  // Update overall validation state
  function updateOverallValidation() {
    const allErrors: ValidationError[] = [];
    const allWarnings: ValidationError[] = [];
    let isOverallValid = true;

    for (const [field, state] of Object.entries(fieldValidationState)) {
      allErrors.push(...state.errors);
      allWarnings.push(...state.warnings);
      if (!state.isValid) {
        isOverallValid = false;
      }
    }

    validationErrors = allErrors;
    validationWarnings = allWarnings;

    // Emit validation state change
    dispatch('validation-change', {
      isValid: isOverallValid,
      errors: allErrors,
      warnings: allWarnings
    });

    // If valid, emit config change
    if (isOverallValid) {
      try {
        const validated = QueryConfigSchema.parse(localConfig);
        dispatch('config-change', validated);
      } catch (error) {
        console.error('Config validation error:', error);
      }
    }
  }

  // Handle individual config changes with validation
  function updateConfig<K extends keyof QueryConfig>(key: K, value: QueryConfig[K]) {
    localConfig = { ...localConfig, [key]: value };
    
    // Validate numeric fields
    if (typeof value === 'number') {
      const validationRules = getValidationRules(key as string);
      if (validationRules) {
        debouncedFieldValidation(
          key as string, 
          value, 
          validationRules.min, 
          validationRules.max, 
          validationRules.step
        );
      }
    } else {
      // For non-numeric fields, validate immediately
      updateOverallValidation();
    }
  }

  // Get validation rules for each field
  function getValidationRules(field: string): { min: number; max: number; step?: number } | null {
    switch (field) {
      case 'max_chunks':
        return { min: 1, max: 20, step: 1 };
      case 'similarity_threshold':
        return { min: 0.0, max: 1.0, step: 0.05 };
      case 'max_response_tokens':
        return { min: 50, max: 4000, step: 50 };
      case 'temperature':
        return { min: 0.0, max: 1.0, step: 0.1 };
      default:
        return null;
    }
  }

  // Toggle expanded state
  function toggleExpanded() {
    isExpanded = !isExpanded;
    announceToScreenReader(
      isExpanded ? 'Advanced options expanded' : 'Advanced options collapsed',
      'polite'
    );
  }

  // Reset to defaults
  function resetConfig() {
    localConfig = { ...defaultConfig };
    fieldValidationState = {};
    validationErrors = [];
    validationWarnings = [];
    
    // Re-validate all fields
    for (const [key, value] of Object.entries(localConfig)) {
      if (typeof value === 'number') {
        const rules = getValidationRules(key);
        if (rules) {
          const result = RealTimeValidator.validateConfigParameter(key, value, rules.min, rules.max, rules.step);
          fieldValidationState[key] = result;
        }
      }
    }
    
    updateOverallValidation();
    dispatch('reset');
    
    // Announce reset to screen readers
    announceToScreenReader('Configuration reset to default values', 'polite');
  }

  // Helper functions to get validation state for a field
  function getFieldError(field: string): string | undefined {
    return fieldValidationState[field]?.errors[0]?.message;
  }

  function getFieldWarning(field: string): string | undefined {
    return fieldValidationState[field]?.warnings[0]?.message;
  }

  function isFieldValid(field: string): boolean {
    return fieldValidationState[field]?.isValid ?? true;
  }

  function hasFieldWarning(field: string): boolean {
    return (fieldValidationState[field]?.warnings.length ?? 0) > 0;
  }

  // Cleanup on destroy
  onDestroy(() => {
    debouncedFieldValidation.cancel();
  });
</script>

{#if visible}
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm" id="advanced-options">
    <div class="space-y-6">
      <!-- Collapsible header -->
      <div class="flex justify-between items-center p-6 pb-0">
        <button
          type="button"
          class="flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
          on:click={toggleExpanded}
          aria-expanded={isExpanded}
          aria-controls="config-content"
          {disabled}
        >
          {#if isExpanded}
            <ChevronUp size={20} />
          {:else}
            <ChevronDown size={20} />
          {/if}
          Advanced Search Configuration
        </button>
        
        {#if isExpanded}
          <button
            type="button"
            on:click={resetConfig}
            class="inline-flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700"
            {disabled}
            aria-label="Reset configuration to defaults"
          >
            <RotateCcw size={16} />
            Reset to Defaults
          </button>
        {/if}
      </div>

      {#if isExpanded}
        <div id="config-content" class="px-6 pb-6">
          <hr class="border-gray-200 dark:border-gray-700 mb-6" />

      <hr class="border-gray-200 dark:border-gray-700" />

      <!-- Max Chunks Configuration -->
      <div class="config-section">
        <div class="flex flex-col md:flex-row md:justify-between md:items-start gap-4">
          <div class="config-label">
            <h4 class="font-medium text-gray-900 dark:text-white">Maximum Chunks</h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Number of document chunks to retrieve for context
            </p>
          </div>
          <div class="config-input">
            <div class="relative">
              <input
                type="number"
                bind:value={localConfig.max_chunks}
                on:input={(e) => updateConfig('max_chunks', parseInt(e.target.value))}
                min="1"
                max="20"
                step="1"
                {disabled}
                class="w-full px-3 py-2 pr-8 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                class:border-red-500={getFieldError('max_chunks')}
                class:border-orange-400={hasFieldWarning('max_chunks') && !getFieldError('max_chunks')}
                class:border-green-500={isFieldValid('max_chunks') && localConfig.max_chunks > 0}
                aria-label="Maximum chunks"
                aria-describedby="max-chunks-help max-chunks-validation"
                aria-invalid={!!getFieldError('max_chunks')}
              />
              <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                {#if getFieldError('max_chunks')}
                  <AlertCircle size={16} class="text-red-500" aria-hidden="true" />
                {:else if hasFieldWarning('max_chunks')}
                  <AlertCircle size={16} class="text-orange-500" aria-hidden="true" />
                {:else if isFieldValid('max_chunks') && localConfig.max_chunks > 0}
                  <CheckCircle size={16} class="text-green-500" aria-hidden="true" />
                {/if}
              </div>
            </div>
            <p id="max-chunks-help" class="text-xs text-gray-500 mt-1">
              Range: 1-20 chunks
            </p>
            <div id="max-chunks-validation" class="mt-1">
              {#if getFieldError('max_chunks')}
                <p class="text-sm text-red-600 dark:text-red-400 flex items-center gap-1">
                  <AlertCircle size={14} aria-hidden="true" />
                  {getFieldError('max_chunks')}
                </p>
              {:else if getFieldWarning('max_chunks')}
                <p class="text-sm text-orange-600 dark:text-orange-400 flex items-center gap-1">
                  <AlertCircle size={14} aria-hidden="true" />
                  {getFieldWarning('max_chunks')}
                </p>
              {/if}
            </div>
          </div>
        </div>
      </div>

      <!-- Similarity Threshold Configuration -->
      <div class="config-section">
        <div class="flex flex-col md:flex-row md:justify-between md:items-start gap-4">
          <div class="config-label">
            <h4 class="font-medium text-gray-900 dark:text-white">Similarity Threshold</h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Minimum relevance score for including chunks
            </p>
          </div>
          <div class="config-input">
            <div class="space-y-2">
              <input
                type="range"
                bind:value={localConfig.similarity_threshold}
                on:input={(e) => updateConfig('similarity_threshold', parseFloat(e.target.value))}
                min="0.0"
                max="1.0"
                step="0.05"
                {disabled}
                class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 slider"
                class:slider-error={getFieldError('similarity_threshold')}
                class:slider-warning={hasFieldWarning('similarity_threshold') && !getFieldError('similarity_threshold')}
                class:slider-success={isFieldValid('similarity_threshold')}
                aria-label="Similarity threshold"
                aria-describedby="similarity-help similarity-validation"
                aria-invalid={!!getFieldError('similarity_threshold')}
              />
              <div class="flex justify-between text-xs text-gray-500">
                <span>0.0</span>
                <span>0.5</span>
                <span>1.0</span>
              </div>
              <p class="text-sm text-center text-gray-700 dark:text-gray-300 flex items-center justify-center gap-1">
                Current: {localConfig.similarity_threshold.toFixed(2)}
                {#if getFieldError('similarity_threshold')}
                  <AlertCircle size={14} class="text-red-500" aria-hidden="true" />
                {:else if hasFieldWarning('similarity_threshold')}
                  <AlertCircle size={14} class="text-orange-500" aria-hidden="true" />
                {:else if isFieldValid('similarity_threshold')}
                  <CheckCircle size={14} class="text-green-500" aria-hidden="true" />
                {/if}
              </p>
              <p id="similarity-help" class="text-xs text-gray-500">
                Higher values = more relevant results
              </p>
              <div id="similarity-validation" class="mt-1">
                {#if getFieldError('similarity_threshold')}
                  <p class="text-sm text-red-600 dark:text-red-400 flex items-center gap-1">
                    <AlertCircle size={14} aria-hidden="true" />
                    {getFieldError('similarity_threshold')}
                  </p>
                {:else if getFieldWarning('similarity_threshold')}
                  <p class="text-sm text-orange-600 dark:text-orange-400 flex items-center gap-1">
                    <AlertCircle size={14} aria-hidden="true" />
                    {getFieldWarning('similarity_threshold')}
                  </p>
                {/if}
              </div>
            </div>
            {#if getFieldError('similarity_threshold')}
              <p class="text-sm text-red-600 dark:text-red-400 mt-1">{getFieldError('similarity_threshold')}</p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Max Response Tokens Configuration -->
      <div class="config-section">
        <div class="flex flex-col md:flex-row md:justify-between md:items-start gap-4">
          <div class="config-label">
            <h4 class="font-medium text-gray-900 dark:text-white">Max Response Tokens</h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Maximum length of AI response
            </p>
          </div>
          <div class="config-input">
            <input
              type="number"
              bind:value={localConfig.max_response_tokens}
              on:input={(e) => updateConfig('max_response_tokens', parseInt(e.target.value))}
              min="50"
              max="4000"
              step="50"
              {disabled}
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              class:border-red-500={getFieldError('max_response_tokens')}
              aria-label="Maximum response tokens"
              aria-describedby="tokens-help"
            />
            <p id="tokens-help" class="text-xs text-gray-500 mt-1">
              Range: 50-4000 tokens (~{Math.round(localConfig.max_response_tokens * 0.75)} words)
            </p>
            {#if getFieldError('max_response_tokens')}
              <p class="text-sm text-red-600 dark:text-red-400 mt-1">{getFieldError('max_response_tokens')}</p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Temperature Configuration -->
      <div class="config-section">
        <div class="flex flex-col md:flex-row md:justify-between md:items-start gap-4">
          <div class="config-label">
            <h4 class="font-medium text-gray-900 dark:text-white">Temperature</h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Controls randomness in AI responses
            </p>
          </div>
          <div class="config-input">
            <div class="space-y-2">
              <input
                type="range"
                bind:value={localConfig.temperature}
                on:input={(e) => updateConfig('temperature', parseFloat(e.target.value))}
                min="0.0"
                max="1.0"
                step="0.1"
                {disabled}
                class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 slider"
                aria-label="Temperature setting"
                aria-describedby="temperature-help"
              />
              <div class="flex justify-between text-xs text-gray-500">
                <span>Precise</span>
                <span>Balanced</span>
                <span>Creative</span>
              </div>
              <p class="text-sm text-center text-gray-700 dark:text-gray-300">
                Current: {localConfig.temperature.toFixed(1)}
              </p>
              <p id="temperature-help" class="text-xs text-gray-500">
                Lower = more focused, Higher = more creative
              </p>
            </div>
            {#if getFieldError('temperature')}
              <p class="text-sm text-red-600 dark:text-red-400 mt-1">{getFieldError('temperature')}</p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Include Low Confidence Configuration -->
      <div class="config-section">
        <div class="flex justify-between items-center">
          <div class="config-label">
            <h4 class="font-medium text-gray-900 dark:text-white">Include Low Confidence Results</h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Include results with lower relevance scores
            </p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              bind:checked={localConfig.include_low_confidence}
              on:change={(e) => updateConfig('include_low_confidence', e.target.checked)}
              {disabled}
              class="sr-only peer"
              aria-label="Include low confidence results"
              aria-describedby="confidence-help"
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
          </label>
        </div>
        <p id="confidence-help" class="text-xs text-gray-500 flex items-center gap-1 mt-2">
          <Info size={12} />
          May include less relevant but potentially useful information
        </p>
      </div>

      <!-- Validation Errors -->
      {#if validationErrors.length > 0}
        <div class="validation-errors bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4" role="alert" aria-live="polite">
          <h5 class="font-medium text-red-800 dark:text-red-400 mb-2">Configuration Errors:</h5>
          {#each validationErrors as error}
            <p class="text-sm text-red-700 dark:text-red-300">
              • {error.message}
            </p>
          {/each}
        </div>
      {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .config-section {
    padding: 1rem 0;
  }

  .config-section:not(:last-child) {
    border-bottom: 1px solid var(--color-surface-200);
  }

  .config-label {
    flex: 1;
    min-width: 200px;
  }

  .config-input {
    flex: 0 0 250px;
    min-width: 200px;
  }

  /* Custom slider styles */
  .slider::-webkit-slider-thumb {
    appearance: none;
    height: 20px;
    width: 20px;
    border-radius: 50%;
    background: #2563eb;
    cursor: pointer;
    border: 2px solid #ffffff;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .slider::-moz-range-thumb {
    height: 20px;
    width: 20px;
    border-radius: 50%;
    background: #2563eb;
    cursor: pointer;
    border: 2px solid #ffffff;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .slider:disabled::-webkit-slider-thumb {
    background: #9ca3af;
    cursor: not-allowed;
  }

  .slider:disabled::-moz-range-thumb {
    background: #9ca3af;
    cursor: not-allowed;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .config-section:not(:last-child) {
      border-bottom-color: var(--color-surface-700);
    }

    .slider::-webkit-slider-thumb {
      background: #3b82f6;
    }

    .slider::-moz-range-thumb {
      background: #3b82f6;
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .config-input {
      flex: 1;
      width: 100%;
      min-width: unset;
    }

    .config-label {
      min-width: unset;
    }
  }
</style>