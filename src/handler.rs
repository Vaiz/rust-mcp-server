use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListPromptsRequest, ListPromptsResult, ListToolsRequest,
    ListToolsResult, RpcError, schema_utils::CallToolError,
};
use rust_mcp_sdk::schema::{GetPromptRequest, GetPromptResult};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::AllTools;

// Custom Handler to handle MCP Messages
pub struct MyServerHandler {
    disabled_tools: Vec<String>,
    prompt_handler: crate::prompts::PromptHandler,
}

impl MyServerHandler {
    /// Create a new instance of MyServerHandler with the provided disabled tools.
    pub fn new(disabled_tools: Vec<String>) -> Self {
        let this = Self {
            disabled_tools,
            prompt_handler: crate::prompts::PromptHandler::new(),
        };
        let enabled_tools = this.enabled_tools();
        tracing::info!(enabled_tools = ?enabled_tools, disabled_tools = ?this.disabled_tools, "Staring MCP Server");
        this
    }

    fn enabled_tools(&self) -> Vec<String> {
        AllTools::tools()
            .into_iter()
            .filter(|t| !self.disabled_tools.contains(&t.name))
            .map(|t| t.name)
            .collect()
    }
}

// To check out a list of all the methods in the trait that you can override, take a look at
// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs
#[async_trait]
impl ServerHandler for MyServerHandler {
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

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        crate::tools::handle_request(request, &self.disabled_tools)
    }

    async fn handle_list_prompts_request(
        &self,
        _request: ListPromptsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListPromptsResult, RpcError> {
        Ok(ListPromptsResult {
            meta: None,
            next_cursor: None,
            prompts: self.prompt_handler.list_prompts(),
        })
    }

    async fn handle_get_prompt_request(
        &self,
        request: GetPromptRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<GetPromptResult, RpcError> {
        let name = &request.params.name;
        tracing::info!(name, "Handling get prompt request");

        if let Some(result) = self.prompt_handler.get_prompt_result(name) {
            return Ok(result.clone());
        }

        tracing::warn!(name, "Prompt not found");
        runtime.assert_server_request_capabilities(request.method())?;
        Err(RpcError::method_not_found().with_message(format!(
            "Prompt not found for '{name}'."            
        )))
    }
}
