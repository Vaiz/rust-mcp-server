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

echo "✅ Documentation generated successfully!"
echo "   - tools.md (Complete MCP tools and capabilities documentation)"