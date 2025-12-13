#!/usr/bin/env python3

import argparse
import os
import platform
import re
import subprocess
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Generate documentation for MCP server")
    parser.add_argument("filename", nargs="?", default="tools.md", 
                       help="Output filename (default: tools.md)")
    args = parser.parse_args()

    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    target_dir = project_root / "target"
    
    binary_name = "rust-mcp-server.exe" if platform.system() == "Windows" else "rust-mcp-server"
    server_binary = target_dir / "release" / binary_name
    output_file = project_root / args.filename

    print("🔧 Building MCP server...")
    os.chdir(project_root)
    subprocess.run(["cargo", "build", "--release"], check=True)

    print("📝 Generating documentation...")
    print(f"   - Creating {args.filename} documentation...")
    
    subprocess.run([
        str(server_binary),
        "--generate-docs", str(output_file)
    ], check=True)

    print("   - Removing git hash from version string for CI stability...")
    content = output_file.read_text(encoding='utf-8')
    content = re.sub(
        r'^## Rust MCP Server ([0-9]+\.[0-9]+\.[0-9]+)\.[a-f0-9]+', 
        r'## Rust MCP Server \1', 
        content, 
        flags=re.MULTILINE
    )
    output_file.write_text(content, encoding='utf-8')

    print("✅ Documentation generated successfully!")
    print(f"   - {args.filename} (Complete MCP tools and capabilities documentation)")


if __name__ == "__main__":
    main()
