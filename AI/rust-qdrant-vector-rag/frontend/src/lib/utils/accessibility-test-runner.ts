/**
 * Test runner for accessibility fixes
 */

import { validateAccessibility, logAccessibilityValidation } from './accessibility-validator.js';
import { testContrastRatios } from './contrast-test.js';

/**
 * Run all accessibility tests
 */
export function runAccessibilityTests(): void {
  if (import.meta.env.DEV) {
    console.group('🧪 Running Accessibility Tests');
    
    // Test 1: Color contrast ratios
    console.log('1. Testing color contrast ratios...');
    testContrastRatios();
    
    // Test 2: Form labels and general accessibility
    console.log('2. Testing form labels and accessibility...');
    const result = validateAccessibility();
    logAccessibilityValidation(result);
    
    // Summary
    console.log('3. Test Summary:');
    if (result.passed) {
      console.log('✅ All accessibility tests passed!');
    } else {
      console.log(`❌ Found ${result.issues.length} issues that need attention.`);
    }
    
    console.groupEnd();
  }
}

/**
 * Test specific component for accessibility
 */
export function testComponentAccessibility(componentSelector: string): void {
  if (import.meta.env.DEV) {
    const component = document.querySelector(componentSelector);
    if (component) {
      console.group(`🔍 Testing ${componentSelector} accessibility`);
      const result = validateAccessibility(component);
      logAccessibilityValidation(result);
      console.groupEnd();
    } else {
      console.warn(`Component ${componentSelector} not found`);
    }
  }
}

// Auto-run tests in development when DOM is ready
if (import.meta.env.DEV) {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      setTimeout(runAccessibilityTests, 1000); // Wait for components to render
    });
  } else {
    setTimeout(runAccessibilityTests, 1000);
  }
}