#!/bin/bash

# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery
# Usage: ./generate-docs.sh [filename]
# If filename is not provided, defaults to tools.md

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TARGET_DIR="$PROJECT_ROOT/target"
SERVER_BINARY="$TARGET_DIR/release/rustmcp"

# Use provided filename or default to tools.md
FILENAME="${1:-tools.md}"
OUTPUT_FILE="$PROJECT_ROOT/$FILENAME"

echo "🔧 Building MCP server..."
cd "$PROJECT_ROOT"
cargo build --release

echo "📝 Generating documentation using mcp-discovery..."

# Generate tools documentation
echo "   - Creating $FILENAME documentation..."
mcp-discovery create --template md-plain --filename "$OUTPUT_FILE" -- "$SERVER_BINARY"

# Post-process to remove git hash from version for CI stability
echo "   - Removing git hash from version string for CI stability..."
sed -i '1s/## Rust MCP Server \([0-9]\+\.[0-9]\+\.[0-9]\+\)\.[a-f0-9]\+/## Rust MCP Server \1/' "$OUTPUT_FILE"

echo "✅ Documentation generated successfully!"
echo "   - $FILENAME (Complete MCP tools and capabilities documentation)"