<script lang="ts">
  import { onMount } from 'svelte';
  import { BreakpointUtils, type Breakpoint } from '../utils/responsive.js';
  import { generateId, announceToScreenReader } from '../utils/accessibility.js';

  // Props
  export let variant: 'container' | 'grid' | 'flex' | 'stack' = 'container';
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
    large: 'var(--spacing-2xl)'
  };
  export let gap: Partial<Record<Breakpoint, string>> = {
    mobile: 'var(--spacing-md)',
    tablet: 'var(--spacing-lg)',
    desktop: 'var(--spacing-xl)',
    large: 'var(--spacing-xl)'
  };
  export let columns: Partial<Record<Breakpoint, number>> = {
    mobile: 1,
    tablet: 2,
    desktop: 3,
    large: 4
  };
  export let alignItems: 'start' | 'center' | 'end' | 'stretch' = 'stretch';
  export let justifyContent: 'start' | 'center' | 'end' | 'between' | 'around' | 'evenly' = 'start';
  export let className = '';
  export let element: 'div' | 'section' | 'article' | 'main' | 'aside' | 'header' | 'footer' = 'div';
  export let ariaLabel: string | undefined = undefined;
  export let ariaLabelledby: string | undefined = undefined;
  export let role: string | undefined = undefined;
  export let id: string | undefined = undefined;
  export let announceBreakpointChanges = false;

  // Reactive state
  let currentBreakpoint: Breakpoint = 'mobile';
  let layoutElement: HTMLElement;
  let layoutId = id || generateId('responsive-layout');
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

  // Get current values based on breakpoint
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

  // Reactive computed values
  $: currentMaxWidth = getCurrentValue(maxWidth, currentBreakpoint);
  $: currentPadding = getCurrentValue(padding, currentBreakpoint);
  $: currentGap = getCurrentValue(gap, currentBreakpoint);
  $: currentColumns = getCurrentValue(columns, currentBreakpoint) || 1;

  // Generate CSS classes
  $: layoutClasses = [
    'responsive-layout',
    `variant-${variant}`,
    `breakpoint-${currentBreakpoint}`,
    `align-${alignItems}`,
    `justify-${justifyContent}`,
    className
  ].filter(Boolean).join(' ');

  // Generate CSS custom properties for dynamic values
  $: cssProps = {
    '--layout-max-width': currentMaxWidth,
    '--layout-padding': currentPadding,
    '--layout-gap': currentGap,
    '--layout-columns': currentColumns.toString()
  };
</script>

<svelte:element 
  this={element}
  bind:this={layoutElement}
  id={layoutId}
  class={layoutClasses}
  style={Object.entries(cssProps).map(([key, value]) => `${key}: ${value}`).join('; ')}
  aria-label={ariaLabel}
  aria-labelledby={ariaLabelledby}
  {role}
  {...$$restProps}
>
  <slot {currentBreakpoint} {layoutId} {currentColumns} />
</svelte:element>

<style>
  .responsive-layout {
    width: 100%;
    margin: 0 auto;
    box-sizing: border-box;
    transition: padding var(--duration-fast) ease, gap var(--duration-fast) ease;
    position: relative;
  }

  /* Container variant */
  .responsive-layout.variant-container {
    max-width: var(--layout-max-width);
    padding: var(--layout-padding);
  }

  /* Grid variant */
  .responsive-layout.variant-grid {
    display: grid;
    grid-template-columns: repeat(var(--layout-columns), 1fr);
    gap: var(--layout-gap);
    max-width: var(--layout-max-width);
    padding: var(--layout-padding);
  }

  /* Flex variant */
  .responsive-layout.variant-flex {
    display: flex;
    flex-wrap: wrap;
    gap: var(--layout-gap);
    max-width: var(--layout-max-width);
    padding: var(--layout-padding);
  }

  .responsive-layout.variant-flex > :global(*) {
    flex: 1 1 auto;
    min-width: 0; /* Prevent flex items from overflowing */
  }

  /* Stack variant */
  .responsive-layout.variant-stack {
    display: flex;
    flex-direction: column;
    gap: var(--layout-gap);
    max-width: var(--layout-max-width);
    padding: var(--layout-padding);
  }

  /* Alignment utilities */
  .responsive-layout.align-start {
    align-items: flex-start;
  }

  .responsive-layout.align-center {
    align-items: center;
  }

  .responsive-layout.align-end {
    align-items: flex-end;
  }

  .responsive-layout.align-stretch {
    align-items: stretch;
  }

  /* Justification utilities */
  .responsive-layout.justify-start {
    justify-content: flex-start;
  }

  .responsive-layout.justify-center {
    justify-content: center;
  }

  .responsive-layout.justify-end {
    justify-content: flex-end;
  }

  .responsive-layout.justify-between {
    justify-content: space-between;
  }

  .responsive-layout.justify-around {
    justify-content: space-around;
  }

  .responsive-layout.justify-evenly {
    justify-content: space-evenly;
  }

  /* Responsive behavior adjustments */
  @media (max-width: 767px) {
    .responsive-layout.variant-flex {
      flex-direction: column;
    }
    
    .responsive-layout.variant-flex > :global(*) {
      flex: none;
      width: 100%;
    }
    
    .responsive-layout.variant-grid {
      grid-template-columns: 1fr;
    }
  }

  /* Ensure container doesn't exceed viewport */
  @media (max-width: 767px) {
    .responsive-layout {
      max-width: 100vw !important;
      overflow-x: hidden;
    }
  }

  /* Accessibility enhancements */
  .responsive-layout:focus-within {
    /* Enhance focus visibility when layout has focused children */
    outline: 2px solid transparent;
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .responsive-layout {
      border: 1px solid;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .responsive-layout {
      transition: none;
    }
  }

  /* Print styles */
  @media print {
    .responsive-layout {
      max-width: none !important;
      padding: 1rem !important;
      break-inside: avoid;
    }
    
    .responsive-layout.variant-grid {
      display: block;
    }
    
    .responsive-layout.variant-flex {
      display: block;
    }
    
    .responsive-layout.variant-stack {
      display: block;
    }
  }
</style>