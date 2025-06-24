mod handler;
mod tools;

use clap::Parser;
use rust_mcp_sdk::McpServer;
use rust_mcp_sdk::{
    StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::server_runtime,
    schema::{
        Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
        ServerCapabilitiesTools,
    },
};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust MCP Server", long_about = None)]
struct Args {
    /// Timeout for processing a request (seconds)
    #[arg(long, default_value_t = 600)]
    timeout: u64,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Log file path (if not set, logs to stderr)
    #[arg(long)]
    log_file: Option<String>,
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Set up logging
    let env_filter = EnvFilter::new(&args.log_level);
    if let Some(ref path) = args.log_file {
        // Use rolling log file (daily rotation, keep old logs)
        use std::path::Path;
        let log_path = Path::new(path);
        let (dir, file_name) = match (log_path.parent(), log_path.file_name()) {
            (Some(d), Some(f)) => (d, f),
            _ => (Path::new("."), log_path.as_os_str()),
        };
        let file_appender = rolling::daily(dir, file_name);
        fmt()
            .with_env_filter(env_filter)
            .with_writer(file_appender)
            .with_ansi(false)
            .init();
    }
    tracing::info!(?args, "Starting Rust MCP Server");

    // Warn about long-running requests
    if args.timeout < 60 {
        tracing::warn!(
            timeout = args.timeout,
            "Short timeout may interrupt long-running requests like cargo-build"
        );
    } else if args.timeout >= 600 {
        tracing::info!(
            timeout = args.timeout,
            "Long timeout set; some requests (e.g., cargo-build) may take a while"
        );
    }

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Rust MCP Server".to_string(),
            version: "0.1.0".to_string(),
        },
        capabilities: ServerCapabilities {
            // indicates that server support mcp tools
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default() // Using default values for other fields
        },
        meta: None,
        instructions: Some("server instructions...".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let transport = StdioTransport::new(TransportOptions {
        timeout: std::time::Duration::from_secs(args.timeout),
    })?;

    let server =
        server_runtime::create_server(server_details, transport, handler::MyServerHandler {});
    server.start().await
}
