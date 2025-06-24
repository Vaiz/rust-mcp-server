use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::AllTools;

// Custom Handler to handle MCP Messages
pub struct MyServerHandler;

// To check out a list of all the methods in the trait that you can override, take a look at
// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs

#[async_trait]
#[allow(unused)]
impl ServerHandler for MyServerHandler {
    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        let mut tools = AllTools::tools();

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
        runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        // Attempt to convert request parameters into GreetingTools enum
        let tool_params: AllTools =
            AllTools::try_from(request.params).map_err(CallToolError::new)?;

        // Match the tool variant and execute its corresponding logic
        match tool_params {
            AllTools::SayHelloTool(say_hello_tool) => say_hello_tool.call_tool(),
            AllTools::SayGoodbyeTool(say_goodbye_tool) => say_goodbye_tool.call_tool(),
            AllTools::CargoBuildTool(cargo_build_tool) => cargo_build_tool.call_tool(),
        }
    }
}
