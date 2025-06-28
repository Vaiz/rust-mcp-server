mod handler;
mod prompts;
pub(crate) mod serde_utils;
mod tools;

use clap::Parser;
use rust_mcp_sdk::McpServer;
use rust_mcp_sdk::schema::ServerCapabilitiesPrompts;
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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

struct AppVersion;

impl AppVersion {
    fn version() -> String {
        match GIT_HASH {
            Some(hash) => format!("{VERSION}.{hash}"),
            None => VERSION.into(),
        }
    }
}

impl From<AppVersion> for clap::builder::Str {
    fn from(_: AppVersion) -> Self {
        AppVersion::version().into()
    }
}

#[derive(Parser, Debug)]
#[command(author, version = AppVersion, about = "Rust MCP Server", long_about = None)]
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

    /// Disable a tool by name. Can be specified multiple times.
    #[arg(long = "disable-tool")]
    disabled_tools: Vec<String>,

    /// Rust project workspace path. By default, uses the current directory.
    #[arg(long)]
    workspace: Option<String>,
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

    tracing::info!(version = AppVersion::version(), "Server version");

    if let Some(workspace) = args.workspace {
        tracing::info!(workspace = %workspace, "Workspace root has been overridden");
        tools::set_workspace_root(workspace);
    } else {
        tracing::info!("No workspace root specified, using current directory");
    }

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Rust MCP Server".into(),
            version: AppVersion::version(),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            prompts: Some(ServerCapabilitiesPrompts { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some(include_str!("../docs/instructions.md").into()),
        protocol_version: LATEST_PROTOCOL_VERSION.into(),
    };

    let transport = StdioTransport::new(TransportOptions {
        timeout: std::time::Duration::from_secs(args.timeout),
    })?;

    let server = server_runtime::create_server(
        server_details,
        transport,
        handler::McpServerHandler::new(args.disabled_tools),
    );
    server.start().await
}
