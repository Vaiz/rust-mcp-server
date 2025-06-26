# Script to generate documentation using mcp-discovery
# Requires mcp-discovery to be installed: cargo install mcp-discovery

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$targetDir = Join-Path $projectRoot "target"
$docsDir = Join-Path $projectRoot "docs"
$serverBinary = Join-Path $targetDir "release\rustmcp.exe"

Write-Host "🔧 Building MCP server..." -ForegroundColor Blue
Set-Location $projectRoot
cargo build --release

Write-Host "📝 Generating documentation using mcp-discovery..." -ForegroundColor Blue

# Ensure docs directory exists
if (-not (Test-Path $docsDir)) {
    New-Item -ItemType Directory -Path $docsDir | Out-Null
}

# Generate comprehensive markdown documentation
Write-Host "   - Creating comprehensive documentation..." -ForegroundColor Gray
mcp-discovery create --template md --filename "$docsDir\mcp-capabilities.md" -- $serverBinary

# Generate plain markdown documentation (without HTML styling)
Write-Host "   - Creating plain markdown documentation..." -ForegroundColor Gray
mcp-discovery create --template md-plain --filename "$docsDir\mcp-capabilities-plain.md" -- $serverBinary

# Generate HTML documentation
Write-Host "   - Creating HTML documentation..." -ForegroundColor Gray
mcp-discovery create --template html --filename "$docsDir\mcp-capabilities.html" -- $serverBinary

# Generate text documentation
Write-Host "   - Creating text documentation..." -ForegroundColor Gray
mcp-discovery create --template txt --filename "$docsDir\mcp-capabilities.txt" -- $serverBinary

Write-Host "✅ Documentation generated successfully in $docsDir\" -ForegroundColor Green
Write-Host "   - mcp-capabilities.md (Markdown with styling)" -ForegroundColor Gray
Write-Host "   - mcp-capabilities-plain.md (Plain Markdown)" -ForegroundColor Gray
Write-Host "   - mcp-capabilities.html (HTML)" -ForegroundColor Gray
Write-Host "   - mcp-capabilities.txt (Plain text)" -ForegroundColor Gray