use std::{collections::HashMap, sync::Arc};

use rmcp::{
    ErrorData,
    model::{ListToolsResult, PaginatedRequestParams, ServerInfo},
    service::{NotificationContext, RequestContext},
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
            CargoTreeRmcpTool, CargoUpdateRmcpTool, CargoWorkspaceInfoRmcpTool,
        },
        cargo_deny::{
            CargoDenyCheckRmcpTool, CargoDenyInitRmcpTool, CargoDenyInstallRmcpTool,
            CargoDenyListRmcpTool,
        },
        cargo_expand::CargoExpandRmcpTool,
        cargo_hack::{CargoHackInstallRmcpTool, CargoHackRmcpTool},
        cargo_insta::CargoInstaUpdateSnapshotsRmcpTool,
        cargo_machete::{CargoMacheteInstallRmcpTool, CargoMacheteRmcpTool},
        rustc::RustcExplainRmcpTool,
        rustup::{RustupShowRmcpTool, RustupToolchainAddRmcpTool, RustupUpdateRmcpTool},
    },
    version::AppVersion,
};

pub struct Server {
    ignore_recommendations: bool,
    detect_workspace: bool,
    tools: HashMap<&'static str, Arc<dyn DynTool + Send + Sync>>,
}

impl Server {
    pub fn new(
        disabled_tools: &[String],
        ignore_recommendations: bool,
        detect_workspace: bool,
    ) -> Self {
        let mut tools: HashMap<&'static str, Arc<dyn DynTool + Send + Sync>> = HashMap::new();

        // Cargo tools
        tools.insert(CargoAddRmcpTool::NAME, Arc::new(CargoAddRmcpTool));
        tools.insert(CargoBuildRmcpTool::NAME, Arc::new(CargoBuildRmcpTool));
        tools.insert(CargoCheckRmcpTool::NAME, Arc::new(CargoCheckRmcpTool));
        tools.insert(CargoCleanRmcpTool::NAME, Arc::new(CargoCleanRmcpTool));
        tools.insert(CargoClippyRmcpTool::NAME, Arc::new(CargoClippyRmcpTool));
        tools.insert(CargoDocRmcpTool::NAME, Arc::new(CargoDocRmcpTool));
        tools.insert(CargoExpandRmcpTool::NAME, Arc::new(CargoExpandRmcpTool));
        tools.insert(CargoFmtRmcpTool::NAME, Arc::new(CargoFmtRmcpTool));
        tools.insert(
            CargoGenerateLockfileRmcpTool::NAME,
            Arc::new(CargoGenerateLockfileRmcpTool),
        );
        tools.insert(CargoInfoRmcpTool::NAME, Arc::new(CargoInfoRmcpTool));
        tools.insert(CargoListRmcpTool::NAME, Arc::new(CargoListRmcpTool));
        tools.insert(CargoMetadataRmcpTool::NAME, Arc::new(CargoMetadataRmcpTool));
        tools.insert(CargoNewRmcpTool::NAME, Arc::new(CargoNewRmcpTool));
        tools.insert(CargoPackageRmcpTool::NAME, Arc::new(CargoPackageRmcpTool));
        tools.insert(CargoRemoveRmcpTool::NAME, Arc::new(CargoRemoveRmcpTool));
        tools.insert(CargoSearchRmcpTool::NAME, Arc::new(CargoSearchRmcpTool));
        tools.insert(CargoTestRmcpTool::NAME, Arc::new(CargoTestRmcpTool));
        tools.insert(CargoTreeRmcpTool::NAME, Arc::new(CargoTreeRmcpTool));
        tools.insert(CargoUpdateRmcpTool::NAME, Arc::new(CargoUpdateRmcpTool));
        tools.insert(
            CargoWorkspaceInfoRmcpTool::NAME,
            Arc::new(CargoWorkspaceInfoRmcpTool),
        );

        // Cargo-deny tools
        tools.insert(
            CargoDenyCheckRmcpTool::NAME,
            Arc::new(CargoDenyCheckRmcpTool),
        );
        tools.insert(CargoDenyInitRmcpTool::NAME, Arc::new(CargoDenyInitRmcpTool));
        tools.insert(
            CargoDenyInstallRmcpTool::NAME,
            Arc::new(CargoDenyInstallRmcpTool),
        );
        tools.insert(CargoDenyListRmcpTool::NAME, Arc::new(CargoDenyListRmcpTool));

        // Cargo-hack tools
        tools.insert(CargoHackRmcpTool::NAME, Arc::new(CargoHackRmcpTool));
        tools.insert(
            CargoHackInstallRmcpTool::NAME,
            Arc::new(CargoHackInstallRmcpTool),
        );

        // Cargo-insta tools
        tools.insert(
            CargoInstaUpdateSnapshotsRmcpTool::NAME,
            Arc::new(CargoInstaUpdateSnapshotsRmcpTool),
        );

        // Cargo-machete tools
        tools.insert(CargoMacheteRmcpTool::NAME, Arc::new(CargoMacheteRmcpTool));
        tools.insert(
            CargoMacheteInstallRmcpTool::NAME,
            Arc::new(CargoMacheteInstallRmcpTool),
        );

        // Rustc tools
        tools.insert(RustcExplainRmcpTool::NAME, Arc::new(RustcExplainRmcpTool));

        // Rustup tools
        tools.insert(RustupShowRmcpTool::NAME, Arc::new(RustupShowRmcpTool));
        tools.insert(
            RustupToolchainAddRmcpTool::NAME,
            Arc::new(RustupToolchainAddRmcpTool),
        );
        tools.insert(RustupUpdateRmcpTool::NAME, Arc::new(RustupUpdateRmcpTool));

        if !disabled_tools.is_empty() {
            tracing::info!("Disabled tools: {}", disabled_tools.join(", "));
            for tool_name in disabled_tools {
                if tools.remove(tool_name.as_str()).is_none() {
                    tracing::warn!("Tool not found: {}", tool_name);
                }
            }
        }

        Self {
            ignore_recommendations,
            detect_workspace,
            tools,
        }
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

        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        let mut server_info = Implementation::default();
        server_info.name = "Rust MCP Server".to_owned();
        server_info.title = Some("Rust MCP Server".to_owned());
        server_info.description = Some(
            "Provides access to cargo, rustc, rustup, and other Rust-related tools via the MCP protocol"
                .to_owned(),
        );
        server_info.version = AppVersion::version();
        server_info.website_url = Some("https://github.com/Vaiz/rust-mcp-server".to_owned());

        let mut result = InitializeResult::default();
        result.protocol_version = ProtocolVersion::LATEST;
        result.capabilities = capabilities;
        result.server_info = server_info;
        result.instructions = Some(include_str!("../docs/instructions.md").to_owned());
        result
    }

    async fn on_initialized(&self, context: NotificationContext<rmcp::RoleServer>) {
        tracing::info!("MCP client initialized");
        if self.detect_workspace {
            crate::workspace::detect_rust_workspace(context);
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let mut tools: Vec<rmcp::model::Tool> = Vec::new();
        // currently none of the tools support tasking
        let execution = rmcp::model::ToolExecution::new()
            .with_task_support(rmcp::model::TaskSupport::Forbidden);

        for tool in self.tools.values() {
            let schema = Arc::new(tool.json_schema());
            let mut tool_def = rmcp::model::Tool::default();
            tool_def.name = tool.name().into();
            tool_def.title = Some(tool.title().into());
            tool_def.description = Some(tool.description().trim().trim_matches('\n').into());
            tool_def.input_schema = schema;
            tool_def.execution = Some(execution.clone());
            tools.push(tool_def);
        }

        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let (&tool_name, tool) =
            self.tools
                .get_key_value(request.name.as_ref())
                .ok_or_else(|| {
                    ErrorData::invalid_request(format!("Tool '{}' not found", request.name), None)
                })?;
        let tool = Arc::clone(tool);
        let ignore_recommendations = self.ignore_recommendations;

        tokio::task::spawn_blocking(move || {
            tool.call_rmcp_tool(request)
                .map(|r| r.into_rmcp_result(ignore_recommendations))
        })
        .await
        .map_err(|e| {
            ErrorData::internal_error(
                format!("Tool execution task failed for '{tool_name}': {e}"),
                None,
            )
        })?
    }
}
