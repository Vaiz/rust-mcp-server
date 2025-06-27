#!/usr/bin/env pwsh
param(
    [string]$InstallFolder = "./rust-mcp-server",
    [string]$Tag = "stable",
    [string[]]$ServerArgs = @()
)

$ErrorActionPreference = "Stop"
$RepoUrl = "https://github.com/Vaiz/rust-mcp-server.git"
$BinaryPath = (Resolve-Path -Path "$InstallFolder/rust-mcp-server.exe").Path
$TempPath = "$env:TEMP/rust-mcp-server-build"

# Get remote commit hash for tag without cloning
function Get-RemoteCommit([string]$Tag) {
    try {
        $commit = git ls-remote --tags $RepoUrl "refs/tags/$Tag" 2>$null | ForEach-Object { $_.Split()[0].Substring(0,7) }
        return $commit
    } catch { return $null }
}

# Get existing binary version
function Get-BinaryVersion([string]$Path) {
    if (Test-Path $Path) {
        try { return (& $Path --version 2>$null) } catch { return $null }
    }
    return $null
}

# Check if rebuild is needed
$remoteCommit = Get-RemoteCommit $Tag
Write-Host "Remote commit for tag '$Tag': $remoteCommit"
$existingVersion = Get-BinaryVersion $BinaryPath
Write-Host "Existing binary version: $existingVersion"
$needsBuild = $true

if ($existingVersion -and $remoteCommit -and $existingVersion -match "\.([a-f0-9]+)$") {
    $existingCommit = $matches[1]
    if ($existingCommit -eq $remoteCommit) {
        Write-Host "Binary up-to-date (commit: $remoteCommit)"
        $needsBuild = $false
    } else {
        Write-Host "Binary outdated ($existingCommit → $remoteCommit)"
    }
}

# Clone/update and build if needed
if ($needsBuild) {
    # Ensure install directory exists
    if (-not (Test-Path $InstallFolder)) { New-Item -ItemType Directory -Path $InstallFolder -Force | Out-Null }
    
    # Build in temp directory
    if (Test-Path $TempPath) { Remove-Item $TempPath -Recurse -Force }
    
    Write-Host "Cloning to temp directory..."
    git clone $RepoUrl $TempPath
    Push-Location $TempPath
    
    git checkout $Tag
    
    Write-Host "Building..."
    cargo build --release
    
    # Copy executable to install path
    Copy-Item "target/release/rust-mcp-server.exe" $BinaryPath -Force
    Pop-Location
    
    # Clean up temp directory
    Remove-Item $TempPath -Recurse -Force
    Write-Host "Installed to: $BinaryPath"
}

Write-Host "Starting server..."
& $BinaryPath @ServerArgs
