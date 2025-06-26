#!/bin/bash

# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TARGET_DIR="$PROJECT_ROOT/target"
DOCS_DIR="$PROJECT_ROOT/docs"
SERVER_BINARY="$TARGET_DIR/release/rustmcp"

echo "🔧 Building MCP server..."
cd "$PROJECT_ROOT"
cargo build --release

echo "📝 Generating documentation using mcp-discovery..."

# Ensure docs directory exists
mkdir -p "$DOCS_DIR"

# Generate comprehensive markdown documentation
echo "   - Creating comprehensive documentation..."
mcp-discovery create --template md --filename "$DOCS_DIR/mcp-capabilities.md" -- "$SERVER_BINARY"

# Generate plain markdown documentation (without HTML styling)
echo "   - Creating plain markdown documentation..."
mcp-discovery create --template md-plain --filename "$DOCS_DIR/mcp-capabilities-plain.md" -- "$SERVER_BINARY"

# Generate HTML documentation
echo "   - Creating HTML documentation..."
mcp-discovery create --template html --filename "$DOCS_DIR/mcp-capabilities.html" -- "$SERVER_BINARY"

# Generate text documentation
echo "   - Creating text documentation..."
mcp-discovery create --template txt --filename "$DOCS_DIR/mcp-capabilities.txt" -- "$SERVER_BINARY"

echo "✅ Documentation generated successfully in $DOCS_DIR/"
echo "   - mcp-capabilities.md (Markdown with styling)"
echo "   - mcp-capabilities-plain.md (Plain Markdown)"
echo "   - mcp-capabilities.html (HTML)"
echo "   - mcp-capabilities.txt (Plain text)"