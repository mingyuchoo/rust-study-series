/**
 * Accessibility utilities and helpers
 */

// Generate unique IDs for ARIA attributes
let idCounter = 0;
export function generateId(prefix = 'id'): string {
  return `${prefix}-${++idCounter}`;
}

// Screen reader only text utility
export function createScreenReaderText(text: string): string {
  return `<span class="sr-only">${text}</span>`;
}

// Focus management utilities
export class FocusManager {
  private static focusableSelectors = [
    'button:not([disabled])',
    'input:not([disabled])',
    'select:not([disabled])',
    'textarea:not([disabled])',
    'a[href]',
    '[tabindex]:not([tabindex="-1"])',
    '[contenteditable="true"]',
  ].join(', ');

  static getFocusableElements(container: Element): HTMLElement[] {
    return Array.from(
      container.querySelectorAll(this.focusableSelectors)
    ) as HTMLElement[];
  }

  static getFirstFocusableElement(container: Element): HTMLElement | null {
    const focusable = this.getFocusableElements(container);
    return focusable[0] || null;
  }

  static getLastFocusableElement(container: Element): HTMLElement | null {
    const focusable = this.getFocusableElements(container);
    return focusable[focusable.length - 1] || null;
  }

  static trapFocus(container: Element, event: KeyboardEvent): void {
    if (event.key !== 'Tab') return;

    const focusable = this.getFocusableElements(container);
    if (focusable.length === 0) return;

    const first = focusable[0];
    const last = focusable[focusable.length - 1];

    if (event.shiftKey) {
      // Shift + Tab
      if (document.activeElement === first) {
        event.preventDefault();
        last.focus();
      }
    } else {
      // Tab
      if (document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    }
  }

  static restoreFocus(element: HTMLElement | null): void {
    if (element && typeof element.focus === 'function') {
      element.focus();
    }
  }
}

// Keyboard navigation utilities
export class KeyboardNavigation {
  static readonly KEYS = {
    ENTER: 'Enter',
    SPACE: ' ',
    ESCAPE: 'Escape',
    ARROW_UP: 'ArrowUp',
    ARROW_DOWN: 'ArrowDown',
    ARROW_LEFT: 'ArrowLeft',
    ARROW_RIGHT: 'ArrowRight',
    HOME: 'Home',
    END: 'End',
    TAB: 'Tab',
  } as const;

  static isActivationKey(key: string): boolean {
    return key === this.KEYS.ENTER || key === this.KEYS.SPACE;
  }

  static isArrowKey(key: string): boolean {
    return [
      this.KEYS.ARROW_UP,
      this.KEYS.ARROW_DOWN,
      this.KEYS.ARROW_LEFT,
      this.KEYS.ARROW_RIGHT,
    ].includes(key as any);
  }

  static handleMenuNavigation(
    event: KeyboardEvent,
    items: HTMLElement[],
    currentIndex: number,
    onSelect?: (index: number) => void
  ): number {
    let newIndex = currentIndex;

    switch (event.key) {
      case this.KEYS.ARROW_DOWN:
        event.preventDefault();
        newIndex = (currentIndex + 1) % items.length;
        break;
      case this.KEYS.ARROW_UP:
        event.preventDefault();
        newIndex = currentIndex === 0 ? items.length - 1 : currentIndex - 1;
        break;
      case this.KEYS.HOME:
        event.preventDefault();
        newIndex = 0;
        break;
      case this.KEYS.END:
        event.preventDefault();
        newIndex = items.length - 1;
        break;
      case this.KEYS.ENTER:
      case this.KEYS.SPACE:
        event.preventDefault();
        onSelect?.(currentIndex);
        return currentIndex;
    }

    if (newIndex !== currentIndex && items[newIndex]) {
      items[newIndex].focus();
    }

    return newIndex;
  }
}

// ARIA utilities
export class AriaUtils {
  static setExpanded(element: Element, expanded: boolean): void {
    element.setAttribute('aria-expanded', expanded.toString());
  }

  static setSelected(element: Element, selected: boolean): void {
    element.setAttribute('aria-selected', selected.toString());
  }

  static setPressed(element: Element, pressed: boolean): void {
    element.setAttribute('aria-pressed', pressed.toString());
  }

  static setHidden(element: Element, hidden: boolean): void {
    if (hidden) {
      element.setAttribute('aria-hidden', 'true');
    } else {
      element.removeAttribute('aria-hidden');
    }
  }

  static setLive(element: Element, politeness: 'polite' | 'assertive' | 'off'): void {
    element.setAttribute('aria-live', politeness);
  }

  static describedBy(element: Element, ids: string[]): void {
    element.setAttribute('aria-describedby', ids.join(' '));
  }

  static labelledBy(element: Element, ids: string[]): void {
    element.setAttribute('aria-labelledby', ids.join(' '));
  }
}

// Announce to screen readers
export function announceToScreenReader(message: string, priority: 'polite' | 'assertive' = 'polite'): void {
  const announcement = document.createElement('div');
  announcement.setAttribute('aria-live', priority);
  announcement.setAttribute('aria-atomic', 'true');
  announcement.className = 'sr-only';
  announcement.textContent = message;

  document.body.appendChild(announcement);

  // Remove after announcement
  setTimeout(() => {
    document.body.removeChild(announcement);
  }, 1000);
}

// Color contrast utilities
export class ColorContrast {
  // Calculate relative luminance
  static getRelativeLuminance(color: string): number {
    // Convert hex to RGB
    const hex = color.replace('#', '');
    const r = parseInt(hex.substr(0, 2), 16) / 255;
    const g = parseInt(hex.substr(2, 2), 16) / 255;
    const b = parseInt(hex.substr(4, 2), 16) / 255;

    // Apply gamma correction
    const sRGB = [r, g, b].map(c => {
      return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
    });

    // Calculate relative luminance
    return 0.2126 * sRGB[0] + 0.7152 * sRGB[1] + 0.0722 * sRGB[2];
  }

  // Calculate contrast ratio between two colors
  static getContrastRatio(color1: string, color2: string): number {
    const l1 = this.getRelativeLuminance(color1);
    const l2 = this.getRelativeLuminance(color2);
    const lighter = Math.max(l1, l2);
    const darker = Math.min(l1, l2);
    return (lighter + 0.05) / (darker + 0.05);
  }

  // Check if contrast meets WCAG AA standards
  static meetsWCAGAA(foreground: string, background: string, isLargeText = false): boolean {
    const ratio = this.getContrastRatio(foreground, background);
    return isLargeText ? ratio >= 3 : ratio >= 4.5;
  }

  // Check if contrast meets WCAG AAA standards
  static meetsWCAGAAA(foreground: string, background: string, isLargeText = false): boolean {
    const ratio = this.getContrastRatio(foreground, background);
    return isLargeText ? ratio >= 4.5 : ratio >= 7;
  }
}

// Reduced motion utilities
export function prefersReducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

// High contrast mode detection
export function prefersHighContrast(): boolean {
  return window.matchMedia('(prefers-contrast: high)').matches;
}

// Dark mode detection
export function prefersDarkMode(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

// Viewport utilities for responsive design
export class ViewportUtils {
  static getViewportWidth(): number {
    return Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
  }

  static getViewportHeight(): number {
    return Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0);
  }

  static isMobile(): boolean {
    return this.getViewportWidth() < 768;
  }

  static isTablet(): boolean {
    const width = this.getViewportWidth();
    return width >= 768 && width < 1024;
  }

  static isDesktop(): boolean {
    return this.getViewportWidth() >= 1024;
  }
}

// Touch utilities
export class TouchUtils {
  static isTouchDevice(): boolean {
    return 'ontouchstart' in window || navigator.maxTouchPoints > 0;
  }

  static addTouchSupport(element: HTMLElement): void {
    if (this.isTouchDevice()) {
      element.style.minHeight = '44px';
      element.style.minWidth = '44px';
    }
  }
}