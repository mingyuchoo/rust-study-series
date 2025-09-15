/**
 * Test script to verify color contrast improvements
 */

import { ColorContrast } from './accessibility.js';

// Test color combinations
const testCombinations = [
  // Original problematic combinations
  { name: 'gray-500 on white (original)', fg: '#6b7280', bg: '#ffffff' },
  { name: 'gray-400 on white (original)', fg: '#9ca3af', bg: '#ffffff' },
  
  // Improved combinations
  { name: 'gray-600 on white (improved)', fg: '#4b5563', bg: '#ffffff' },
  { name: 'gray-700 on white (improved)', fg: '#374151', bg: '#ffffff' },
  
  // Dark mode combinations
  { name: 'gray-300 on gray-800 (dark)', fg: '#d1d5db', bg: '#1f2937' },
  { name: 'gray-200 on gray-900 (dark)', fg: '#e5e7eb', bg: '#111827' },
];

export function testContrastRatios(): void {
  console.group('🎨 Color Contrast Test Results');
  
  testCombinations.forEach(({ name, fg, bg }) => {
    const ratio = ColorContrast.getContrastRatio(fg, bg);
    const meetsAA = ColorContrast.meetsWCAGAA(fg, bg);
    const meetsAAA = ColorContrast.meetsWCAGAAA(fg, bg);
    
    const status = meetsAAA ? '✅ AAA' : meetsAA ? '✅ AA' : '❌ Fail';
    console.log(`${status} ${name}: ${ratio.toFixed(2)}:1`);
  });
  
  console.groupEnd();
}

// Run test in development
if (import.meta.env.DEV) {
  testContrastRatios();
}