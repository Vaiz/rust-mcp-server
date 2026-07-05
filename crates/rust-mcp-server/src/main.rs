mod command;
mod globals;
#[cfg(feature = "http")]
mod http;
mod meta;
mod response;
mod rmcp_server;
mod serde_utils;
mod tool;
mod tools;
mod version;
mod workspace;

use clap::Parser;
use command::execute_command;
use ohno::IntoAppError;
use response::Response;
use rmcp::ServiceExt;
use rmcp::service::QuitReason;
use std::path::Path;
use tool::Tool;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};
use version::AppVersion;

const RMCP_VERSION: &str = env!("RMCP_VERSION");

#[derive(Parser, Debug)]
#[command(author, version = AppVersion, about = "Rust MCP Server", long_about = None)]
struct Args {
    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Enables logging to a file at the specified path
    #[arg(long)]
    log_file: Option<String>,

    /// Disable a tool by name. Can be specified multiple times.
    #[arg(long = "disable-tool")]
    disabled_tools: Vec<String>,

    /// Rust project workspace path. By default, uses the current directory.
    #[arg(long)]
    workspace: Option<String>,

    /// Default cargo registry to use for commands that support registry option
    #[arg(long)]
    registry: Option<String>,

    /// Generate tools.md documentation file and exit
    #[arg(long)]
    generate_docs: Option<String>,

    /// Disable experimental recommendations for agent in tool responses
    #[arg(long)]
    no_recommendations: bool,

    /// Serve over the localhost HTTP streamable transport instead of stdio.
    /// Only available when built with the `http` feature.
    #[cfg(feature = "http")]
    #[arg(long)]
    http: bool,

    /// Port for the HTTP streamable transport (used with --http).
    #[cfg(feature = "http")]
    #[arg(long, short = 'p', value_name = "PORT", default_value_t = 7270)]
    port: u16,
}

/// Number of async worker threads. The async workload is limited to MCP
/// protocol handling, so a small pool is sufficient; the heavy lifting happens
/// on the blocking pool (see [`MAX_BLOCKING_THREADS`]).
const WORKER_THREADS: usize = 2;

/// Upper bound on blocking threads. Every tool invocation (cargo, rustc,
/// rustup, etc.) runs on a blocking thread (via `spawn_blocking`), so this caps
/// how many commands execute concurrently while leaving headroom for tokio's
/// internal blocking work.
const MAX_BLOCKING_THREADS: usize = 4;

fn main() -> Result<(), ohno::AppError> {
    let args = Args::parse();

    if let Some(output_file) = args.generate_docs.as_deref() {
        let server = build_server(&args, false);
        return generate_docs(&server, output_file);
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(WORKER_THREADS)
        .max_blocking_threads(MAX_BLOCKING_THREADS)
        .thread_name("rust-mcp-server")
        .enable_all()
        .build()
        .into_app_err("Failed to build tokio runtime")?;

    runtime.block_on(run(args))
}

async fn run(args: Args) -> Result<(), ohno::AppError> {
    init_logging(&args);
    tracing::info!("Starting Rust MCP Server: {args:?}");
    tracing::info!("Server version: {}", AppVersion::version());
    tracing::info!("RMCP crate version: {RMCP_VERSION}");
    let detect_workspace = configure_globals(&args);

    #[cfg(feature = "http")]
    if args.http {
        return serve_http(&args, detect_workspace).await;
    }

    serve_stdio(build_server(&args, detect_workspace)).await
}

/// Initializes the tracing subscriber. When a log file is configured, logs are
/// written to a daily-rotating file; otherwise no subscriber is installed.
fn init_logging(args: &Args) {
    let env_filter = EnvFilter::new(&args.log_level);
    if let Some(path) = args.log_file.as_deref() {
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
}

/// Applies workspace and registry overrides to the global state, returning
/// whether workspace auto-detection should run.
fn configure_globals(args: &Args) -> bool {
    let detect_workspace = args.workspace.is_none();
    if let Some(workspace) = args.workspace.as_deref() {
        tracing::info!("Workspace root has been overridden: {workspace}");
        globals::set_workspace_root(workspace);
    } else {
        tracing::info!("No workspace root specified, workspace auto-detection enabled");
    }

    if let Some(registry) = args.registry.as_deref() {
        tracing::info!("Default cargo registry has been set: {registry}");
        globals::set_default_registry(registry.to_owned());
    }

    detect_workspace
}

/// Builds the MCP server with the tools enabled by the given arguments.
fn build_server(args: &Args, detect_workspace: bool) -> rmcp_server::Server {
    rmcp_server::Server::new(
        &args.disabled_tools,
        args.no_recommendations,
        detect_workspace,
    )
}

/// Writes the generated tools documentation to `output_file`.
fn generate_docs(server: &rmcp_server::Server, output_file: &str) -> Result<(), ohno::AppError> {
    println!("Generating documentation to: {output_file}");
    let docs = server.generate_markdown_docs();
    std::fs::write(output_file, docs).into_app_err("Failed to write documentation file")?;
    println!("Documentation generated successfully: {output_file}");
    Ok(())
}

/// Serves the MCP server over the localhost HTTP streamable transport.
#[cfg(feature = "http")]
async fn serve_http(args: &Args, detect_workspace: bool) -> Result<(), ohno::AppError> {
    use std::net::{Ipv4Addr, SocketAddr};

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, args.port));
    let disabled_tools = args.disabled_tools.clone();
    let no_recommendations = args.no_recommendations;
    let factory = move || {
        Ok(rmcp_server::Server::new(
            &disabled_tools,
            no_recommendations,
            detect_workspace,
        ))
    };
    tracing::info!("Starting HTTP transport on {addr}");
    eprintln!("Rust MCP Server started on http://{addr}/");
    http::serve(addr, factory)
        .await
        .into_app_err("HTTP server failed")
}

/// Serves the MCP server over stdio and waits for it to quit.
async fn serve_stdio(server: rmcp_server::Server) -> Result<(), ohno::AppError> {
    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .into_app_err("Failed to start server")?;

    eprintln!("Rust MCP Server started on stdio");

    match service.waiting().await {
        Ok(QuitReason::Closed) => tracing::info!("Server closed normally"),
        Ok(QuitReason::Cancelled) => tracing::info!("Server was cancelled"),
        Ok(QuitReason::JoinError(error)) => {
            tracing::error!("Server join error: {error}");
            return Err(error.into());
        }
        Ok(reason) => {
            tracing::info!("Server exited with reason: {reason:?}");
        }
        Err(error) => {
            tracing::error!("Server encountered an error: {error}");
            return Err(error.into());
        }
    }

    Ok(())
}
