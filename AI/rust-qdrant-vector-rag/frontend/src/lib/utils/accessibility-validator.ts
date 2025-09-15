/**
 * Comprehensive accessibility validator for form controls and interactive elements
 */

export interface AccessibilityValidationResult {
  passed: boolean;
  issues: AccessibilityIssue[];
  summary: {
    formLabels: number;
    colorContrast: number;
    focusManagement: number;
    ariaAttributes: number;
  };
}

export interface AccessibilityIssue {
  type: 'error' | 'warning' | 'info';
  rule: string;
  message: string;
  element?: Element;
  severity: 'critical' | 'serious' | 'moderate' | 'minor';
}

export class AccessibilityValidator {
  private issues: AccessibilityIssue[] = [];

  /**
   * Validate all accessibility aspects of a container
   */
  validate(container: Element = document.body): AccessibilityValidationResult {
    this.issues = [];

    // Run all validation checks
    this.validateFormLabels(container);
    this.validateColorContrast(container);
    this.validateFocusManagement(container);
    this.validateAriaAttributes(container);

    const summary = this.calculateSummary();

    return {
      passed: this.issues.filter(issue => issue.type === 'error').length === 0,
      issues: this.issues,
      summary,
    };
  }

  /**
   * Validate form labels - the main issue we just fixed
   */
  private validateFormLabels(container: Element): void {
    const formControls = Array.from(container.querySelectorAll('input, select, textarea'));
    
    formControls.forEach(control => {
      const element = control as HTMLElement;
      const type = element.getAttribute('type');
      
      // Skip non-interactive input types
      if (type === 'hidden' || type === 'submit' || type === 'button') {
        return;
      }

      const id = element.getAttribute('id');
      const ariaLabel = element.getAttribute('aria-label');
      const ariaLabelledby = element.getAttribute('aria-labelledby');
      
      let hasLabel = false;
      
      // Check for explicit label
      if (id) {
        const label = container.querySelector(`label[for="${id}"]`);
        if (label) hasLabel = true;
      }
      
      // Check for aria-label or aria-labelledby
      if (ariaLabel || ariaLabelledby) {
        hasLabel = true;
      }
      
      // Check if wrapped in label
      const parentLabel = element.closest('label');
      if (parentLabel) hasLabel = true;
      
      if (!hasLabel) {
        this.addIssue({
          type: 'error',
          rule: 'form-labels',
          message: `Form control missing accessible label. Element: ${element.tagName.toLowerCase()}${type ? `[type="${type}"]` : ''}`,
          element: element,
          severity: 'serious',
        });
      }
    });
  }

  /**
   * Validate color contrast ratios
   */
  private validateColorContrast(container: Element): void {
    const textElements = Array.from(container.querySelectorAll('*')).filter(el => {
      const style = window.getComputedStyle(el);
      return style.color && style.backgroundColor && el.textContent?.trim();
    });

    textElements.forEach(element => {
      const style = window.getComputedStyle(element);
      const color = this.rgbToHex(style.color);
      const backgroundColor = this.rgbToHex(style.backgroundColor);
      
      if (color && backgroundColor && color !== backgroundColor) {
        const ratio = this.calculateContrastRatio(color, backgroundColor);
        const fontSize = parseFloat(style.fontSize);
        const fontWeight = style.fontWeight;
        const isLargeText = fontSize >= 18 || (fontSize >= 14 && (fontWeight === 'bold' || parseInt(fontWeight) >= 700));
        
        const requiredRatio = isLargeText ? 3.0 : 4.5;
        
        if (ratio < requiredRatio) {
          this.addIssue({
            type: 'error',
            rule: 'color-contrast',
            message: `Insufficient color contrast ratio: ${ratio.toFixed(2)}:1. Required: ${requiredRatio}:1`,
            element: element,
            severity: 'serious',
          });
        }
      }
    });
  }

  /**
   * Validate focus management
   */
  private validateFocusManagement(container: Element): void {
    const focusableElements = Array.from(container.querySelectorAll(
      'button, input, select, textarea, a[href], [tabindex]:not([tabindex="-1"])'
    ));

    focusableElements.forEach(element => {
      const tabindex = element.getAttribute('tabindex');
      
      // Check for positive tabindex (anti-pattern)
      if (tabindex && parseInt(tabindex) > 0) {
        this.addIssue({
          type: 'warning',
          rule: 'focus-management',
          message: 'Avoid positive tabindex values. Use 0 or -1.',
          element: element,
          severity: 'moderate',
        });
      }
    });
  }

  /**
   * Validate ARIA attributes
   */
  private validateAriaAttributes(container: Element): void {
    const elementsWithAria = Array.from(container.querySelectorAll('*')).filter((el) =>
      Array.from(el.attributes).some((attr) => attr.name.toLowerCase().startsWith('aria-'))
    );

    elementsWithAria.forEach(element => {
      // Check aria-labelledby references
      const labelledby = element.getAttribute('aria-labelledby');
      if (labelledby) {
        const ids = labelledby.split(' ').filter(id => id.trim() !== '');
        ids.forEach(id => {
          const trimmedId = id.trim();
          if (trimmedId && !container.querySelector(`#${trimmedId}`)) {
            this.addIssue({
              type: 'error',
              rule: 'aria-attributes',
              message: `aria-labelledby references non-existent element: ${trimmedId}`,
              element: element,
              severity: 'serious',
            });
          }
        });
      }

      // Check aria-describedby references
      const describedby = element.getAttribute('aria-describedby');
      if (describedby) {
        const ids = describedby.split(' ').filter(id => id.trim() !== '');
        ids.forEach(id => {
          const trimmedId = id.trim();
          if (trimmedId && !container.querySelector(`#${trimmedId}`)) {
            this.addIssue({
              type: 'error',
              rule: 'aria-attributes',
              message: `aria-describedby references non-existent element: ${trimmedId}`,
              element: element,
              severity: 'serious',
            });
          }
        });
      }
    });
  }

  /**
   * Helper methods
   */
  private addIssue(issue: AccessibilityIssue): void {
    this.issues.push(issue);
  }

  private rgbToHex(rgb: string): string | null {
    const match = rgb.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
    if (!match || !match[1] || !match[2] || !match[3]) return null;
    
    const r = parseInt(match[1], 10);
    const g = parseInt(match[2], 10);
    const b = parseInt(match[3], 10);
    
    return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
  }

  private calculateContrastRatio(color1: string, color2: string): number {
    const l1 = this.getRelativeLuminance(color1);
    const l2 = this.getRelativeLuminance(color2);
    const lighter = Math.max(l1, l2);
    const darker = Math.min(l1, l2);
    return (lighter + 0.05) / (darker + 0.05);
  }

  private getRelativeLuminance(color: string): number {
    const hex = color.replace('#', '');
    const r = parseInt(hex.substr(0, 2), 16) / 255;
    const g = parseInt(hex.substr(2, 2), 16) / 255;
    const b = parseInt(hex.substr(4, 2), 16) / 255;

    const sRGB = [r, g, b].map(c => {
      return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
    });

    return 0.2126 * sRGB[0] + 0.7152 * sRGB[1] + 0.0722 * sRGB[2];
  }

  private calculateSummary() {
    const summary = {
      formLabels: 0,
      colorContrast: 0,
      focusManagement: 0,
      ariaAttributes: 0,
    };

    this.issues.forEach(issue => {
      switch (issue.rule) {
        case 'form-labels':
          summary.formLabels++;
          break;
        case 'color-contrast':
          summary.colorContrast++;
          break;
        case 'focus-management':
          summary.focusManagement++;
          break;
        case 'aria-attributes':
          summary.ariaAttributes++;
          break;
      }
    });

    return summary;
  }
}

/**
 * Quick validation function for development use
 */
export function validateAccessibility(container?: Element): AccessibilityValidationResult {
  const validator = new AccessibilityValidator();
  return validator.validate(container);
}

/**
 * Log validation results to console (development only)
 */
export function logAccessibilityValidation(result: AccessibilityValidationResult): void {
  if (import.meta.env.DEV) {
    console.group('🔍 Accessibility Validation Results');
    
    if (result.passed) {
      console.log('✅ All accessibility checks passed!');
    } else {
      console.log(`❌ Found ${result.issues.length} accessibility issues:`);
      
      result.issues.forEach(issue => {
        const emoji = issue.type === 'error' ? '❌' : issue.type === 'warning' ? '⚠️' : 'ℹ️';
        console.log(`${emoji} [${issue.rule}] ${issue.severity}: ${issue.message}`, issue.element);
      });
    }
    
    console.log('Summary:', result.summary);
    console.groupEnd();
  }
}