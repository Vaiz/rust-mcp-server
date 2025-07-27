use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{deserialize_string, output_verbosity_to_cli_flags};
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-search",
    description = "Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search [QUERY]'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, JsonSchema)]
pub struct CargoSearchTool {
    /// The query to search for. Generally, this is a substring of the package name or description.
    pub query: String,
    /// Limit the number of results (default: 10, max: 100)
    pub limit: Option<u32>,
    /// Registry to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
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
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);
        execute_command(cmd)
    }
}
