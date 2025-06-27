
# Prerequisites:
# - winget install OpenJS.NodeJS

$tempDir = "$PSScriptRoot/../tmp"
$toolPath = "$PSScriptRoot/../target/debug/rust-mcp-server.exe"

cargo b

if (-not (Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

cp $toolPath $tempDir/rust-mcp-server-inspect.exe
npx @modelcontextprotocol/inspector $tempDir/rust-mcp-server-inspect.exe "--log-file", "./tmp/rust-mcp-server-inspect.log"
