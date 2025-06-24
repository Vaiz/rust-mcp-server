# RustMcp - Model Context Protocol Server

A high-performance Model Context Protocol (MCP) server implementation in Rust.

## Overview

RustMcp is a robust implementation of the Model Context Protocol server that provides tools and resources to LLM clients through JSON-RPC communication over stdio.

## Features

- ✅ **Full MCP Protocol Support**: Implements MCP specification version 2024-11-05
- ✅ **Tools**: Execute custom tools with structured input/output
- ✅ **Resources**: Serve resources to clients with proper metadata
- ✅ **Async/Await**: Built on Tokio for high-performance async operations
- ✅ **Structured Logging**: Comprehensive logging with tracing
- ✅ **Error Handling**: Robust error handling with anyhow
- ✅ **CLI Interface**: Command-line interface with configurable options

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Installation

1. Clone or download this project
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

### Running

Run the MCP server:

```bash
cargo run
```

Or with custom log level:

```bash
cargo run -- --log-level debug
```

## Usage

The server communicates over stdio using JSON-RPC 2.0 protocol. Here are some example interactions:

### Initialize the server

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "example-client",
      "version": "1.0.0"
    }
  }
}
```

### List available tools

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

### Call a tool

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "echo",
    "arguments": {
      "text": "Hello, MCP!"
    }
  }
}
```

### List resources

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "resources/list",
  "params": {}
}
```

## Project Structure

```
rustmcp/
├── src/
│   └── main.rs          # Main server implementation
├── Cargo.toml           # Project dependencies and metadata
├── README.md            # This file
└── .github/
    └── copilot-instructions.md  # Copilot development guidelines
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Extending the Server

### Adding New Tools

1. Define your tool in the `add_sample_tools()` method
2. Add the tool execution logic in `handle_tools_call()`
3. Rebuild and test

### Adding New Resources

1. Define your resource in the `add_sample_resources()` method  
2. Add the resource reading logic in `handle_resources_read()`
3. Rebuild and test

## Configuration

The server accepts the following command-line arguments:

- `--log-level`: Set logging level (debug, info, warn, error) - defaults to "info"

## Dependencies

- **tokio**: Async runtime for high-performance I/O
- **serde**: Serialization framework
- **jsonrpc-core**: JSON-RPC 2.0 protocol implementation
- **anyhow**: Error handling utilities
- **clap**: Command-line argument parsing
- **tracing**: Structured logging

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Ensure `cargo test` and `cargo clippy` pass
6. Submit a pull request

## Resources

- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Tokio Documentation](https://tokio.rs/)
