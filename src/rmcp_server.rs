use std::{collections::HashMap, sync::Arc};

use rmcp::{
    ErrorData,
    model::{ListToolsResult, PaginatedRequestParam, ServerInfo},
    service::RequestContext,
};

use crate::{ToolImpl, tool::Tool, tools::cargo::CargoCheckRmcpTool, version::AppVersion};

pub struct Server {
    tools: HashMap<&'static str, Box<dyn Tool + Send + Sync>>,
}

impl Server {
    pub fn new() -> Self {
        let mut tools: HashMap<&'static str, Box<dyn Tool + Send + Sync>> = HashMap::new();
        tools.insert(CargoCheckRmcpTool::NAME, Box::new(CargoCheckRmcpTool));
        Self { tools }
    }
}

impl rmcp::ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        use rmcp::model::{
            Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ToolsCapability,
        };

        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "Rust MCP Server".to_owned(),
                version: AppVersion::version(),
                icons: None,
                title: Some("Rust MCP Server".to_owned()),
                website_url: Some("https://github.com/Vaiz/rust-mcp-server".to_owned()),
            },
            instructions: Some(include_str!("../docs/instructions.md").to_owned()),
        }
    }
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let mut tools: Vec<rmcp::model::Tool> = Vec::new();

        for tool in self.tools.values() {
            let schema = Arc::new(tool.json_schema());
            tools.push(rmcp::model::Tool {
                name: tool.name().into(),
                title: Some(tool.title().into()),
                description: Some(tool.description().into()),
                input_schema: schema,
                output_schema: None,
                annotations: None,
                icons: None,
                meta: None,
            });
        }

        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let tool = self.tools.get(request.name.as_ref()).ok_or_else(|| {
            ErrorData::invalid_request(format!("Tool '{}' not found", request.name), None)
        })?;

        tool.call_rmcp_tool(request)
    }
}
