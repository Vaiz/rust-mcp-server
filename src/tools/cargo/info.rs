use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::deserialize_string;
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-info",
    description = "Display information about a package. Equivalent to 'cargo info <SPEC>'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoInfoTool {
    /// Package to inspect
    pub spec: String,
    /// Registry to search packages in
    #[serde(deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
}

impl CargoInfoTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("info");
        cmd.arg(&self.spec);

        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }
        execute_command(cmd)
    }
}
