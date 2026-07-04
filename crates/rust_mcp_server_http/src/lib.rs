//! Localhost-only HTTP streamable transport for the Rust MCP server.
//!
//! This crate isolates all HTTP transport dependencies (axum, the rmcp
//! streamable HTTP server transport and its transitive dependencies) so that
//! they are only compiled when the `http` feature of `rust-mcp-server` is
//! enabled.
//!
//! The transport is intentionally limited to `127.0.0.1`. There is no HTTPS
//! and no authentication.

use std::net::SocketAddr;
use std::sync::Arc;

use rmcp::service::{RoleServer, Service};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

/// The HTTP path that exposes the MCP streamable endpoint.
pub const MCP_PATH: &str = "/mcp";

/// Serves an MCP server over a localhost-only HTTP streamable transport.
///
/// The listener is bound exclusively to `127.0.0.1:<port>`. rmcp's default
/// DNS-rebinding protection (loopback-only allowed hosts) is kept enabled, so
/// requests carrying a non-loopback `Host` header are rejected.
///
/// `service_factory` is invoked once per MCP session to build a fresh handler.
///
/// # Errors
///
/// Returns an error if the TCP listener cannot bind to the requested port or
/// if the underlying HTTP server fails while running.
pub async fn serve<S, F>(port: u16, service_factory: F) -> std::io::Result<()>
where
    S: Service<RoleServer> + Send + 'static,
    F: Fn() -> Result<S, std::io::Error> + Send + Sync + 'static,
{
    let service = StreamableHttpService::new(
        service_factory,
        Arc::new(LocalSessionManager::default()),
        Default::default(),
    );

    let router = axum::Router::new().nest_service(MCP_PATH, service);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Rust MCP Server listening on http://{addr}{MCP_PATH}");

    axum::serve(listener, router).await
}
