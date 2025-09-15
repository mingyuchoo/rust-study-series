/**
 * Comprehensive contrast verification utility
 * Tests all the color combinations we've fixed to ensure WCAG AA compliance
 */

export interface ContrastTestResult {
  colorCombination: string;
  ratio: number;
  meetsAA: boolean;
  meetsAAA: boolean;
  status: 'pass' | 'fail';
}

export interface ContrastVerificationResult {
  allPassed: boolean;
  totalTests: number;
  passedTests: number;
  failedTests: number;
  results: ContrastTestResult[];
}

/**
 * Calculate contrast ratio between two hex colors
 */
function calculateContrastRatio(color1: string, color2: string): number {
  const getLuminance = (color: string): number => {
    const hex = color.replace('#', '');
    const r = parseInt(hex.substr(0, 2), 16) / 255;
    const g = parseInt(hex.substr(2, 2), 16) / 255;
    const b = parseInt(hex.substr(4, 2), 16) / 255;

    const sRGB = [r, g, b].map(c => {
      return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
    });

    return 0.2126 * sRGB[0] + 0.7152 * sRGB[1] + 0.0722 * sRGB[2];
  };

  const l1 = getLuminance(color1);
  const l2 = getLuminance(color2);
  const lighter = Math.max(l1, l2);
  const darker = Math.min(l1, l2);
  return (lighter + 0.05) / (darker + 0.05);
}

/**
 * Test all the color combinations we've updated
 */
export function verifyContrastFixes(): ContrastVerificationResult {
  const testCombinations = [
    // Original problematic combinations (should now be fixed)
    { name: 'gray-600 on white (was gray-500)', fg: '#4b5563', bg: '#ffffff' },
    { name: 'gray-700 on white (was gray-600)', fg: '#374151', bg: '#ffffff' },
    { name: 'gray-800 on white (improved)', fg: '#1f2937', bg: '#ffffff' },
    
    // Background combinations
    { name: 'gray-800 on gray-200 (was gray-100)', fg: '#1f2937', bg: '#e5e7eb' },
    { name: 'gray-700 on gray-200', fg: '#374151', bg: '#e5e7eb' },
    
    // Border combinations (visual contrast)
    { name: 'gray-400 border visibility', fg: '#9ca3af', bg: '#ffffff' },
    
    // Dark mode combinations
    { name: 'gray-200 on gray-800 (dark mode)', fg: '#e5e7eb', bg: '#1f2937' },
    { name: 'gray-300 on gray-700 (dark mode)', fg: '#d1d5db', bg: '#374151' },
    { name: 'gray-200 on gray-600 (dark mode)', fg: '#e5e7eb', bg: '#4b5563' },
    
    // Interactive element combinations
    { name: 'gray-800 on gray-200 hover', fg: '#1f2937', bg: '#e5e7eb' },
    { name: 'gray-700 on gray-200 badge', fg: '#374151', bg: '#e5e7eb' },
  ];

  const results: ContrastTestResult[] = testCombinations.map(({ name, fg, bg }) => {
    const ratio = calculateContrastRatio(fg, bg);
    const meetsAA = ratio >= 4.5;
    const meetsAAA = ratio >= 7.0;
    
    return {
      colorCombination: name,
      ratio: Math.round(ratio * 100) / 100,
      meetsAA,
      meetsAAA,
      status: meetsAA ? 'pass' : 'fail'
    };
  });

  const passedTests = results.filter(r => r.status === 'pass').length;
  const failedTests = results.filter(r => r.status === 'fail').length;

  return {
    allPassed: failedTests === 0,
    totalTests: results.length,
    passedTests,
    failedTests,
    results
  };
}

/**
 * Log verification results to console
 */
export function logContrastVerification(): void {
  if (import.meta.env.DEV) {
    const verification = verifyContrastFixes();
    
    console.group('🎨 Contrast Fixes Verification');
    
    if (verification.allPassed) {
      console.log(`✅ All ${verification.totalTests} contrast tests passed!`);
    } else {
      console.log(`❌ ${verification.failedTests} of ${verification.totalTests} tests failed`);
    }
    
    console.log('\nDetailed Results:');
    verification.results.forEach(result => {
      const emoji = result.status === 'pass' ? '✅' : '❌';
      const level = result.meetsAAA ? 'AAA' : result.meetsAA ? 'AA' : 'Fail';
      console.log(`${emoji} ${result.colorCombination}: ${result.ratio}:1 (${level})`);
    });
    
    console.log(`\nSummary: ${verification.passedTests}/${verification.totalTests} passed`);
    console.groupEnd();
  }
}

/**
 * Quick test function for development
 */
export function testContrastFixes(): boolean {
  const result = verifyContrastFixes();
  logContrastVerification();
  return result.allPassed;
}

// Auto-run verification in development
if (import.meta.env.DEV) {
  // Run after a short delay to ensure DOM is ready
  setTimeout(() => {
    logContrastVerification();
  }, 2000);
}