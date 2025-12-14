use std::{collections::HashMap, sync::Arc};

use rmcp::{
    ErrorData,
    model::{ListToolsResult, PaginatedRequestParam, ServerInfo},
    service::RequestContext,
};

use crate::{
    Tool,
    tool::DynTool,
    tools::{
        cargo::{
            CargoAddRmcpTool, CargoBuildRmcpTool, CargoCheckRmcpTool, CargoCleanRmcpTool,
            CargoClippyRmcpTool, CargoDocRmcpTool, CargoFmtRmcpTool, CargoGenerateLockfileRmcpTool,
            CargoInfoRmcpTool, CargoListRmcpTool, CargoMetadataRmcpTool, CargoNewRmcpTool,
            CargoPackageRmcpTool, CargoRemoveRmcpTool, CargoSearchRmcpTool, CargoTestRmcpTool,
            CargoUpdateRmcpTool, CargoWorkspaceInfoRmcpTool,
        },
        cargo_deny::{
            CargoDenyCheckRmcpTool, CargoDenyInitRmcpTool, CargoDenyInstallRmcpTool,
            CargoDenyListRmcpTool,
        },
        cargo_hack::{CargoHackInstallRmcpTool, CargoHackRmcpTool},
        cargo_machete::{CargoMacheteInstallRmcpTool, CargoMacheteRmcpTool},
        rustc::RustcExplainRmcpTool,
        rustup::{RustupShowRmcpTool, RustupToolchainAddRmcpTool, RustupUpdateRmcpTool},
    },
    version::AppVersion,
};

pub struct Server {
    tools: HashMap<&'static str, Box<dyn DynTool + Send + Sync>>,
}

impl Server {
    pub fn new(disabled_tools: &[String]) -> Self {
        let mut tools: HashMap<&'static str, Box<dyn DynTool + Send + Sync>> = HashMap::new();

        // Cargo tools
        tools.insert(CargoAddRmcpTool::NAME, Box::new(CargoAddRmcpTool));
        tools.insert(CargoBuildRmcpTool::NAME, Box::new(CargoBuildRmcpTool));
        tools.insert(CargoCheckRmcpTool::NAME, Box::new(CargoCheckRmcpTool));
        tools.insert(CargoCleanRmcpTool::NAME, Box::new(CargoCleanRmcpTool));
        tools.insert(CargoClippyRmcpTool::NAME, Box::new(CargoClippyRmcpTool));
        tools.insert(CargoDocRmcpTool::NAME, Box::new(CargoDocRmcpTool));
        tools.insert(CargoFmtRmcpTool::NAME, Box::new(CargoFmtRmcpTool));
        tools.insert(
            CargoGenerateLockfileRmcpTool::NAME,
            Box::new(CargoGenerateLockfileRmcpTool),
        );
        tools.insert(CargoInfoRmcpTool::NAME, Box::new(CargoInfoRmcpTool));
        tools.insert(CargoListRmcpTool::NAME, Box::new(CargoListRmcpTool));
        tools.insert(CargoMetadataRmcpTool::NAME, Box::new(CargoMetadataRmcpTool));
        tools.insert(CargoNewRmcpTool::NAME, Box::new(CargoNewRmcpTool));
        tools.insert(CargoPackageRmcpTool::NAME, Box::new(CargoPackageRmcpTool));
        tools.insert(CargoRemoveRmcpTool::NAME, Box::new(CargoRemoveRmcpTool));
        tools.insert(CargoSearchRmcpTool::NAME, Box::new(CargoSearchRmcpTool));
        tools.insert(CargoTestRmcpTool::NAME, Box::new(CargoTestRmcpTool));
        tools.insert(CargoUpdateRmcpTool::NAME, Box::new(CargoUpdateRmcpTool));
        tools.insert(
            CargoWorkspaceInfoRmcpTool::NAME,
            Box::new(CargoWorkspaceInfoRmcpTool),
        );

        // Cargo-deny tools
        tools.insert(
            CargoDenyCheckRmcpTool::NAME,
            Box::new(CargoDenyCheckRmcpTool),
        );
        tools.insert(CargoDenyInitRmcpTool::NAME, Box::new(CargoDenyInitRmcpTool));
        tools.insert(
            CargoDenyInstallRmcpTool::NAME,
            Box::new(CargoDenyInstallRmcpTool),
        );
        tools.insert(CargoDenyListRmcpTool::NAME, Box::new(CargoDenyListRmcpTool));

        // Cargo-hack tools
        tools.insert(CargoHackRmcpTool::NAME, Box::new(CargoHackRmcpTool));
        tools.insert(
            CargoHackInstallRmcpTool::NAME,
            Box::new(CargoHackInstallRmcpTool),
        );

        // Cargo-machete tools
        tools.insert(CargoMacheteRmcpTool::NAME, Box::new(CargoMacheteRmcpTool));
        tools.insert(
            CargoMacheteInstallRmcpTool::NAME,
            Box::new(CargoMacheteInstallRmcpTool),
        );

        // Rustc tools
        tools.insert(RustcExplainRmcpTool::NAME, Box::new(RustcExplainRmcpTool));

        // Rustup tools
        tools.insert(RustupShowRmcpTool::NAME, Box::new(RustupShowRmcpTool));
        tools.insert(
            RustupToolchainAddRmcpTool::NAME,
            Box::new(RustupToolchainAddRmcpTool),
        );
        tools.insert(RustupUpdateRmcpTool::NAME, Box::new(RustupUpdateRmcpTool));

        if !disabled_tools.is_empty() {
            tracing::info!("Disabled tools: {}", disabled_tools.join(", "));
            for tool_name in disabled_tools {
                if tools.remove(tool_name.as_str()).is_none() {
                    tracing::warn!("Tool not found: {}", tool_name);
                }
            }
        }

        Self { tools }
    }

    /// Generate markdown documentation for all tools
    pub fn generate_markdown_docs(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("## Rust MCP Server\n");
        output.push_str(&format!("| 🟢 Tools ({}) | 🟢 Prompts (0) | 🟢 Resources (0) | <span style=\"opacity:0.6\">🔴 Logging</span> | <span style=\"opacity:0.6\">🔴 Completions</span> | <span style=\"opacity:0.6\">🔴 Experimental</span> |\n", self.tools.len()));
        output.push_str("| --- | --- | --- | --- | --- | --- |\n\n");

        // Tools section
        output.push_str(&format!("## 🛠️ Tools ({})\n\n\n", self.tools.len()));

        // Sort tools by name for consistent output
        let mut tool_names: Vec<&str> = self.tools.keys().copied().collect();
        tool_names.sort();

        for tool_name in tool_names {
            let tool = &self.tools[tool_name];
            output.push_str(&format!("- **{}**\n", tool.name()));
            output.push_str(&format!("  - {}\n", tool.description()));

            let schema = tool.json_schema();
            if let Some(serde_json::Value::Object(properties)) = schema.get("properties")
                && !properties.is_empty()
            {
                output.push_str("  - **Inputs:**\n");

                // Sort properties for consistent output
                let mut prop_names: Vec<&String> = properties.keys().collect();
                prop_names.sort();

                for prop_name in prop_names {
                    let prop = &properties[prop_name];
                    let type_str = self.format_property_type(prop);
                    output.push_str(&format!(
                        "      - <code>{}</code> : {}<br />\n",
                        prop_name, type_str
                    ));
                }
            }
            output.push('\n');
        }

        output.pop();
        output
    }

    fn format_property_type(&self, prop: &serde_json::Value) -> String {
        if let Some(type_val) = prop.get("type") {
            match type_val.as_str() {
                Some("array") => {
                    if let Some(items) = prop.get("items")
                        && let Some(item_type) = items.get("type")
                    {
                        return format!("{} [ ]", item_type.as_str().unwrap_or("unknown"));
                    }
                    "array".to_string()
                }
                Some(type_str) => type_str.to_string(),
                None => "unknown".to_string(),
            }
        } else {
            "unknown".to_string()
        }
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
                description: Some(tool.description().trim().trim_matches('\n').into()),
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
