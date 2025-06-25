use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::AllTools;

// Custom Handler to handle MCP Messages
pub struct MyServerHandler {
    disabled_tools: Vec<String>,
}

impl MyServerHandler {
    /// Create a new instance of MyServerHandler with the provided disabled tools.
    pub fn new(disabled_tools: Vec<String>) -> Self {
        Self { disabled_tools }
    }
}

// To check out a list of all the methods in the trait that you can override, take a look at
// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs
#[async_trait]
impl ServerHandler for MyServerHandler {
    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        let mut tools = AllTools::tools();
        tools.retain(|t| !self.disabled_tools.contains(&t.name));

        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools,
        })
    }

    /// Handles incoming CallToolRequest and processes it using the appropriate tool.
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        crate::tools::handle_request(request, &self.disabled_tools)
    }
}
