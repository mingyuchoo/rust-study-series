#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DIST_DIR = path.join(__dirname, '../dist');

function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function analyzeBundle() {
  console.log('📊 Bundle Analysis');
  console.log('==================');

  if (!fs.existsSync(DIST_DIR)) {
    console.log('❌ Dist directory not found. Run "pnpm run build" first.');
    return;
  }

  const stats = {
    totalSize: 0,
    files: []
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
        
        stats.files.push({
          path: relativeFilePath,
          size: size,
          sizeFormatted: formatBytes(size)
        });
      }
    });
  }

  analyzeDirectory(DIST_DIR);

  // Sort files by size (largest first)
  stats.files.sort((a, b) => b.size - a.size);

  console.log(`\n📦 Total Bundle Size: ${formatBytes(stats.totalSize)}`);
  
  // Size limits
  const SIZE_LIMIT = 1000 * 1024; // 1MB
  if (stats.totalSize > SIZE_LIMIT) {
    console.log(`⚠️  Bundle size exceeds recommended limit (${formatBytes(SIZE_LIMIT)})`);
  } else {
    console.log(`✅ Bundle size within recommended limit (${formatBytes(SIZE_LIMIT)})`);
  }

  console.log('\n📁 Files by size:');
  stats.files.forEach((file, index) => {
    const ext = path.extname(file.path).toLowerCase();
    let icon = '📄';
    if (ext === '.js') icon = '🟨';
    else if (ext === '.css') icon = '🟦';
    else if (['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp'].includes(ext)) icon = '🖼️';
    else if (ext === '.html') icon = '📝';
    
    console.log(`  ${icon} ${file.path}: ${file.sizeFormatted}`);
  });

  // Recommendations
  console.log('\n💡 Recommendations:');
  const jsFiles = stats.files.filter(f => f.path.endsWith('.js'));
  const largeJsFiles = jsFiles.filter(f => f.size > 500 * 1024);
  
  if (largeJsFiles.length > 0) {
    console.log('  - Consider code splitting for large JavaScript files');
  }
  
  if (stats.files.length > 10) {
    console.log('  - Consider implementing lazy loading for non-critical assets');
  }
  
  if (stats.totalSize > SIZE_LIMIT) {
    console.log('  - Consider removing unused dependencies');
    console.log('  - Implement tree shaking and dead code elimination');
  }

  console.log('\n✨ Analysis complete!');
  return stats;
}

analyzeBundle();