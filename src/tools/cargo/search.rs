use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::deserialize_string;
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-search",
    description = "Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search [QUERY]'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoSearchTool {
    /// The query to search for. Generally, this is a substring of the package name or description.
    pub query: String,
    /// Limit the number of results (default: 10, max: 100)
    pub limit: Option<u32>,
    /// Registry to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
}

impl CargoSearchTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("search");
        cmd.arg(&self.query);
        if let Some(limit) = self.limit {
            cmd.arg("--limit").arg(limit.to_string());
        }
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }
        execute_command(cmd)
    }
}
