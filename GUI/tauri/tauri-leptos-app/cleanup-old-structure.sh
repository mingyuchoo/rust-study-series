#!/bin/bash

# Cleanup script for old monolithic structure
# Run this after confirming the new Clean Architecture works

echo "🧹 Cleaning up old monolithic structure..."

# Backup old files first
echo "📦 Creating backup..."
mkdir -p backup
cp -r src backup/ 2>/dev/null || true
cp -r src-tauri backup/ 2>/dev/null || true

# Remove old structure
echo "🗑️  Removing old files..."
rm -rf src/
rm -rf src-tauri/

# Remove old config files that are now in presentation layers
rm -f index.html 2>/dev/null || true
rm -f Trunk.toml 2>/dev/null || true

# Keep these files as they're still needed
# - README.md (updated)
# - styles.css (still used)
# - Cargo.lock (workspace lock file)
# - .gitignore, .taurignore, etc.

echo "✅ Cleanup complete!"
echo "📁 Old files backed up to ./backup/"
echo "🚀 You can now run: cd presentation-backend && cargo tauri dev"