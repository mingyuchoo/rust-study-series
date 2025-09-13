<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  // Props
  export let enabled: boolean = true;
  export let showMetrics: boolean = false;
  export let logToConsole: boolean = false;
  export let reportInterval: number = 30000; // 30 seconds

  // Performance metrics
  interface PerformanceMetrics {
    loadTime: number;
    domContentLoaded: number;
    firstContentfulPaint: number;
    largestContentfulPaint: number;
    cumulativeLayoutShift: number;
    firstInputDelay: number;
    memoryUsage?: {
      used: number;
      total: number;
      limit: number;
    };
    connectionType?: string;
    effectiveType?: string;
  }

  let metrics: PerformanceMetrics = {
    loadTime: 0,
    domContentLoaded: 0,
    firstContentfulPaint: 0,
    largestContentfulPaint: 0,
    cumulativeLayoutShift: 0,
    firstInputDelay: 0
  };

  let observers: PerformanceObserver[] = [];
  let reportTimer: number | null = null;
  let isVisible = false;

  const dispatch = createEventDispatcher<{
    metrics: PerformanceMetrics;
    warning: { metric: string; value: number; threshold: number };
  }>();

  // Performance thresholds (in milliseconds)
  const THRESHOLDS = {
    loadTime: 3000,
    domContentLoaded: 2000,
    firstContentfulPaint: 1800,
    largestContentfulPaint: 2500,
    cumulativeLayoutShift: 0.1,
    firstInputDelay: 100
  };

  onMount(() => {
    if (!enabled) return;

    collectBasicMetrics();
    setupPerformanceObservers();
    collectMemoryMetrics();
    collectNetworkMetrics();

    if (reportInterval > 0) {
      reportTimer = window.setInterval(reportMetrics, reportInterval);
    }

    return () => {
      cleanup();
    };
  });

  onDestroy(() => {
    cleanup();
  });

  function cleanup() {
    observers.forEach(observer => observer.disconnect());
    observers = [];
    
    if (reportTimer) {
      clearInterval(reportTimer);
      reportTimer = null;
    }
  }

  function collectBasicMetrics() {
    if (!('performance' in window)) return;

    const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
    if (navigation) {
      metrics.loadTime = navigation.loadEventEnd - navigation.loadEventStart;
      metrics.domContentLoaded = navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart;
    }

    // First Contentful Paint
    const paintEntries = performance.getEntriesByType('paint');
    const fcp = paintEntries.find(entry => entry.name === 'first-contentful-paint');
    if (fcp) {
      metrics.firstContentfulPaint = fcp.startTime;
    }
  }

  function setupPerformanceObservers() {
    if (!('PerformanceObserver' in window)) return;

    // Largest Contentful Paint
    try {
      const lcpObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        const lastEntry = entries[entries.length - 1];
        metrics.largestContentfulPaint = lastEntry.startTime;
        
        if (lastEntry.startTime > THRESHOLDS.largestContentfulPaint) {
          dispatch('warning', {
            metric: 'largestContentfulPaint',
            value: lastEntry.startTime,
            threshold: THRESHOLDS.largestContentfulPaint
          });
        }
      });
      lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
      observers.push(lcpObserver);
    } catch (error) {
      console.warn('LCP observer not supported:', error);
    }

    // First Input Delay
    try {
      const fidObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        entries.forEach((entry: any) => {
          const fid = entry.processingStart - entry.startTime;
          metrics.firstInputDelay = fid;
          
          if (fid > THRESHOLDS.firstInputDelay) {
            dispatch('warning', {
              metric: 'firstInputDelay',
              value: fid,
              threshold: THRESHOLDS.firstInputDelay
            });
          }
        });
      });
      fidObserver.observe({ entryTypes: ['first-input'] });
      observers.push(fidObserver);
    } catch (error) {
      console.warn('FID observer not supported:', error);
    }

    // Cumulative Layout Shift
    try {
      let clsValue = 0;
      const clsObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        entries.forEach((entry: any) => {
          if (!entry.hadRecentInput) {
            clsValue += entry.value;
          }
        });
        metrics.cumulativeLayoutShift = clsValue;
        
        if (clsValue > THRESHOLDS.cumulativeLayoutShift) {
          dispatch('warning', {
            metric: 'cumulativeLayoutShift',
            value: clsValue,
            threshold: THRESHOLDS.cumulativeLayoutShift
          });
        }
      });
      clsObserver.observe({ entryTypes: ['layout-shift'] });
      observers.push(clsObserver);
    } catch (error) {
      console.warn('CLS observer not supported:', error);
    }
  }

  function collectMemoryMetrics() {
    if ('memory' in performance) {
      const memory = (performance as any).memory;
      metrics.memoryUsage = {
        used: memory.usedJSHeapSize,
        total: memory.totalJSHeapSize,
        limit: memory.jsHeapSizeLimit
      };
    }
  }

  function collectNetworkMetrics() {
    if ('connection' in navigator) {
      const connection = (navigator as any).connection;
      metrics.connectionType = connection.type;
      metrics.effectiveType = connection.effectiveType;
    }
  }

  function reportMetrics() {
    collectMemoryMetrics();
    collectNetworkMetrics();
    
    dispatch('metrics', { ...metrics });
    
    if (logToConsole) {
      console.group('🔍 Performance Metrics');
      console.log('Load Time:', `${metrics.loadTime.toFixed(2)}ms`);
      console.log('DOM Content Loaded:', `${metrics.domContentLoaded.toFixed(2)}ms`);
      console.log('First Contentful Paint:', `${metrics.firstContentfulPaint.toFixed(2)}ms`);
      console.log('Largest Contentful Paint:', `${metrics.largestContentfulPaint.toFixed(2)}ms`);
      console.log('Cumulative Layout Shift:', metrics.cumulativeLayoutShift.toFixed(4));
      console.log('First Input Delay:', `${metrics.firstInputDelay.toFixed(2)}ms`);
      
      if (metrics.memoryUsage) {
        console.log('Memory Usage:', {
          used: `${(metrics.memoryUsage.used / 1024 / 1024).toFixed(2)}MB`,
          total: `${(metrics.memoryUsage.total / 1024 / 1024).toFixed(2)}MB`,
          limit: `${(metrics.memoryUsage.limit / 1024 / 1024).toFixed(2)}MB`
        });
      }
      
      if (metrics.connectionType) {
        console.log('Connection:', `${metrics.effectiveType} (${metrics.connectionType})`);
      }
      
      console.groupEnd();
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function getMetricStatus(value: number, threshold: number): 'good' | 'warning' | 'poor' {
    if (value <= threshold * 0.75) return 'good';
    if (value <= threshold) return 'warning';
    return 'poor';
  }

  function toggleVisibility() {
    isVisible = !isVisible;
  }

  // Export metrics for external use
  export function getCurrentMetrics(): PerformanceMetrics {
    collectMemoryMetrics();
    collectNetworkMetrics();
    return { ...metrics };
  }

  export function forceReport() {
    reportMetrics();
  }
</script>

{#if showMetrics}
  <div class="performance-monitor" class:visible={isVisible}>
    <button class="toggle-button" on:click={toggleVisibility} aria-label="Toggle performance metrics">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 3v5h5"/>
        <path d="M3 8c0-5.5 4.5-10 10-10s10 4.5 10 10-4.5 10-10 10c-2.5 0-4.8-.9-6.5-2.5"/>
        <path d="M12 7v5l3 3"/>
      </svg>
      {isVisible ? 'Hide' : 'Show'} Metrics
    </button>

    {#if isVisible}
      <div class="metrics-panel">
        <h3>Performance Metrics</h3>
        
        <div class="metrics-grid">
          <div class="metric-item" class:good={getMetricStatus(metrics.loadTime, THRESHOLDS.loadTime) === 'good'}
               class:warning={getMetricStatus(metrics.loadTime, THRESHOLDS.loadTime) === 'warning'}
               class:poor={getMetricStatus(metrics.loadTime, THRESHOLDS.loadTime) === 'poor'}>
            <span class="metric-label">Load Time</span>
            <span class="metric-value">{metrics.loadTime.toFixed(0)}ms</span>
          </div>

          <div class="metric-item" class:good={getMetricStatus(metrics.firstContentfulPaint, THRESHOLDS.firstContentfulPaint) === 'good'}
               class:warning={getMetricStatus(metrics.firstContentfulPaint, THRESHOLDS.firstContentfulPaint) === 'warning'}
               class:poor={getMetricStatus(metrics.firstContentfulPaint, THRESHOLDS.firstContentfulPaint) === 'poor'}>
            <span class="metric-label">FCP</span>
            <span class="metric-value">{metrics.firstContentfulPaint.toFixed(0)}ms</span>
          </div>

          <div class="metric-item" class:good={getMetricStatus(metrics.largestContentfulPaint, THRESHOLDS.largestContentfulPaint) === 'good'}
               class:warning={getMetricStatus(metrics.largestContentfulPaint, THRESHOLDS.largestContentfulPaint) === 'warning'}
               class:poor={getMetricStatus(metrics.largestContentfulPaint, THRESHOLDS.largestContentfulPaint) === 'poor'}>
            <span class="metric-label">LCP</span>
            <span class="metric-value">{metrics.largestContentfulPaint.toFixed(0)}ms</span>
          </div>

          <div class="metric-item" class:good={getMetricStatus(metrics.cumulativeLayoutShift * 1000, THRESHOLDS.cumulativeLayoutShift * 1000) === 'good'}
               class:warning={getMetricStatus(metrics.cumulativeLayoutShift * 1000, THRESHOLDS.cumulativeLayoutShift * 1000) === 'warning'}
               class:poor={getMetricStatus(metrics.cumulativeLayoutShift * 1000, THRESHOLDS.cumulativeLayoutShift * 1000) === 'poor'}>
            <span class="metric-label">CLS</span>
            <span class="metric-value">{metrics.cumulativeLayoutShift.toFixed(3)}</span>
          </div>

          {#if metrics.firstInputDelay > 0}
            <div class="metric-item" class:good={getMetricStatus(metrics.firstInputDelay, THRESHOLDS.firstInputDelay) === 'good'}
                 class:warning={getMetricStatus(metrics.firstInputDelay, THRESHOLDS.firstInputDelay) === 'warning'}
                 class:poor={getMetricStatus(metrics.firstInputDelay, THRESHOLDS.firstInputDelay) === 'poor'}>
              <span class="metric-label">FID</span>
              <span class="metric-value">{metrics.firstInputDelay.toFixed(0)}ms</span>
            </div>
          {/if}

          {#if metrics.memoryUsage}
            <div class="metric-item">
              <span class="metric-label">Memory</span>
              <span class="metric-value">{formatBytes(metrics.memoryUsage.used)}</span>
            </div>
          {/if}

          {#if metrics.connectionType}
            <div class="metric-item">
              <span class="metric-label">Connection</span>
              <span class="metric-value">{metrics.effectiveType}</span>
            </div>
          {/if}
        </div>

        <div class="actions">
          <button class="refresh-button" on:click={forceReport}>
            Refresh
          </button>
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .performance-monitor {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 9999;
    font-family: monospace;
    font-size: 12px;
  }

  .toggle-button {
    background: rgba(0, 0, 0, 0.8);
    color: white;
    border: none;
    padding: 8px 12px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-weight: 500;
    backdrop-filter: blur(10px);
    transition: all 0.2s ease;
  }

  .toggle-button:hover {
    background: rgba(0, 0, 0, 0.9);
    transform: translateY(-1px);
  }

  .metrics-panel {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 8px;
    background: rgba(255, 255, 255, 0.95);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    padding: 16px;
    min-width: 280px;
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  }

  .metrics-panel h3 {
    margin: 0 0 12px 0;
    font-size: 14px;
    font-weight: 600;
    color: #333;
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    margin-bottom: 12px;
  }

  .metric-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 8px;
    background: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
    border-left: 3px solid #ddd;
  }

  .metric-item.good {
    border-left-color: #10b981;
    background: rgba(16, 185, 129, 0.1);
  }

  .metric-item.warning {
    border-left-color: #f59e0b;
    background: rgba(245, 158, 11, 0.1);
  }

  .metric-item.poor {
    border-left-color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }

  .metric-label {
    font-weight: 500;
    color: #666;
  }

  .metric-value {
    font-weight: 600;
    color: #333;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
  }

  .refresh-button {
    background: #2563eb;
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 11px;
    font-weight: 500;
    transition: background-color 0.2s ease;
  }

  .refresh-button:hover {
    background: #1d4ed8;
  }

  /* Dark mode */
  @media (prefers-color-scheme: dark) {
    .metrics-panel {
      background: rgba(0, 0, 0, 0.9);
      border-color: rgba(255, 255, 255, 0.1);
    }

    .metrics-panel h3 {
      color: #fff;
    }

    .metric-item {
      background: rgba(255, 255, 255, 0.1);
    }

    .metric-label {
      color: #ccc;
    }

    .metric-value {
      color: #fff;
    }
  }

  /* Mobile responsive */
  @media (max-width: 768px) {
    .performance-monitor {
      top: 10px;
      right: 10px;
    }

    .metrics-panel {
      min-width: 240px;
      right: -20px;
    }

    .metrics-grid {
      grid-template-columns: 1fr;
    }
  }

  /* Reduced motion */
  @media (prefers-reduced-motion: reduce) {
    .toggle-button {
      transition: none;
    }

    .toggle-button:hover {
      transform: none;
    }
  }
</style>