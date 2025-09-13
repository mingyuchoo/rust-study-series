#!/usr/bin/env node

// Performance monitoring script for the RAG Document Search app
// Measures bundle size, load times, and other performance metrics

import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Configuration
const DIST_DIR = path.join(__dirname, '../dist');
const PERFORMANCE_LOG = path.join(__dirname, '../performance-metrics.json');
const SIZE_LIMITS = {
  totalBundle: 1000 * 1024, // 1MB
  jsChunk: 500 * 1024,      // 500KB
  cssFile: 100 * 1024,      // 100KB
  imageFile: 200 * 1024     // 200KB
};

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function analyzeBundle() {
  log('\n📊 Bundle Analysis', 'cyan');
  log('================', 'cyan');

  if (!fs.existsSync(DIST_DIR)) {
    log('❌ Dist directory not found. Run "pnpm run build" first.', 'red');
    process.exit(1);
  }

  const stats = {
    totalSize: 0,
    files: [],
    chunks: {
      js: [],
      css: [],
      images: [],
      other: []
    }
  };

  function analyzeDirectory(dir, relativePath = '') {
    const files = fs.readdirSync(dir);
    
    files.forEach(file => {
      const filePath = path.join(dir, file);
      const relativeFilePath = path.join(relativePath, file);
      const stat = fs.statSync(filePath);
      
      if (stat.isDirectory()) {
        analyzeDirectory(filePath, relativeFilePath);
      } else {
        const size = stat.size;
        stats.totalSize += size;
        
        const fileInfo = {
          path: relativeFilePath,
          size: size,
          sizeFormatted: formatBytes(size)
        };
        
        stats.files.push(fileInfo);
        
        // Categorize files
        const ext = path.extname(file).toLowerCase();
        if (ext === '.js') {
          stats.chunks.js.push(fileInfo);
        } else if (ext === '.css') {
          stats.chunks.css.push(fileInfo);
        } else if (['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp'].includes(ext)) {
          stats.chunks.images.push(fileInfo);
        } else {
          stats.chunks.other.push(fileInfo);
        }
      }
    });
  }

  analyzeDirectory(DIST_DIR);

  // Sort files by size (largest first)
  stats.files.sort((a, b) => b.size - a.size);
  Object.keys(stats.chunks).forEach(key => {
    stats.chunks[key].sort((a, b) => b.size - a.size);
  });

  // Display results
  log(`\n📦 Total Bundle Size: ${formatBytes(stats.totalSize)}`, 'blue');
  
  if (stats.totalSize > SIZE_LIMITS.totalBundle) {
    log(`⚠️  Bundle size exceeds limit (${formatBytes(SIZE_LIMITS.totalBundle)})`, 'yellow');
  } else {
    log(`✅ Bundle size within limit (${formatBytes(SIZE_LIMITS.totalBundle)})`, 'green');
  }

  // JavaScript chunks
  log('\n🟨 JavaScript Chunks:', 'yellow');
  stats.chunks.js.forEach(file => {
    const status = file.size > SIZE_LIMITS.jsChunk ? '⚠️ ' : '✅ ';
    log(`  ${status}${file.path}: ${file.sizeFormatted}`);
  });

  // CSS files
  log('\n🟦 CSS Files:', 'blue');
  stats.chunks.css.forEach(file => {
    const status = file.size > SIZE_LIMITS.cssFile ? '⚠️ ' : '✅ ';
    log(`  ${status}${file.path}: ${file.sizeFormatted}`);
  });

  // Images
  log('\n🟩 Images:', 'green');
  stats.chunks.images.forEach(file => {
    const status = file.size > SIZE_LIMITS.imageFile ? '⚠️ ' : '✅ ';
    log(`  ${status}${file.path}: ${file.sizeFormatted}`);
  });

  // Other files
  if (stats.chunks.other.length > 0) {
    log('\n🟪 Other Files:', 'magenta');
    stats.chunks.other.forEach(file => {
      log(`  📄 ${file.path}: ${file.sizeFormatted}`);
    });
  }

  // Top 10 largest files
  log('\n🔝 Top 10 Largest Files:', 'cyan');
  stats.files.slice(0, 10).forEach((file, index) => {
    log(`  ${index + 1}. ${file.path}: ${file.sizeFormatted}`);
  });

  return stats;
}

function checkDependencies() {
  log('\n📋 Dependency Analysis', 'cyan');
  log('=====================', 'cyan');

  try {
    const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '../package.json'), 'utf8'));
    const dependencies = { ...packageJson.dependencies, ...packageJson.devDependencies };
    
    log(`\n📦 Total Dependencies: ${Object.keys(dependencies).length}`);
    
    // Check for potentially unused dependencies
    log('\n🔍 Checking for potentially unused dependencies...');
    
    const srcDir = path.join(__dirname, '../src');
    const srcFiles = getAllFiles(srcDir, ['.ts', '.js', '.svelte']);
    const srcContent = srcFiles.map(file => fs.readFileSync(file, 'utf8')).join('\n');
    
    const unusedDeps = [];
    Object.keys(dependencies).forEach(dep => {
      // Skip certain dependencies that might not be directly imported
      const skipCheck = [
        '@types/',
        'eslint',
        'prettier',
        'vite',
        'vitest',
        'typescript',
        'postcss',
        'autoprefixer',
        'tailwindcss'
      ];
      
      if (skipCheck.some(skip => dep.includes(skip))) {
        return;
      }
      
      // Check if dependency is imported anywhere
      const importPatterns = [
        new RegExp(`import.*from\\s+['"]${dep}['"]`, 'g'),
        new RegExp(`import\\s+['"]${dep}['"]`, 'g'),
        new RegExp(`require\\(['"]${dep}['"]\\)`, 'g')
      ];
      
      const isUsed = importPatterns.some(pattern => pattern.test(srcContent));
      if (!isUsed) {
        unusedDeps.push(dep);
      }
    });
    
    if (unusedDeps.length > 0) {
      log('\n⚠️  Potentially unused dependencies:', 'yellow');
      unusedDeps.forEach(dep => {
        log(`  - ${dep}`, 'yellow');
      });
      log('\n💡 Consider removing unused dependencies to reduce bundle size.', 'blue');
    } else {
      log('\n✅ No obviously unused dependencies found.', 'green');
    }
    
  } catch (error) {
    log(`❌ Failed to analyze dependencies: ${error.message}`, 'red');
  }
}

function getAllFiles(dir, extensions) {
  let files = [];
  
  function traverse(currentDir) {
    const items = fs.readdirSync(currentDir);
    
    items.forEach(item => {
      const itemPath = path.join(currentDir, item);
      const stat = fs.statSync(itemPath);
      
      if (stat.isDirectory() && !item.startsWith('.') && item !== 'node_modules') {
        traverse(itemPath);
      } else if (stat.isFile() && extensions.some(ext => item.endsWith(ext))) {
        files.push(itemPath);
      }
    });
  }
  
  traverse(dir);
  return files;
}

function generateReport(bundleStats) {
  log('\n📄 Generating Performance Report', 'cyan');
  log('================================', 'cyan');

  const report = {
    timestamp: new Date().toISOString(),
    bundleSize: {
      total: bundleStats.totalSize,
      totalFormatted: formatBytes(bundleStats.totalSize),
      withinLimit: bundleStats.totalSize <= SIZE_LIMITS.totalBundle
    },
    chunks: {
      javascript: bundleStats.chunks.js.length,
      css: bundleStats.chunks.css.length,
      images: bundleStats.chunks.images.length,
      other: bundleStats.chunks.other.length
    },
    largestFiles: bundleStats.files.slice(0, 5).map(file => ({
      path: file.path,
      size: file.size,
      sizeFormatted: file.sizeFormatted
    })),
    recommendations: []
  };

  // Generate recommendations
  if (bundleStats.totalSize > SIZE_LIMITS.totalBundle) {
    report.recommendations.push('Bundle size exceeds recommended limit. Consider code splitting or removing unused dependencies.');
  }

  const largeJsChunks = bundleStats.chunks.js.filter(file => file.size > SIZE_LIMITS.jsChunk);
  if (largeJsChunks.length > 0) {
    report.recommendations.push(`${largeJsChunks.length} JavaScript chunks exceed size limit. Consider further code splitting.`);
  }

  const largeCssFiles = bundleStats.chunks.css.filter(file => file.size > SIZE_LIMITS.cssFile);
  if (largeCssFiles.length > 0) {
    report.recommendations.push(`${largeCssFiles.length} CSS files exceed size limit. Consider CSS optimization.`);
  }

  if (bundleStats.chunks.images.length > 10) {
    report.recommendations.push('Consider implementing lazy loading for images to improve initial load time.');
  }

  // Save report
  try {
    fs.writeFileSync(PERFORMANCE_LOG, JSON.stringify(report, null, 2));
    log(`\n✅ Performance report saved to: ${PERFORMANCE_LOG}`, 'green');
  } catch (error) {
    log(`❌ Failed to save performance report: ${error.message}`, 'red');
  }

  // Display summary
  log('\n📊 Performance Summary:', 'blue');
  log(`  Bundle Size: ${report.bundleSize.totalFormatted} ${report.bundleSize.withinLimit ? '✅' : '⚠️'}`, 'blue');
  log(`  JS Chunks: ${report.chunks.javascript}`, 'blue');
  log(`  CSS Files: ${report.chunks.css}`, 'blue');
  log(`  Images: ${report.chunks.images}`, 'blue');
  
  if (report.recommendations.length > 0) {
    log('\n💡 Recommendations:', 'yellow');
    report.recommendations.forEach(rec => {
      log(`  - ${rec}`, 'yellow');
    });
  } else {
    log('\n🎉 All performance metrics look good!', 'green');
  }

  return report;
}

function main() {
  log('🚀 Performance Monitor', 'green');
  log('====================', 'green');
  
  try {
    const bundleStats = analyzeBundle();
    checkDependencies();
    generateReport(bundleStats);
    
    log('\n✨ Performance analysis complete!', 'green');
  } catch (error) {
    log(`❌ Performance analysis failed: ${error.message}`, 'red');
    process.exit(1);
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}

export {
  analyzeBundle,
  checkDependencies,
  generateReport,
  formatBytes
};