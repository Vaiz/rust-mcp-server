use std::{collections::HashMap, sync::Arc};

use rmcp::{
    ErrorData,
    model::{ListToolsResult, PaginatedRequestParam},
    service::RequestContext,
};

use crate::{ToolImpl, tool::Tool, tools::cargo::CargoCheckRmcpTool};

struct Server {
    tools: HashMap<&'static str, Box<dyn Tool + Send + Sync>>,
}

impl Server {
    fn new() -> Self {
        let mut tools: HashMap<&'static str, Box<dyn Tool + Send + Sync>> = HashMap::new();
        tools.insert(CargoCheckRmcpTool::NAME, Box::new(CargoCheckRmcpTool));
        Self { tools }
    }
}

impl rmcp::ServerHandler for Server {
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
}
