/**
 * Responsive design utilities and breakpoint management
 */

import { breakpoints, type Breakpoint } from '../config/theme.js';

// Breakpoint values in pixels
const breakpointValues = {
  mobile: 320,
  tablet: 768,
  desktop: 1024,
  large: 1440,
} as const;

// Media query utilities
export class MediaQuery {
  private static queries: Map<string, MediaQueryList> = new Map();

  static get(query: string): MediaQueryList {
    if (!this.queries.has(query)) {
      this.queries.set(query, window.matchMedia(query));
    }
    return this.queries.get(query)!;
  }

  static matches(query: string): boolean {
    return this.get(query).matches;
  }

  static addListener(query: string, callback: (event: MediaQueryListEvent) => void): () => void {
    const mq = this.get(query);
    mq.addEventListener('change', callback);
    
    return () => {
      mq.removeEventListener('change', callback);
    };
  }

  // Clear cached queries (useful for tests when matchMedia is re-mocked)
  static clearCache(): void {
    this.queries.clear();
  }
}

// Breakpoint utilities
export class BreakpointUtils {
  static isMobile(): boolean {
    return MediaQuery.matches(`(max-width: ${breakpoints.tablet})`);
  }

  static isTablet(): boolean {
    return MediaQuery.matches(`(min-width: ${breakpoints.tablet}) and (max-width: ${breakpoints.desktop})`);
  }

  static isDesktop(): boolean {
    return MediaQuery.matches(`(min-width: ${breakpoints.desktop})`);
  }

  static isLarge(): boolean {
    return MediaQuery.matches(`(min-width: ${breakpoints.large})`);
  }

  static getCurrentBreakpoint(): Breakpoint {
    if (this.isLarge()) return 'large';
    if (this.isDesktop()) return 'desktop';
    if (this.isTablet()) return 'tablet';
    return 'mobile';
  }

  static onBreakpointChange(callback: (breakpoint: Breakpoint) => void): () => void {
    const listeners: (() => void)[] = [];
    
    const updateBreakpoint = () => {
      callback(this.getCurrentBreakpoint());
    };

    // Listen to all breakpoint changes
    listeners.push(
      MediaQuery.addListener(`(max-width: ${breakpoints.tablet})`, updateBreakpoint),
      MediaQuery.addListener(`(min-width: ${breakpoints.tablet})`, updateBreakpoint),
      MediaQuery.addListener(`(min-width: ${breakpoints.desktop})`, updateBreakpoint),
      MediaQuery.addListener(`(min-width: ${breakpoints.large})`, updateBreakpoint)
    );

    // Initial call
    updateBreakpoint();

    // Return cleanup function
    return () => {
      listeners.forEach(cleanup => cleanup());
    };
  }
}

// Container utilities for responsive layouts
export class ContainerUtils {
  static getMaxWidth(breakpoint: Breakpoint): string {
    const maxWidths = {
      mobile: '100%',
      tablet: '768px',
      desktop: '1024px',
      large: '1200px',
    };
    return maxWidths[breakpoint];
  }

  static getPadding(breakpoint: Breakpoint): string {
    const paddings = {
      mobile: '1rem',
      tablet: '1.5rem',
      desktop: '2rem',
      large: '2rem',
    };
    return paddings[breakpoint];
  }
}

// Grid utilities for responsive layouts
export class GridUtils {
  static getColumns(breakpoint: Breakpoint, config: Partial<Record<Breakpoint, number>>): number {
    // Return the appropriate column count for the current breakpoint
    const breakpointOrder: Breakpoint[] = ['mobile', 'tablet', 'desktop', 'large'];
    const currentIndex = breakpointOrder.indexOf(breakpoint);
    
    // Find the closest defined breakpoint
    for (let i = currentIndex; i >= 0; i--) {
      const bp = breakpointOrder[i];
      if (config[bp] !== undefined) {
        return config[bp]!;
      }
    }
    
    return 1; // Default to 1 column
  }

  static generateGridClasses(config: Partial<Record<Breakpoint, number>>): string {
    const classes: string[] = [];
    
    Object.entries(config).forEach(([breakpoint, columns]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        classes.push(`grid-cols-${columns}`);
      } else {
        classes.push(`${bp}:grid-cols-${columns}`);
      }
    });
    
    return classes.join(' ');
  }
}

// Typography utilities for responsive text
export class TypographyUtils {
  static getResponsiveFontSize(
    config: Partial<Record<Breakpoint, string>>
  ): Record<string, string> {
    const styles: Record<string, string> = {};
    
    Object.entries(config).forEach(([breakpoint, fontSize]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        styles['font-size'] = fontSize;
      } else {
        const mediaQuery = `(min-width: ${breakpoints[bp]})`;
        styles[`@media ${mediaQuery}`] = { 'font-size': fontSize };
      }
    });
    
    return styles;
  }

  static getResponsiveLineHeight(
    config: Partial<Record<Breakpoint, string>>
  ): Record<string, string> {
    const styles: Record<string, string> = {};
    
    Object.entries(config).forEach(([breakpoint, lineHeight]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        styles['line-height'] = lineHeight;
      } else {
        const mediaQuery = `(min-width: ${breakpoints[bp]})`;
        styles[`@media ${mediaQuery}`] = { 'line-height': lineHeight };
      }
    });
    
    return styles;
  }
}

// Spacing utilities for responsive margins and padding
export class SpacingUtils {
  static getResponsiveSpacing(
    property: 'margin' | 'padding',
    config: Partial<Record<Breakpoint, string>>
  ): Record<string, string> {
    const styles: Record<string, string> = {};
    
    Object.entries(config).forEach(([breakpoint, spacing]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        styles[property] = spacing;
      } else {
        const mediaQuery = `(min-width: ${breakpoints[bp]})`;
        styles[`@media ${mediaQuery}`] = { [property]: spacing };
      }
    });
    
    return styles;
  }
}

// Layout utilities
export class LayoutUtils {
  static getFlexDirection(breakpoint: Breakpoint, config: Partial<Record<Breakpoint, 'row' | 'column'>>): string {
    const breakpointOrder: Breakpoint[] = ['mobile', 'tablet', 'desktop', 'large'];
    const currentIndex = breakpointOrder.indexOf(breakpoint);
    
    for (let i = currentIndex; i >= 0; i--) {
      const bp = breakpointOrder[i];
      if (config[bp] !== undefined) {
        return config[bp]!;
      }
    }
    
    return 'column'; // Default to column
  }

  static generateFlexClasses(config: Partial<Record<Breakpoint, 'row' | 'column'>>): string {
    const classes: string[] = [];
    
    Object.entries(config).forEach(([breakpoint, direction]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        classes.push(`flex-${direction}`);
      } else {
        classes.push(`${bp}:flex-${direction}`);
      }
    });
    
    return classes.join(' ');
  }
}

// Image utilities for responsive images
export class ImageUtils {
  static generateSrcSet(
    basePath: string,
    sizes: number[],
    extension = 'jpg'
  ): string {
    return sizes
      .map(size => `${basePath}-${size}w.${extension} ${size}w`)
      .join(', ');
  }

  static generateSizes(config: Partial<Record<Breakpoint, string>>): string {
    const sizes: string[] = [];
    
    Object.entries(config).forEach(([breakpoint, size]) => {
      const bp = breakpoint as Breakpoint;
      if (bp === 'mobile') {
        sizes.push(size);
      } else {
        sizes.push(`(min-width: ${breakpoints[bp]}) ${size}`);
      }
    });
    
    return sizes.reverse().join(', ');
  }
}

// Viewport utilities
export class ViewportUtils {
  static getViewportSize(): { width: number; height: number } {
    return {
      width: Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0),
      height: Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0),
    };
  }

  static onViewportChange(callback: (size: { width: number; height: number }) => void): () => void {
    const handleResize = () => {
      callback(this.getViewportSize());
    };

    window.addEventListener('resize', handleResize);
    
    // Initial call
    handleResize();

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }
}

// CSS-in-JS utilities for responsive styles
export function createResponsiveStyles(
  baseStyles: Record<string, string>,
  responsiveStyles: Partial<Record<Breakpoint, Record<string, string>>>
): string {
  let css = '';
  
  // Base styles
  Object.entries(baseStyles).forEach(([property, value]) => {
    css += `${property}: ${value}; `;
  });
  
  // Responsive styles
  Object.entries(responsiveStyles).forEach(([breakpoint, styles]) => {
    const bp = breakpoint as Breakpoint;
    if (bp !== 'mobile') {
      css += `@media (min-width: ${breakpoints[bp]}) { `;
      Object.entries(styles).forEach(([property, value]) => {
        css += `${property}: ${value}; `;
      });
      css += '} ';
    }
  });
  
  return css.trim();
}