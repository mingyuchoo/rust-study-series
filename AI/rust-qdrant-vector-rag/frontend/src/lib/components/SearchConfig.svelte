<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { RotateCcw, Info } from 'lucide-svelte';
  import { QueryConfigSchema } from '../schemas/validation.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationErrorInput } from '../schemas/validation.js';

  // Props
  export let config: QueryConfig = {};
  export let visible = false;
  export let disabled = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    'config-change': QueryConfig;
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

  let validationErrors: ValidationErrorInput[] = [];

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

  // Validate and emit changes
  $: {
    validationErrors = [];
    try {
      const validated = QueryConfigSchema.parse(localConfig);
      dispatch('config-change', validated);
    } catch (error) {
      if (error instanceof Error && 'issues' in error) {
        const zodError = error as any;
        validationErrors = zodError.issues.map((issue: any) => ({
          field: issue.path.join('.'),
          message: issue.message
        }));
      }
    }
  }

  // Handle individual config changes
  function updateConfig<K extends keyof QueryConfig>(key: K, value: QueryConfig[K]) {
    localConfig = { ...localConfig, [key]: value };
  }

  // Reset to defaults
  function resetConfig() {
    localConfig = { ...defaultConfig };
    dispatch('reset');
  }

  // Helper function to get validation error for a field
  function getFieldError(field: string): string | undefined {
    return validationErrors.find(e => e.field === field)?.message;
  }
</script>

{#if visible}
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-sm" id="advanced-options">
    <div class="space-y-6">
      <div class="flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Advanced Search Configuration</h3>
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
      </div>

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
            <input
              type="number"
              bind:value={localConfig.max_chunks}
              on:input={(e) => updateConfig('max_chunks', parseInt(e.target.value))}
              min="1"
              max="20"
              step="1"
              {disabled}
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              class:border-red-500={getFieldError('max_chunks')}
              aria-label="Maximum chunks"
              aria-describedby="max-chunks-help"
            />
            <p id="max-chunks-help" class="text-xs text-gray-500 mt-1">
              Range: 1-20 chunks
            </p>
            {#if getFieldError('max_chunks')}
              <p class="text-sm text-red-600 dark:text-red-400 mt-1">{getFieldError('max_chunks')}</p>
            {/if}
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
                aria-label="Similarity threshold"
                aria-describedby="similarity-help"
              />
              <div class="flex justify-between text-xs text-gray-500">
                <span>0.0</span>
                <span>0.5</span>
                <span>1.0</span>
              </div>
              <p class="text-sm text-center text-gray-700 dark:text-gray-300">
                Current: {localConfig.similarity_threshold.toFixed(2)}
              </p>
              <p id="similarity-help" class="text-xs text-gray-500">
                Higher values = more relevant results
              </p>
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