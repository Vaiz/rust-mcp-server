$ErrorActionPreference = "Stop"

$buildPath = "$PSScriptRoot/../target/release/rustmcp.exe"
$installPath = "$PSScriptRoot/../tmp/rustmcp.exe"
$oldInstallPath = "$PSScriptRoot/../tmp/rustmcp-old.exe"

cargo b --release

if (Test-Path $oldInstallPath) {
    Write-Host "Removing old version of rustmcp-old.exe"
    Remove-Item $oldInstallPath
}

if (Test-Path $installPath) {
    Write-Host "Moving current version of rustmcp.exe"
    Move-Item $installPath "$PSScriptRoot/../tmp/rustmcp-old.exe"
} else {
    Write-Host "No current version of rustmcp.exe found, continuing..."
}

Copy-Item $buildPath $installPath

if (Test-Path $oldInstallPath) {
    Write-Host "Removing old version of rustmcp-old.exe"
    Remove-Item $oldInstallPath
}
