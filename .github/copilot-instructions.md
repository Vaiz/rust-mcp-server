# Copilot Instructions for RustMcp

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust implementation of a Model Context Protocol (MCP) server.

## Project Context
- **Language**: Rust
- **Protocol**: Model Context Protocol (MCP)
- **Communication**: JSON-RPC over stdio
- **Architecture**: Async/await with Tokio runtime

## Key Guidelines
1. Follow Rust best practices and idioms
2. Use proper error handling with `Result<T, E>` and `anyhow` for error propagation
3. Implement MCP protocol methods according to the specification
4. Use structured logging with the `tracing` crate
5. Ensure all async operations are properly awaited

## MCP Protocol Implementation
- **Initialize**: Handle client initialization and capability negotiation
- **Tools**: Implement tool listing and execution
- **Resources**: Handle resource discovery and reading
- **Logging**: Support MCP logging protocol

## Dependencies
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `rust-mcp-sdk`: MCP protocol handling
- `anyhow`: Error handling
- `tracing`: Structured logging

You can find more info and examples at https://modelcontextprotocol.io/llms-full.txt
