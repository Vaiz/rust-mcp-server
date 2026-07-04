# rust_mcp_server_http

This crate provides the optional localhost HTTP streamable transport for the [`rust-mcp-server`](https://crates.io/crates/rust-mcp-server) binary. Its sole purpose is to isolate the HTTP transport dependencies (axum, the rmcp streamable HTTP server transport, and their transitive crates) so they are only compiled when the main crate's `http` feature is enabled, keeping the default build free of the HTTP stack.
