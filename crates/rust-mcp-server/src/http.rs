//! HTTP streamable transport for the Rust MCP server.
//!
//! This module isolates all HTTP transport dependencies (axum, the rmcp
//! streamable HTTP server transport and its transitive dependencies) behind the
//! `http` feature so that they are only compiled when that feature is enabled.
//!
//! The MCP endpoint is served at the root path (`/`). rmcp's default
//! DNS-rebinding protection (loopback-only allowed hosts) is kept enabled, so
//! requests carrying a non-loopback `Host` header are rejected regardless of
//! the bind address. There is no HTTPS and no authentication.

use std::net::SocketAddr;
use std::sync::Arc;

use rmcp::service::{RoleServer, Service};
use rmcp::transport::StreamableHttpServerConfig;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

/// Serves an MCP server over an HTTP streamable transport at the root path.
///
/// The listener is bound to `addr`. rmcp's default DNS-rebinding protection
/// (loopback-only allowed hosts) is kept enabled, so requests carrying a
/// non-loopback `Host` header are rejected.
///
/// `service_factory` is invoked once per MCP session to build a fresh handler.
///
/// # Errors
///
/// Returns an error if the TCP listener cannot bind to the requested address
/// or if the underlying HTTP server fails while running.
pub async fn serve<S, F>(addr: SocketAddr, service_factory: F) -> std::io::Result<()>
where
    S: Service<RoleServer> + Send + 'static,
    F: Fn() -> Result<S, std::io::Error> + Send + Sync + 'static,
{
    let service = StreamableHttpService::new(
        service_factory,
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new().route_service("/", service);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Rust MCP Server listening on http://{addr}/");

    axum::serve(listener, router).await
}
