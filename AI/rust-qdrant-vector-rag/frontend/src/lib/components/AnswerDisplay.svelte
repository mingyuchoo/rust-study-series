<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { 
    Copy, 
    CheckCircle, 
    Brain, 
    Clock, 
    TrendingUp,
    Share,
    Bookmark
  } from 'lucide-svelte';
  import { announceToScreenReader } from '../utils/accessibility.js';
  import type { RAGResponse } from '../types/api.js';

  // Props
  export let response: RAGResponse;
  export let showMetadata = true;
  export let allowCopy = true;
  export let allowShare = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    copy: string;
    share: RAGResponse;
    bookmark: RAGResponse;
  }>();

  // Local state
  let copied = false;

  // Format response time
  $: formattedResponseTime = response.response_time_ms < 1000 
    ? `${response.response_time_ms}ms`
    : `${(response.response_time_ms / 1000).toFixed(1)}s`;

  // Format confidence as percentage
  $: confidencePercentage = Math.round(response.confidence * 100);

  // Get confidence color based on value
  $: confidenceColor = response.confidence >= 0.8 ? 'green' : 
                       response.confidence >= 0.6 ? 'yellow' : 
                       response.confidence >= 0.4 ? 'orange' : 'red';

  // Format timestamp
  $: formattedTimestamp = new Date(response.timestamp).toLocaleString();

  // Copy answer to clipboard
  async function copyAnswer() {
    if (!allowCopy) return;

    try {
      // Create a clean text version without HTML formatting
      const cleanText = response.answer
        .replace(/<[^>]*>/g, '') // Remove HTML tags
        .replace(/&nbsp;/g, ' ') // Replace non-breaking spaces
        .replace(/&amp;/g, '&') // Replace HTML entities
        .replace(/&lt;/g, '<')
        .replace(/&gt;/g, '>')
        .replace(/&quot;/g, '"')
        .trim();

      await navigator.clipboard.writeText(cleanText);
      copied = true;
      dispatch('copy', cleanText);
      
      // Announce to screen readers
      announceToScreenReader('Answer copied to clipboard', 'polite');
      
      // Reset copied state after 2 seconds
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
      // Fallback for older browsers
      try {
        const textArea = document.createElement('textarea');
        textArea.value = response.answer.replace(/<[^>]*>/g, '');
        document.body.appendChild(textArea);
        textArea.select();
        document.execCommand('copy');
        document.body.removeChild(textArea);
        
        copied = true;
        dispatch('copy', response.answer);
        announceToScreenReader('Answer copied to clipboard', 'polite');
        
        setTimeout(() => {
          copied = false;
        }, 2000);
      } catch (fallbackError) {
        console.error('Fallback copy failed:', fallbackError);
        announceToScreenReader('Failed to copy answer', 'assertive');
      }
    }
  }

  // Share response
  function shareResponse() {
    dispatch('share', response);
  }

  // Bookmark response
  function bookmarkResponse() {
    dispatch('bookmark', response);
  }

  // Format answer text with basic markdown-like formatting
  function formatAnswer(text: string): string {
    return text
      .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
      .replace(/\*(.*?)\*/g, '<em>$1</em>')
      .replace(/\n\n/g, '</p><p>')
      .replace(/\n/g, '<br>');
  }

  $: formattedAnswer = formatAnswer(response.answer);
</script>

<div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-sm answer-display">
  <div class="space-y-4">
    <!-- Header with metadata -->
    {#if showMetadata}
      <div class="flex justify-between items-start">
        <div class="flex items-center gap-2">
          <Brain size={20} class="text-blue-600 dark:text-blue-400" />
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">AI Response</h3>
        </div>
        
        <div class="flex items-center gap-2">
          <!-- Confidence badge -->
          <div class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded-full" 
               class:bg-green-100={confidenceColor === 'green'} 
               class:text-green-800={confidenceColor === 'green'}
               class:bg-yellow-100={confidenceColor === 'yellow'} 
               class:text-yellow-800={confidenceColor === 'yellow'}
               class:bg-orange-100={confidenceColor === 'orange'} 
               class:text-orange-800={confidenceColor === 'orange'}
               class:bg-red-100={confidenceColor === 'red'} 
               class:text-red-800={confidenceColor === 'red'}
               title="Response confidence level">
            <TrendingUp size={12} />
            {confidencePercentage}% confident
          </div>

          <!-- Response time -->
          <div class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-gray-700 bg-gray-200 rounded-full dark:bg-gray-600 dark:text-gray-200" 
               title="Response generation time">
            <Clock size={12} />
            {formattedResponseTime}
          </div>
        </div>
      </div>
    {/if}

    <!-- Main answer content -->
    <div class="answer-content" role="main" aria-label="AI generated answer">
      <div class="answer-text text-gray-900 dark:text-gray-100">
        {@html formattedAnswer}
      </div>
    </div>

    <!-- Action buttons -->
    <hr class="border-gray-200 dark:border-gray-700" />
    <div class="flex justify-between items-center">
      <div class="flex items-center gap-2">
        {#if allowCopy}
          <button
            type="button"
            on:click={copyAnswer}
            class="copy-button"
            class:copied
            title={copied ? 'Copied!' : 'Copy answer to clipboard'}
            aria-label={copied ? 'Answer copied to clipboard' : 'Copy answer to clipboard'}
          >
            {#if copied}
              <CheckCircle size={16} />
            {:else}
              <Copy size={16} />
            {/if}
            {copied ? 'Copied!' : 'Copy'}
          </button>
        {/if}

        {#if allowShare}
          <button
            type="button"
            on:click={shareResponse}
            class="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded-md transition-colors dark:text-gray-300 dark:hover:text-gray-100 dark:hover:bg-gray-600"
            title="Share this response"
            aria-label="Share response"
          >
            <Share size={16} />
          </button>
        {/if}

        <button
          type="button"
          on:click={bookmarkResponse}
          class="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded-md transition-colors dark:text-gray-300 dark:hover:text-gray-100 dark:hover:bg-gray-600"
          title="Bookmark this response"
          aria-label="Bookmark response"
        >
          <Bookmark size={16} />
        </button>
      </div>

      {#if showMetadata}
        <p class="text-xs text-gray-600">
          Generated at {formattedTimestamp}
        </p>
      {/if}
    </div>

    <!-- Query context (what was asked) -->
    <div class="query-context">
      <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
        Your question:
      </p>
      <p class="text-sm text-gray-700 dark:text-gray-300 italic">
        "{response.query}"
      </p>
    </div>
  </div>
</div>

<style>
  .answer-display {
    max-width: 100%;
    margin: 0 auto;
  }

  .answer-content {
    background: var(--color-surface-50);
    border-radius: 0.5rem;
    padding: 1.5rem;
    border-left: 4px solid var(--color-primary-500);
  }

  .answer-content :global(.answer-text) {
    line-height: 1.6;
    word-wrap: break-word;
    hyphens: auto;
  }

  .answer-content :global(.answer-text p) {
    margin: 0 0 1rem 0;
  }

  .answer-content :global(.answer-text p:last-child) {
    margin-bottom: 0;
  }

  .answer-content :global(.answer-text strong) {
    font-weight: 600;
    color: var(--color-surface-900);
  }

  .answer-content :global(.answer-text em) {
    font-style: italic;
    color: var(--color-surface-700);
  }

  .query-context {
    background: var(--color-surface-50);
    border-radius: 0.375rem;
    padding: 0.75rem;
    border: 1px solid var(--color-surface-200);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .answer-content {
      background: var(--color-surface-800);
      border-left-color: var(--color-primary-400);
    }

    .answer-content :global(.answer-text strong) {
      color: var(--color-surface-100);
    }

    .answer-content :global(.answer-text em) {
      color: var(--color-surface-300);
    }

    .query-context {
      background: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .answer-content {
      padding: 1rem;
    }

    .query-context {
      padding: 0.5rem;
    }
  }

  /* Print styles */
  @media print {
    .answer-display {
      box-shadow: none;
      border: 1px solid #ccc;
    }

    .answer-content {
      background: white;
      border-left: 2px solid #333;
    }

    /* Hide action buttons in print */
    .answer-display button {
      display: none;
    }
  }

  /* Accessibility improvements */
  .answer-content {
    scroll-margin-top: 2rem;
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .answer-content {
      border-left-width: 6px;
      border-left-color: #000;
    }

    .query-context {
      border-width: 2px;
      border-color: #000;
    }
  }

  /* Copy button styles */
  .copy-button {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.375rem;
    transition: all 0.2s ease;
    color: #1d4ed8;
    background-color: #dbeafe;
  }

  .copy-button:hover {
    background-color: #bfdbfe;
  }

  .copy-button.copied {
    color: #166534;
    background-color: #dcfce7;
  }

  .copy-button.copied:hover {
    background-color: #bbf7d0;
  }

  /* Dark mode for copy button */
  @media (prefers-color-scheme: dark) {
    .copy-button {
      color: #60a5fa;
      background-color: rgba(30, 58, 138, 0.2);
    }

    .copy-button:hover {
      background-color: rgba(30, 58, 138, 0.3);
    }

    .copy-button.copied {
      color: #4ade80;
      background-color: rgba(20, 83, 45, 0.2);
    }

    .copy-button.copied:hover {
      background-color: rgba(20, 83, 45, 0.3);
    }
  }

  /* Focus styles for interactive elements */
  .answer-display button:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }
</style>