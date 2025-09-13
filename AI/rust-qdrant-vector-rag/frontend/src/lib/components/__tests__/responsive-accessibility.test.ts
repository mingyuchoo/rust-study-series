import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { BreakpointUtils, MediaQuery } from '../../utils/responsive.js';
import { FocusManager, ColorContrast, announceToScreenReader } from '../../utils/accessibility.js';
import ResponsiveLayout from '../ResponsiveLayout.svelte';
import AccessibleButton from '../AccessibleButton.svelte';
import AccessibleInput from '../AccessibleInput.svelte';

// Mock window.matchMedia
const mockMatchMedia = vi.fn();
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: mockMatchMedia,
});

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

describe('Responsive Design Tests', () => {
  beforeEach(() => {
    // Ensure MediaQuery cache does not keep stale MediaQueryList instances between tests
    MediaQuery.clearCache();
    mockMatchMedia.mockClear();
    mockMatchMedia.mockReturnValue({
      matches: false,
      media: '',
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    });
  });

  describe('BreakpointUtils', () => {
    it('should detect mobile breakpoint correctly', () => {
      MediaQuery.clearCache();
      mockMatchMedia.mockImplementation((query) => ({
        matches: query === '(max-width: 768px)',
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      }));

      expect(BreakpointUtils.isMobile()).toBe(true);
      expect(BreakpointUtils.isTablet()).toBe(false);
      expect(BreakpointUtils.isDesktop()).toBe(false);
    });

    it('should detect tablet breakpoint correctly', () => {
      MediaQuery.clearCache();
      mockMatchMedia.mockImplementation((query) => ({
        matches: query === '(min-width: 768px) and (max-width: 1024px)',
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      }));

      expect(BreakpointUtils.isMobile()).toBe(false);
      expect(BreakpointUtils.isTablet()).toBe(true);
      expect(BreakpointUtils.isDesktop()).toBe(false);
    });

    it('should detect desktop breakpoint correctly', () => {
      MediaQuery.clearCache();
      mockMatchMedia.mockImplementation((query) => ({
        matches: query === '(min-width: 1024px)',
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      }));

      expect(BreakpointUtils.isMobile()).toBe(false);
      expect(BreakpointUtils.isTablet()).toBe(false);
      expect(BreakpointUtils.isDesktop()).toBe(true);
    });

    it('should return correct current breakpoint', () => {
      MediaQuery.clearCache();
      mockMatchMedia.mockImplementation((query) => ({
        matches: query === '(min-width: 1024px)',
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      }));

      expect(BreakpointUtils.getCurrentBreakpoint()).toBe('desktop');
    });
  });

  describe('ResponsiveLayout Component', () => {
    it('should render with correct container variant', () => {
      render(ResponsiveLayout, {
        props: {
          variant: 'container',
          className: 'test-container'
        }
      });

      const layout = screen.getByRole('generic');
      expect(layout).toHaveClass('responsive-layout');
      expect(layout).toHaveClass('variant-container');
      expect(layout).toHaveClass('test-container');
    });

    it('should render with correct grid variant', () => {
      render(ResponsiveLayout, {
        props: {
          variant: 'grid',
          columns: { mobile: 1, tablet: 2, desktop: 3 }
        }
      });

      const layout = screen.getByRole('generic');
      expect(layout).toHaveClass('variant-grid');
    });

    it('should apply correct ARIA attributes', () => {
      render(ResponsiveLayout, {
        props: {
          ariaLabel: 'Test layout',
          role: 'main'
        }
      });

      const layout = screen.getByRole('main');
      expect(layout).toHaveAttribute('aria-label', 'Test layout');
    });

    it('should use semantic HTML elements', () => {
      render(ResponsiveLayout, {
        props: {
          element: 'section',
          ariaLabel: 'Test section'
        }
      });

      const section = screen.getByRole('generic');
      expect(section.tagName.toLowerCase()).toBe('section');
    });
  });
});

describe('Accessibility Tests', () => {
  describe('FocusManager', () => {
    let container: HTMLElement;

    beforeEach(() => {
      container = document.createElement('div');
      container.innerHTML = `
        <button>Button 1</button>
        <input type="text" />
        <a href="#">Link</a>
        <button disabled>Disabled Button</button>
        <div tabindex="0">Focusable Div</div>
      `;
      document.body.appendChild(container);
    });

    afterEach(() => {
      document.body.removeChild(container);
    });

    it('should find all focusable elements', () => {
      const focusable = FocusManager.getFocusableElements(container);
      expect(focusable).toHaveLength(4); // Excludes disabled button
    });

    it('should get first focusable element', () => {
      const first = FocusManager.getFirstFocusableElement(container);
      expect(first?.tagName.toLowerCase()).toBe('button');
      expect(first?.textContent).toBe('Button 1');
    });

    it('should get last focusable element', () => {
      const last = FocusManager.getLastFocusableElement(container);
      expect(last?.tagName.toLowerCase()).toBe('div');
      expect(last?.getAttribute('tabindex')).toBe('0');
    });
  });

  describe('ColorContrast', () => {
    it('should calculate contrast ratio correctly', () => {
      const ratio = ColorContrast.getContrastRatio('#000000', '#ffffff');
      expect(ratio).toBeCloseTo(21, 0);
    });

    it('should check WCAG AA compliance', () => {
      expect(ColorContrast.meetsWCAGAA('#000000', '#ffffff')).toBe(true);
      expect(ColorContrast.meetsWCAGAA('#777777', '#ffffff')).toBe(false);
    });

    it('should check WCAG AAA compliance', () => {
      expect(ColorContrast.meetsWCAGAAA('#000000', '#ffffff')).toBe(true);
      expect(ColorContrast.meetsWCAGAAA('#666666', '#ffffff')).toBe(false);
    });

    it('should handle large text correctly', () => {
      // Large text has lower contrast requirements
      expect(ColorContrast.meetsWCAGAA('#777777', '#ffffff', true)).toBe(true);
      expect(ColorContrast.meetsWCAGAA('#777777', '#ffffff', false)).toBe(false);
    });
  });

  describe('AccessibleButton Component', () => {
    it('should render with correct ARIA attributes', () => {
      render(AccessibleButton, {
        props: {
          ariaLabel: 'Test button',
          ariaExpanded: false,
          ariaHaspopup: 'menu'
        }
      });

      const button = screen.getByRole('button');
      expect(button).toHaveAttribute('aria-label', 'Test button');
      expect(button).toHaveAttribute('aria-expanded', 'false');
      expect(button).toHaveAttribute('aria-haspopup', 'menu');
    });

    it('should handle keyboard activation', async () => {
      const handleClick = vi.fn();
      render(AccessibleButton, {
        props: {
          onclick: handleClick
        }
      });

      const button = screen.getByRole('button');
      
      // Test Enter key
      await fireEvent.keyDown(button, { key: 'Enter' });
      expect(handleClick).toHaveBeenCalledTimes(1);

      // Test Space key
      await fireEvent.keyDown(button, { key: ' ' });
      expect(handleClick).toHaveBeenCalledTimes(2);
    });

    it('should show loading state correctly', () => {
      render(AccessibleButton, {
        props: {
          loading: true
        }
      });

      const button = screen.getByRole('button');
      expect(button).toHaveClass('loading');
      expect(screen.getByText('Loading...')).toBeInTheDocument();
    });

    it('should be disabled when loading', () => {
      render(AccessibleButton, {
        props: {
          loading: true
        }
      });

      const button = screen.getByRole('button');
      expect(button).toHaveClass('loading');
      // Loading buttons should not be clickable
      expect(button).toHaveStyle({ pointerEvents: 'none' });
    });

    it('should render as link when href is provided', () => {
      render(AccessibleButton, {
        props: {
          href: 'https://example.com',
          target: '_blank'
        }
      });

      const link = screen.getByRole('link');
      expect(link).toHaveAttribute('href', 'https://example.com');
      expect(link).toHaveAttribute('target', '_blank');
    });

    it('should meet minimum touch target size', () => {
      render(AccessibleButton, {
        props: {
          size: 'md'
        }
      });

      const button = screen.getByRole('button');
      const styles = window.getComputedStyle(button);
      
      // Should have minimum 44px touch target
      expect(parseInt(styles.minHeight)).toBeGreaterThanOrEqual(44);
      expect(parseInt(styles.minWidth)).toBeGreaterThanOrEqual(44);
    });
  });

  describe('AccessibleInput Component', () => {
    it('should render with proper label association', () => {
      render(AccessibleInput, {
        props: {
          label: 'Test Input',
          id: 'test-input'
        }
      });

      const input = screen.getByRole('textbox');
      const label = screen.getByText('Test Input');
      
      expect(input).toHaveAttribute('id', 'test-input');
      expect(label).toHaveAttribute('for', 'test-input');
    });

    it('should show required indicator', () => {
      render(AccessibleInput, {
        props: {
          label: 'Required Input',
          required: true
        }
      });

      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('aria-required', 'true');
      expect(screen.getByText('*')).toBeInTheDocument();
    });

    it('should display error messages with proper ARIA', async () => {
      render(AccessibleInput, {
        props: {
          label: 'Test Input',
          error: 'This field is required'
        }
      });

      const input = screen.getByRole('textbox');
      
      // Focus and blur to trigger error display
      await fireEvent.focus(input);
      await fireEvent.blur(input);

      await waitFor(() => {
        expect(input).toHaveAttribute('aria-invalid', 'true');
        expect(screen.getByRole('alert')).toBeInTheDocument();
        expect(screen.getByText('This field is required')).toBeInTheDocument();
      });
    });

    it('should show character count when enabled', () => {
      render(AccessibleInput, {
        props: {
          label: 'Test Input',
          maxlength: 100,
          showCharacterCount: true,
          value: 'Hello'
        }
      });

      expect(screen.getByText('5/100 characters')).toBeInTheDocument();
    });

    it('should validate input on blur', async () => {
      render(AccessibleInput, {
        props: {
          label: 'Test Input',
          required: true,
          validateOnBlur: true
        }
      });

      const input = screen.getByRole('textbox');
      
      await fireEvent.focus(input);
      await fireEvent.blur(input);

      await waitFor(() => {
        expect(input).toHaveAttribute('aria-invalid', 'true');
      });
    });
  });

  describe('Screen Reader Announcements', () => {
    it('should create announcement element', () => {
      announceToScreenReader('Test announcement', 'polite');
      
      const announcement = document.querySelector('[aria-live="polite"]');
      expect(announcement).toBeInTheDocument();
      expect(announcement).toHaveTextContent('Test announcement');
      expect(announcement).toHaveClass('sr-only');
    });

    it('should remove announcement after timeout', async () => {
      announceToScreenReader('Test announcement', 'assertive');
      
      const announcement = document.querySelector('[aria-live="assertive"]');
      expect(announcement).toBeInTheDocument();

      // Wait for cleanup
      await waitFor(() => {
        expect(document.querySelector('[aria-live="assertive"]')).not.toBeInTheDocument();
      }, { timeout: 1500 });
    });
  });
});

describe('Responsive Behavior Tests', () => {
  it('should apply responsive classes correctly', () => {
    MediaQuery.clearCache();
    // Mock mobile viewport
    mockMatchMedia.mockImplementation((query) => ({
      matches: query === '(max-width: 768px)',
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    }));

    render(ResponsiveLayout, {
      props: {
        variant: 'grid',
        columns: { mobile: 1, tablet: 2, desktop: 3 }
      }
    });

    const layout = screen.getByRole('generic');
    expect(layout).toHaveClass('breakpoint-mobile');
  });

  it('should handle breakpoint changes', async () => {
    MediaQuery.clearCache();
    const mockAddEventListener = vi.fn();
    const mockRemoveEventListener = vi.fn();
    
    mockMatchMedia.mockImplementation(() => ({
      matches: false,
      media: '',
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: mockAddEventListener,
      removeEventListener: mockRemoveEventListener,
      dispatchEvent: vi.fn(),
    }));

    const { unmount } = render(ResponsiveLayout);
    
    expect(mockAddEventListener).toHaveBeenCalled();
    
    unmount();
    
    expect(mockRemoveEventListener).toHaveBeenCalled();
  });
});

describe('Accessibility Compliance Tests', () => {
  it('should have proper heading hierarchy', () => {
    const container = document.createElement('div');
    container.innerHTML = `
      <h1>Main Title</h1>
      <h2>Section Title</h2>
      <h3>Subsection Title</h3>
    `;

    const headings = Array.from(container.querySelectorAll('h1, h2, h3, h4, h5, h6'));
    let previousLevel = 0;
    let hasValidHierarchy = true;

    headings.forEach((heading) => {
      const currentLevel = parseInt(heading.tagName.charAt(1));
      if (currentLevel > previousLevel + 1) {
        hasValidHierarchy = false;
      }
      previousLevel = currentLevel;
    });

    expect(hasValidHierarchy).toBe(true);
  });

  it('should have proper form labels', () => {
    const container = document.createElement('div');
    container.innerHTML = `
      <label for="input1">Label 1</label>
      <input id="input1" type="text" />
      
      <input aria-label="Label 2" type="text" />
      
      <label>
        Label 3
        <input type="text" />
      </label>
    `;

    const inputs = container.querySelectorAll('input');
    let allHaveLabels = true;

    inputs.forEach((input) => {
      const hasLabel = input.getAttribute('aria-label') || 
                      input.getAttribute('aria-labelledby') ||
                      container.querySelector(`label[for="${input.id}"]`) ||
                      input.closest('label');
      
      if (!hasLabel) {
        allHaveLabels = false;
      }
    });

    expect(allHaveLabels).toBe(true);
  });

  it('should have sufficient color contrast', () => {
    // Test common color combinations
    const combinations = [
      { fg: '#000000', bg: '#ffffff', expected: true }, // Black on white
      { fg: '#ffffff', bg: '#000000', expected: true }, // White on black
      { fg: '#767676', bg: '#ffffff', expected: false }, // Light gray on white (fails AA)
      { fg: '#595959', bg: '#ffffff', expected: true }, // Dark gray on white (passes AA)
    ];

    combinations.forEach(({ fg, bg, expected }) => {
      const meetsAA = ColorContrast.meetsWCAGAA(fg, bg);
      expect(meetsAA).toBe(expected);
    });
  });
});