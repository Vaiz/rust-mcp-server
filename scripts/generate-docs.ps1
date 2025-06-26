# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery
# Usage: .\generate-docs.ps1 [filename]
# If filename is not provided, defaults to tools.md

param(
    [string]$Filename = "tools.md"
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$targetDir = Join-Path $projectRoot "target"
$serverBinary = Join-Path $targetDir "release\rustmcp.exe"
$outputFile = Join-Path $projectRoot $Filename

Write-Host "🔧 Building MCP server..." -ForegroundColor Blue
Set-Location $projectRoot
cargo build --release

Write-Host "📝 Generating documentation using mcp-discovery..." -ForegroundColor Blue

# Generate tools documentation
Write-Host "   - Creating $Filename documentation..." -ForegroundColor Gray
mcp-discovery create --template md-plain --filename $outputFile -- $serverBinary

# Post-process to remove git hash from version for CI stability
Write-Host "   - Removing git hash from version string for CI stability..." -ForegroundColor Gray
$content = Get-Content $outputFile -Raw
$content = $content -replace '^## Rust MCP Server ([0-9]+\.[0-9]+\.[0-9]+)\.[a-f0-9]+', '## Rust MCP Server $1'
Set-Content $outputFile $content -NoNewline

Write-Host "✅ Documentation generated successfully!" -ForegroundColor Green
Write-Host "   - $Filename (Complete MCP tools and capabilities documentation)" -ForegroundColor Gray
