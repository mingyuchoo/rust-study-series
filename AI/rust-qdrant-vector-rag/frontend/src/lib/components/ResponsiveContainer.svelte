<script lang="ts">
  import { onMount } from 'svelte';
  import { BreakpointUtils, type Breakpoint } from '../utils/responsive.js';
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';

  // Props
  export let maxWidth: Partial<Record<Breakpoint, string>> = {
    mobile: '100%',
    tablet: '768px',
    desktop: '1024px',
    large: '1200px'
  };
  
  export let padding: Partial<Record<Breakpoint, string>> = {
    mobile: 'var(--spacing-md)',
    tablet: 'var(--spacing-lg)',
    desktop: 'var(--spacing-xl)',
    large: 'var(--spacing-xl)'
  };
  
  export let gap: Partial<Record<Breakpoint, string>> = {
    mobile: 'var(--spacing-md)',
    tablet: 'var(--spacing-lg)',
    desktop: 'var(--spacing-xl)',
    large: 'var(--spacing-xl)'
  };
  
  export let className = '';
  export let element: 'div' | 'section' | 'article' | 'main' | 'aside' | 'header' | 'footer' = 'div';
  export let ariaLabel: string | undefined = undefined;
  export let ariaLabelledby: string | undefined = undefined;
  export let role: string | undefined = undefined;
  export let id: string | undefined = undefined;
  export let announceBreakpointChanges = false;

  // Reactive state
  let currentBreakpoint: Breakpoint = 'mobile';
  let containerElement: HTMLElement;
  let containerId = id || generateId('responsive-container');
  let previousBreakpoint: Breakpoint | null = null;

  // Update breakpoint on mount and resize
  onMount(() => {
    const cleanup = BreakpointUtils.onBreakpointChange((breakpoint) => {
      previousBreakpoint = currentBreakpoint;
      currentBreakpoint = breakpoint;
      
      // Announce breakpoint changes to screen readers if enabled
      if (announceBreakpointChanges && previousBreakpoint && previousBreakpoint !== breakpoint) {
        announceToScreenReader(`Layout changed to ${breakpoint} view`, 'polite');
      }
    });

    return cleanup;
  });

  // Get current styles based on breakpoint
  $: currentMaxWidth = getCurrentValue(maxWidth, currentBreakpoint);
  $: currentPadding = getCurrentValue(padding, currentBreakpoint);
  $: currentGap = getCurrentValue(gap, currentBreakpoint);

  function getCurrentValue<T>(config: Partial<Record<Breakpoint, T>>, breakpoint: Breakpoint): T | undefined {
    const breakpointOrder: Breakpoint[] = ['mobile', 'tablet', 'desktop', 'large'];
    const currentIndex = breakpointOrder.indexOf(breakpoint);
    
    // Find the closest defined breakpoint
    for (let i = currentIndex; i >= 0; i--) {
      const bp = breakpointOrder[i];
      if (config[bp] !== undefined) {
        return config[bp];
      }
    }
    
    return undefined;
  }

  // Generate responsive CSS classes
  $: responsiveClasses = [
    className,
    `breakpoint-${currentBreakpoint}`,
    'responsive-container'
  ].filter(Boolean).join(' ');
</script>

<svelte:element 
  this={element}
  bind:this={containerElement}
  id={containerId}
  class={responsiveClasses}
  style:max-width={currentMaxWidth}
  style:padding={currentPadding}
  style:gap={currentGap}
  aria-label={ariaLabel}
  aria-labelledby={ariaLabelledby}
  {role}
  {...$$restProps}
>
  <slot {currentBreakpoint} {containerId} />
</svelte:element>

<style>
  .responsive-container {
    width: 100%;
    margin: 0 auto;
    box-sizing: border-box;
    transition: padding var(--duration-fast) ease, gap var(--duration-fast) ease;
    position: relative;
  }

  /* Ensure container doesn't exceed viewport */
  @media (max-width: 767px) {
    .responsive-container {
      max-width: 100vw !important;
      overflow-x: hidden;
    }
  }

  /* Breakpoint-specific styles with enhanced responsive behavior */
  .responsive-container.breakpoint-mobile {
    /* Mobile-first approach - base styles */
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .responsive-container.breakpoint-tablet {
    /* Tablet-specific enhancements */
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .responsive-container.breakpoint-desktop {
    /* Desktop-specific enhancements */
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .responsive-container.breakpoint-large {
    /* Large desktop-specific enhancements */
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  /* Grid layout support for responsive containers */
  .responsive-container.grid-layout {
    display: grid;
    grid-template-columns: 1fr;
    align-items: start;
  }

  @media (min-width: 768px) {
    .responsive-container.grid-layout {
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    }
  }

  @media (min-width: 1024px) {
    .responsive-container.grid-layout {
      grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    }
  }

  /* Flex layout support for responsive containers */
  .responsive-container.flex-layout {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
  }

  .responsive-container.flex-layout > :global(*) {
    flex: 1 1 auto;
    min-width: 0; /* Prevent flex items from overflowing */
  }

  @media (max-width: 767px) {
    .responsive-container.flex-layout {
      flex-direction: column;
    }
    
    .responsive-container.flex-layout > :global(*) {
      flex: none;
      width: 100%;
    }
  }

  /* Accessibility enhancements */
  .responsive-container:focus-within {
    /* Enhance focus visibility when container has focused children */
    outline: 2px solid transparent;
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .responsive-container {
      border: 1px solid;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .responsive-container {
      transition: none;
    }
  }

  /* Print styles */
  @media print {
    .responsive-container {
      max-width: none !important;
      padding: 1rem !important;
      break-inside: avoid;
    }
  }
</style>