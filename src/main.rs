#![allow(unused, dead_code)] // FIXME: Remove this when the codebase is more complete.

mod rmcp_server;
pub(crate) mod serde_utils;
mod tool;
mod tools;
mod version;

use anyhow::Context;
use clap::Parser;
use rmcp::ServiceExt;
use rmcp::service::QuitReason;
pub(crate) use tool::{ToolImpl, execute_rmcp_command};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};
use version::AppVersion;

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

    /// Generate tools.md documentation file and exit
    #[arg(long)]
    generate_docs: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    tracing::info!("Starting Rust MCP Server: {:?}", args);

    // Warn about long-running requests
    if args.timeout < 60 {
        tracing::warn!(
            "Short timeout ({}) may interrupt long-running requests like cargo-build",
            args.timeout
        );
    } else if args.timeout >= 600 {
        tracing::info!(
            "Long timeout ({}) set; some requests (e.g., cargo-build) may take a while",
            args.timeout
        );
    }

    tracing::info!("Server version: {}", AppVersion::version());

    if let Some(workspace) = args.workspace {
        tracing::info!("Workspace root has been overridden: {}", workspace);
        tools::set_workspace_root(workspace);
    } else {
        tracing::info!("No workspace root specified, using current directory");
    }

    let server = rmcp_server::Server::new(&args.disabled_tools);

    // Handle documentation generation mode
    if let Some(output_file) = args.generate_docs {
        tracing::info!("Generating documentation to: {}", output_file);
        let docs = server.generate_markdown_docs();
        std::fs::write(&output_file, docs).context("Failed to write documentation file")?;
        println!("Documentation generated successfully: {}", output_file);
        return Ok(());
    }

    /// FIXME: How to pass timeout to StdioTransport?
    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .context("Failed to start server")?;

    eprintln!("Precisely File Editor MCP Server started on stdio");

    // Keep the service running until cancelled
    let result = service.waiting().await;

    match result {
        Ok(QuitReason::Closed) => tracing::info!("Server closed normally"),
        Ok(QuitReason::Cancelled) => tracing::info!("Server was cancelled"),
        Ok(QuitReason::JoinError(error)) => {
            tracing::error!("Server join error: {error}");
            return Err(error.into());
        }
        Err(error) => {
            tracing::error!("Server encountered an error: {error}");
            return Err(error.into());
        }
    }

    Ok(())
}
