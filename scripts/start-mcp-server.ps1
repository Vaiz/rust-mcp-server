#!/usr/bin/env pwsh
param(
    [string]$InstallFolder = "./rust-mcp-server",
    [string]$Tag = "stable",
    [switch]$KeepTemp,
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
    if (-not (Test-Path $InstallFolder)) { New-Item -ItemType Directory -Path $InstallFolder -Force | Out-Null }
    
    if (Test-Path $TempPath) {
        Write-Host "Updating existing temp directory..."
        git -C $TempPath fetch --all --tags
        git -C $TempPath reset --hard HEAD
    } else {
        Write-Host "Cloning to temp directory..."
        git clone $RepoUrl $TempPath
    }
    
    git -C $TempPath checkout $Tag
    
    Write-Host "Building..."
    cargo build --release --manifest-path "$TempPath/Cargo.toml"
    
    Copy-Item "$TempPath/target/release/rust-mcp-server.exe" $BinaryPath -Force
    
    # Clean up temp directory unless keeping it
    if (-not $KeepTemp) {
        Remove-Item $TempPath -Recurse -Force
    } 
    Write-Host "Installed to: $BinaryPath"
}

Write-Host "Starting server..."
& $BinaryPath @ServerArgs
