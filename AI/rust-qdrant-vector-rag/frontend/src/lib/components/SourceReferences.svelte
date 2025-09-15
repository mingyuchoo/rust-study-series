<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { 
    FileText, 
    ChevronDown, 
    ChevronUp, 
    ExternalLink,
    Hash,
    TrendingUp,
    Copy,
    CheckCircle
  } from 'lucide-svelte';
  import { announceToScreenReader } from '../utils/accessibility.js';
  import type { SourceReference } from '../types/api.js';

  // Props
  export let sources: SourceReference[] = [];
  export let maxVisible = 3;
  export let allowExpansion = true;
  export let showSnippets = true;
  export let highlightQuery = '';

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    'source-click': SourceReference;
    'snippet-copy': { source: SourceReference; snippet: string };
    'view-document': string; // document_id
  }>();

  // Local state
  let expandedSources = new Set<string>();
  let showAllSources = false;
  let copiedSnippets = new Set<string>();

  // Computed values
  $: visibleSources = showAllSources ? sources : sources.slice(0, maxVisible);
  $: hasMoreSources = sources.length > maxVisible;
  $: sortedSources = [...sources].sort((a, b) => b.relevance_score - a.relevance_score);

  // Toggle source expansion
  function toggleSourceExpansion(sourceId: string) {
    if (expandedSources.has(sourceId)) {
      expandedSources.delete(sourceId);
      announceToScreenReader('Snippet collapsed', 'polite');
    } else {
      expandedSources.add(sourceId);
      announceToScreenReader('Snippet expanded', 'polite');
    }
    expandedSources = expandedSources; // Trigger reactivity
  }

  // Handle source click
  function handleSourceClick(source: SourceReference) {
    dispatch('source-click', source);
  }

  // Copy snippet to clipboard
  async function copySnippet(source: SourceReference) {
    try {
      await navigator.clipboard.writeText(source.snippet);
      copiedSnippets.add(source.chunk_id);
      copiedSnippets = copiedSnippets; // Trigger reactivity
      
      dispatch('snippet-copy', { source, snippet: source.snippet });
      
      // Reset copied state after 2 seconds
      setTimeout(() => {
        copiedSnippets.delete(source.chunk_id);
        copiedSnippets = copiedSnippets;
      }, 2000);
    } catch (error) {
      console.error('Failed to copy snippet:', error);
    }
  }

  // View full document
  function viewDocument(documentId: string) {
    dispatch('view-document', documentId);
  }

  // Get relevance color based on score
  function getRelevanceColor(score: number): string {
    if (score >= 0.8) return 'green';
    if (score >= 0.6) return 'blue';
    if (score >= 0.4) return 'yellow';
    return 'orange';
  }

  // Format relevance score as percentage
  function formatRelevance(score: number): string {
    return `${Math.round(score * 100)}%`;
  }

  // Highlight query terms in snippet
  function highlightSnippet(snippet: string, query: string): string {
    if (!query.trim()) return snippet;
    
    const queryTerms = query.toLowerCase().split(/\s+/).filter(term => term.length > 2);
    let highlighted = snippet;
    
    queryTerms.forEach(term => {
      const regex = new RegExp(`(${term})`, 'gi');
      highlighted = highlighted.replace(regex, '<mark>$1</mark>');
    });
    
    return highlighted;
  }

  // Truncate long filenames
  function truncateFilename(filename: string, maxLength = 30): string {
    if (filename.length <= maxLength) return filename;
    const extension = filename.split('.').pop();
    const nameWithoutExt = filename.substring(0, filename.lastIndexOf('.'));
    const truncated = nameWithoutExt.substring(0, maxLength - extension!.length - 4) + '...';
    return `${truncated}.${extension}`;
  }
</script>

{#if sources.length > 0}
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-sm source-references">
    <div class="space-y-4">
      <!-- Header -->
      <div class="flex justify-between items-center">
        <div class="flex items-center gap-2">
          <FileText size={20} class="text-gray-700 dark:text-gray-300" />
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            Source References
          </h3>
          <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-gray-700 bg-gray-200 rounded-full dark:bg-gray-600 dark:text-gray-200">
            {sources.length} {sources.length === 1 ? 'source' : 'sources'}
          </span>
        </div>

        {#if hasMoreSources && allowExpansion}
          <button
            type="button"
            on:click={() => showAllSources = !showAllSources}
            class="inline-flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-800 bg-white border border-gray-400 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:bg-gray-700 dark:text-gray-200 dark:border-gray-500 dark:hover:bg-gray-600"
          >
            {showAllSources ? 'Show Less' : `Show All (${sources.length})`}
            {#if showAllSources}
              <ChevronUp size={16} />
            {:else}
              <ChevronDown size={16} />
            {/if}
          </button>
        {/if}
      </div>

      <hr class="border-gray-200 dark:border-gray-700" />

      <!-- Source list -->
      <div class="space-y-3">
        {#each visibleSources as source, index (source.chunk_id)}
          <div 
            class="border border-gray-200 dark:border-gray-700 rounded-lg p-4 transition-all duration-200 hover:shadow-md hover:-translate-y-0.5 source-item"
            class:expanded={expandedSources.has(source.chunk_id)}
          >
            <div class="space-y-3">
              <!-- Source header -->
              <div class="flex justify-between items-start">
                <div class="flex-1 min-w-0">
                  <div class="source-info">
                    <div class="flex items-center gap-2 mb-1">
                      <h4 class="text-sm font-medium text-gray-900 dark:text-white truncate source-filename">
                        {truncateFilename(source.source_file)}
                      </h4>
                      <span class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-gray-700 bg-gray-200 rounded-full dark:bg-gray-600 dark:text-gray-200">
                        <Hash size={10} />
                        Chunk {source.chunk_index + 1}
                      </span>
                    </div>

                    <!-- Headers (if available) -->
                    {#if source.headers.length > 0}
                      <p class="text-xs text-gray-600 dark:text-gray-300 italic source-headers">
                        {source.headers.join(' > ')}
                      </p>
                    {/if}
                  </div>
                </div>

                <div class="flex items-center gap-2">
                  <!-- Relevance score -->
                  <div class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded-full" 
                       class:bg-green-100={getRelevanceColor(source.relevance_score) === 'green'} 
                       class:text-green-800={getRelevanceColor(source.relevance_score) === 'green'}
                       class:bg-blue-100={getRelevanceColor(source.relevance_score) === 'blue'} 
                       class:text-blue-800={getRelevanceColor(source.relevance_score) === 'blue'}
                       class:bg-yellow-100={getRelevanceColor(source.relevance_score) === 'yellow'} 
                       class:text-yellow-800={getRelevanceColor(source.relevance_score) === 'yellow'}
                       class:bg-orange-100={getRelevanceColor(source.relevance_score) === 'orange'} 
                       class:text-orange-800={getRelevanceColor(source.relevance_score) === 'orange'}
                       title="Relevance score">
                    <TrendingUp size={12} />
                    {formatRelevance(source.relevance_score)}
                  </div>

                  <!-- Actions -->
                  <div class="flex items-center gap-1">
                    <button
                      type="button"
                      on:click={() => viewDocument(source.document_id)}
                      class="p-1 text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded dark:text-gray-300 dark:hover:text-gray-100 dark:hover:bg-gray-600"
                      title="View full document"
                      aria-label="View full document"
                    >
                      <ExternalLink size={14} />
                    </button>

                    {#if showSnippets}
                      <button
                        type="button"
                        on:click={() => toggleSourceExpansion(source.chunk_id)}
                        class="p-1 text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded dark:text-gray-300 dark:hover:text-gray-100 dark:hover:bg-gray-600"
                        title={expandedSources.has(source.chunk_id) ? 'Collapse snippet' : 'Expand snippet'}
                        aria-label={expandedSources.has(source.chunk_id) ? 'Collapse snippet' : 'Expand snippet'}
                        aria-expanded={expandedSources.has(source.chunk_id)}
                      >
                        {#if expandedSources.has(source.chunk_id)}
                          <ChevronUp size={14} />
                        {:else}
                          <ChevronDown size={14} />
                        {/if}
                      </button>
                    {/if}
                  </div>
                </div>
              </div>

              <!-- Snippet preview (always visible, truncated) -->
              {#if showSnippets}
                <div class="snippet-preview">
                  <div class="text-sm text-gray-700 dark:text-gray-300 snippet-text">
                    {@html highlightSnippet(
                      expandedSources.has(source.chunk_id) 
                        ? source.snippet 
                        : source.snippet.substring(0, 150) + (source.snippet.length > 150 ? '...' : ''),
                      highlightQuery
                    )}
                  </div>
                </div>

                <!-- Full snippet (collapsible) -->
                {#if expandedSources.has(source.chunk_id)}
                  <div class="snippet-full">
                    <div class="max-h-48 overflow-y-auto">
                      <div class="text-sm text-gray-800 dark:text-gray-200 snippet-text">
                        {@html highlightSnippet(source.snippet, highlightQuery)}
                      </div>
                    </div>
                    
                    <div class="flex justify-end mt-2">
                      <button
                        type="button"
                        on:click={() => copySnippet(source)}
                        class="copy-snippet-button"
                        class:copied={copiedSnippets.has(source.chunk_id)}
                        title={copiedSnippets.has(source.chunk_id) ? 'Copied!' : 'Copy snippet'}
                      >
                        {#if copiedSnippets.has(source.chunk_id)}
                          <CheckCircle size={12} />
                        {:else}
                          <Copy size={12} />
                        {/if}
                        {copiedSnippets.has(source.chunk_id) ? 'Copied!' : 'Copy'}
                      </button>
                    </div>
                  </div>
                {/if}
              {/if}
            </div>
          </div>
        {/each}
      </div>

      <!-- Show more/less toggle at bottom -->
      {#if hasMoreSources && allowExpansion && !showAllSources}
        <div class="flex justify-center">
          <button
            type="button"
            on:click={() => showAllSources = true}
            class="px-4 py-2 text-sm font-medium text-blue-700 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:text-blue-400 dark:bg-blue-900/20 dark:border-blue-800 dark:hover:bg-blue-900/30"
          >
            Show {sources.length - maxVisible} more sources
          </button>
        </div>
      {/if}
    </div>
  </div>
{:else}
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-sm no-sources">
    <div class="flex items-center justify-center gap-2 text-center">
      <FileText size={20} class="text-gray-600 dark:text-gray-300" />
      <p class="text-sm text-gray-600 dark:text-gray-300">
        No source references available
      </p>
    </div>
  </div>
{/if}

<style>
  .source-references {
    max-width: 100%;
  }

  .source-item {
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .source-item.expanded {
    background-color: #eff6ff;
    border-color: #93c5fd;
  }

  /* Dark mode for expanded source item */
  @media (prefers-color-scheme: dark) {
    .source-item.expanded {
      background-color: rgba(30, 58, 138, 0.2);
      border-color: #1d4ed8;
    }
  }

  .source-info {
    flex: 1;
    min-width: 0; /* Allow text truncation */
  }

  .source-filename {
    word-break: break-all;
    line-height: 1.3;
  }

  .source-headers {
    margin-top: 0.25rem;
    font-style: italic;
    line-height: 1.2;
  }

  .snippet-preview {
    background: var(--color-surface-50);
    border-radius: 0.375rem;
    padding: 0.75rem;
    border-left: 3px solid var(--color-surface-300);
  }

  .snippet-full {
    background: var(--color-surface-50);
    border-radius: 0.375rem;
    padding: 0.75rem;
    border-left: 3px solid var(--color-primary-400);
    margin-top: 0.5rem;
  }

  .snippet-text {
    line-height: 1.5;
    word-wrap: break-word;
    hyphens: auto;
  }

  .snippet-text :global(mark) {
    background: #fef08a;
    color: var(--color-surface-900);
    padding: 0.1em 0.2em;
    border-radius: 0.2em;
    font-weight: 500;
  }

  .no-sources {
    text-align: center;
    padding: 2rem;
    background: var(--color-surface-50);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .snippet-preview {
      background: var(--color-surface-800);
      border-left-color: var(--color-surface-600);
    }

    .snippet-full {
      background: var(--color-surface-800);
      border-left-color: var(--color-primary-500);
    }

    .snippet-text :global(mark) {
      background: #a16207;
      color: #fef3c7;
    }

    .no-sources {
      background: var(--color-surface-800);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .snippet-preview,
    .snippet-full {
      padding: 0.5rem;
    }

    .source-filename {
      font-size: 0.875rem;
    }
  }

  /* Print styles */
  @media print {
    .source-item {
      box-shadow: none;
      border: 1px solid #ccc;
      break-inside: avoid;
    }

    .snippet-preview,
    .snippet-full {
      background: white;
      border-left: 2px solid #333;
    }

    /* Hide interactive elements in print */
    .source-references button {
      display: none;
    }
  }

  /* Accessibility improvements */
  .source-item {
    scroll-margin-top: 2rem;
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .snippet-preview {
      border-left-width: 4px;
      border-left-color: #000;
    }

    .snippet-full {
      border-left-width: 4px;
      border-left-color: #0066cc;
    }

    .snippet-text :global(mark) {
      background: #ffff00;
      color: #000;
      border: 1px solid #000;
    }
  }

  /* Copy snippet button styles */
  .copy-snippet-button {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 0.375rem;
    transition: all 0.2s ease;
    color: #1d4ed8;
    background-color: #dbeafe;
  }

  .copy-snippet-button:hover {
    background-color: #bfdbfe;
  }

  .copy-snippet-button.copied {
    color: #166534;
    background-color: #dcfce7;
  }

  .copy-snippet-button.copied:hover {
    background-color: #bbf7d0;
  }

  /* Dark mode for copy snippet button */
  @media (prefers-color-scheme: dark) {
    .copy-snippet-button {
      color: #60a5fa;
      background-color: rgba(30, 58, 138, 0.2);
    }

    .copy-snippet-button:hover {
      background-color: rgba(30, 58, 138, 0.3);
    }

    .copy-snippet-button.copied {
      color: #4ade80;
      background-color: rgba(20, 83, 45, 0.2);
    }

    .copy-snippet-button.copied:hover {
      background-color: rgba(20, 83, 45, 0.3);
    }
  }

  /* Focus styles for interactive elements */
  .source-references button:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }
</style>