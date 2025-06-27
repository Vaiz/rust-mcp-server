$ErrorActionPreference = "Stop"

$buildPath = "$PSScriptRoot/../target/release/rust-mcp-server.exe"
$installPath = "$PSScriptRoot/../tmp/rust-mcp-server.exe"
$oldInstallPath = "$PSScriptRoot/../tmp/rust-mcp-server-old.exe"

cargo b --release

if (Test-Path $oldInstallPath) {
    Write-Host "Removing old version of rust-mcp-server-old.exe"
    Remove-Item $oldInstallPath
}

if (Test-Path $installPath) {
    Write-Host "Moving current version of rust-mcp-server.exe"
    Move-Item $installPath "$PSScriptRoot/../tmp/rust-mcp-server-old.exe"
} else {
    Write-Host "No current version of rust-mcp-server.exe found, continuing..."
}

Copy-Item $buildPath $installPath

if (Test-Path $oldInstallPath) {
    Write-Host "Removing old version of rust-mcp-server-old.exe"
    Remove-Item $oldInstallPath
}
