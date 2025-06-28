# Copilot Instructions for Rust MCP Server

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

## AI Agent Guidelines

### 1. Always Use Rust MCP Tools

- **DO**: Use `Rust-cargo-build` instead of direct `bash` commands like `cargo build`
- **DO**: Use `Rust-cargo-check` for quick code validation
- **DO**: Use `Rust-cargo-clippy` for linting instead of manual clippy commands
- **WHY**: MCP tools provide better defaults, structured output, and superior error handling

### 2. Development Workflow

Follow this systematic approach when working on code changes:

1. **Check current state**: Use `Rust-cargo-check` with `all_targets: true, all_features: true`
2. **Make changes**: Edit code using appropriate development tools
3. **Validate**: Use `Rust-cargo-clippy` with `workspace: true, all_targets: true`
4. **Format**: Use `Rust-cargo-fmt` with `all: true`
5. **Test**: Use `Rust-cargo-test` with `all_features: true`
6. **Build**: Use `Rust-cargo-build` with `all_targets: true, all_features: true` for final verification
7. **Check unused dependencies**: Use `Rust-cargo-machete` to identify unused dependencies
8. **Verify security compliance**: Use `Rust-cargo-deny-check` to ensure security and licensing compliance

### 3. Dependency Management

- When adding dependencies, prefer workspace-level dependencies in the root `Cargo.toml`
- Use `Rust-cargo-add` and `Rust-cargo-remove` for dependency management
- Regularly run `Rust-cargo-update` to keep dependencies current

### 4. Code Quality Standards

This project maintains strict code quality standards:

- **Clippy**: All clippy warnings must be resolved
- **Formatting**: Code must be formatted with rustfmt using the nightly toolchain
- **Tests**: All changes must maintain or improve test coverage
- **Documentation**: Public APIs must be thoroughly documented
- **Security**: All dependencies must pass security and licensing checks
