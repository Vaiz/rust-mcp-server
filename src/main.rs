use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{server_runtime, ServerHandler, ServerRuntime},
    schema::{
        Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
        LATEST_PROTOCOL_VERSION,
    },
    StdioTransport, TransportOptions,
};
use rust_mcp_sdk::McpServer;

#[tokio::main]
async fn main() -> SdkResult<()> {
    // STEP 1: Define server details and capabilities
    let server_details = InitializeResult {
        // server name and version
        server_info: Implementation {
            name: "Hello World MCP Server".to_string(),
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

    // STEP 2: create a std transport with default options
    let transport = StdioTransport::new(TransportOptions::default())?;

    // STEP 3: instantiate our custom handler for handling MCP messages
    let handler = MyServerHandler {};

    // STEP 4: create a MCP server
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    // STEP 5: Start the server
    server.start().await
}

struct MyServerHandler;

#[async_trait::async_trait]
impl ServerHandler for MyServerHandler {
    // Implement the required methods for handling MCP messages
    // For example, handle initialization, requests, notifications, etc.
    async fn on_initialized(&self, _runtime: &dyn McpServer) {
        println!("Server initialized successfully!");
    }

}