
# Prerequisites:
# - winget install OpenJS.NodeJS

$tempDir = "$PSScriptRoot/../tmp"
$toolPath = "$PSScriptRoot/../target/debug/rustmcp.exe"

cargo b

if (-not (Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

cp $toolPath $tempDir/rustmcp-inspect.exe
npx @modelcontextprotocol/inspector $tempDir/rustmcp-inspect.exe
