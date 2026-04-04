# Copilot Instructions for Rust MCP Server

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
- `rmcp`: MCP protocol handling
- `anyhow`: Error handling
- `tracing`: Structured logging

## AI Agent Guidelines

### 1. Always Use Rust MCP Tools

- Use `#cargo-check` for quick code validation
- Use `#cargo-clippy` for linting
- Use `#cargo-fmt` for code formatting
- Use `#cargo-test` for running tests

### 2. use @rust-docs subagent

- For any questions about public API for dependencies or standard library, consult the `@rust-docs` subagent. Just ask it what you looking for in natural language, and it will fetch the relevant documentation for you.

### 2. Development Workflow

Follow this systematic approach when working on code changes:

1. **Check current state**: Use `#cargo-check` with `all_targets: true, all_features: true`
2. **Make changes**: Edit code using appropriate development tools
3. **Validate**: Use `#cargo-clippy` with `workspace: true, all_targets: true`
4. **Format**: Use `#cargo-fmt` with `all: true`
5. **Test**: Use `#cargo-test` with `all_features: true`
6. **Build**: Use `#cargo-build` with `all_targets: true, all_features: true` for final verification
7. **Check unused dependencies**: Use `#cargo-machete` to identify unused dependencies
8. **Verify security compliance**: Use `#cargo-deny-check` to ensure security and licensing compliance

### 3. Dependency Management

- When adding dependencies, prefer workspace-level dependencies in the root `Cargo.toml`
- Use `#cargo-add` and `#cargo-remove` for dependency management
- Regularly run `#cargo-update` to keep dependencies current

### 4. Code Quality Standards

This project maintains strict code quality standards:

- **Clippy**: All clippy warnings must be resolved
- **Formatting**: Code must be formatted with rustfmt using the nightly toolchain
- **Tests**: All changes must maintain or improve test coverage
- **Documentation**: Public APIs must be thoroughly documented
- **Security**: All dependencies must pass security and licensing checks
