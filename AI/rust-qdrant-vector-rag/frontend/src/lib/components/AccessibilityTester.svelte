<script lang="ts">
  import { onMount } from 'svelte';
  import { runAccessibilityAudit, logAccessibilityIssues, type AccessibilityAuditResult } from '../utils/accessibility-testing.js';
  import { generateId } from '../utils/accessibility.js';

  // Props
  export let autoRun = false;
  export let container: Element | undefined = undefined;
  export let showResults = true;
  export let logToConsole = true;

  // State
  let auditResult: AccessibilityAuditResult | null = null;
  let isRunning = false;
  let lastRunTime: Date | null = null;
  let testerId = generateId('accessibility-tester');

  // Run audit
  async function runAudit() {
    if (isRunning) return;
    
    isRunning = true;
    
    try {
      // Small delay to ensure DOM is ready
      await new Promise(resolve => setTimeout(resolve, 100));
      
      auditResult = runAccessibilityAudit(container);
      lastRunTime = new Date();
      
      if (logToConsole) {
        logAccessibilityIssues(auditResult);
      }
    } catch (error) {
      console.error('Accessibility audit failed:', error);
    } finally {
      isRunning = false;
    }
  }

  // Auto-run on mount if enabled
  onMount(() => {
    if (autoRun) {
      runAudit();
    }
  });

  // Get severity color
  function getSeverityColor(severity: string): string {
    switch (severity) {
      case 'critical': return 'var(--color-error-600)';
      case 'serious': return 'var(--color-error-500)';
      case 'moderate': return 'var(--color-warning-500)';
      case 'minor': return 'var(--color-info-500)';
      default: return 'var(--color-surface-600)';
    }
  }

  // Get issue type icon
  function getIssueIcon(type: string): string {
    switch (type) {
      case 'error': return '❌';
      case 'warning': return '⚠️';
      case 'info': return 'ℹ️';
      default: return '•';
    }
  }

  // Format score color
  function getScoreColor(score: number): string {
    if (score >= 90) return 'var(--color-success-600)';
    if (score >= 70) return 'var(--color-warning-600)';
    return 'var(--color-error-600)';
  }
</script>

{#if import.meta.env.DEV}
  <div class="accessibility-tester" id={testerId}>
    <div class="tester-header">
      <h3>Accessibility Tester</h3>
      <button 
        class="run-button"
        on:click={runAudit}
        disabled={isRunning}
        aria-describedby="{testerId}-description"
      >
        {isRunning ? 'Running...' : 'Run Audit'}
      </button>
    </div>

    <p id="{testerId}-description" class="tester-description">
      This tool runs accessibility checks on the current page. Only visible in development mode.
    </p>

    {#if isRunning}
      <div class="loading-state" role="status" aria-live="polite">
        <div class="spinner" aria-hidden="true"></div>
        <span>Running accessibility audit...</span>
      </div>
    {/if}

    {#if auditResult && showResults}
      <div class="audit-results" role="region" aria-labelledby="{testerId}-results-title">
        <h4 id="{testerId}-results-title">Audit Results</h4>
        
        <!-- Score and summary -->
        <div class="score-summary">
          <div class="score" style:color={getScoreColor(auditResult.score)}>
            <span class="score-value">{auditResult.score}</span>
            <span class="score-label">/100</span>
          </div>
          
          <div class="summary">
            <div class="summary-item">
              <span class="count critical">{auditResult.summary.critical}</span>
              <span class="label">Critical</span>
            </div>
            <div class="summary-item">
              <span class="count serious">{auditResult.summary.serious}</span>
              <span class="label">Serious</span>
            </div>
            <div class="summary-item">
              <span class="count moderate">{auditResult.summary.moderate}</span>
              <span class="label">Moderate</span>
            </div>
            <div class="summary-item">
              <span class="count minor">{auditResult.summary.minor}</span>
              <span class="label">Minor</span>
            </div>
          </div>
        </div>

        <!-- Issues list -->
        {#if auditResult.issues.length > 0}
          <div class="issues-list">
            <h5>Issues Found ({auditResult.issues.length})</h5>
            <ul role="list">
              {#each auditResult.issues as issue, index}
                <li class="issue-item" class:error={issue.type === 'error'} class:warning={issue.type === 'warning'}>
                  <div class="issue-header">
                    <span class="issue-icon" aria-hidden="true">{getIssueIcon(issue.type)}</span>
                    <span class="issue-rule">[{issue.rule}]</span>
                    <span class="issue-severity" style:color={getSeverityColor(issue.severity)}>
                      {issue.severity}
                    </span>
                  </div>
                  <p class="issue-message">{issue.message}</p>
                  {#if issue.element}
                    <button 
                      class="inspect-element"
                      on:click={() => {
                        if (issue.element) {
                          console.log('Element with issue:', issue.element);
                          issue.element.scrollIntoView({ behavior: 'smooth', block: 'center' });
                          // Temporarily highlight the element
                          const originalOutline = issue.element.style.outline;
                          issue.element.style.outline = '3px solid red';
                          setTimeout(() => {
                            issue.element!.style.outline = originalOutline;
                          }, 3000);
                        }
                      }}
                      aria-label="Inspect element with issue"
                    >
                      Inspect Element
                    </button>
                  {/if}
                </li>
              {/each}
            </ul>
          </div>
        {:else}
          <div class="no-issues" role="status">
            <span class="success-icon" aria-hidden="true">✅</span>
            <p>No accessibility issues found!</p>
          </div>
        {/if}

        {#if lastRunTime}
          <div class="run-info">
            <small>Last run: {lastRunTime.toLocaleTimeString()}</small>
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .accessibility-tester {
    position: fixed;
    bottom: 20px;
    right: 20px;
    width: 400px;
    max-height: 80vh;
    background: var(--color-surface-50);
    border: 2px solid var(--color-surface-300);
    border-radius: 0.75rem;
    box-shadow: var(--shadow-xl);
    z-index: var(--z-toast);
    font-family: var(--font-family-base);
    font-size: var(--font-size-sm);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .tester-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md);
    background: var(--color-primary-500);
    color: white;
  }

  .tester-header h3 {
    margin: 0;
    font-size: var(--font-size-base);
    font-weight: 600;
  }

  .run-button {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 0.375rem;
    padding: var(--spacing-xs) var(--spacing-sm);
    font-size: var(--font-size-xs);
    cursor: pointer;
    transition: all var(--duration-fast) ease;
  }

  .run-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.3);
  }

  .run-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .tester-description {
    padding: var(--spacing-sm) var(--spacing-md);
    margin: 0;
    color: var(--color-surface-600);
    font-size: var(--font-size-xs);
    border-bottom: 1px solid var(--color-surface-200);
  }

  .loading-state {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    color: var(--color-surface-700);
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-surface-300);
    border-top: 2px solid var(--color-primary-500);
    border-radius: 50%;
    animation: spin var(--duration-normal) linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .audit-results {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-md);
  }

  .audit-results h4 {
    margin: 0 0 var(--spacing-md) 0;
    font-size: var(--font-size-base);
    color: var(--color-surface-800);
  }

  .score-summary {
    display: flex;
    align-items: center;
    gap: var(--spacing-lg);
    margin-bottom: var(--spacing-lg);
    padding: var(--spacing-md);
    background: var(--color-surface-100);
    border-radius: 0.5rem;
  }

  .score {
    display: flex;
    align-items: baseline;
    gap: var(--spacing-xs);
  }

  .score-value {
    font-size: var(--font-size-2xl);
    font-weight: 700;
  }

  .score-label {
    font-size: var(--font-size-base);
    color: var(--color-surface-600);
  }

  .summary {
    display: flex;
    gap: var(--spacing-md);
    flex-wrap: wrap;
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .count {
    font-size: var(--font-size-lg);
    font-weight: 600;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: var(--font-size-xs);
  }

  .count.critical {
    background-color: var(--color-error-600);
  }

  .count.serious {
    background-color: var(--color-error-500);
  }

  .count.moderate {
    background-color: var(--color-warning-500);
  }

  .count.minor {
    background-color: var(--color-info-500);
  }

  .label {
    font-size: var(--font-size-xs);
    color: var(--color-surface-600);
  }

  .issues-list h5 {
    margin: 0 0 var(--spacing-sm) 0;
    font-size: var(--font-size-sm);
    color: var(--color-surface-700);
  }

  .issues-list ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .issue-item {
    padding: var(--spacing-sm);
    border: 1px solid var(--color-surface-200);
    border-radius: 0.375rem;
    margin-bottom: var(--spacing-sm);
  }

  .issue-item.error {
    border-color: var(--color-error-300);
    background-color: var(--color-error-50);
  }

  .issue-item.warning {
    border-color: var(--color-warning-300);
    background-color: var(--color-warning-50);
  }

  .issue-header {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-xs);
  }

  .issue-icon {
    font-size: var(--font-size-base);
  }

  .issue-rule {
    font-family: var(--font-family-mono);
    font-size: var(--font-size-xs);
    background: var(--color-surface-200);
    padding: 2px 4px;
    border-radius: 0.25rem;
  }

  .issue-severity {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    margin-left: auto;
  }

  .issue-message {
    margin: 0 0 var(--spacing-xs) 0;
    color: var(--color-surface-700);
    line-height: var(--line-height-normal);
  }

  .inspect-element {
    background: var(--color-primary-500);
    color: white;
    border: none;
    border-radius: 0.25rem;
    padding: var(--spacing-xs) var(--spacing-sm);
    font-size: var(--font-size-xs);
    cursor: pointer;
    transition: background-color var(--duration-fast) ease;
  }

  .inspect-element:hover {
    background: var(--color-primary-600);
  }

  .no-issues {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-success-600);
  }

  .success-icon {
    font-size: var(--font-size-2xl);
  }

  .no-issues p {
    margin: 0;
    font-weight: 600;
  }

  .run-info {
    margin-top: var(--spacing-md);
    padding-top: var(--spacing-sm);
    border-top: 1px solid var(--color-surface-200);
    text-align: center;
    color: var(--color-surface-500);
  }

  /* Mobile responsive */
  @media (max-width: 767px) {
    .accessibility-tester {
      width: calc(100vw - 40px);
      right: 20px;
      left: 20px;
      max-height: 60vh;
    }

    .score-summary {
      flex-direction: column;
      align-items: stretch;
      gap: var(--spacing-md);
    }

    .summary {
      justify-content: space-around;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .accessibility-tester {
      background: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .tester-description {
      color: var(--color-surface-400);
      border-color: var(--color-surface-700);
    }

    .audit-results h4 {
      color: var(--color-surface-200);
    }

    .score-summary {
      background: var(--color-surface-700);
    }

    .issue-item {
      border-color: var(--color-surface-600);
      background: var(--color-surface-700);
    }

    .issue-item.error {
      border-color: var(--color-error-600);
      background-color: var(--color-error-900);
    }

    .issue-item.warning {
      border-color: var(--color-warning-600);
      background-color: var(--color-warning-900);
    }

    .issue-rule {
      background: var(--color-surface-600);
      color: var(--color-surface-200);
    }

    .issue-message {
      color: var(--color-surface-300);
    }

    .run-info {
      border-color: var(--color-surface-700);
      color: var(--color-surface-400);
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .spinner {
      animation: none;
    }

    .run-button {
      transition: none;
    }

    .inspect-element {
      transition: none;
    }
  }
</style>