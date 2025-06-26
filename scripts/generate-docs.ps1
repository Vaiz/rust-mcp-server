# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$targetDir = Join-Path $projectRoot "target"
$serverBinary = Join-Path $targetDir "release\rustmcp.exe"

Write-Host "🔧 Building MCP server..." -ForegroundColor Blue
Set-Location $projectRoot
cargo build --release

Write-Host "📝 Generating documentation using mcp-discovery..." -ForegroundColor Blue

# Generate tools documentation in the root directory
Write-Host "   - Creating tools.md documentation..." -ForegroundColor Gray
mcp-discovery create --template md-plain --filename "$projectRoot\tools.md" -- $serverBinary

# Post-process to remove git hash from version for CI stability
Write-Host "   - Removing git hash from version string for CI stability..." -ForegroundColor Gray
$toolsPath = Join-Path $projectRoot "tools.md"
$content = Get-Content $toolsPath -Raw
$content = $content -replace '^## Rust MCP Server ([0-9]+\.[0-9]+\.[0-9]+)\.[a-f0-9]+', '## Rust MCP Server $1'
Set-Content $toolsPath $content -NoNewline

Write-Host "✅ Documentation generated successfully!" -ForegroundColor Green
Write-Host "   - tools.md (Complete MCP tools and capabilities documentation)" -ForegroundColor Gray