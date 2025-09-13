/**
 * Demonstration script for responsive design and accessibility features
 * This script showcases the enhanced features implemented in task 10
 */

import { BreakpointUtils } from '../utils/responsive.js';
import { FocusManager, ColorContrast, announceToScreenReader } from '../utils/accessibility.js';

export class ResponsiveAccessibilityDemo {
  private demoContainer: HTMLElement;
  private currentBreakpoint: string = 'mobile';

  constructor(container: HTMLElement) {
    this.demoContainer = container;
    this.init();
  }

  private init() {
    this.createDemoUI();
    this.setupBreakpointListener();
    this.setupAccessibilityTests();
  }

  private createDemoUI() {
    this.demoContainer.innerHTML = `
      <div class="demo-container">
        <header class="demo-header">
          <h1 class="text-responsive-xl">Responsive Design & Accessibility Demo</h1>
          <p class="text-responsive-base">This demo showcases the enhanced features implemented in task 10</p>
        </header>

        <section class="demo-section" aria-labelledby="breakpoint-title">
          <h2 id="breakpoint-title" class="text-responsive-lg">Current Breakpoint</h2>
          <div class="breakpoint-display">
            <span id="current-breakpoint" class="breakpoint-value">${this.currentBreakpoint}</span>
          </div>
          <div class="breakpoint-info">
            <p>Resize your browser window to see breakpoint changes</p>
          </div>
        </section>

        <section class="demo-section" aria-labelledby="responsive-title">
          <h2 id="responsive-title" class="text-responsive-lg">Responsive Layout Examples</h2>
          
          <div class="responsive-grid grid-responsive">
            <div class="demo-card">
              <h3>Mobile First</h3>
              <p>Layouts adapt from mobile to desktop</p>
            </div>
            <div class="demo-card">
              <h3>Flexible Grid</h3>
              <p>Grid columns adjust based on screen size</p>
            </div>
            <div class="demo-card">
              <h3>Touch Targets</h3>
              <p>Minimum 44px touch targets on mobile</p>
            </div>
            <div class="demo-card">
              <h3>Responsive Text</h3>
              <p>Text scales appropriately across devices</p>
            </div>
          </div>
        </section>

        <section class="demo-section" aria-labelledby="accessibility-title">
          <h2 id="accessibility-title" class="text-responsive-lg">Accessibility Features</h2>
          
          <div class="accessibility-demos flex-responsive">
            <div class="demo-group">
              <h3>Focus Management</h3>
              <button class="demo-button focus-ring-enhanced touch-target-enhanced" id="focus-test-1">
                Button 1
              </button>
              <button class="demo-button focus-ring-enhanced touch-target-enhanced" id="focus-test-2">
                Button 2
              </button>
              <button class="demo-button focus-ring-enhanced touch-target-enhanced" id="focus-test-3">
                Button 3
              </button>
            </div>

            <div class="demo-group">
              <h3>Color Contrast</h3>
              <div class="contrast-examples">
                <div class="contrast-good" role="img" aria-label="Good contrast example">
                  Good Contrast (AA)
                </div>
                <div class="contrast-better" role="img" aria-label="Better contrast example">
                  Better Contrast (AAA)
                </div>
              </div>
            </div>

            <div class="demo-group">
              <h3>Screen Reader Support</h3>
              <button class="demo-button" id="announce-test">
                Test Announcement
                <span class="sr-only">This will announce a message to screen readers</span>
              </button>
              <div class="sr-only" aria-live="polite" id="announcement-area"></div>
            </div>
          </div>
        </section>

        <section class="demo-section" aria-labelledby="features-title">
          <h2 id="features-title" class="text-responsive-lg">Enhanced Features</h2>
          
          <ul class="features-list" role="list">
            <li>✅ Mobile-first responsive design</li>
            <li>✅ WCAG 2.1 AA compliant color contrast</li>
            <li>✅ Enhanced focus indicators</li>
            <li>✅ Minimum touch target sizes (44px)</li>
            <li>✅ Screen reader announcements</li>
            <li>✅ Keyboard navigation support</li>
            <li>✅ Semantic HTML structure</li>
            <li>✅ High contrast mode support</li>
            <li>✅ Reduced motion preferences</li>
            <li>✅ Dark mode compatibility</li>
          </ul>
        </section>
      </div>
    `;

    this.addDemoStyles();
    this.bindDemoEvents();
  }

  private addDemoStyles() {
    const style = document.createElement('style');
    style.textContent = `
      .demo-container {
        max-width: 1200px;
        margin: 0 auto;
        padding: var(--spacing-xl);
        font-family: var(--font-family-base);
      }

      .demo-header {
        text-align: center;
        margin-bottom: var(--spacing-3xl);
        padding-bottom: var(--spacing-xl);
        border-bottom: 2px solid var(--color-surface-200);
      }

      .demo-section {
        margin-bottom: var(--spacing-3xl);
        padding: var(--spacing-xl);
        background: var(--color-surface-100);
        border-radius: 1rem;
        border: 2px solid var(--color-surface-200);
      }

      .breakpoint-display {
        text-align: center;
        margin: var(--spacing-lg) 0;
      }

      .breakpoint-value {
        display: inline-block;
        padding: var(--spacing-md) var(--spacing-xl);
        background: var(--color-primary-500);
        color: white;
        border-radius: 0.5rem;
        font-size: var(--font-size-xl);
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.05em;
      }

      .breakpoint-info {
        text-align: center;
        margin-top: var(--spacing-lg);
        color: var(--color-surface-600);
        font-style: italic;
      }

      .demo-card {
        padding: var(--spacing-lg);
        background: var(--color-surface-50);
        border-radius: 0.75rem;
        border: 1px solid var(--color-surface-300);
        text-align: center;
      }

      .demo-card h3 {
        margin: 0 0 var(--spacing-sm) 0;
        color: var(--color-primary-600);
        font-size: var(--font-size-lg);
      }

      .demo-card p {
        margin: 0;
        color: var(--color-surface-600);
        font-size: var(--font-size-sm);
        line-height: var(--line-height-relaxed);
      }

      .demo-group {
        padding: var(--spacing-lg);
        background: var(--color-surface-50);
        border-radius: 0.75rem;
        border: 1px solid var(--color-surface-300);
      }

      .demo-group h3 {
        margin: 0 0 var(--spacing-md) 0;
        color: var(--color-surface-800);
        font-size: var(--font-size-base);
      }

      .demo-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: var(--spacing-sm) var(--spacing-md);
        margin: var(--spacing-xs);
        background: var(--color-primary-500);
        color: white;
        border: 2px solid var(--color-primary-500);
        border-radius: 0.5rem;
        font-size: var(--font-size-sm);
        font-weight: 600;
        cursor: pointer;
        transition: all var(--duration-fast) ease;
      }

      .demo-button:hover {
        background: var(--color-primary-600);
        border-color: var(--color-primary-600);
        transform: translateY(-1px);
        box-shadow: var(--shadow-md);
      }

      .demo-button:active {
        transform: translateY(0);
        box-shadow: var(--shadow-sm);
      }

      .contrast-examples {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
      }

      .contrast-good {
        padding: var(--spacing-sm);
        background: #ffffff;
        color: #595959;
        border-radius: 0.375rem;
        text-align: center;
        font-weight: 600;
      }

      .contrast-better {
        padding: var(--spacing-sm);
        background: #ffffff;
        color: #000000;
        border-radius: 0.375rem;
        text-align: center;
        font-weight: 600;
      }

      .features-list {
        list-style: none;
        padding: 0;
        margin: 0;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: var(--spacing-sm);
      }

      .features-list li {
        padding: var(--spacing-sm);
        background: var(--color-success-50);
        border-radius: 0.375rem;
        border-left: 4px solid var(--color-success-500);
        font-weight: 500;
      }

      /* Responsive adjustments */
      @media (max-width: 767px) {
        .demo-container {
          padding: var(--spacing-lg);
        }

        .demo-section {
          padding: var(--spacing-lg);
        }

        .features-list {
          grid-template-columns: 1fr;
        }

        .breakpoint-value {
          font-size: var(--font-size-lg);
          padding: var(--spacing-sm) var(--spacing-lg);
        }
      }

      /* Dark mode support */
      @media (prefers-color-scheme: dark) {
        .demo-section {
          background: var(--color-surface-800);
          border-color: var(--color-surface-700);
        }

        .demo-card,
        .demo-group {
          background: var(--color-surface-700);
          border-color: var(--color-surface-600);
        }

        .demo-card h3 {
          color: var(--color-primary-400);
        }

        .demo-card p {
          color: var(--color-surface-300);
        }

        .demo-group h3 {
          color: var(--color-surface-200);
        }

        .contrast-good,
        .contrast-better {
          background: var(--color-surface-900);
          color: var(--color-surface-100);
        }

        .features-list li {
          background: var(--color-surface-700);
          border-left-color: var(--color-success-400);
          color: var(--color-surface-200);
        }
      }

      /* High contrast mode */
      @media (prefers-contrast: high) {
        .demo-section {
          border-width: 3px;
        }

        .demo-button {
          border-width: 3px;
        }

        .features-list li {
          border-left-width: 6px;
        }
      }

      /* Reduced motion */
      @media (prefers-reduced-motion: reduce) {
        .demo-button {
          transition: none;
        }

        .demo-button:hover,
        .demo-button:active {
          transform: none;
        }
      }
    `;
    document.head.appendChild(style);
  }

  private bindDemoEvents() {
    // Focus management demo
    const focusButtons = this.demoContainer.querySelectorAll('[id^="focus-test-"]');
    focusButtons.forEach((button, index) => {
      button.addEventListener('click', () => {
        const nextIndex = (index + 1) % focusButtons.length;
        const nextButton = focusButtons[nextIndex] as HTMLElement;
        nextButton.focus();
        announceToScreenReader(`Focused on button ${nextIndex + 1}`, 'polite');
      });
    });

    // Screen reader announcement demo
    const announceButton = this.demoContainer.querySelector('#announce-test');
    if (announceButton) {
      announceButton.addEventListener('click', () => {
        const messages = [
          'This is a test announcement for screen readers',
          'Accessibility features are working correctly',
          'Enhanced focus management is active',
          'Responsive design is adapting to your screen size'
        ];
        const randomMessage = messages[Math.floor(Math.random() * messages.length)];
        announceToScreenReader(randomMessage, 'polite');
      });
    }
  }

  private setupBreakpointListener() {
    const updateBreakpoint = () => {
      this.currentBreakpoint = BreakpointUtils.getCurrentBreakpoint();
      const breakpointElement = this.demoContainer.querySelector('#current-breakpoint');
      if (breakpointElement) {
        breakpointElement.textContent = this.currentBreakpoint;
      }
    };

    // Initial update
    updateBreakpoint();

    // Listen for changes
    BreakpointUtils.onBreakpointChange((breakpoint) => {
      this.currentBreakpoint = breakpoint;
      updateBreakpoint();
      announceToScreenReader(`Breakpoint changed to ${breakpoint}`, 'polite');
    });
  }

  private setupAccessibilityTests() {
    // Test color contrast
    this.testColorContrast();
    
    // Test focus management
    this.testFocusManagement();
  }

  private testColorContrast() {
    const contrastTests = [
      { fg: '#595959', bg: '#ffffff', element: '.contrast-good' },
      { fg: '#000000', bg: '#ffffff', element: '.contrast-better' }
    ];

    contrastTests.forEach(({ fg, bg, element }) => {
      const ratio = ColorContrast.getContrastRatio(fg, bg);
      const meetsAA = ColorContrast.meetsWCAGAA(fg, bg);
      const meetsAAA = ColorContrast.meetsWCAGAAA(fg, bg);
      
      const el = this.demoContainer.querySelector(element);
      if (el) {
        const compliance = meetsAAA ? 'AAA' : meetsAA ? 'AA' : 'Fail';
        el.setAttribute('title', `Contrast ratio: ${ratio.toFixed(2)}:1 (${compliance})`);
      }
    });
  }

  private testFocusManagement() {
    const focusableElements = FocusManager.getFocusableElements(this.demoContainer);
    console.log(`Found ${focusableElements.length} focusable elements in demo`);
    
    // Ensure all focusable elements have proper focus indicators
    focusableElements.forEach((element) => {
      if (!element.classList.contains('focus-ring-enhanced')) {
        element.classList.add('focus-ring-enhanced');
      }
    });
  }

  public runAccessibilityAudit(): void {
    const results = {
      focusableElements: FocusManager.getFocusableElements(this.demoContainer).length,
      colorContrastTests: this.runColorContrastTests(),
      semanticStructure: this.checkSemanticStructure(),
      ariaLabels: this.checkAriaLabels(),
      touchTargets: this.checkTouchTargets()
    };

    console.log('Accessibility Audit Results:', results);
    announceToScreenReader('Accessibility audit completed. Check console for results.', 'polite');
  }

  private runColorContrastTests() {
    const textElements = this.demoContainer.querySelectorAll('p, span, div, button, h1, h2, h3');
    let passCount = 0;
    let totalCount = 0;

    textElements.forEach((element) => {
      const styles = window.getComputedStyle(element);
      const color = styles.color;
      const backgroundColor = styles.backgroundColor;

      if (backgroundColor !== 'rgba(0, 0, 0, 0)' && backgroundColor !== 'transparent') {
        totalCount++;
        try {
          const meetsAA = ColorContrast.meetsWCAGAA(color, backgroundColor);
          if (meetsAA) passCount++;
        } catch (error) {
          // Skip elements where we can't calculate contrast
        }
      }
    });

    return { passed: passCount, total: totalCount };
  }

  private checkSemanticStructure() {
    const headings = this.demoContainer.querySelectorAll('h1, h2, h3, h4, h5, h6');
    const sections = this.demoContainer.querySelectorAll('section, article, aside, nav, main, header, footer');
    
    return {
      headings: headings.length,
      semanticElements: sections.length,
      hasProperHierarchy: this.validateHeadingHierarchy(headings)
    };
  }

  private validateHeadingHierarchy(headings: NodeListOf<Element>) {
    let previousLevel = 0;
    let isValid = true;

    headings.forEach((heading) => {
      const currentLevel = parseInt(heading.tagName.charAt(1));
      if (currentLevel > previousLevel + 1) {
        isValid = false;
      }
      previousLevel = currentLevel;
    });

    return isValid;
  }

  private checkAriaLabels() {
    const elementsWithAria = this.demoContainer.querySelectorAll('[aria-label], [aria-labelledby], [aria-describedby]');
    const interactiveElements = this.demoContainer.querySelectorAll('button, input, select, textarea, a');
    
    let labeledInteractive = 0;
    interactiveElements.forEach((element) => {
      if (element.getAttribute('aria-label') || 
          element.getAttribute('aria-labelledby') ||
          this.demoContainer.querySelector(`label[for="${element.id}"]`)) {
        labeledInteractive++;
      }
    });

    return {
      totalAriaElements: elementsWithAria.length,
      interactiveElements: interactiveElements.length,
      labeledInteractive
    };
  }

  private checkTouchTargets() {
    const interactiveElements = this.demoContainer.querySelectorAll('button, a, input, select, textarea');
    let adequateTargets = 0;

    interactiveElements.forEach((element) => {
      const styles = window.getComputedStyle(element);
      const width = parseFloat(styles.width);
      const height = parseFloat(styles.height);
      
      if (width >= 44 && height >= 44) {
        adequateTargets++;
      }
    });

    return {
      total: interactiveElements.length,
      adequate: adequateTargets
    };
  }
}

// Export for use in development
export function createDemo(container: HTMLElement): ResponsiveAccessibilityDemo {
  return new ResponsiveAccessibilityDemo(container);
}