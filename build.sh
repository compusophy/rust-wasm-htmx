#!/bin/bash
set -e

echo "🦀 Starting WASM + Rust build process..."

# Build WASM first
echo "📦 Building WASM with wasm-pack..."
wasm-pack build --target web --out-dir pkg --verbose

# Check if WASM files were created
echo "📁 Checking WASM build output:"
ls -la pkg/ || echo "❌ No pkg directory found!"

# Build Rust server
echo "🦀 Building Rust server..."
cargo build --release --bin server --verbose

echo "✅ Build complete!" 