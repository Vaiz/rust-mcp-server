#!/bin/bash

# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TARGET_DIR="$PROJECT_ROOT/target"
SERVER_BINARY="$TARGET_DIR/release/rustmcp"

echo "🔧 Building MCP server..."
cd "$PROJECT_ROOT"
cargo build --release

echo "📝 Generating documentation using mcp-discovery..."

# Generate tools documentation in the root directory
echo "   - Creating tools.md documentation..."
mcp-discovery create --template md-plain --filename "$PROJECT_ROOT/tools.md" -- "$SERVER_BINARY"

# Post-process to remove git hash from version for CI stability
echo "   - Removing git hash from version string for CI stability..."
sed -i '1s/## Rust MCP Server 0\.1\.0\.[a-f0-9]\+/## Rust MCP Server 0.1.0/' "$PROJECT_ROOT/tools.md"

echo "✅ Documentation generated successfully!"
echo "   - tools.md (Complete MCP tools and capabilities documentation)"