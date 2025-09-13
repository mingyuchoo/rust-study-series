<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { RotateCcw, Info, AlertCircle, CheckCircle } from 'lucide-svelte';
  import { RealTimeValidator, DebouncedValidation, FormValidationManager } from '../utils/validation.js';
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';
  import type { QueryConfig } from '../types/api.js';
  import type { ValidationError } from '../types/state.js';

  // Props
  export let config: QueryConfig = {};
  export let visible = false;
  export let disabled = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    'config-change': QueryConfig;
    reset: void;
    'validation-change': { isValid: boolean; errors: ValidationError[]; warnings: ValidationError[] };
  }>();

  // Default values with validation rules
  const configRules = {
    max_chunks: { min: 1, max: 20, step: 1, default: 5 },
    similarity_threshold: { min: 0.0, max: 1.0, step: 0.05, default: 0.7 },
    max_response_tokens: { min: 50, max: 4000, step: 50, default: 500 },
    temperature: { min: 0.0, max: 1.0, step: 0.1, default: 0.3 },
    include_low_confidence: { default: false }
  };

  // Local state
  let localConfig: Required<QueryConfig> = {
    max_chunks: config.max_chunks ?? configRules.max_chunks.default,
    similarity_threshold: config.similarity_threshold ?? configRules.similarity_threshold.default,
    max_response_tokens: config.max_response_tokens ?? configRules.max_response_tokens.default,
    temperature: config.temperature ?? configRules.temperature.default,
    include_low_confidence: config.include_low_confidence ?? configRules.include_low_confidence.default
  };

  let fieldValidationState: Record<string, { 
    errors: ValidationError[]; 
    warnings: ValidationError[];
    isValidating: boolean;
    sanitizedValue: number;
  }> = {};
  let formValidationManager: FormValidationManager<Required<QueryConfig>>;
  let allErrors: ValidationError[] = [];
  let allWarnings: ValidationError[] = [];

  // Generate unique IDs for accessibility
  const formId = generateId('config-form');
  const maxChunksId = generateId('max-chunks');
  const similarityId = generateId('similarity');
  const tokensId = generateId('tokens');
  const temperatureId = generateId('temperature');
  const confidenceId = generateId('confidence');

  // Initialize form validation manager
  $: {
    if (!formValidationManager) {
      formValidationManager = new FormValidationManager(localConfig);
      
      // Add validators for each field
      formValidationManager.addValidator('max_chunks', (value) => 
        RealTimeValidator.validateConfigParameter('max_chunks', value, 
          configRules.max_chunks.min, configRules.max_chunks.max, configRules.max_chunks.step)
      );
      
      form