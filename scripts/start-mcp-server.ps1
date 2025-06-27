#!/usr/bin/env pwsh
param(
    [string]$Tag = "stable",
    
    [string[]]$ServerArgs = @()
)

$RepoUrl = "https://github.com/Vaiz/rust-mcp-server.git"
$RepoPath = "./rust-mcp-server"
$BinaryPath = "$RepoPath/target/release/rust-mcp-server.exe"

# Clone or update repository
if (Test-Path $RepoPath) {
    Write-Host "Updating repository..."
    Set-Location $RepoPath
    git fetch --all --tags
    git reset --hard HEAD
} else {
    Write-Host "Cloning repository..."
    git clone $RepoUrl
    Set-Location $RepoPath
}

# Checkout tag
Write-Host "Checking out tag: $Tag"
git checkout $Tag
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to checkout tag: $Tag"
    exit 1
}

# Build release
Write-Host "Building release..."
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
    exit 1
}

# Start server
Write-Host "Starting MCP server..."
& $BinaryPath @ServerArgs
