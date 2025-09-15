/**
 * Accessibility testing utilities for development and testing
 */

import { ColorContrast } from './accessibility.js';

// Accessibility audit results
export interface AccessibilityIssue {
  type: 'error' | 'warning' | 'info';
  rule: string;
  message: string;
  element?: Element;
  severity: 'critical' | 'serious' | 'moderate' | 'minor';
}

export interface AccessibilityAuditResult {
  passed: boolean;
  issues: AccessibilityIssue[];
  score: number; // 0-100
  summary: {
    critical: number;
    serious: number;
    moderate: number;
    minor: number;
  };
}

// Accessibility audit class
export class AccessibilityAuditor {
  private issues: AccessibilityIssue[] = [];

  // Run comprehensive accessibility audit
  audit(container: Element = document.body): AccessibilityAuditResult {
    this.issues = [];

    // Run all audit checks
    this.checkHeadingStructure(container);
    this.checkImageAltText(container);
    this.checkFormLabels(container);
    this.checkColorContrast(container);
    this.checkFocusableElements(container);
    this.checkAriaAttributes(container);
    this.checkLandmarks(container);
    this.checkKeyboardNavigation(container);
    this.checkTouchTargets(container);

    // Calculate score and summary
    const summary = this.calculateSummary();
    const score = this.calculateScore(summary);

    return {
      passed: this.issues.filter(issue => issue.type === 'error').length === 0,
      issues: this.issues,
      score,
      summary,
    };
  }

  // Check heading structure (h1-h6)
  private checkHeadingStructure(container: Element): void {
    const headings = Array.from(container.querySelectorAll('h1, h2, h3, h4, h5, h6'));
    
    if (headings.length === 0) {
      this.addIssue({
        type: 'warning',
        rule: 'heading-structure',
        message: 'No headings found. Use headings to structure content.',
        severity: 'moderate',
      });
      return;
    }

    // Check for h1
    const h1Elements = headings.filter(h => h.tagName === 'H1');
    if (h1Elements.length === 0) {
      this.addIssue({
        type: 'error',
        rule: 'heading-structure',
        message: 'Missing h1 element. Every page should have exactly one h1.',
        severity: 'serious',
      });
    } else if (h1Elements.length > 1) {
      this.addIssue({
        type: 'warning',
        rule: 'heading-structure',
        message: 'Multiple h1 elements found. Use only one h1 per page.',
        severity: 'moderate',
      });
    }

    // Check heading hierarchy
    let previousLevel = 0;
    headings.forEach((heading, index) => {
      const level = parseInt(heading.tagName.charAt(1));
      
      if (index === 0 && level !== 1) {
        this.addIssue({
          type: 'warning',
          rule: 'heading-structure',
          message: 'First heading should be h1.',
          element: heading,
          severity: 'moderate',
        });
      }
      
      if (level > previousLevel + 1) {
        this.addIssue({
          type: 'warning',
          rule: 'heading-structure',
          message: `Heading level skipped from h${previousLevel} to h${level}.`,
          element: heading,
          severity: 'minor',
        });
      }
      
      previousLevel = level;
    });
  }

  // Check image alt text
  private checkImageAltText(container: Element): void {
    const images = Array.from(container.querySelectorAll('img'));
    
    images.forEach(img => {
      const alt = img.getAttribute('alt');
      const ariaLabel = img.getAttribute('aria-label');
      const ariaLabelledby = img.getAttribute('aria-labelledby');
      
      if (!alt && !ariaLabel && !ariaLabelledby) {
        this.addIssue({
          type: 'error',
          rule: 'image-alt',
          message: 'Image missing alt text or aria-label.',
          element: img,
          severity: 'serious',
        });
      } else if (alt === '') {
        // Empty alt is okay for decorative images
        const role = img.getAttribute('role');
        if (role !== 'presentation' && role !== 'none') {
          this.addIssue({
            type: 'info',
            rule: 'image-alt',
            message: 'Image has empty alt text. Ensure this is decorative.',
            element: img,
            severity: 'minor',
          });
        }
      }
    });
  }

  // Check form labels
  private checkFormLabels(container: Element): void {
    const formControls = Array.from(container.querySelectorAll('input, select, textarea'));
    
    formControls.forEach(control => {
      const type = control.getAttribute('type');
      if (type === 'hidden' || type === 'submit' || type === 'button') {
        return; // Skip these types
      }

      const id = control.getAttribute('id');
      const ariaLabel = control.getAttribute('aria-label');
      const ariaLabelledby = control.getAttribute('aria-labelledby');
      
      let hasLabel = false;
      
      if (id) {
        const label = container.querySelector(`label[for="${id}"]`);
        if (label) hasLabel = true;
      }
      
      if (ariaLabel || ariaLabelledby) {
        hasLabel = true;
      }
      
      // Check if wrapped in label
      const parentLabel = control.closest('label');
      if (parentLabel) hasLabel = true;
      
      if (!hasLabel) {
        this.addIssue({
          type: 'error',
          rule: 'form-labels',
          message: 'Form control missing accessible label.',
          element: control,
          severity: 'serious',
        });
      }
    });
  }

  // Check color contrast
  private checkColorContrast(container: Element): void {
    const textElements = Array.from(container.querySelectorAll('*')).filter(el => {
      const style = window.getComputedStyle(el);
      return style.color && style.backgroundColor && el.textContent?.trim();
    });

    textElements.forEach(element => {
      const style = window.getComputedStyle(element);
      const color = this.rgbToHex(style.color);
      const backgroundColor = this.rgbToHex(style.backgroundColor);
      
      if (color && backgroundColor && color !== backgroundColor) {
        const fontSize = parseFloat(style.fontSize);
        const fontWeight = style.fontWeight;
        const isLargeText = fontSize >= 18 || (fontSize >= 14 && (fontWeight === 'bold' || parseInt(fontWeight) >= 700));
        
        if (!ColorContrast.meetsWCAGAA(color, backgroundColor, isLargeText)) {
          const ratio = ColorContrast.getContrastRatio(color, backgroundColor);
          this.addIssue({
            type: 'error',
            rule: 'color-contrast',
            message: `Insufficient color contrast ratio: ${ratio.toFixed(2)}:1. Minimum required: ${isLargeText ? '3:1' : '4.5:1'}.`,
            element: element,
            severity: 'serious',
          });
        }
      }
    });
  }

  // Check focusable elements
  private checkFocusableElements(container: Element): void {
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

      // Check if element is visible but not focusable
      const style = window.getComputedStyle(element);
      if (style.display !== 'none' && style.visibility !== 'hidden') {
        if (element.hasAttribute('disabled') || tabindex === '-1') {
          // This is okay, element is intentionally not focusable
        } else {
          // Check if element has focus styles
          this.checkFocusStyles(element);
        }
      }
    });
  }

  // ARIA 속성 검사 (유효하지 않은 CSS 셀렉터 [aria-*] 대신 DOM 속성 순회를 사용)
  private checkAriaAttributes(container: Element): void {
    // 모든 요소를 순회하며 aria-로 시작하는 속성을 가진 요소만 필터링
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

      // 불필요한 ARIA 속성 검사
      const role = element.getAttribute('role');
      const tagName = element.tagName.toLowerCase();
      if (role === 'button' && tagName === 'button') {
        this.addIssue({
          type: 'info',
          rule: 'aria-attributes',
          message: 'Redundant role="button" on button element.',
          element: element,
          severity: 'minor',
        });
      }
    });
  }

  // Check landmarks
  private checkLandmarks(container: Element): void {
    const landmarks = Array.from(container.querySelectorAll(
      'main, nav, aside, header, footer, section, [role="main"], [role="navigation"], [role="complementary"], [role="banner"], [role="contentinfo"]'
    ));

    const mainElements = landmarks.filter(el => 
      el.tagName === 'MAIN' || el.getAttribute('role') === 'main'
    );

    if (mainElements.length === 0) {
      this.addIssue({
        type: 'warning',
        rule: 'landmarks',
        message: 'No main landmark found. Use <main> or role="main".',
        severity: 'moderate',
      });
    } else if (mainElements.length > 1) {
      this.addIssue({
        type: 'warning',
        rule: 'landmarks',
        message: 'Multiple main landmarks found. Use only one per page.',
        severity: 'moderate',
      });
    }
  }

  // Check keyboard navigation
  private checkKeyboardNavigation(container: Element): void {
    const interactiveElements = Array.from(container.querySelectorAll(
      'button, input, select, textarea, a[href], [onclick], [onkeydown], [tabindex]:not([tabindex="-1"])'
    ));

    interactiveElements.forEach(element => {
      // Check if clickable elements have keyboard handlers
      if (element.hasAttribute('onclick') && !element.hasAttribute('onkeydown')) {
        this.addIssue({
          type: 'warning',
          rule: 'keyboard-navigation',
          message: 'Clickable element should also handle keyboard events.',
          element: element,
          severity: 'moderate',
        });
      }
    });
  }

  // Check touch targets
  private checkTouchTargets(container: Element): void {
    const interactiveElements = Array.from(container.querySelectorAll(
      'button, input, select, textarea, a[href], [onclick], [role="button"]'
    ));

    interactiveElements.forEach(element => {
      const rect = element.getBoundingClientRect();
      const minSize = 44; // 44px minimum touch target

      if (rect.width < minSize || rect.height < minSize) {
        this.addIssue({
          type: 'warning',
          rule: 'touch-targets',
          message: `Touch target too small: ${rect.width}x${rect.height}px. Minimum: ${minSize}x${minSize}px.`,
          element: element,
          severity: 'moderate',
        });
      }
    });
  }

  // Helper methods
  private addIssue(issue: AccessibilityIssue): void {
    this.issues.push(issue);
  }

  private checkFocusStyles(element: Element): void {
    // This is a simplified check - in a real implementation,
    // you'd need to trigger focus and check computed styles
    const style = window.getComputedStyle(element);
    if (!style.outline && !style.boxShadow) {
      this.addIssue({
        type: 'warning',
        rule: 'focus-styles',
        message: 'Focusable element may be missing focus styles.',
        element: element,
        severity: 'moderate',
      });
    }
  }

  private rgbToHex(rgb: string): string | null {
    const match = rgb.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
    if (!match || !match[1] || !match[2] || !match[3]) return null;
    
    const r = parseInt(match[1], 10);
    const g = parseInt(match[2], 10);
    const b = parseInt(match[3], 10);
    
    return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
  }

  private calculateSummary() {
    return this.issues.reduce(
      (summary, issue) => {
        summary[issue.severity]++;
        return summary;
      },
      { critical: 0, serious: 0, moderate: 0, minor: 0 }
    );
  }

  private calculateScore(summary: { critical: number; serious: number; moderate: number; minor: number }): number {
    const totalIssues = summary.critical + summary.serious + summary.moderate + summary.minor;
    if (totalIssues === 0) return 100;

    // Weight issues by severity
    const weightedScore = 
      (summary.critical * 25) + 
      (summary.serious * 15) + 
      (summary.moderate * 8) + 
      (summary.minor * 3);

    return Math.max(0, 100 - weightedScore);
  }
}

// Development helper to run accessibility audit
export function runAccessibilityAudit(container?: Element): AccessibilityAuditResult {
  const auditor = new AccessibilityAuditor();
  return auditor.audit(container);
}

// Log accessibility issues to console (development only)
export function logAccessibilityIssues(result: AccessibilityAuditResult): void {
  if (import.meta.env.DEV) {
    console.group(`🔍 Accessibility Audit (Score: ${result.score}/100)`);
    
    if (result.issues.length === 0) {
      console.log('✅ No accessibility issues found!');
    } else {
      result.issues.forEach(issue => {
        const emoji = issue.type === 'error' ? '❌' : issue.type === 'warning' ? '⚠️' : 'ℹ️';
        console.log(`${emoji} [${issue.rule}] ${issue.message}`, issue.element);
      });
    }
    
    console.groupEnd();
  }
}